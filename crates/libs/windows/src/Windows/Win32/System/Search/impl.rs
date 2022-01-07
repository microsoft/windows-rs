pub trait DataSourceImpl: Sized {
    fn getDataMember();
    fn getDataMemberName();
    fn getDataMemberCount();
    fn addDataSourceListener();
    fn removeDataSourceListener();
}
impl ::windows::core::RuntimeName for DataSource {
    const NAME: &'static str = "Windows.Win32.System.Search.DataSource";
}
impl DataSourceVtbl {
    pub const fn new<Impl: DataSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> DataSourceVtbl {
        unsafe extern "system" fn getDataMember<Impl: DataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getDataMember(bstrdm, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberName<Impl: DataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrdm: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getDataMemberName(lindex, ::core::mem::transmute_copy(&pbstrdm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberCount<Impl: DataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getDataMemberCount(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addDataSourceListener<Impl: DataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addDataSourceListener(&*(&pdsl as *const <DataSourceListener as ::windows::core::Abi>::Abi as *const <DataSourceListener as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeDataSourceListener<Impl: DataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeDataSourceListener(&*(&pdsl as *const <DataSourceListener as ::windows::core::Abi>::Abi as *const <DataSourceListener as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<DataSource>, base.5, getDataMember::<Impl, OFFSET>, getDataMemberName::<Impl, OFFSET>, getDataMemberCount::<Impl, OFFSET>, addDataSourceListener::<Impl, OFFSET>, removeDataSourceListener::<Impl, OFFSET>)
    }
}
pub trait DataSourceListenerImpl: Sized {
    fn dataMemberChanged();
    fn dataMemberAdded();
    fn dataMemberRemoved();
}
impl ::windows::core::RuntimeName for DataSourceListener {
    const NAME: &'static str = "Windows.Win32.System.Search.DataSourceListener";
}
impl DataSourceListenerVtbl {
    pub const fn new<Impl: DataSourceListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> DataSourceListenerVtbl {
        unsafe extern "system" fn dataMemberChanged<Impl: DataSourceListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dataMemberChanged(bstrdm) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dataMemberAdded<Impl: DataSourceListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dataMemberAdded(bstrdm) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dataMemberRemoved<Impl: DataSourceListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dataMemberRemoved(bstrdm) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<DataSourceListener>, base.5, dataMemberChanged::<Impl, OFFSET>, dataMemberAdded::<Impl, OFFSET>, dataMemberRemoved::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DataSourceObjectImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DataSourceObject {
    const NAME: &'static str = "Windows.Win32.System.Search.DataSourceObject";
}
#[cfg(feature = "Win32_System_Com")]
impl DataSourceObjectVtbl {
    pub const fn new<Impl: DataSourceObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> DataSourceObjectVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<DataSourceObject>, base.5)
    }
}
pub trait IAccessorImpl: Sized {
    fn AddRefAccessor();
    fn CreateAccessor();
    fn GetBindings();
    fn ReleaseAccessor();
}
impl ::windows::core::RuntimeName for IAccessor {
    const NAME: &'static str = "Windows.Win32.System.Search.IAccessor";
}
impl IAccessorVtbl {
    pub const fn new<Impl: IAccessorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccessorVtbl {
        unsafe extern "system" fn AddRefAccessor<Impl: IAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRefAccessor(haccessor, ::core::mem::transmute_copy(&pcrefcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccessor<Impl: IAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut usize, rgstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAccessor(dwaccessorflags, cbindings, &*(&rgbindings as *const <DBBINDING as ::windows::core::Abi>::Abi as *const <DBBINDING as ::windows::core::DefaultType>::DefaultType), cbrowsize, ::core::mem::transmute_copy(&phaccessor), ::core::mem::transmute_copy(&rgstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindings<Impl: IAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBindings(haccessor, ::core::mem::transmute_copy(&pdwaccessorflags), ::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAccessor<Impl: IAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseAccessor(haccessor, ::core::mem::transmute_copy(&pcrefcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccessor>, base.5, AddRefAccessor::<Impl, OFFSET>, CreateAccessor::<Impl, OFFSET>, GetBindings::<Impl, OFFSET>, ReleaseAccessor::<Impl, OFFSET>)
    }
}
pub trait IAlterIndexImpl: Sized {
    fn AlterIndex();
}
impl ::windows::core::RuntimeName for IAlterIndex {
    const NAME: &'static str = "Windows.Win32.System.Search.IAlterIndex";
}
impl IAlterIndexVtbl {
    pub const fn new<Impl: IAlterIndexImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAlterIndexVtbl {
        unsafe extern "system" fn AlterIndex<Impl: IAlterIndexImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pindexid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlterIndex(
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pnewindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAlterIndex>, base.5, AlterIndex::<Impl, OFFSET>)
    }
}
pub trait IAlterTableImpl: Sized {
    fn AlterColumn();
    fn AlterTable();
}
impl ::windows::core::RuntimeName for IAlterTable {
    const NAME: &'static str = "Windows.Win32.System.Search.IAlterTable";
}
impl IAlterTableVtbl {
    pub const fn new<Impl: IAlterTableImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAlterTableVtbl {
        unsafe extern "system" fn AlterColumn<Impl: IAlterTableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pcolumnid: *mut super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *mut DBCOLUMNDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlterColumn(
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pcolumnid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                dwcolumndescflags,
                &*(&pcolumndesc as *const <DBCOLUMNDESC as ::windows::core::Abi>::Abi as *const <DBCOLUMNDESC as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlterTable<Impl: IAlterTableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlterTable(
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pnewtableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAlterTable>, base.5, AlterColumn::<Impl, OFFSET>, AlterTable::<Impl, OFFSET>)
    }
}
pub trait IBindResourceImpl: Sized {
    fn Bind();
}
impl ::windows::core::RuntimeName for IBindResource {
    const NAME: &'static str = "Windows.Win32.System.Search.IBindResource";
}
impl IBindResourceVtbl {
    pub const fn new<Impl: IBindResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindResourceVtbl {
        unsafe extern "system" fn Bind<Impl: IBindResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: ::windows::core::RawPtr, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bind(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwbindurlflags,
                &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pauthenticate as *const <super::Com::IAuthenticate as ::windows::core::Abi>::Abi as *const <super::Com::IAuthenticate as ::windows::core::DefaultType>::DefaultType),
                &*(&pimplsession as *const <DBIMPLICITSESSION as ::windows::core::Abi>::Abi as *const <DBIMPLICITSESSION as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwbindstatus),
                ::core::mem::transmute_copy(&ppunk),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindResource>, base.5, Bind::<Impl, OFFSET>)
    }
}
pub trait IChapteredRowsetImpl: Sized {
    fn AddRefChapter();
    fn ReleaseChapter();
}
impl ::windows::core::RuntimeName for IChapteredRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IChapteredRowset";
}
impl IChapteredRowsetVtbl {
    pub const fn new<Impl: IChapteredRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChapteredRowsetVtbl {
        unsafe extern "system" fn AddRefChapter<Impl: IChapteredRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRefChapter(hchapter, ::core::mem::transmute_copy(&pcrefcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseChapter<Impl: IChapteredRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseChapter(hchapter, ::core::mem::transmute_copy(&pcrefcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IChapteredRowset>, base.5, AddRefChapter::<Impl, OFFSET>, ReleaseChapter::<Impl, OFFSET>)
    }
}
pub trait IColumnMapperImpl: Sized {
    fn GetPropInfoFromName();
    fn GetPropInfoFromId();
    fn EnumPropInfo();
    fn IsMapUpToDate();
}
impl ::windows::core::RuntimeName for IColumnMapper {
    const NAME: &'static str = "Windows.Win32.System.Search.IColumnMapper";
}
impl IColumnMapperVtbl {
    pub const fn new<Impl: IColumnMapperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColumnMapperVtbl {
        unsafe extern "system" fn GetPropInfoFromName<Impl: IColumnMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wcspropname: super::super::Foundation::PWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropInfoFromName(&*(&wcspropname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pppropid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), pproptype, puiwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropInfoFromId<Impl: IColumnMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropInfoFromId(&*(&ppropid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), pwcsname, pproptype, puiwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPropInfo<Impl: IColumnMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumPropInfo(ientry, pwcsname, &*(&pppropid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), pproptype, puiwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMapUpToDate<Impl: IColumnMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMapUpToDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColumnMapper>, base.5, GetPropInfoFromName::<Impl, OFFSET>, GetPropInfoFromId::<Impl, OFFSET>, EnumPropInfo::<Impl, OFFSET>, IsMapUpToDate::<Impl, OFFSET>)
    }
}
pub trait IColumnMapperCreatorImpl: Sized {
    fn GetColumnMapper();
}
impl ::windows::core::RuntimeName for IColumnMapperCreator {
    const NAME: &'static str = "Windows.Win32.System.Search.IColumnMapperCreator";
}
impl IColumnMapperCreatorVtbl {
    pub const fn new<Impl: IColumnMapperCreatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColumnMapperCreatorVtbl {
        unsafe extern "system" fn GetColumnMapper<Impl: IColumnMapperCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wcsmachinename: super::super::Foundation::PWSTR, wcscatalogname: super::super::Foundation::PWSTR, ppcolumnmapper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColumnMapper(&*(&wcsmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wcscatalogname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcolumnmapper)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColumnMapperCreator>, base.5, GetColumnMapper::<Impl, OFFSET>)
    }
}
pub trait IColumnsInfoImpl: Sized {
    fn GetColumnInfo();
    fn MapColumnIDs();
}
impl ::windows::core::RuntimeName for IColumnsInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.IColumnsInfo";
}
impl IColumnsInfoVtbl {
    pub const fn new<Impl: IColumnsInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColumnsInfoVtbl {
        unsafe extern "system" fn GetColumnInfo<Impl: IColumnsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColumnInfo(::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prginfo), ::core::mem::transmute_copy(&ppstringsbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapColumnIDs<Impl: IColumnsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapColumnIDs(ccolumnids, &*(&rgcolumnids as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rgcolumns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColumnsInfo>, base.5, GetColumnInfo::<Impl, OFFSET>, MapColumnIDs::<Impl, OFFSET>)
    }
}
pub trait IColumnsInfo2Impl: Sized + IColumnsInfoImpl {
    fn GetRestrictedColumnInfo();
}
impl ::windows::core::RuntimeName for IColumnsInfo2 {
    const NAME: &'static str = "Windows.Win32.System.Search.IColumnsInfo2";
}
impl IColumnsInfo2Vtbl {
    pub const fn new<Impl: IColumnsInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColumnsInfo2Vtbl {
        unsafe extern "system" fn GetRestrictedColumnInfo<Impl: IColumnsInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRestrictedColumnInfo(ccolumnidmasks, &*(&rgcolumnidmasks as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prgcolumnids), ::core::mem::transmute_copy(&prgcolumninfo), ::core::mem::transmute_copy(&ppstringsbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColumnsInfo2>, base.5, GetRestrictedColumnInfo::<Impl, OFFSET>)
    }
}
pub trait IColumnsRowsetImpl: Sized {
    fn GetAvailableColumns();
    fn GetColumnsRowset();
}
impl ::windows::core::RuntimeName for IColumnsRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IColumnsRowset";
}
impl IColumnsRowsetVtbl {
    pub const fn new<Impl: IColumnsRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColumnsRowsetVtbl {
        unsafe extern "system" fn GetAvailableColumns<Impl: IColumnsRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableColumns(::core::mem::transmute_copy(&pcoptcolumns), ::core::mem::transmute_copy(&prgoptcolumns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnsRowset<Impl: IColumnsRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColumnsRowset(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                coptcolumns,
                &*(&rgoptcolumns as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcolrowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColumnsRowset>, base.5, GetAvailableColumns::<Impl, OFFSET>, GetColumnsRowset::<Impl, OFFSET>)
    }
}
pub trait ICommandImpl: Sized {
    fn Cancel();
    fn Execute();
    fn GetDBSession();
}
impl ::windows::core::RuntimeName for ICommand {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommand";
}
impl ICommandVtbl {
    pub const fn new<Impl: ICommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandVtbl {
        unsafe extern "system" fn Cancel<Impl: ICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Impl: ICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Execute(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pparams as *const <DBPARAMS as ::windows::core::Abi>::Abi as *const <DBPARAMS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pcrowsaffected),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDBSession<Impl: ICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDBSession(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommand>, base.5, Cancel::<Impl, OFFSET>, Execute::<Impl, OFFSET>, GetDBSession::<Impl, OFFSET>)
    }
}
pub trait ICommandCostImpl: Sized {
    fn GetAccumulatedCost();
    fn GetCostEstimate();
    fn GetCostGoals();
    fn GetCostLimits();
    fn SetCostGoals();
    fn SetCostLimits();
}
impl ::windows::core::RuntimeName for ICommandCost {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandCost";
}
impl ICommandCostVtbl {
    pub const fn new<Impl: ICommandCostImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandCostVtbl {
        unsafe extern "system" fn GetAccumulatedCost<Impl: ICommandCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccumulatedCost(&*(&pwszrowsetname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pccostlimits, &*(&prgcostlimits as *const <DBCOST as ::windows::core::Abi>::Abi as *const <DBCOST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCostEstimate<Impl: ICommandCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCostEstimate(&*(&pwszrowsetname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pccostestimates, &*(&prgcostestimates as *const <DBCOST as ::windows::core::Abi>::Abi as *const <DBCOST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCostGoals<Impl: ICommandCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCostGoals(&*(&pwszrowsetname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pccostgoals, &*(&prgcostgoals as *const <DBCOST as ::windows::core::Abi>::Abi as *const <DBCOST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCostLimits<Impl: ICommandCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCostLimits(&*(&pwszrowsetname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pccostlimits, &*(&prgcostlimits as *const <DBCOST as ::windows::core::Abi>::Abi as *const <DBCOST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostGoals<Impl: ICommandCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCostGoals(&*(&pwszrowsetname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ccostgoals, &*(&rgcostgoals as *const <DBCOST as ::windows::core::Abi>::Abi as *const <DBCOST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostLimits<Impl: ICommandCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, ccostlimits: u32, prgcostlimits: *mut DBCOST, dwexecutionflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCostLimits(&*(&pwszrowsetname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ccostlimits, &*(&prgcostlimits as *const <DBCOST as ::windows::core::Abi>::Abi as *const <DBCOST as ::windows::core::DefaultType>::DefaultType), dwexecutionflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandCost>, base.5, GetAccumulatedCost::<Impl, OFFSET>, GetCostEstimate::<Impl, OFFSET>, GetCostGoals::<Impl, OFFSET>, GetCostLimits::<Impl, OFFSET>, SetCostGoals::<Impl, OFFSET>, SetCostLimits::<Impl, OFFSET>)
    }
}
pub trait ICommandPersistImpl: Sized {
    fn DeleteCommand();
    fn GetCurrentCommand();
    fn LoadCommand();
    fn SaveCommand();
}
impl ::windows::core::RuntimeName for ICommandPersist {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandPersist";
}
impl ICommandPersistVtbl {
    pub const fn new<Impl: ICommandPersistImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandPersistVtbl {
        unsafe extern "system" fn DeleteCommand<Impl: ICommandPersistImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteCommand(&*(&pcommandid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentCommand<Impl: ICommandPersistImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentCommand(&*(&ppcommandid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCommand<Impl: ICommandPersistImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadCommand(&*(&pcommandid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveCommand<Impl: ICommandPersistImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveCommand(&*(&pcommandid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandPersist>, base.5, DeleteCommand::<Impl, OFFSET>, GetCurrentCommand::<Impl, OFFSET>, LoadCommand::<Impl, OFFSET>, SaveCommand::<Impl, OFFSET>)
    }
}
pub trait ICommandPrepareImpl: Sized {
    fn Prepare();
    fn Unprepare();
}
impl ::windows::core::RuntimeName for ICommandPrepare {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandPrepare";
}
impl ICommandPrepareVtbl {
    pub const fn new<Impl: ICommandPrepareImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandPrepareVtbl {
        unsafe extern "system" fn Prepare<Impl: ICommandPrepareImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexpectedruns: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Prepare(cexpectedruns) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unprepare<Impl: ICommandPrepareImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unprepare() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandPrepare>, base.5, Prepare::<Impl, OFFSET>, Unprepare::<Impl, OFFSET>)
    }
}
pub trait ICommandPropertiesImpl: Sized {
    fn GetProperties();
    fn SetProperties();
}
impl ::windows::core::RuntimeName for ICommandProperties {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandProperties";
}
impl ICommandPropertiesVtbl {
    pub const fn new<Impl: ICommandPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandPropertiesVtbl {
        unsafe extern "system" fn GetProperties<Impl: ICommandPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperties(cpropertyidsets, &*(&rgpropertyidsets as *const <DBPROPIDSET as ::windows::core::Abi>::Abi as *const <DBPROPIDSET as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperties<Impl: ICommandPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperties(cpropertysets, &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandProperties>, base.5, GetProperties::<Impl, OFFSET>, SetProperties::<Impl, OFFSET>)
    }
}
pub trait ICommandStreamImpl: Sized {
    fn GetCommandStream();
    fn SetCommandStream();
}
impl ::windows::core::RuntimeName for ICommandStream {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandStream";
}
impl ICommandStreamVtbl {
    pub const fn new<Impl: ICommandStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandStreamVtbl {
        unsafe extern "system" fn GetCommandStream<Impl: ICommandStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pguiddialect: *mut ::windows::core::GUID, ppcommandstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCommandStream(::core::mem::transmute_copy(&piid), &*(&pguiddialect as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommandstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommandStream<Impl: ICommandStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rguiddialect: *const ::windows::core::GUID, pcommandstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCommandStream(
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&rguiddialect as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pcommandstream as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandStream>, base.5, GetCommandStream::<Impl, OFFSET>, SetCommandStream::<Impl, OFFSET>)
    }
}
pub trait ICommandTextImpl: Sized + ICommandImpl {
    fn GetCommandText();
    fn SetCommandText();
}
impl ::windows::core::RuntimeName for ICommandText {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandText";
}
impl ICommandTextVtbl {
    pub const fn new<Impl: ICommandTextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandTextVtbl {
        unsafe extern "system" fn GetCommandText<Impl: ICommandTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguiddialect: *mut ::windows::core::GUID, ppwszcommand: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCommandText(&*(&pguiddialect as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwszcommand)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommandText<Impl: ICommandTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguiddialect: *const ::windows::core::GUID, pwszcommand: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCommandText(&*(&rguiddialect as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pwszcommand as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandText>, base.5, GetCommandText::<Impl, OFFSET>, SetCommandText::<Impl, OFFSET>)
    }
}
pub trait ICommandValidateImpl: Sized {
    fn ValidateCompletely();
    fn ValidateSyntax();
}
impl ::windows::core::RuntimeName for ICommandValidate {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandValidate";
}
impl ICommandValidateVtbl {
    pub const fn new<Impl: ICommandValidateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandValidateVtbl {
        unsafe extern "system" fn ValidateCompletely<Impl: ICommandValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateCompletely() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateSyntax<Impl: ICommandValidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateSyntax() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandValidate>, base.5, ValidateCompletely::<Impl, OFFSET>, ValidateSyntax::<Impl, OFFSET>)
    }
}
pub trait ICommandWithParametersImpl: Sized {
    fn GetParameterInfo();
    fn MapParameterNames();
    fn SetParameterInfo();
}
impl ::windows::core::RuntimeName for ICommandWithParameters {
    const NAME: &'static str = "Windows.Win32.System.Search.ICommandWithParameters";
}
impl ICommandWithParametersVtbl {
    pub const fn new<Impl: ICommandWithParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandWithParametersVtbl {
        unsafe extern "system" fn GetParameterInfo<Impl: ICommandWithParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParameterInfo(::core::mem::transmute_copy(&pcparams), ::core::mem::transmute_copy(&prgparaminfo), ::core::mem::transmute_copy(&ppnamesbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapParameterNames<Impl: ICommandWithParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cparamnames: usize, rgparamnames: *const super::super::Foundation::PWSTR, rgparamordinals: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapParameterNames(cparamnames, &*(&rgparamnames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rgparamordinals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameterInfo<Impl: ICommandWithParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParameterInfo(cparams, rgparamordinals, &*(&rgparambindinfo as *const <DBPARAMBINDINFO as ::windows::core::Abi>::Abi as *const <DBPARAMBINDINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandWithParameters>, base.5, GetParameterInfo::<Impl, OFFSET>, MapParameterNames::<Impl, OFFSET>, SetParameterInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConditionImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn GetConditionType();
    fn GetSubConditions();
    fn GetComparisonInfo();
    fn GetValueType();
    fn GetValueNormalization();
    fn GetInputTerms();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICondition {
    const NAME: &'static str = "Windows.Win32.System.Search.ICondition";
}
#[cfg(feature = "Win32_System_Com")]
impl IConditionVtbl {
    pub const fn new<Impl: IConditionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConditionVtbl {
        unsafe extern "system" fn GetConditionType<Impl: IConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnodetype: *mut Common::CONDITION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConditionType(::core::mem::transmute_copy(&pnodetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubConditions<Impl: IConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubConditions(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComparisonInfo<Impl: IConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpropertyname: *mut super::super::Foundation::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetComparisonInfo(::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueType<Impl: IConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszvaluetypename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueType(::core::mem::transmute_copy(&ppszvaluetypename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueNormalization<Impl: IConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsznormalization: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueNormalization(::core::mem::transmute_copy(&ppsznormalization)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTerms<Impl: IConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyterm: *mut ::windows::core::RawPtr, ppoperationterm: *mut ::windows::core::RawPtr, ppvalueterm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputTerms(::core::mem::transmute_copy(&pppropertyterm), ::core::mem::transmute_copy(&ppoperationterm), ::core::mem::transmute_copy(&ppvalueterm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICondition>, base.5, GetConditionType::<Impl, OFFSET>, GetSubConditions::<Impl, OFFSET>, GetComparisonInfo::<Impl, OFFSET>, GetValueType::<Impl, OFFSET>, GetValueNormalization::<Impl, OFFSET>, GetInputTerms::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICondition2Impl: Sized + IConditionImpl + IPersistStreamImpl + IPersistImpl {
    fn GetLocale();
    fn GetLeafConditionInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICondition2 {
    const NAME: &'static str = "Windows.Win32.System.Search.ICondition2";
}
#[cfg(feature = "Win32_System_Com")]
impl ICondition2Vtbl {
    pub const fn new<Impl: ICondition2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICondition2Vtbl {
        unsafe extern "system" fn GetLocale<Impl: ICondition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszlocalename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocale(::core::mem::transmute_copy(&ppszlocalename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLeafConditionInfo<Impl: ICondition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLeafConditionInfo(::core::mem::transmute_copy(&ppropkey), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICondition2>, base.5, GetLocale::<Impl, OFFSET>, GetLeafConditionInfo::<Impl, OFFSET>)
    }
}
pub trait IConditionFactoryImpl: Sized {
    fn MakeNot();
    fn MakeAndOr();
    fn MakeLeaf();
    fn Resolve();
}
impl ::windows::core::RuntimeName for IConditionFactory {
    const NAME: &'static str = "Windows.Win32.System.Search.IConditionFactory";
}
impl IConditionFactoryVtbl {
    pub const fn new<Impl: IConditionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConditionFactoryVtbl {
        unsafe extern "system" fn MakeNot<Impl: IConditionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsub: ::windows::core::RawPtr, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeNot(&*(&pcsub as *const <ICondition as ::windows::core::Abi>::Abi as *const <ICondition as ::windows::core::DefaultType>::DefaultType), &*(&fsimplify as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeAndOr<Impl: IConditionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, peusubs: ::windows::core::RawPtr, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeAndOr(ct, &*(&peusubs as *const <super::Com::IEnumUnknown as ::windows::core::Abi>::Abi as *const <super::Com::IEnumUnknown as ::windows::core::DefaultType>::DefaultType), &*(&fsimplify as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeLeaf<Impl: IConditionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: super::super::Foundation::PWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: ::windows::core::RawPtr, poperationterm: ::windows::core::RawPtr, pvalueterm: ::windows::core::RawPtr, fexpand: super::super::Foundation::BOOL, ppcresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeLeaf(
                &*(&pszpropertyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cop,
                &*(&pszvaluetype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppropvar as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&ppropertynameterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&poperationterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&pvalueterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&fexpand as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resolve<Impl: IConditionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pc: ::windows::core::RawPtr, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, ppcresolved: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resolve(&*(&pc as *const <ICondition as ::windows::core::Abi>::Abi as *const <ICondition as ::windows::core::DefaultType>::DefaultType), sqro, &*(&pstreferencetime as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcresolved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConditionFactory>, base.5, MakeNot::<Impl, OFFSET>, MakeAndOr::<Impl, OFFSET>, MakeLeaf::<Impl, OFFSET>, Resolve::<Impl, OFFSET>)
    }
}
pub trait IConditionFactory2Impl: Sized + IConditionFactoryImpl {
    fn CreateTrueFalse();
    fn CreateNegation();
    fn CreateCompoundFromObjectArray();
    fn CreateCompoundFromArray();
    fn CreateStringLeaf();
    fn CreateIntegerLeaf();
    fn CreateBooleanLeaf();
    fn CreateLeaf();
    fn ResolveCondition();
}
impl ::windows::core::RuntimeName for IConditionFactory2 {
    const NAME: &'static str = "Windows.Win32.System.Search.IConditionFactory2";
}
impl IConditionFactory2Vtbl {
    pub const fn new<Impl: IConditionFactory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConditionFactory2Vtbl {
        unsafe extern "system" fn CreateTrueFalse<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTrueFalse(&*(&fval as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), cco, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNegation<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsub: ::windows::core::RawPtr, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNegation(&*(&pcsub as *const <ICondition as ::windows::core::Abi>::Abi as *const <ICondition as ::windows::core::DefaultType>::DefaultType), cco, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompoundFromObjectArray<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, poasubs: ::windows::core::RawPtr, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCompoundFromObjectArray(ct, &*(&poasubs as *const <super::super::UI::Shell::Common::IObjectArray as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::Common::IObjectArray as ::windows::core::DefaultType>::DefaultType), cco, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompoundFromArray<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, ppcondsubs: *const ::windows::core::RawPtr, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCompoundFromArray(ct, &*(&ppcondsubs as *const <ICondition as ::windows::core::Abi>::Abi as *const <ICondition as ::windows::core::DefaultType>::DefaultType), csubs, cco, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStringLeaf<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: super::super::Foundation::PWSTR, pszlocalename: super::super::Foundation::PWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStringLeaf(
                &*(&propkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                cop,
                &*(&pszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszlocalename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cco,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIntegerLeaf<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateIntegerLeaf(&*(&propkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), cop, lvalue, cco, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBooleanLeaf<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBooleanLeaf(
                &*(&propkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                cop,
                &*(&fvalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                cco,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaf<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: super::super::Foundation::PWSTR, pszlocalename: super::super::Foundation::PWSTR, ppropertynameterm: ::windows::core::RawPtr, poperationterm: ::windows::core::RawPtr, pvalueterm: ::windows::core::RawPtr, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLeaf(
                &*(&propkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                cop,
                &*(&propvar as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsemantictype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszlocalename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppropertynameterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&poperationterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&pvalueterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                cco,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveCondition<Impl: IConditionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pc: ::windows::core::RawPtr, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolveCondition(
                &*(&pc as *const <ICondition as ::windows::core::Abi>::Abi as *const <ICondition as ::windows::core::DefaultType>::DefaultType),
                sqro,
                &*(&pstreferencetime as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConditionFactory2>, base.5, CreateTrueFalse::<Impl, OFFSET>, CreateNegation::<Impl, OFFSET>, CreateCompoundFromObjectArray::<Impl, OFFSET>, CreateCompoundFromArray::<Impl, OFFSET>, CreateStringLeaf::<Impl, OFFSET>, CreateIntegerLeaf::<Impl, OFFSET>, CreateBooleanLeaf::<Impl, OFFSET>, CreateLeaf::<Impl, OFFSET>, ResolveCondition::<Impl, OFFSET>)
    }
}
pub trait IConditionGeneratorImpl: Sized {
    fn Initialize();
    fn RecognizeNamedEntities();
    fn GenerateForLeaf();
    fn DefaultPhrase();
}
impl ::windows::core::RuntimeName for IConditionGenerator {
    const NAME: &'static str = "Windows.Win32.System.Search.IConditionGenerator";
}
impl IConditionGeneratorVtbl {
    pub const fn new<Impl: IConditionGeneratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConditionGeneratorVtbl {
        unsafe extern "system" fn Initialize<Impl: IConditionGeneratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pschemaprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pschemaprovider as *const <ISchemaProvider as ::windows::core::Abi>::Abi as *const <ISchemaProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizeNamedEntities<Impl: IConditionGeneratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinputstring: super::super::Foundation::PWSTR, lciduserlocale: u32, ptokencollection: ::windows::core::RawPtr, pnamedentities: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizeNamedEntities(
                &*(&pszinputstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lciduserlocale,
                &*(&ptokencollection as *const <ITokenCollection as ::windows::core::Abi>::Abi as *const <ITokenCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&pnamedentities as *const <INamedEntityCollector as ::windows::core::Abi>::Abi as *const <INamedEntityCollector as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateForLeaf<Impl: IConditionGeneratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconditionfactory: ::windows::core::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, pszvalue2: super::super::Foundation::PWSTR, ppropertynameterm: ::windows::core::RawPtr, poperationterm: ::windows::core::RawPtr, pvalueterm: ::windows::core::RawPtr, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateForLeaf(
                &*(&pconditionfactory as *const <IConditionFactory as ::windows::core::Abi>::Abi as *const <IConditionFactory as ::windows::core::DefaultType>::DefaultType),
                &*(&pszpropertyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cop,
                &*(&pszvaluetype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszvalue2 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppropertynameterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&poperationterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&pvalueterm as *const <IRichChunk as ::windows::core::Abi>::Abi as *const <IRichChunk as ::windows::core::DefaultType>::DefaultType),
                &*(&automaticwildcard as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pnostringquery),
                ::core::mem::transmute_copy(&ppqueryexpression),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Impl: IConditionGeneratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluetype: super::super::Foundation::PWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultPhrase(
                &*(&pszvaluetype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppropvar as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&fuseenglish as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppszphrase),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConditionGenerator>, base.5, Initialize::<Impl, OFFSET>, RecognizeNamedEntities::<Impl, OFFSET>, GenerateForLeaf::<Impl, OFFSET>, DefaultPhrase::<Impl, OFFSET>)
    }
}
pub trait IConvertTypeImpl: Sized {
    fn CanConvert();
}
impl ::windows::core::RuntimeName for IConvertType {
    const NAME: &'static str = "Windows.Win32.System.Search.IConvertType";
}
impl IConvertTypeVtbl {
    pub const fn new<Impl: IConvertTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConvertTypeVtbl {
        unsafe extern "system" fn CanConvert<Impl: IConvertTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanConvert(wfromtype, wtotype, dwconvertflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConvertType>, base.5, CanConvert::<Impl, OFFSET>)
    }
}
pub trait ICreateRowImpl: Sized {
    fn CreateRow();
}
impl ::windows::core::RuntimeName for ICreateRow {
    const NAME: &'static str = "Windows.Win32.System.Search.ICreateRow";
}
impl ICreateRowVtbl {
    pub const fn new<Impl: ICreateRowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateRowVtbl {
        unsafe extern "system" fn CreateRow<Impl: ICreateRowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: ::windows::core::RawPtr, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut super::super::Foundation::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRow(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwbindurlflags,
                &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pauthenticate as *const <super::Com::IAuthenticate as ::windows::core::Abi>::Abi as *const <super::Com::IAuthenticate as ::windows::core::DefaultType>::DefaultType),
                &*(&pimplsession as *const <DBIMPLICITSESSION as ::windows::core::Abi>::Abi as *const <DBIMPLICITSESSION as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwbindstatus),
                ::core::mem::transmute_copy(&ppwsznewurl),
                ::core::mem::transmute_copy(&ppunk),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateRow>, base.5, CreateRow::<Impl, OFFSET>)
    }
}
pub trait IDBAsynchNotifyImpl: Sized {
    fn OnLowResource();
    fn OnProgress();
    fn OnStop();
}
impl ::windows::core::RuntimeName for IDBAsynchNotify {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBAsynchNotify";
}
impl IDBAsynchNotifyVtbl {
    pub const fn new<Impl: IDBAsynchNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBAsynchNotifyVtbl {
        unsafe extern "system" fn OnLowResource<Impl: IDBAsynchNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnLowResource(dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProgress<Impl: IDBAsynchNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnProgress(hchapter, eoperation, ulprogress, ulprogressmax, easynchphase, &*(&pwszstatustext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStop<Impl: IDBAsynchNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, hrstatus: ::windows::core::HRESULT, pwszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStop(hchapter, eoperation, hrstatus, &*(&pwszstatustext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBAsynchNotify>, base.5, OnLowResource::<Impl, OFFSET>, OnProgress::<Impl, OFFSET>, OnStop::<Impl, OFFSET>)
    }
}
pub trait IDBAsynchStatusImpl: Sized {
    fn Abort();
    fn GetStatus();
}
impl ::windows::core::RuntimeName for IDBAsynchStatus {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBAsynchStatus";
}
impl IDBAsynchStatusVtbl {
    pub const fn new<Impl: IDBAsynchStatusImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBAsynchStatusVtbl {
        unsafe extern "system" fn Abort<Impl: IDBAsynchStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abort(hchapter, eoperation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDBAsynchStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(hchapter, eoperation, ::core::mem::transmute_copy(&pulprogress), ::core::mem::transmute_copy(&pulprogressmax), ::core::mem::transmute_copy(&peasynchphase), &*(&ppwszstatustext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBAsynchStatus>, base.5, Abort::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>)
    }
}
pub trait IDBBinderPropertiesImpl: Sized + IDBPropertiesImpl {
    fn Reset();
}
impl ::windows::core::RuntimeName for IDBBinderProperties {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBBinderProperties";
}
impl IDBBinderPropertiesVtbl {
    pub const fn new<Impl: IDBBinderPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBBinderPropertiesVtbl {
        unsafe extern "system" fn Reset<Impl: IDBBinderPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBBinderProperties>, base.5, Reset::<Impl, OFFSET>)
    }
}
pub trait IDBCreateCommandImpl: Sized {
    fn CreateCommand();
}
impl ::windows::core::RuntimeName for IDBCreateCommand {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBCreateCommand";
}
impl IDBCreateCommandVtbl {
    pub const fn new<Impl: IDBCreateCommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBCreateCommandVtbl {
        unsafe extern "system" fn CreateCommand<Impl: IDBCreateCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCommand(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommand)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBCreateCommand>, base.5, CreateCommand::<Impl, OFFSET>)
    }
}
pub trait IDBCreateSessionImpl: Sized {
    fn CreateSession();
}
impl ::windows::core::RuntimeName for IDBCreateSession {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBCreateSession";
}
impl IDBCreateSessionVtbl {
    pub const fn new<Impl: IDBCreateSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBCreateSessionVtbl {
        unsafe extern "system" fn CreateSession<Impl: IDBCreateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSession(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdbsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBCreateSession>, base.5, CreateSession::<Impl, OFFSET>)
    }
}
pub trait IDBDataSourceAdminImpl: Sized {
    fn CreateDataSource();
    fn DestroyDataSource();
    fn GetCreationProperties();
    fn ModifyDataSource();
}
impl ::windows::core::RuntimeName for IDBDataSourceAdmin {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBDataSourceAdmin";
}
impl IDBDataSourceAdminVtbl {
    pub const fn new<Impl: IDBDataSourceAdminImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBDataSourceAdminVtbl {
        unsafe extern "system" fn CreateDataSource<Impl: IDBDataSourceAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDataSource(
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppdbsession),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyDataSource<Impl: IDBDataSourceAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DestroyDataSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreationProperties<Impl: IDBDataSourceAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCreationProperties(cpropertyidsets, &*(&rgpropertyidsets as *const <DBPROPIDSET as ::windows::core::Abi>::Abi as *const <DBPROPIDSET as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyDataSource<Impl: IDBDataSourceAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifyDataSource(cpropertysets, &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBDataSourceAdmin>, base.5, CreateDataSource::<Impl, OFFSET>, DestroyDataSource::<Impl, OFFSET>, GetCreationProperties::<Impl, OFFSET>, ModifyDataSource::<Impl, OFFSET>)
    }
}
pub trait IDBInfoImpl: Sized {
    fn GetKeywords();
    fn GetLiteralInfo();
}
impl ::windows::core::RuntimeName for IDBInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBInfo";
}
impl IDBInfoVtbl {
    pub const fn new<Impl: IDBInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBInfoVtbl {
        unsafe extern "system" fn GetKeywords<Impl: IDBInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszkeywords: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKeywords(::core::mem::transmute_copy(&ppwszkeywords)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiteralInfo<Impl: IDBInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLiteralInfo(cliterals, rgliterals, ::core::mem::transmute_copy(&pcliteralinfo), ::core::mem::transmute_copy(&prgliteralinfo), ::core::mem::transmute_copy(&ppcharbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBInfo>, base.5, GetKeywords::<Impl, OFFSET>, GetLiteralInfo::<Impl, OFFSET>)
    }
}
pub trait IDBInitializeImpl: Sized {
    fn Initialize();
    fn Uninitialize();
}
impl ::windows::core::RuntimeName for IDBInitialize {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBInitialize";
}
impl IDBInitializeVtbl {
    pub const fn new<Impl: IDBInitializeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBInitializeVtbl {
        unsafe extern "system" fn Initialize<Impl: IDBInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IDBInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uninitialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBInitialize>, base.5, Initialize::<Impl, OFFSET>, Uninitialize::<Impl, OFFSET>)
    }
}
pub trait IDBPromptInitializeImpl: Sized {
    fn PromptDataSource();
    fn PromptFileName();
}
impl ::windows::core::RuntimeName for IDBPromptInitialize {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBPromptInitialize";
}
impl IDBPromptInitializeVtbl {
    pub const fn new<Impl: IDBPromptInitializeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBPromptInitializeVtbl {
        unsafe extern "system" fn PromptDataSource<Impl: IDBPromptInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PromptDataSource(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwpromptoptions,
                csourcetypefilter,
                rgsourcetypefilter,
                &*(&pwszszzproviderfilter as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&ppdatasource as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptFileName<Impl: IDBPromptInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: super::super::Foundation::PWSTR, pwszinitialfile: super::super::Foundation::PWSTR, ppwszselectedfile: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PromptFileName(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwpromptoptions,
                &*(&pwszinitialdirectory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszinitialfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppwszselectedfile),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBPromptInitialize>, base.5, PromptDataSource::<Impl, OFFSET>, PromptFileName::<Impl, OFFSET>)
    }
}
pub trait IDBPropertiesImpl: Sized {
    fn GetProperties();
    fn GetPropertyInfo();
    fn SetProperties();
}
impl ::windows::core::RuntimeName for IDBProperties {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBProperties";
}
impl IDBPropertiesVtbl {
    pub const fn new<Impl: IDBPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBPropertiesVtbl {
        unsafe extern "system" fn GetProperties<Impl: IDBPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperties(cpropertyidsets, &*(&rgpropertyidsets as *const <DBPROPIDSET as ::windows::core::Abi>::Abi as *const <DBPROPIDSET as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Impl: IDBPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyInfo(cpropertyidsets, &*(&rgpropertyidsets as *const <DBPROPIDSET as ::windows::core::Abi>::Abi as *const <DBPROPIDSET as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperties<Impl: IDBPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperties(cpropertysets, &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBProperties>, base.5, GetProperties::<Impl, OFFSET>, GetPropertyInfo::<Impl, OFFSET>, SetProperties::<Impl, OFFSET>)
    }
}
pub trait IDBSchemaCommandImpl: Sized {
    fn GetCommand();
    fn GetSchemas();
}
impl ::windows::core::RuntimeName for IDBSchemaCommand {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBSchemaCommand";
}
impl IDBSchemaCommandVtbl {
    pub const fn new<Impl: IDBSchemaCommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBSchemaCommandVtbl {
        unsafe extern "system" fn GetCommand<Impl: IDBSchemaCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows::core::GUID, ppcommand: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCommand(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&rguidschema as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommand)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemas<Impl: IDBSchemaCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSchemas(pcschemas, &*(&prgschemas as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBSchemaCommand>, base.5, GetCommand::<Impl, OFFSET>, GetSchemas::<Impl, OFFSET>)
    }
}
pub trait IDBSchemaRowsetImpl: Sized {
    fn GetRowset();
    fn GetSchemas();
}
impl ::windows::core::RuntimeName for IDBSchemaRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IDBSchemaRowset";
}
impl IDBSchemaRowsetVtbl {
    pub const fn new<Impl: IDBSchemaRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDBSchemaRowsetVtbl {
        unsafe extern "system" fn GetRowset<Impl: IDBSchemaRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows::core::GUID, crestrictions: u32, rgrestrictions: *const super::Com::VARIANT, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowset(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&rguidschema as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                crestrictions,
                &*(&rgrestrictions as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemas<Impl: IDBSchemaRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSchemas(::core::mem::transmute_copy(&pcschemas), ::core::mem::transmute_copy(&prgschemas), ::core::mem::transmute_copy(&prgrestrictionsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDBSchemaRowset>, base.5, GetRowset::<Impl, OFFSET>, GetSchemas::<Impl, OFFSET>)
    }
}
pub trait IDCInfoImpl: Sized {
    fn GetInfo();
    fn SetInfo();
}
impl ::windows::core::RuntimeName for IDCInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.IDCInfo";
}
impl IDCInfoVtbl {
    pub const fn new<Impl: IDCInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDCInfoVtbl {
        unsafe extern "system" fn GetInfo<Impl: IDCInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInfo(cinfo, rgeinfotype, ::core::mem::transmute_copy(&prginfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfo<Impl: IDCInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinfo: u32, rginfo: *const DCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInfo(cinfo, &*(&rginfo as *const <DCINFO as ::windows::core::Abi>::Abi as *const <DCINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDCInfo>, base.5, GetInfo::<Impl, OFFSET>, SetInfo::<Impl, OFFSET>)
    }
}
pub trait IDataConvertImpl: Sized {
    fn DataConvert();
    fn CanConvert();
    fn GetConversionSize();
}
impl ::windows::core::RuntimeName for IDataConvert {
    const NAME: &'static str = "Windows.Win32.System.Search.IDataConvert";
}
impl IDataConvertVtbl {
    pub const fn new<Impl: IDataConvertImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataConvertVtbl {
        unsafe extern "system" fn DataConvert<Impl: IDataConvertImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataConvert(wsrctype, wdsttype, cbsrclength, ::core::mem::transmute_copy(&pcbdstlength), &*(&psrc as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdst), cbdstmaxlength, dbssrcstatus, ::core::mem::transmute_copy(&pdbsstatus), bprecision, bscale, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanConvert<Impl: IDataConvertImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanConvert(wsrctype, wdsttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionSize<Impl: IDataConvertImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConversionSize(wsrctype, wdsttype, pcbsrclength, ::core::mem::transmute_copy(&pcbdstlength), &*(&psrc as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataConvert>, base.5, DataConvert::<Impl, OFFSET>, CanConvert::<Impl, OFFSET>, GetConversionSize::<Impl, OFFSET>)
    }
}
pub trait IDataInitializeImpl: Sized {
    fn GetDataSource();
    fn GetInitializationString();
    fn CreateDBInstance();
    fn CreateDBInstanceEx();
    fn LoadStringFromStorage();
    fn WriteStringToStorage();
}
impl ::windows::core::RuntimeName for IDataInitialize {
    const NAME: &'static str = "Windows.Win32.System.Search.IDataInitialize";
}
impl IDataInitializeVtbl {
    pub const fn new<Impl: IDataInitializeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataInitializeVtbl {
        unsafe extern "system" fn GetDataSource<Impl: IDataInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszinitializationstring: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataSource(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwclsctx,
                &*(&pwszinitializationstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&ppdatasource as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInitializationString<Impl: IDataInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatasource: *mut ::core::ffi::c_void, fincludepassword: u8, ppwszinitstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInitializationString(&*(&pdatasource as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), fincludepassword, ::core::mem::transmute_copy(&ppwszinitstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstance<Impl: IDataInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDBInstance(
                &*(&clsidprovider as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwclsctx,
                &*(&pwszreserved as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppdatasource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstanceEx<Impl: IDataInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: super::super::Foundation::PWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDBInstanceEx(
                &*(&clsidprovider as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwclsctx,
                &*(&pwszreserved as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pserverinfo as *const <super::Com::COSERVERINFO as ::windows::core::Abi>::Abi as *const <super::Com::COSERVERINFO as ::windows::core::DefaultType>::DefaultType),
                cmq,
                ::core::mem::transmute_copy(&rgmqresults),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStringFromStorage<Impl: IDataInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, ppwszinitializationstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadStringFromStorage(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwszinitializationstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteStringToStorage<Impl: IDataInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, pwszinitializationstring: super::super::Foundation::PWSTR, dwcreationdisposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteStringToStorage(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszinitializationstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwcreationdisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataInitialize>, base.5, GetDataSource::<Impl, OFFSET>, GetInitializationString::<Impl, OFFSET>, CreateDBInstance::<Impl, OFFSET>, CreateDBInstanceEx::<Impl, OFFSET>, LoadStringFromStorage::<Impl, OFFSET>, WriteStringToStorage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataSourceLocatorImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn SethWnd();
    fn PromptNew();
    fn PromptEdit();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDataSourceLocator {
    const NAME: &'static str = "Windows.Win32.System.Search.IDataSourceLocator";
}
#[cfg(feature = "Win32_System_Com")]
impl IDataSourceLocatorVtbl {
    pub const fn new<Impl: IDataSourceLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataSourceLocatorVtbl {
        unsafe extern "system" fn hWnd<Impl: IDataSourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwndparent: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).hWnd(::core::mem::transmute_copy(&phwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Impl: IDataSourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SethWnd(hwndparent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptNew<Impl: IDataSourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PromptNew(::core::mem::transmute_copy(&ppadoconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptEdit<Impl: IDataSourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut ::windows::core::RawPtr, pbsuccess: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PromptEdit(&*(&ppadoconnection as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbsuccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataSourceLocator>, base.5, hWnd::<Impl, OFFSET>, SethWnd::<Impl, OFFSET>, PromptNew::<Impl, OFFSET>, PromptEdit::<Impl, OFFSET>)
    }
}
pub trait IEntityImpl: Sized {
    fn Name();
    fn Base();
    fn Relationships();
    fn GetRelationship();
    fn MetaData();
    fn NamedEntities();
    fn GetNamedEntity();
    fn DefaultPhrase();
}
impl ::windows::core::RuntimeName for IEntity {
    const NAME: &'static str = "Windows.Win32.System.Search.IEntity";
}
impl IEntityVtbl {
    pub const fn new<Impl: IEntityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEntityVtbl {
        unsafe extern "system" fn Name<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Base<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbaseentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Base(::core::mem::transmute_copy(&pbaseentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Relationships<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Relationships(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prelationships)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationship<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrelationname: super::super::Foundation::PWSTR, prelationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationship(&*(&pszrelationname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prelationship)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MetaData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamedEntities<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NamedEntities(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnamedentities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedEntity<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvalue: super::super::Foundation::PWSTR, ppnamedentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamedEntity(&*(&pszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnamedentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Impl: IEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultPhrase(::core::mem::transmute_copy(&ppszphrase)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEntity>, base.5, Name::<Impl, OFFSET>, Base::<Impl, OFFSET>, Relationships::<Impl, OFFSET>, GetRelationship::<Impl, OFFSET>, MetaData::<Impl, OFFSET>, NamedEntities::<Impl, OFFSET>, GetNamedEntity::<Impl, OFFSET>, DefaultPhrase::<Impl, OFFSET>)
    }
}
pub trait IEnumItemPropertiesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumItemProperties {
    const NAME: &'static str = "Windows.Win32.System.Search.IEnumItemProperties";
}
impl IEnumItemPropertiesVtbl {
    pub const fn new<Impl: IEnumItemPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumItemPropertiesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumItemPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumItemPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumItemPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumItemPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumItemPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumItemProperties>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumSearchRootsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSearchRoots {
    const NAME: &'static str = "Windows.Win32.System.Search.IEnumSearchRoots";
}
impl IEnumSearchRootsVtbl {
    pub const fn new<Impl: IEnumSearchRootsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSearchRootsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSearchRootsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSearchRootsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSearchRootsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSearchRootsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSearchRoots>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSearchScopeRulesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSearchScopeRules {
    const NAME: &'static str = "Windows.Win32.System.Search.IEnumSearchScopeRules";
}
impl IEnumSearchScopeRulesVtbl {
    pub const fn new<Impl: IEnumSearchScopeRulesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSearchScopeRulesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSearchScopeRulesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pprgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&pprgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSearchScopeRulesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSearchScopeRulesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSearchScopeRulesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSearchScopeRules>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSubscriptionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumSubscription {
    const NAME: &'static str = "Windows.Win32.System.Search.IEnumSubscription";
}
impl IEnumSubscriptionVtbl {
    pub const fn new<Impl: IEnumSubscriptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSubscriptionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSubscriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSubscriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSubscriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSubscriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumSubscriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSubscription>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IErrorLookupImpl: Sized {
    fn GetErrorDescription();
    fn GetHelpInfo();
    fn ReleaseErrors();
}
impl ::windows::core::RuntimeName for IErrorLookup {
    const NAME: &'static str = "Windows.Win32.System.Search.IErrorLookup";
}
impl IErrorLookupVtbl {
    pub const fn new<Impl: IErrorLookupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IErrorLookupVtbl {
        unsafe extern "system" fn GetErrorDescription<Impl: IErrorLookupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut super::super::Foundation::BSTR, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorDescription(hrerror, dwlookupid, &*(&pdispparams as *const <super::Com::DISPPARAMS as ::windows::core::Abi>::Abi as *const <super::Com::DISPPARAMS as ::windows::core::DefaultType>::DefaultType), lcid, ::core::mem::transmute_copy(&pbstrsource), ::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpInfo<Impl: IErrorLookupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHelpInfo(hrerror, dwlookupid, lcid, ::core::mem::transmute_copy(&pbstrhelpfile), ::core::mem::transmute_copy(&pdwhelpcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseErrors<Impl: IErrorLookupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseErrors(dwdynamicerrorid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IErrorLookup>, base.5, GetErrorDescription::<Impl, OFFSET>, GetHelpInfo::<Impl, OFFSET>, ReleaseErrors::<Impl, OFFSET>)
    }
}
pub trait IErrorRecordsImpl: Sized {
    fn AddErrorRecord();
    fn GetBasicErrorInfo();
    fn GetCustomErrorObject();
    fn GetErrorInfo();
    fn GetErrorParameters();
    fn GetRecordCount();
}
impl ::windows::core::RuntimeName for IErrorRecords {
    const NAME: &'static str = "Windows.Win32.System.Search.IErrorRecords";
}
impl IErrorRecordsVtbl {
    pub const fn new<Impl: IErrorRecordsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IErrorRecordsVtbl {
        unsafe extern "system" fn AddErrorRecord<Impl: IErrorRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddErrorRecord(
                &*(&perrorinfo as *const <ERRORINFO as ::windows::core::Abi>::Abi as *const <ERRORINFO as ::windows::core::DefaultType>::DefaultType),
                dwlookupid,
                &*(&pdispparams as *const <super::Com::DISPPARAMS as ::windows::core::Abi>::Abi as *const <super::Com::DISPPARAMS as ::windows::core::DefaultType>::DefaultType),
                &*(&punkcustomerror as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwdynamicerrorid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBasicErrorInfo<Impl: IErrorRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBasicErrorInfo(ulrecordnum, ::core::mem::transmute_copy(&perrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomErrorObject<Impl: IErrorRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomErrorObject(ulrecordnum, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorInfo<Impl: IErrorRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, lcid: u32, pperrorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorInfo(ulrecordnum, lcid, ::core::mem::transmute_copy(&pperrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorParameters<Impl: IErrorRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, pdispparams: *mut super::Com::DISPPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorParameters(ulrecordnum, ::core::mem::transmute_copy(&pdispparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordCount<Impl: IErrorRecordsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcrecords: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecordCount(::core::mem::transmute_copy(&pcrecords)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IErrorRecords>, base.5, AddErrorRecord::<Impl, OFFSET>, GetBasicErrorInfo::<Impl, OFFSET>, GetCustomErrorObject::<Impl, OFFSET>, GetErrorInfo::<Impl, OFFSET>, GetErrorParameters::<Impl, OFFSET>, GetRecordCount::<Impl, OFFSET>)
    }
}
pub trait IGetDataSourceImpl: Sized {
    fn GetDataSource();
}
impl ::windows::core::RuntimeName for IGetDataSource {
    const NAME: &'static str = "Windows.Win32.System.Search.IGetDataSource";
}
impl IGetDataSourceVtbl {
    pub const fn new<Impl: IGetDataSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetDataSourceVtbl {
        unsafe extern "system" fn GetDataSource<Impl: IGetDataSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataSource(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdatasource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetDataSource>, base.5, GetDataSource::<Impl, OFFSET>)
    }
}
pub trait IGetRowImpl: Sized {
    fn GetRowFromHROW();
    fn GetURLFromHROW();
}
impl ::windows::core::RuntimeName for IGetRow {
    const NAME: &'static str = "Windows.Win32.System.Search.IGetRow";
}
impl IGetRowVtbl {
    pub const fn new<Impl: IGetRowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetRowVtbl {
        unsafe extern "system" fn GetRowFromHROW<Impl: IGetRowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hrow: usize, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowFromHROW(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), hrow, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLFromHROW<Impl: IGetRowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, ppwszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetURLFromHROW(hrow, ::core::mem::transmute_copy(&ppwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetRow>, base.5, GetRowFromHROW::<Impl, OFFSET>, GetURLFromHROW::<Impl, OFFSET>)
    }
}
pub trait IGetSessionImpl: Sized {
    fn GetSession();
}
impl ::windows::core::RuntimeName for IGetSession {
    const NAME: &'static str = "Windows.Win32.System.Search.IGetSession";
}
impl IGetSessionVtbl {
    pub const fn new<Impl: IGetSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetSessionVtbl {
        unsafe extern "system" fn GetSession<Impl: IGetSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSession(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetSession>, base.5, GetSession::<Impl, OFFSET>)
    }
}
pub trait IGetSourceRowImpl: Sized {
    fn GetSourceRow();
}
impl ::windows::core::RuntimeName for IGetSourceRow {
    const NAME: &'static str = "Windows.Win32.System.Search.IGetSourceRow";
}
impl IGetSourceRowVtbl {
    pub const fn new<Impl: IGetSourceRowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetSourceRowVtbl {
        unsafe extern "system" fn GetSourceRow<Impl: IGetSourceRowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprow: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceRow(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetSourceRow>, base.5, GetSourceRow::<Impl, OFFSET>)
    }
}
pub trait IIndexDefinitionImpl: Sized {
    fn CreateIndex();
    fn DropIndex();
}
impl ::windows::core::RuntimeName for IIndexDefinition {
    const NAME: &'static str = "Windows.Win32.System.Search.IIndexDefinition";
}
impl IIndexDefinitionVtbl {
    pub const fn new<Impl: IIndexDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIndexDefinitionVtbl {
        unsafe extern "system" fn CreateIndex<Impl: IIndexDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateIndex(
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                cindexcolumndescs,
                &*(&rgindexcolumndescs as *const <DBINDEXCOLUMNDESC as ::windows::core::Abi>::Abi as *const <DBINDEXCOLUMNDESC as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppindexid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropIndex<Impl: IIndexDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropIndex(&*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), &*(&pindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIndexDefinition>, base.5, CreateIndex::<Impl, OFFSET>, DropIndex::<Impl, OFFSET>)
    }
}
pub trait IIntervalImpl: Sized {
    fn GetLimits();
}
impl ::windows::core::RuntimeName for IInterval {
    const NAME: &'static str = "Windows.Win32.System.Search.IInterval";
}
impl IIntervalVtbl {
    pub const fn new<Impl: IIntervalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIntervalVtbl {
        unsafe extern "system" fn GetLimits<Impl: IIntervalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLimits(::core::mem::transmute_copy(&pilklower), ::core::mem::transmute_copy(&ppropvarlower), ::core::mem::transmute_copy(&pilkupper), ::core::mem::transmute_copy(&ppropvarupper)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInterval>, base.5, GetLimits::<Impl, OFFSET>)
    }
}
pub trait ILoadFilterImpl: Sized {
    fn LoadIFilter();
    fn LoadIFilterFromStorage();
    fn LoadIFilterFromStream();
}
impl ::windows::core::RuntimeName for ILoadFilter {
    const NAME: &'static str = "Windows.Win32.System.Search.ILoadFilter";
}
impl ILoadFilterVtbl {
    pub const fn new<Impl: ILoadFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoadFilterVtbl {
        unsafe extern "system" fn LoadIFilter<Impl: ILoadFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcspath: super::super::Foundation::PWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadIFilter(
                &*(&pwcspath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilteredsources as *const <FILTERED_DATA_SOURCES as ::windows::core::Abi>::Abi as *const <FILTERED_DATA_SOURCES as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&fusedefault as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilterclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                searchdecsize,
                ::core::mem::transmute_copy(&pwcssearchdesc),
                &*(&ppifilt as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadIFilterFromStorage<Impl: ILoadFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void, pwcsoverride: super::super::Foundation::PWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadIFilterFromStorage(
                &*(&pstg as *const <super::Com::StructuredStorage::IStorage as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::IStorage as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pwcsoverride as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fusedefault as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilterclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                searchdecsize,
                ::core::mem::transmute_copy(&pwcssearchdesc),
                &*(&ppifilt as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadIFilterFromStream<Impl: ILoadFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadIFilterFromStream(
                &*(&pstm as *const <super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilteredsources as *const <FILTERED_DATA_SOURCES as ::windows::core::Abi>::Abi as *const <FILTERED_DATA_SOURCES as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&fusedefault as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilterclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                searchdecsize,
                ::core::mem::transmute_copy(&pwcssearchdesc),
                &*(&ppifilt as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoadFilter>, base.5, LoadIFilter::<Impl, OFFSET>, LoadIFilterFromStorage::<Impl, OFFSET>, LoadIFilterFromStream::<Impl, OFFSET>)
    }
}
pub trait ILoadFilterWithPrivateComActivationImpl: Sized + ILoadFilterImpl {
    fn LoadIFilterWithPrivateComActivation();
}
impl ::windows::core::RuntimeName for ILoadFilterWithPrivateComActivation {
    const NAME: &'static str = "Windows.Win32.System.Search.ILoadFilterWithPrivateComActivation";
}
impl ILoadFilterWithPrivateComActivationVtbl {
    pub const fn new<Impl: ILoadFilterWithPrivateComActivationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoadFilterWithPrivateComActivationVtbl {
        unsafe extern "system" fn LoadIFilterWithPrivateComActivation<Impl: ILoadFilterWithPrivateComActivationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows::core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadIFilterWithPrivateComActivation(
                &*(&filteredsources as *const <FILTERED_DATA_SOURCES as ::windows::core::Abi>::Abi as *const <FILTERED_DATA_SOURCES as ::windows::core::DefaultType>::DefaultType),
                &*(&usedefault as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&filterclsid),
                ::core::mem::transmute_copy(&isfilterprivatecomactivated),
                &*(&filterobj as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoadFilterWithPrivateComActivation>, base.5, LoadIFilterWithPrivateComActivation::<Impl, OFFSET>)
    }
}
pub trait IMDDatasetImpl: Sized {
    fn FreeAxisInfo();
    fn GetAxisInfo();
    fn GetAxisRowset();
    fn GetCellData();
    fn GetSpecification();
}
impl ::windows::core::RuntimeName for IMDDataset {
    const NAME: &'static str = "Windows.Win32.System.Search.IMDDataset";
}
impl IMDDatasetVtbl {
    pub const fn new<Impl: IMDDatasetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMDDatasetVtbl {
        unsafe extern "system" fn FreeAxisInfo<Impl: IMDDatasetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, caxes: usize, rgaxisinfo: *mut MDAXISINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreeAxisInfo(caxes, &*(&rgaxisinfo as *const <MDAXISINFO as ::windows::core::Abi>::Abi as *const <MDAXISINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAxisInfo<Impl: IMDDatasetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAxisInfo(pcaxes, &*(&prgaxisinfo as *const <MDAXISINFO as ::windows::core::Abi>::Abi as *const <MDAXISINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAxisRowset<Impl: IMDDatasetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iaxis: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAxisRowset(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                iaxis,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellData<Impl: IMDDatasetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCellData(haccessor, ulstartcell, ulendcell, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecification<Impl: IMDDatasetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpecification(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppspecification)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMDDataset>, base.5, FreeAxisInfo::<Impl, OFFSET>, GetAxisInfo::<Impl, OFFSET>, GetAxisRowset::<Impl, OFFSET>, GetCellData::<Impl, OFFSET>, GetSpecification::<Impl, OFFSET>)
    }
}
pub trait IMDFindImpl: Sized {
    fn FindCell();
    fn FindTuple();
}
impl ::windows::core::RuntimeName for IMDFind {
    const NAME: &'static str = "Windows.Win32.System.Search.IMDFind";
}
impl IMDFindVtbl {
    pub const fn new<Impl: IMDFindImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMDFindVtbl {
        unsafe extern "system" fn FindCell<Impl: IMDFindImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut super::super::Foundation::PWSTR, pulcellordinal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindCell(ulstartingordinal, cmembers, &*(&rgpwszmember as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pulcellordinal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTuple<Impl: IMDFindImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut super::super::Foundation::PWSTR, pultupleordinal: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindTuple(ulaxisidentifier, ulstartingordinal, cmembers, &*(&rgpwszmember as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pultupleordinal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMDFind>, base.5, FindCell::<Impl, OFFSET>, FindTuple::<Impl, OFFSET>)
    }
}
pub trait IMDRangeRowsetImpl: Sized {
    fn GetRangeRowset();
}
impl ::windows::core::RuntimeName for IMDRangeRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IMDRangeRowset";
}
impl IMDRangeRowsetVtbl {
    pub const fn new<Impl: IMDRangeRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMDRangeRowsetVtbl {
        unsafe extern "system" fn GetRangeRowset<Impl: IMDRangeRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ulstartcell: usize, ulendcell: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRangeRowset(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ulstartcell,
                ulendcell,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMDRangeRowset>, base.5, GetRangeRowset::<Impl, OFFSET>)
    }
}
pub trait IMetaDataImpl: Sized {
    fn GetData();
}
impl ::windows::core::RuntimeName for IMetaData {
    const NAME: &'static str = "Windows.Win32.System.Search.IMetaData";
}
impl IMetaDataVtbl {
    pub const fn new<Impl: IMetaDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMetaDataVtbl {
        unsafe extern "system" fn GetData<Impl: IMetaDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszkey: *mut super::super::Foundation::PWSTR, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(::core::mem::transmute_copy(&ppszkey), ::core::mem::transmute_copy(&ppszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMetaData>, base.5, GetData::<Impl, OFFSET>)
    }
}
pub trait IMultipleResultsImpl: Sized {
    fn GetResult();
}
impl ::windows::core::RuntimeName for IMultipleResults {
    const NAME: &'static str = "Windows.Win32.System.Search.IMultipleResults";
}
impl IMultipleResultsVtbl {
    pub const fn new<Impl: IMultipleResultsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMultipleResultsVtbl {
        unsafe extern "system" fn GetResult<Impl: IMultipleResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, lresultflag: isize, riid: *const ::windows::core::GUID, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResult(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), lresultflag, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcrowsaffected), ::core::mem::transmute_copy(&pprowset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMultipleResults>, base.5, GetResult::<Impl, OFFSET>)
    }
}
pub trait INamedEntityImpl: Sized {
    fn GetValue();
    fn DefaultPhrase();
}
impl ::windows::core::RuntimeName for INamedEntity {
    const NAME: &'static str = "Windows.Win32.System.Search.INamedEntity";
}
impl INamedEntityVtbl {
    pub const fn new<Impl: INamedEntityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INamedEntityVtbl {
        unsafe extern "system" fn GetValue<Impl: INamedEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&ppszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Impl: INamedEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultPhrase(::core::mem::transmute_copy(&ppszphrase)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INamedEntity>, base.5, GetValue::<Impl, OFFSET>, DefaultPhrase::<Impl, OFFSET>)
    }
}
pub trait INamedEntityCollectorImpl: Sized {
    fn Add();
}
impl ::windows::core::RuntimeName for INamedEntityCollector {
    const NAME: &'static str = "Windows.Win32.System.Search.INamedEntityCollector";
}
impl INamedEntityCollectorVtbl {
    pub const fn new<Impl: INamedEntityCollectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INamedEntityCollectorVtbl {
        unsafe extern "system" fn Add<Impl: INamedEntityCollectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: ::windows::core::RawPtr, pszvalue: super::super::Foundation::PWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(beginspan, endspan, beginactual, endactual, &*(&ptype as *const <IEntity as ::windows::core::Abi>::Abi as *const <IEntity as ::windows::core::DefaultType>::DefaultType), &*(&pszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), certainty) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INamedEntityCollector>, base.5, Add::<Impl, OFFSET>)
    }
}
pub trait IObjectAccessControlImpl: Sized {
    fn GetObjectAccessRights();
    fn GetObjectOwner();
    fn IsObjectAccessAllowed();
    fn SetObjectAccessRights();
    fn SetObjectOwner();
}
impl ::windows::core::RuntimeName for IObjectAccessControl {
    const NAME: &'static str = "Windows.Win32.System.Search.IObjectAccessControl";
}
impl IObjectAccessControlVtbl {
    pub const fn new<Impl: IObjectAccessControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectAccessControlVtbl {
        unsafe extern "system" fn GetObjectAccessRights<Impl: IObjectAccessControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectAccessRights(&*(&pobject as *const <SEC_OBJECT as ::windows::core::Abi>::Abi as *const <SEC_OBJECT as ::windows::core::DefaultType>::DefaultType), pcaccessentries, &*(&prgaccessentries as *const <super::super::Security::Authorization::EXPLICIT_ACCESS_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::EXPLICIT_ACCESS_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectOwner<Impl: IObjectAccessControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectOwner(&*(&pobject as *const <SEC_OBJECT as ::windows::core::Abi>::Abi as *const <SEC_OBJECT as ::windows::core::DefaultType>::DefaultType), &*(&ppowner as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsObjectAccessAllowed<Impl: IObjectAccessControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, paccessentry: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsObjectAccessAllowed(
                &*(&pobject as *const <SEC_OBJECT as ::windows::core::Abi>::Abi as *const <SEC_OBJECT as ::windows::core::DefaultType>::DefaultType),
                &*(&paccessentry as *const <super::super::Security::Authorization::EXPLICIT_ACCESS_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::EXPLICIT_ACCESS_W as ::windows::core::DefaultType>::DefaultType),
                &*(&pfresult as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectAccessRights<Impl: IObjectAccessControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetObjectAccessRights(&*(&pobject as *const <SEC_OBJECT as ::windows::core::Abi>::Abi as *const <SEC_OBJECT as ::windows::core::DefaultType>::DefaultType), caccessentries, &*(&prgaccessentries as *const <super::super::Security::Authorization::EXPLICIT_ACCESS_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::EXPLICIT_ACCESS_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectOwner<Impl: IObjectAccessControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, powner: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetObjectOwner(&*(&pobject as *const <SEC_OBJECT as ::windows::core::Abi>::Abi as *const <SEC_OBJECT as ::windows::core::DefaultType>::DefaultType), &*(&powner as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectAccessControl>, base.5, GetObjectAccessRights::<Impl, OFFSET>, GetObjectOwner::<Impl, OFFSET>, IsObjectAccessAllowed::<Impl, OFFSET>, SetObjectAccessRights::<Impl, OFFSET>, SetObjectOwner::<Impl, OFFSET>)
    }
}
pub trait IOpLockStatusImpl: Sized {
    fn IsOplockValid();
    fn IsOplockBroken();
    fn GetOplockEventHandle();
}
impl ::windows::core::RuntimeName for IOpLockStatus {
    const NAME: &'static str = "Windows.Win32.System.Search.IOpLockStatus";
}
impl IOpLockStatusVtbl {
    pub const fn new<Impl: IOpLockStatusImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpLockStatusVtbl {
        unsafe extern "system" fn IsOplockValid<Impl: IOpLockStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisoplockvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOplockValid(::core::mem::transmute_copy(&pfisoplockvalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOplockBroken<Impl: IOpLockStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisoplockbroken: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOplockBroken(::core::mem::transmute_copy(&pfisoplockbroken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOplockEventHandle<Impl: IOpLockStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phoplockev: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOplockEventHandle(::core::mem::transmute_copy(&phoplockev)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpLockStatus>, base.5, IsOplockValid::<Impl, OFFSET>, IsOplockBroken::<Impl, OFFSET>, GetOplockEventHandle::<Impl, OFFSET>)
    }
}
pub trait IOpenRowsetImpl: Sized {
    fn OpenRowset();
}
impl ::windows::core::RuntimeName for IOpenRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IOpenRowset";
}
impl IOpenRowsetVtbl {
    pub const fn new<Impl: IOpenRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpenRowsetVtbl {
        unsafe extern "system" fn OpenRowset<Impl: IOpenRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenRowset(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpenRowset>, base.5, OpenRowset::<Impl, OFFSET>)
    }
}
pub trait IParentRowsetImpl: Sized {
    fn GetChildRowset();
}
impl ::windows::core::RuntimeName for IParentRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IParentRowset";
}
impl IParentRowsetVtbl {
    pub const fn new<Impl: IParentRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IParentRowsetVtbl {
        unsafe extern "system" fn GetChildRowset<Impl: IParentRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChildRowset(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), iordinal, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprowset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IParentRowset>, base.5, GetChildRowset::<Impl, OFFSET>)
    }
}
pub trait IProtocolHandlerSiteImpl: Sized {
    fn GetFilter();
}
impl ::windows::core::RuntimeName for IProtocolHandlerSite {
    const NAME: &'static str = "Windows.Win32.System.Search.IProtocolHandlerSite";
}
impl IProtocolHandlerSiteVtbl {
    pub const fn new<Impl: IProtocolHandlerSiteImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProtocolHandlerSiteVtbl {
        unsafe extern "system" fn GetFilter<Impl: IProtocolHandlerSiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsidobj: *mut ::windows::core::GUID, pcwszcontenttype: super::super::Foundation::PWSTR, pcwszextension: super::super::Foundation::PWSTR, ppfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilter(
                &*(&pclsidobj as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pcwszcontenttype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcwszextension as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppfilter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProtocolHandlerSite>, base.5, GetFilter::<Impl, OFFSET>)
    }
}
pub trait IProvideMonikerImpl: Sized {
    fn GetMoniker();
}
impl ::windows::core::RuntimeName for IProvideMoniker {
    const NAME: &'static str = "Windows.Win32.System.Search.IProvideMoniker";
}
impl IProvideMonikerVtbl {
    pub const fn new<Impl: IProvideMonikerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProvideMonikerVtbl {
        unsafe extern "system" fn GetMoniker<Impl: IProvideMonikerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMoniker(::core::mem::transmute_copy(&ppimoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProvideMoniker>, base.5, GetMoniker::<Impl, OFFSET>)
    }
}
pub trait IQueryParserImpl: Sized {
    fn Parse();
    fn SetOption();
    fn GetOption();
    fn SetMultiOption();
    fn GetSchemaProvider();
    fn RestateToString();
    fn ParsePropertyValue();
    fn RestatePropertyValueToString();
}
impl ::windows::core::RuntimeName for IQueryParser {
    const NAME: &'static str = "Windows.Win32.System.Search.IQueryParser";
}
impl IQueryParserVtbl {
    pub const fn new<Impl: IQueryParserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQueryParserVtbl {
        unsafe extern "system" fn Parse<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinputstring: super::super::Foundation::PWSTR, pcustomproperties: ::windows::core::RawPtr, ppsolution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&pszinputstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcustomproperties as *const <super::Com::IEnumUnknown as ::windows::core::Abi>::Abi as *const <super::Com::IEnumUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOption(option, &*(&poptionvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOption<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOption(option, ::core::mem::transmute_copy(&poptionvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiOption<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: super::super::Foundation::PWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMultiOption(option, &*(&pszoptionkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&poptionvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemaProvider<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppschemaprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSchemaProvider(::core::mem::transmute_copy(&ppschemaprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestateToString<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, fuseenglish: super::super::Foundation::BOOL, ppszquerystring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestateToString(&*(&pcondition as *const <ICondition as ::windows::core::Abi>::Abi as *const <ICondition as ::windows::core::DefaultType>::DefaultType), &*(&fuseenglish as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszquerystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParsePropertyValue<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, pszinputstring: super::super::Foundation::PWSTR, ppsolution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParsePropertyValue(&*(&pszpropertyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszinputstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestatePropertyValueToString<Impl: IQueryParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut super::super::Foundation::PWSTR, ppszquerystring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestatePropertyValueToString(&*(&pcondition as *const <ICondition as ::windows::core::Abi>::Abi as *const <ICondition as ::windows::core::DefaultType>::DefaultType), &*(&fuseenglish as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&ppszquerystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQueryParser>, base.5, Parse::<Impl, OFFSET>, SetOption::<Impl, OFFSET>, GetOption::<Impl, OFFSET>, SetMultiOption::<Impl, OFFSET>, GetSchemaProvider::<Impl, OFFSET>, RestateToString::<Impl, OFFSET>, ParsePropertyValue::<Impl, OFFSET>, RestatePropertyValueToString::<Impl, OFFSET>)
    }
}
pub trait IQueryParserManagerImpl: Sized {
    fn CreateLoadedParser();
    fn InitializeOptions();
    fn SetOption();
}
impl ::windows::core::RuntimeName for IQueryParserManager {
    const NAME: &'static str = "Windows.Win32.System.Search.IQueryParserManager";
}
impl IQueryParserManagerVtbl {
    pub const fn new<Impl: IQueryParserManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQueryParserManagerVtbl {
        unsafe extern "system" fn CreateLoadedParser<Impl: IQueryParserManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR, langidforkeywords: u16, riid: *const ::windows::core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLoadedParser(&*(&pszcatalog as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), langidforkeywords, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppqueryparser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeOptions<Impl: IQueryParserManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeOptions(
                &*(&funderstandnqs as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fautowildcard as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pqueryparser as *const <IQueryParser as ::windows::core::Abi>::Abi as *const <IQueryParser as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Impl: IQueryParserManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOption(option, &*(&poptionvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQueryParserManager>, base.5, CreateLoadedParser::<Impl, OFFSET>, InitializeOptions::<Impl, OFFSET>, SetOption::<Impl, OFFSET>)
    }
}
pub trait IQuerySolutionImpl: Sized + IConditionFactoryImpl {
    fn GetQuery();
    fn GetErrors();
    fn GetLexicalData();
}
impl ::windows::core::RuntimeName for IQuerySolution {
    const NAME: &'static str = "Windows.Win32.System.Search.IQuerySolution";
}
impl IQuerySolutionVtbl {
    pub const fn new<Impl: IQuerySolutionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQuerySolutionVtbl {
        unsafe extern "system" fn GetQuery<Impl: IQuerySolutionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppquerynode: *mut ::windows::core::RawPtr, ppmaintype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuery(::core::mem::transmute_copy(&ppquerynode), ::core::mem::transmute_copy(&ppmaintype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrors<Impl: IQuerySolutionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrors(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppparseerrors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLexicalData<Impl: IQuerySolutionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszinputstring: *mut super::super::Foundation::PWSTR, pptokens: *mut ::windows::core::RawPtr, plcid: *mut u32, ppwordbreaker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLexicalData(::core::mem::transmute_copy(&ppszinputstring), ::core::mem::transmute_copy(&pptokens), ::core::mem::transmute_copy(&plcid), ::core::mem::transmute_copy(&ppwordbreaker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQuerySolution>, base.5, GetQuery::<Impl, OFFSET>, GetErrors::<Impl, OFFSET>, GetLexicalData::<Impl, OFFSET>)
    }
}
pub trait IReadDataImpl: Sized {
    fn ReadData();
    fn ReleaseChapter();
}
impl ::windows::core::RuntimeName for IReadData {
    const NAME: &'static str = "Windows.Win32.System.Search.IReadData";
}
impl IReadDataVtbl {
    pub const fn new<Impl: IReadDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReadDataVtbl {
        unsafe extern "system" fn ReadData<Impl: IReadDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: usize, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadData(hchapter, cbbookmark, pbookmark, lrowsoffset, haccessor, crows, pcrowsobtained, ppfixeddata, pcbvariabletotal, ppvariabledata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseChapter<Impl: IReadDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseChapter(hchapter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReadData>, base.5, ReadData::<Impl, OFFSET>, ReleaseChapter::<Impl, OFFSET>)
    }
}
pub trait IRegisterProviderImpl: Sized {
    fn GetURLMapping();
    fn SetURLMapping();
    fn UnregisterProvider();
}
impl ::windows::core::RuntimeName for IRegisterProvider {
    const NAME: &'static str = "Windows.Win32.System.Search.IRegisterProvider";
}
impl IRegisterProviderVtbl {
    pub const fn new<Impl: IRegisterProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRegisterProviderVtbl {
        unsafe extern "system" fn GetURLMapping<Impl: IRegisterProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, pclsidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetURLMapping(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwreserved, ::core::mem::transmute_copy(&pclsidprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURLMapping<Impl: IRegisterProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetURLMapping(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwreserved, &*(&rclsidprovider as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterProvider<Impl: IRegisterProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterProvider(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwreserved, &*(&rclsidprovider as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRegisterProvider>, base.5, GetURLMapping::<Impl, OFFSET>, SetURLMapping::<Impl, OFFSET>, UnregisterProvider::<Impl, OFFSET>)
    }
}
pub trait IRelationshipImpl: Sized {
    fn Name();
    fn IsReal();
    fn Destination();
    fn MetaData();
    fn DefaultPhrase();
}
impl ::windows::core::RuntimeName for IRelationship {
    const NAME: &'static str = "Windows.Win32.System.Search.IRelationship";
}
impl IRelationshipVtbl {
    pub const fn new<Impl: IRelationshipImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRelationshipVtbl {
        unsafe extern "system" fn Name<Impl: IRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReal<Impl: IRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisreal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReal(::core::mem::transmute_copy(&pisreal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Impl: IRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Destination(::core::mem::transmute_copy(&pdestinationentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Impl: IRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MetaData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Impl: IRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultPhrase(::core::mem::transmute_copy(&ppszphrase)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRelationship>, base.5, Name::<Impl, OFFSET>, IsReal::<Impl, OFFSET>, Destination::<Impl, OFFSET>, MetaData::<Impl, OFFSET>, DefaultPhrase::<Impl, OFFSET>)
    }
}
pub trait IRichChunkImpl: Sized {
    fn GetData();
}
impl ::windows::core::RuntimeName for IRichChunk {
    const NAME: &'static str = "Windows.Win32.System.Search.IRichChunk";
}
impl IRichChunkVtbl {
    pub const fn new<Impl: IRichChunkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRichChunkVtbl {
        unsafe extern "system" fn GetData<Impl: IRichChunkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut super::super::Foundation::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(::core::mem::transmute_copy(&pfirstpos), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRichChunk>, base.5, GetData::<Impl, OFFSET>)
    }
}
pub trait IRowImpl: Sized {
    fn GetColumns();
    fn GetSourceRowset();
    fn Open();
}
impl ::windows::core::RuntimeName for IRow {
    const NAME: &'static str = "Windows.Win32.System.Search.IRow";
}
impl IRowVtbl {
    pub const fn new<Impl: IRowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowVtbl {
        unsafe extern "system" fn GetColumns<Impl: IRowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColumns(ccolumns, &*(&rgcolumns as *const <DBCOLUMNACCESS as ::windows::core::Abi>::Abi as *const <DBCOLUMNACCESS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceRowset<Impl: IRowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceRowset(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprowset), ::core::mem::transmute_copy(&phrow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IRowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows::core::GUID, dwbindflags: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pcolumnid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&rguidcolumntype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwbindflags,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppunk),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRow>, base.5, GetColumns::<Impl, OFFSET>, GetSourceRowset::<Impl, OFFSET>, Open::<Impl, OFFSET>)
    }
}
pub trait IRowChangeImpl: Sized {
    fn SetColumns();
}
impl ::windows::core::RuntimeName for IRowChange {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowChange";
}
impl IRowChangeVtbl {
    pub const fn new<Impl: IRowChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowChangeVtbl {
        unsafe extern "system" fn SetColumns<Impl: IRowChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColumns(ccolumns, &*(&rgcolumns as *const <DBCOLUMNACCESS as ::windows::core::Abi>::Abi as *const <DBCOLUMNACCESS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowChange>, base.5, SetColumns::<Impl, OFFSET>)
    }
}
pub trait IRowPositionImpl: Sized {
    fn ClearRowPosition();
    fn GetRowPosition();
    fn GetRowset();
    fn Initialize();
    fn SetRowPosition();
}
impl ::windows::core::RuntimeName for IRowPosition {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowPosition";
}
impl IRowPositionVtbl {
    pub const fn new<Impl: IRowPositionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowPositionVtbl {
        unsafe extern "system" fn ClearRowPosition<Impl: IRowPositionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearRowPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowPosition<Impl: IRowPositionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowPosition(::core::mem::transmute_copy(&phchapter), ::core::mem::transmute_copy(&phrow), ::core::mem::transmute_copy(&pdwpositionflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowset<Impl: IRowPositionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowset(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprowset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IRowPositionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&prowset as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRowPosition<Impl: IRowPositionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRowPosition(hchapter, hrow, dwpositionflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowPosition>, base.5, ClearRowPosition::<Impl, OFFSET>, GetRowPosition::<Impl, OFFSET>, GetRowset::<Impl, OFFSET>, Initialize::<Impl, OFFSET>, SetRowPosition::<Impl, OFFSET>)
    }
}
pub trait IRowPositionChangeImpl: Sized {
    fn OnRowPositionChange();
}
impl ::windows::core::RuntimeName for IRowPositionChange {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowPositionChange";
}
impl IRowPositionChangeVtbl {
    pub const fn new<Impl: IRowPositionChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowPositionChangeVtbl {
        unsafe extern "system" fn OnRowPositionChange<Impl: IRowPositionChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRowPositionChange(ereason, ephase, &*(&fcantdeny as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowPositionChange>, base.5, OnRowPositionChange::<Impl, OFFSET>)
    }
}
pub trait IRowSchemaChangeImpl: Sized + IRowChangeImpl {
    fn DeleteColumns();
    fn AddColumns();
}
impl ::windows::core::RuntimeName for IRowSchemaChange {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowSchemaChange";
}
impl IRowSchemaChangeVtbl {
    pub const fn new<Impl: IRowSchemaChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowSchemaChangeVtbl {
        unsafe extern "system" fn DeleteColumns<Impl: IRowSchemaChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteColumns(ccolumns, &*(&rgcolumnids as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), rgdwstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddColumns<Impl: IRowSchemaChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddColumns(ccolumns, &*(&rgnewcolumninfo as *const <DBCOLUMNINFO as ::windows::core::Abi>::Abi as *const <DBCOLUMNINFO as ::windows::core::DefaultType>::DefaultType), &*(&rgcolumns as *const <DBCOLUMNACCESS as ::windows::core::Abi>::Abi as *const <DBCOLUMNACCESS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowSchemaChange>, base.5, DeleteColumns::<Impl, OFFSET>, AddColumns::<Impl, OFFSET>)
    }
}
pub trait IRowsetImpl: Sized {
    fn AddRefRows();
    fn GetData();
    fn GetNextRows();
    fn ReleaseRows();
    fn RestartPosition();
}
impl ::windows::core::RuntimeName for IRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowset";
}
impl IRowsetVtbl {
    pub const fn new<Impl: IRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetVtbl {
        unsafe extern "system" fn AddRefRows<Impl: IRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRefRows(crows, rghrows, rgrefcounts, rgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: IRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(hrow, haccessor, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextRows<Impl: IRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextRows(hreserved, lrowsoffset, crows, pcrowsobtained, prghrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseRows<Impl: IRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseRows(crows, rghrows, rgrowoptions, rgrefcounts, rgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestartPosition<Impl: IRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestartPosition(hreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowset>, base.5, AddRefRows::<Impl, OFFSET>, GetData::<Impl, OFFSET>, GetNextRows::<Impl, OFFSET>, ReleaseRows::<Impl, OFFSET>, RestartPosition::<Impl, OFFSET>)
    }
}
pub trait IRowsetAsynchImpl: Sized {
    fn RatioFinished();
    fn Stop();
}
impl ::windows::core::RuntimeName for IRowsetAsynch {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetAsynch";
}
impl IRowsetAsynchVtbl {
    pub const fn new<Impl: IRowsetAsynchImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetAsynchVtbl {
        unsafe extern "system" fn RatioFinished<Impl: IRowsetAsynchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RatioFinished(puldenominator, pulnumerator, pcrows, &*(&pfnewrows as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IRowsetAsynchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetAsynch>, base.5, RatioFinished::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
pub trait IRowsetBookmarkImpl: Sized {
    fn PositionOnBookmark();
}
impl ::windows::core::RuntimeName for IRowsetBookmark {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetBookmark";
}
impl IRowsetBookmarkVtbl {
    pub const fn new<Impl: IRowsetBookmarkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetBookmarkVtbl {
        unsafe extern "system" fn PositionOnBookmark<Impl: IRowsetBookmarkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionOnBookmark(hchapter, cbbookmark, pbookmark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetBookmark>, base.5, PositionOnBookmark::<Impl, OFFSET>)
    }
}
pub trait IRowsetChangeImpl: Sized {
    fn DeleteRows();
    fn SetData();
    fn InsertRow();
}
impl ::windows::core::RuntimeName for IRowsetChange {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetChange";
}
impl IRowsetChangeVtbl {
    pub const fn new<Impl: IRowsetChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetChangeVtbl {
        unsafe extern "system" fn DeleteRows<Impl: IRowsetChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteRows(hreserved, crows, rghrows, rgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IRowsetChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetData(hrow, haccessor, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertRow<Impl: IRowsetChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertRow(hreserved, haccessor, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), phrow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetChange>, base.5, DeleteRows::<Impl, OFFSET>, SetData::<Impl, OFFSET>, InsertRow::<Impl, OFFSET>)
    }
}
pub trait IRowsetChangeExtInfoImpl: Sized {
    fn GetOriginalRow();
    fn GetPendingColumns();
}
impl ::windows::core::RuntimeName for IRowsetChangeExtInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetChangeExtInfo";
}
impl IRowsetChangeExtInfoVtbl {
    pub const fn new<Impl: IRowsetChangeExtInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetChangeExtInfoVtbl {
        unsafe extern "system" fn GetOriginalRow<Impl: IRowsetChangeExtInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOriginalRow(hreserved, hrow, phroworiginal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPendingColumns<Impl: IRowsetChangeExtInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPendingColumns(hreserved, hrow, ccolumnordinals, rgiordinals, rgcolumnstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetChangeExtInfo>, base.5, GetOriginalRow::<Impl, OFFSET>, GetPendingColumns::<Impl, OFFSET>)
    }
}
pub trait IRowsetChapterMemberImpl: Sized {
    fn IsRowInChapter();
}
impl ::windows::core::RuntimeName for IRowsetChapterMember {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetChapterMember";
}
impl IRowsetChapterMemberVtbl {
    pub const fn new<Impl: IRowsetChapterMemberImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetChapterMemberVtbl {
        unsafe extern "system" fn IsRowInChapter<Impl: IRowsetChapterMemberImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRowInChapter(hchapter, hrow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetChapterMember>, base.5, IsRowInChapter::<Impl, OFFSET>)
    }
}
pub trait IRowsetCopyRowsImpl: Sized {
    fn CloseSource();
    fn CopyByHROWS();
    fn CopyRows();
    fn DefineSource();
}
impl ::windows::core::RuntimeName for IRowsetCopyRows {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetCopyRows";
}
impl IRowsetCopyRowsVtbl {
    pub const fn new<Impl: IRowsetCopyRowsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetCopyRowsVtbl {
        unsafe extern "system" fn CloseSource<Impl: IRowsetCopyRowsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsourceid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseSource(hsourceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyByHROWS<Impl: IRowsetCopyRowsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyByHROWS(hsourceid, hreserved, crows, rghrows, bflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyRows<Impl: IRowsetCopyRowsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyRows(hsourceid, hreserved, crows, bflags, pcrowscopied) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefineSource<Impl: IRowsetCopyRowsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowsetsource: ::windows::core::RawPtr, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefineSource(&*(&prowsetsource as *const <IRowset as ::windows::core::Abi>::Abi as *const <IRowset as ::windows::core::DefaultType>::DefaultType), ccolids, rgsourcecolumns, rgtargetcolumns, phsourceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetCopyRows>, base.5, CloseSource::<Impl, OFFSET>, CopyByHROWS::<Impl, OFFSET>, CopyRows::<Impl, OFFSET>, DefineSource::<Impl, OFFSET>)
    }
}
pub trait IRowsetCurrentIndexImpl: Sized + IRowsetIndexImpl {
    fn GetIndex();
    fn SetIndex();
}
impl ::windows::core::RuntimeName for IRowsetCurrentIndex {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetCurrentIndex";
}
impl IRowsetCurrentIndexVtbl {
    pub const fn new<Impl: IRowsetCurrentIndexImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetCurrentIndexVtbl {
        unsafe extern "system" fn GetIndex<Impl: IRowsetCurrentIndexImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndex(&*(&ppindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Impl: IRowsetCurrentIndexImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIndex(&*(&pindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetCurrentIndex>, base.5, GetIndex::<Impl, OFFSET>, SetIndex::<Impl, OFFSET>)
    }
}
pub trait IRowsetEventsImpl: Sized {
    fn OnNewItem();
    fn OnChangedItem();
    fn OnDeletedItem();
    fn OnRowsetEvent();
}
impl ::windows::core::RuntimeName for IRowsetEvents {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetEvents";
}
impl IRowsetEventsVtbl {
    pub const fn new<Impl: IRowsetEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetEventsVtbl {
        unsafe extern "system" fn OnNewItem<Impl: IRowsetEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnNewItem(&*(&itemid as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), newitemstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChangedItem<Impl: IRowsetEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChangedItem(&*(&itemid as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), rowsetitemstate, changeditemstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDeletedItem<Impl: IRowsetEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDeletedItem(&*(&itemid as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), deleteditemstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnRowsetEvent<Impl: IRowsetEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRowsetEvent(eventtype, &*(&eventdata as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetEvents>, base.5, OnNewItem::<Impl, OFFSET>, OnChangedItem::<Impl, OFFSET>, OnDeletedItem::<Impl, OFFSET>, OnRowsetEvent::<Impl, OFFSET>)
    }
}
pub trait IRowsetFastLoadImpl: Sized {
    fn InsertRow();
    fn Commit();
}
impl ::windows::core::RuntimeName for IRowsetFastLoad {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetFastLoad";
}
impl IRowsetFastLoadVtbl {
    pub const fn new<Impl: IRowsetFastLoadImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetFastLoadVtbl {
        unsafe extern "system" fn InsertRow<Impl: IRowsetFastLoadImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertRow(haccessor, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IRowsetFastLoadImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit(&*(&fdone as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetFastLoad>, base.5, InsertRow::<Impl, OFFSET>, Commit::<Impl, OFFSET>)
    }
}
pub trait IRowsetFindImpl: Sized {
    fn FindNextRow();
}
impl ::windows::core::RuntimeName for IRowsetFind {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetFind";
}
impl IRowsetFindVtbl {
    pub const fn new<Impl: IRowsetFindImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetFindVtbl {
        unsafe extern "system" fn FindNextRow<Impl: IRowsetFindImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, haccessor: usize, pfindvalue: *mut ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNextRow(hchapter, haccessor, &*(&pfindvalue as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), compareop, cbbookmark, pbookmark, lrowsoffset, crows, pcrowsobtained, prghrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetFind>, base.5, FindNextRow::<Impl, OFFSET>)
    }
}
pub trait IRowsetIdentityImpl: Sized {
    fn IsSameRow();
}
impl ::windows::core::RuntimeName for IRowsetIdentity {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetIdentity";
}
impl IRowsetIdentityVtbl {
    pub const fn new<Impl: IRowsetIdentityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetIdentityVtbl {
        unsafe extern "system" fn IsSameRow<Impl: IRowsetIdentityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hthisrow: usize, hthatrow: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSameRow(hthisrow, hthatrow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetIdentity>, base.5, IsSameRow::<Impl, OFFSET>)
    }
}
pub trait IRowsetIndexImpl: Sized {
    fn GetIndexInfo();
    fn Seek();
    fn SetRange();
}
impl ::windows::core::RuntimeName for IRowsetIndex {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetIndex";
}
impl IRowsetIndexVtbl {
    pub const fn new<Impl: IRowsetIndexImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetIndexVtbl {
        unsafe extern "system" fn GetIndexInfo<Impl: IRowsetIndexImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndexInfo(pckeycolumns, &*(&prgindexcolumndesc as *const <DBINDEXCOLUMNDESC as ::windows::core::Abi>::Abi as *const <DBINDEXCOLUMNDESC as ::windows::core::DefaultType>::DefaultType), pcindexpropertysets, &*(&prgindexpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IRowsetIndexImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, ckeyvalues: usize, pdata: *mut ::core::ffi::c_void, dwseekoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Seek(haccessor, ckeyvalues, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwseekoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRange<Impl: IRowsetIndexImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, cstartkeycolumns: usize, pstartdata: *mut ::core::ffi::c_void, cendkeycolumns: usize, penddata: *mut ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRange(haccessor, cstartkeycolumns, &*(&pstartdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cendkeycolumns, &*(&penddata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwrangeoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetIndex>, base.5, GetIndexInfo::<Impl, OFFSET>, Seek::<Impl, OFFSET>, SetRange::<Impl, OFFSET>)
    }
}
pub trait IRowsetInfoImpl: Sized {
    fn GetProperties();
    fn GetReferencedRowset();
    fn GetSpecification();
}
impl ::windows::core::RuntimeName for IRowsetInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetInfo";
}
impl IRowsetInfoVtbl {
    pub const fn new<Impl: IRowsetInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetInfoVtbl {
        unsafe extern "system" fn GetProperties<Impl: IRowsetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperties(cpropertyidsets, &*(&rgpropertyidsets as *const <DBPROPIDSET as ::windows::core::Abi>::Abi as *const <DBPROPIDSET as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReferencedRowset<Impl: IRowsetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows::core::GUID, ppreferencedrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReferencedRowset(iordinal, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppreferencedrowset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecification<Impl: IRowsetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpecification(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppspecification)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetInfo>, base.5, GetProperties::<Impl, OFFSET>, GetReferencedRowset::<Impl, OFFSET>, GetSpecification::<Impl, OFFSET>)
    }
}
pub trait IRowsetKeysImpl: Sized {
    fn ListKeys();
}
impl ::windows::core::RuntimeName for IRowsetKeys {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetKeys";
}
impl IRowsetKeysVtbl {
    pub const fn new<Impl: IRowsetKeysImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetKeysVtbl {
        unsafe extern "system" fn ListKeys<Impl: IRowsetKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListKeys(pccolumns, prgcolumns) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetKeys>, base.5, ListKeys::<Impl, OFFSET>)
    }
}
pub trait IRowsetLocateImpl: Sized + IRowsetImpl {
    fn Compare();
    fn GetRowsAt();
    fn GetRowsByBookmark();
    fn Hash();
}
impl ::windows::core::RuntimeName for IRowsetLocate {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetLocate";
}
impl IRowsetLocateVtbl {
    pub const fn new<Impl: IRowsetLocateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetLocateVtbl {
        unsafe extern "system" fn Compare<Impl: IRowsetLocateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compare(hreserved, cbbookmark1, pbookmark1, cbbookmark2, pbookmark2, pcomparison) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowsAt<Impl: IRowsetLocateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowsAt(hreserved1, hreserved2, cbbookmark, pbookmark, lrowsoffset, crows, pcrowsobtained, prghrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowsByBookmark<Impl: IRowsetLocateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowsByBookmark(hreserved, crows, rgcbbookmarks, rgpbookmarks, rghrows, rgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hash<Impl: IRowsetLocateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hash(hreserved, cbookmarks, rgcbbookmarks, rgpbookmarks, rghashedvalues, rgbookmarkstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetLocate>, base.5, Compare::<Impl, OFFSET>, GetRowsAt::<Impl, OFFSET>, GetRowsByBookmark::<Impl, OFFSET>, Hash::<Impl, OFFSET>)
    }
}
pub trait IRowsetNewRowAfterImpl: Sized {
    fn SetNewDataAfter();
}
impl ::windows::core::RuntimeName for IRowsetNewRowAfter {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetNewRowAfter";
}
impl IRowsetNewRowAfterVtbl {
    pub const fn new<Impl: IRowsetNewRowAfterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetNewRowAfterVtbl {
        unsafe extern "system" fn SetNewDataAfter<Impl: IRowsetNewRowAfterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: usize, pdata: *mut u8, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNewDataAfter(hchapter, cbbmprevious, pbmprevious, haccessor, pdata, phrow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetNewRowAfter>, base.5, SetNewDataAfter::<Impl, OFFSET>)
    }
}
pub trait IRowsetNextRowsetImpl: Sized {
    fn GetNextRowset();
}
impl ::windows::core::RuntimeName for IRowsetNextRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetNextRowset";
}
impl IRowsetNextRowsetVtbl {
    pub const fn new<Impl: IRowsetNextRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetNextRowsetVtbl {
        unsafe extern "system" fn GetNextRowset<Impl: IRowsetNextRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppnextrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextRowset(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnextrowset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetNextRowset>, base.5, GetNextRowset::<Impl, OFFSET>)
    }
}
pub trait IRowsetNotifyImpl: Sized {
    fn OnFieldChange();
    fn OnRowChange();
    fn OnRowsetChange();
}
impl ::windows::core::RuntimeName for IRowsetNotify {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetNotify";
}
impl IRowsetNotifyVtbl {
    pub const fn new<Impl: IRowsetNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetNotifyVtbl {
        unsafe extern "system" fn OnFieldChange<Impl: IRowsetNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnFieldChange(&*(&prowset as *const <IRowset as ::windows::core::Abi>::Abi as *const <IRowset as ::windows::core::DefaultType>::DefaultType), hrow, ccolumns, rgcolumns, ereason, ephase, &*(&fcantdeny as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnRowChange<Impl: IRowsetNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRowChange(&*(&prowset as *const <IRowset as ::windows::core::Abi>::Abi as *const <IRowset as ::windows::core::DefaultType>::DefaultType), crows, rghrows, ereason, ephase, &*(&fcantdeny as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnRowsetChange<Impl: IRowsetNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRowsetChange(&*(&prowset as *const <IRowset as ::windows::core::Abi>::Abi as *const <IRowset as ::windows::core::DefaultType>::DefaultType), ereason, ephase, &*(&fcantdeny as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetNotify>, base.5, OnFieldChange::<Impl, OFFSET>, OnRowChange::<Impl, OFFSET>, OnRowsetChange::<Impl, OFFSET>)
    }
}
pub trait IRowsetPrioritizationImpl: Sized {
    fn SetScopePriority();
    fn GetScopePriority();
    fn GetScopeStatistics();
}
impl ::windows::core::RuntimeName for IRowsetPrioritization {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetPrioritization";
}
impl IRowsetPrioritizationVtbl {
    pub const fn new<Impl: IRowsetPrioritizationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetPrioritizationVtbl {
        unsafe extern "system" fn SetScopePriority<Impl: IRowsetPrioritizationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScopePriority(priority, scopestatisticseventfrequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopePriority<Impl: IRowsetPrioritizationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScopePriority(::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&scopestatisticseventfrequency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopeStatistics<Impl: IRowsetPrioritizationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScopeStatistics(::core::mem::transmute_copy(&indexeddocumentcount), ::core::mem::transmute_copy(&oustandingaddcount), ::core::mem::transmute_copy(&oustandingmodifycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetPrioritization>, base.5, SetScopePriority::<Impl, OFFSET>, GetScopePriority::<Impl, OFFSET>, GetScopeStatistics::<Impl, OFFSET>)
    }
}
pub trait IRowsetQueryStatusImpl: Sized {
    fn GetStatus();
    fn GetStatusEx();
}
impl ::windows::core::RuntimeName for IRowsetQueryStatus {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetQueryStatus";
}
impl IRowsetQueryStatusVtbl {
    pub const fn new<Impl: IRowsetQueryStatusImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetQueryStatusVtbl {
        unsafe extern "system" fn GetStatus<Impl: IRowsetQueryStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(pdwstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusEx<Impl: IRowsetQueryStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatusEx(pdwstatus, pcfiltereddocuments, pcdocumentstofilter, pdwratiofinisheddenominator, pdwratiofinishednumerator, cbbmk, pbmk, pirowbmk, pcrowstotal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetQueryStatus>, base.5, GetStatus::<Impl, OFFSET>, GetStatusEx::<Impl, OFFSET>)
    }
}
pub trait IRowsetRefreshImpl: Sized {
    fn RefreshVisibleData();
    fn GetLastVisibleData();
}
impl ::windows::core::RuntimeName for IRowsetRefresh {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetRefresh";
}
impl IRowsetRefreshVtbl {
    pub const fn new<Impl: IRowsetRefreshImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetRefreshVtbl {
        unsafe extern "system" fn RefreshVisibleData<Impl: IRowsetRefreshImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RefreshVisibleData(hchapter, crows, rghrows, &*(&foverwrite as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), pcrowsrefreshed, prghrowsrefreshed, prgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastVisibleData<Impl: IRowsetRefreshImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastVisibleData(hrow, haccessor, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetRefresh>, base.5, RefreshVisibleData::<Impl, OFFSET>, GetLastVisibleData::<Impl, OFFSET>)
    }
}
pub trait IRowsetResynchImpl: Sized {
    fn GetVisibleData();
    fn ResynchRows();
}
impl ::windows::core::RuntimeName for IRowsetResynch {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetResynch";
}
impl IRowsetResynchVtbl {
    pub const fn new<Impl: IRowsetResynchImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetResynchVtbl {
        unsafe extern "system" fn GetVisibleData<Impl: IRowsetResynchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisibleData(hrow, haccessor, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResynchRows<Impl: IRowsetResynchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResynchRows(crows, rghrows, pcrowsresynched, prghrowsresynched, prgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetResynch>, base.5, GetVisibleData::<Impl, OFFSET>, ResynchRows::<Impl, OFFSET>)
    }
}
pub trait IRowsetScrollImpl: Sized + IRowsetLocateImpl + IRowsetImpl {
    fn GetApproximatePosition();
    fn GetRowsAtRatio();
}
impl ::windows::core::RuntimeName for IRowsetScroll {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetScroll";
}
impl IRowsetScrollVtbl {
    pub const fn new<Impl: IRowsetScrollImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetScrollVtbl {
        unsafe extern "system" fn GetApproximatePosition<Impl: IRowsetScrollImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetApproximatePosition(hreserved, cbbookmark, pbookmark, pulposition, pcrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowsAtRatio<Impl: IRowsetScrollImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowsAtRatio(hreserved1, hreserved2, ulnumerator, uldenominator, crows, pcrowsobtained, prghrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetScroll>, base.5, GetApproximatePosition::<Impl, OFFSET>, GetRowsAtRatio::<Impl, OFFSET>)
    }
}
pub trait IRowsetUpdateImpl: Sized + IRowsetChangeImpl {
    fn GetOriginalData();
    fn GetPendingRows();
    fn GetRowStatus();
    fn Undo();
    fn Update();
}
impl ::windows::core::RuntimeName for IRowsetUpdate {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetUpdate";
}
impl IRowsetUpdateVtbl {
    pub const fn new<Impl: IRowsetUpdateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetUpdateVtbl {
        unsafe extern "system" fn GetOriginalData<Impl: IRowsetUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOriginalData(hrow, haccessor, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPendingRows<Impl: IRowsetUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPendingRows(hreserved, dwrowstatus, pcpendingrows, prgpendingrows, prgpendingstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowStatus<Impl: IRowsetUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRowStatus(hreserved, crows, rghrows, rgpendingstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Undo<Impl: IRowsetUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Undo(hreserved, crows, rghrows, pcrowsundone, prgrowsundone, prgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IRowsetUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Update(hreserved, crows, rghrows, pcrows, prgrows, prgrowstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetUpdate>, base.5, GetOriginalData::<Impl, OFFSET>, GetPendingRows::<Impl, OFFSET>, GetRowStatus::<Impl, OFFSET>, Undo::<Impl, OFFSET>, Update::<Impl, OFFSET>)
    }
}
pub trait IRowsetViewImpl: Sized {
    fn CreateView();
    fn GetView();
}
impl ::windows::core::RuntimeName for IRowsetView {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetView";
}
impl IRowsetViewVtbl {
    pub const fn new<Impl: IRowsetViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetViewVtbl {
        unsafe extern "system" fn CreateView<Impl: IRowsetViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateView(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Impl: IRowsetViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, riid: *const ::windows::core::GUID, phchaptersource: *mut usize, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetView(hchapter, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phchaptersource), ::core::mem::transmute_copy(&ppview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetView>, base.5, CreateView::<Impl, OFFSET>, GetView::<Impl, OFFSET>)
    }
}
pub trait IRowsetWatchAllImpl: Sized {
    fn Acknowledge();
    fn Start();
    fn StopWatching();
}
impl ::windows::core::RuntimeName for IRowsetWatchAll {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetWatchAll";
}
impl IRowsetWatchAllVtbl {
    pub const fn new<Impl: IRowsetWatchAllImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetWatchAllVtbl {
        unsafe extern "system" fn Acknowledge<Impl: IRowsetWatchAllImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Acknowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IRowsetWatchAllImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopWatching<Impl: IRowsetWatchAllImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopWatching() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetWatchAll>, base.5, Acknowledge::<Impl, OFFSET>, Start::<Impl, OFFSET>, StopWatching::<Impl, OFFSET>)
    }
}
pub trait IRowsetWatchNotifyImpl: Sized {
    fn OnChange();
}
impl ::windows::core::RuntimeName for IRowsetWatchNotify {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetWatchNotify";
}
impl IRowsetWatchNotifyVtbl {
    pub const fn new<Impl: IRowsetWatchNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetWatchNotifyVtbl {
        unsafe extern "system" fn OnChange<Impl: IRowsetWatchNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, echangereason: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChange(&*(&prowset as *const <IRowset as ::windows::core::Abi>::Abi as *const <IRowset as ::windows::core::DefaultType>::DefaultType), echangereason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetWatchNotify>, base.5, OnChange::<Impl, OFFSET>)
    }
}
pub trait IRowsetWatchRegionImpl: Sized + IRowsetWatchAllImpl {
    fn CreateWatchRegion();
    fn ChangeWatchMode();
    fn DeleteWatchRegion();
    fn GetWatchRegionInfo();
    fn Refresh();
    fn ShrinkWatchRegion();
}
impl ::windows::core::RuntimeName for IRowsetWatchRegion {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetWatchRegion";
}
impl IRowsetWatchRegionVtbl {
    pub const fn new<Impl: IRowsetWatchRegionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetWatchRegionVtbl {
        unsafe extern "system" fn CreateWatchRegion<Impl: IRowsetWatchRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwatchmode: u32, phregion: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWatchRegion(dwwatchmode, phregion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeWatchMode<Impl: IRowsetWatchRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize, dwwatchmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeWatchMode(hregion, dwwatchmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteWatchRegion<Impl: IRowsetWatchRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteWatchRegion(hregion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatchRegionInfo<Impl: IRowsetWatchRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWatchRegionInfo(hregion, pdwwatchmode, phchapter, pcbbookmark, ppbookmark, pcrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IRowsetWatchRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchangesobtained: *mut usize, prgchanges: *mut *mut tagDBROWWATCHRANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh(pcchangesobtained, &*(&prgchanges as *const <tagDBROWWATCHRANGE as ::windows::core::Abi>::Abi as *const <tagDBROWWATCHRANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShrinkWatchRegion<Impl: IRowsetWatchRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *mut u8, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShrinkWatchRegion(hregion, hchapter, cbbookmark, pbookmark, crows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetWatchRegion>, base.5, CreateWatchRegion::<Impl, OFFSET>, ChangeWatchMode::<Impl, OFFSET>, DeleteWatchRegion::<Impl, OFFSET>, GetWatchRegionInfo::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, ShrinkWatchRegion::<Impl, OFFSET>)
    }
}
pub trait IRowsetWithParametersImpl: Sized {
    fn GetParameterInfo();
    fn Requery();
}
impl ::windows::core::RuntimeName for IRowsetWithParameters {
    const NAME: &'static str = "Windows.Win32.System.Search.IRowsetWithParameters";
}
impl IRowsetWithParametersVtbl {
    pub const fn new<Impl: IRowsetWithParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRowsetWithParametersVtbl {
        unsafe extern "system" fn GetParameterInfo<Impl: IRowsetWithParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParameterInfo(pcparams, &*(&prgparaminfo as *const <DBPARAMINFO as ::windows::core::Abi>::Abi as *const <DBPARAMINFO as ::windows::core::DefaultType>::DefaultType), ppnamesbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Requery<Impl: IRowsetWithParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparams: *mut DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Requery(&*(&pparams as *const <DBPARAMS as ::windows::core::Abi>::Abi as *const <DBPARAMS as ::windows::core::DefaultType>::DefaultType), pulerrorparam, phreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRowsetWithParameters>, base.5, GetParameterInfo::<Impl, OFFSET>, Requery::<Impl, OFFSET>)
    }
}
pub trait ISQLErrorInfoImpl: Sized {
    fn GetSQLInfo();
}
impl ::windows::core::RuntimeName for ISQLErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.ISQLErrorInfo";
}
impl ISQLErrorInfoVtbl {
    pub const fn new<Impl: ISQLErrorInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISQLErrorInfoVtbl {
        unsafe extern "system" fn GetSQLInfo<Impl: ISQLErrorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsqlstate: *mut super::super::Foundation::BSTR, plnativeerror: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSQLInfo(::core::mem::transmute_copy(&pbstrsqlstate), ::core::mem::transmute_copy(&plnativeerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISQLErrorInfo>, base.5, GetSQLInfo::<Impl, OFFSET>)
    }
}
pub trait ISQLGetDiagFieldImpl: Sized {
    fn GetDiagField();
}
impl ::windows::core::RuntimeName for ISQLGetDiagField {
    const NAME: &'static str = "Windows.Win32.System.Search.ISQLGetDiagField";
}
impl ISQLGetDiagFieldVtbl {
    pub const fn new<Impl: ISQLGetDiagFieldImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISQLGetDiagFieldVtbl {
        unsafe extern "system" fn GetDiagField<Impl: ISQLGetDiagFieldImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiaginfo: *mut KAGGETDIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDiagField(&*(&pdiaginfo as *const <KAGGETDIAG as ::windows::core::Abi>::Abi as *const <KAGGETDIAG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISQLGetDiagField>, base.5, GetDiagField::<Impl, OFFSET>)
    }
}
pub trait ISQLRequestDiagFieldsImpl: Sized {
    fn RequestDiagFields();
}
impl ::windows::core::RuntimeName for ISQLRequestDiagFields {
    const NAME: &'static str = "Windows.Win32.System.Search.ISQLRequestDiagFields";
}
impl ISQLRequestDiagFieldsVtbl {
    pub const fn new<Impl: ISQLRequestDiagFieldsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISQLRequestDiagFieldsVtbl {
        unsafe extern "system" fn RequestDiagFields<Impl: ISQLRequestDiagFieldsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestDiagFields(cdiagfields, &*(&rgdiagfields as *const <KAGREQDIAG as ::windows::core::Abi>::Abi as *const <KAGREQDIAG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISQLRequestDiagFields>, base.5, RequestDiagFields::<Impl, OFFSET>)
    }
}
pub trait ISQLServerErrorInfoImpl: Sized {
    fn GetErrorInfo();
}
impl ::windows::core::RuntimeName for ISQLServerErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.ISQLServerErrorInfo";
}
impl ISQLServerErrorInfoVtbl {
    pub const fn new<Impl: ISQLServerErrorInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISQLServerErrorInfoVtbl {
        unsafe extern "system" fn GetErrorInfo<Impl: ISQLServerErrorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperrorinfo: *mut *mut tagSSErrorInfo, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorInfo(&*(&pperrorinfo as *const <tagSSErrorInfo as ::windows::core::Abi>::Abi as *const <tagSSErrorInfo as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstringsbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISQLServerErrorInfo>, base.5, GetErrorInfo::<Impl, OFFSET>)
    }
}
pub trait ISchemaLocalizerSupportImpl: Sized {
    fn Localize();
}
impl ::windows::core::RuntimeName for ISchemaLocalizerSupport {
    const NAME: &'static str = "Windows.Win32.System.Search.ISchemaLocalizerSupport";
}
impl ISchemaLocalizerSupportVtbl {
    pub const fn new<Impl: ISchemaLocalizerSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaLocalizerSupportVtbl {
        unsafe extern "system" fn Localize<Impl: ISchemaLocalizerSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszglobalstring: super::super::Foundation::PWSTR, ppszlocalstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Localize(&*(&pszglobalstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszlocalstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaLocalizerSupport>, base.5, Localize::<Impl, OFFSET>)
    }
}
pub trait ISchemaLockImpl: Sized {
    fn GetSchemaLock();
    fn ReleaseSchemaLock();
}
impl ::windows::core::RuntimeName for ISchemaLock {
    const NAME: &'static str = "Windows.Win32.System.Search.ISchemaLock";
}
impl ISchemaLockVtbl {
    pub const fn new<Impl: ISchemaLockImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaLockVtbl {
        unsafe extern "system" fn GetSchemaLock<Impl: ISchemaLockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSchemaLock(&*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), lmmode, &*(&phlockhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ptableversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseSchemaLock<Impl: ISchemaLockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hlockhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseSchemaLock(&*(&hlockhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaLock>, base.5, GetSchemaLock::<Impl, OFFSET>, ReleaseSchemaLock::<Impl, OFFSET>)
    }
}
pub trait ISchemaProviderImpl: Sized {
    fn Entities();
    fn RootEntity();
    fn GetEntity();
    fn MetaData();
    fn Localize();
    fn SaveBinary();
    fn LookupAuthoredNamedEntity();
}
impl ::windows::core::RuntimeName for ISchemaProvider {
    const NAME: &'static str = "Windows.Win32.System.Search.ISchemaProvider";
}
impl ISchemaProviderVtbl {
    pub const fn new<Impl: ISchemaProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaProviderVtbl {
        unsafe extern "system" fn Entities<Impl: ISchemaProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Entities(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pentities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootEntity<Impl: ISchemaProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prootentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RootEntity(::core::mem::transmute_copy(&prootentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntity<Impl: ISchemaProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszentityname: super::super::Foundation::PWSTR, pentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEntity(&*(&pszentityname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Impl: ISchemaProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MetaData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Localize<Impl: ISchemaProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, pschemalocalizersupport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Localize(lcid, &*(&pschemalocalizersupport as *const <ISchemaLocalizerSupport as ::windows::core::Abi>::Abi as *const <ISchemaLocalizerSupport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveBinary<Impl: ISchemaProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszschemabinarypath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveBinary(&*(&pszschemabinarypath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupAuthoredNamedEntity<Impl: ISchemaProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pentity: ::windows::core::RawPtr, pszinputstring: super::super::Foundation::PWSTR, ptokencollection: ::windows::core::RawPtr, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LookupAuthoredNamedEntity(
                &*(&pentity as *const <IEntity as ::windows::core::Abi>::Abi as *const <IEntity as ::windows::core::DefaultType>::DefaultType),
                &*(&pszinputstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptokencollection as *const <ITokenCollection as ::windows::core::Abi>::Abi as *const <ITokenCollection as ::windows::core::DefaultType>::DefaultType),
                ctokensbegin,
                ::core::mem::transmute_copy(&pctokenslength),
                ::core::mem::transmute_copy(&ppszvalue),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaProvider>, base.5, Entities::<Impl, OFFSET>, RootEntity::<Impl, OFFSET>, GetEntity::<Impl, OFFSET>, MetaData::<Impl, OFFSET>, Localize::<Impl, OFFSET>, SaveBinary::<Impl, OFFSET>, LookupAuthoredNamedEntity::<Impl, OFFSET>)
    }
}
pub trait IScopedOperationsImpl: Sized + IBindResourceImpl {
    fn Copy();
    fn Move();
    fn Delete();
    fn OpenRowset();
}
impl ::windows::core::RuntimeName for IScopedOperations {
    const NAME: &'static str = "Windows.Win32.System.Search.IScopedOperations";
}
impl IScopedOperationsVtbl {
    pub const fn new<Impl: IScopedOperationsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScopedOperationsVtbl {
        unsafe extern "system" fn Copy<Impl: IScopedOperationsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const super::super::Foundation::PWSTR, rgpwszdesturls: *const super::super::Foundation::PWSTR, dwcopyflags: u32, pauthenticate: ::windows::core::RawPtr, rgdwstatus: *mut u32, rgpwsznewurls: *mut super::super::Foundation::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy(
                crows,
                &*(&rgpwszsourceurls as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&rgpwszdesturls as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwcopyflags,
                &*(&pauthenticate as *const <super::Com::IAuthenticate as ::windows::core::Abi>::Abi as *const <super::Com::IAuthenticate as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&rgdwstatus),
                ::core::mem::transmute_copy(&rgpwsznewurls),
                ::core::mem::transmute_copy(&ppstringsbuffer),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IScopedOperationsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const super::super::Foundation::PWSTR, rgpwszdesturls: *const super::super::Foundation::PWSTR, dwmoveflags: u32, pauthenticate: ::windows::core::RawPtr, rgdwstatus: *mut u32, rgpwsznewurls: *mut super::super::Foundation::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Move(
                crows,
                &*(&rgpwszsourceurls as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&rgpwszdesturls as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwmoveflags,
                &*(&pauthenticate as *const <super::Com::IAuthenticate as ::windows::core::Abi>::Abi as *const <super::Com::IAuthenticate as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&rgdwstatus),
                ::core::mem::transmute_copy(&rgpwsznewurls),
                ::core::mem::transmute_copy(&ppstringsbuffer),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IScopedOperationsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszurls: *const super::super::Foundation::PWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(crows, &*(&rgpwszurls as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwdeleteflags, ::core::mem::transmute_copy(&rgdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRowset<Impl: IScopedOperationsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenRowset(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScopedOperations>, base.5, Copy::<Impl, OFFSET>, Move::<Impl, OFFSET>, Delete::<Impl, OFFSET>, OpenRowset::<Impl, OFFSET>)
    }
}
pub trait ISearchCatalogManagerImpl: Sized {
    fn Name();
    fn GetParameter();
    fn SetParameter();
    fn GetCatalogStatus();
    fn Reset();
    fn Reindex();
    fn ReindexMatchingURLs();
    fn ReindexSearchRoot();
    fn SetConnectTimeout();
    fn ConnectTimeout();
    fn SetDataTimeout();
    fn DataTimeout();
    fn NumberOfItems();
    fn NumberOfItemsToIndex();
    fn URLBeingIndexed();
    fn GetURLIndexingState();
    fn GetPersistentItemsChangedSink();
    fn RegisterViewForNotification();
    fn GetItemsChangedSink();
    fn UnregisterViewForNotification();
    fn SetExtensionClusion();
    fn EnumerateExcludedExtensions();
    fn GetQueryHelper();
    fn SetDiacriticSensitivity();
    fn DiacriticSensitivity();
    fn GetCrawlScopeManager();
}
impl ::windows::core::RuntimeName for ISearchCatalogManager {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchCatalogManager";
}
impl ISearchCatalogManagerVtbl {
    pub const fn new<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchCatalogManagerVtbl {
        unsafe extern "system" fn Name<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParameter(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParameter(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCatalogStatus<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCatalogStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppausedreason)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reindex<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reindex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReindexMatchingURLs<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReindexMatchingURLs(&*(&pszpattern as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReindexSearchRoot<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrooturl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReindexSearchRoot(&*(&pszrooturl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectTimeout<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnecttimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConnectTimeout(dwconnecttimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTimeout<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectTimeout(::core::mem::transmute_copy(&pdwconnecttimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataTimeout<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdatatimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDataTimeout(dwdatatimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataTimeout<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdatatimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataTimeout(::core::mem::transmute_copy(&pdwdatatimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItems<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfItems(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItemsToIndex<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfItemsToIndex(::core::mem::transmute_copy(&plincrementalcount), ::core::mem::transmute_copy(&plnotificationqueue), ::core::mem::transmute_copy(&plhighpriorityqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URLBeingIndexed<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).URLBeingIndexed(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLIndexingState<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetURLIndexingState(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistentItemsChangedSink<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppisearchpersistentitemschangedsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPersistentItemsChangedSink(::core::mem::transmute_copy(&ppisearchpersistentitemschangedsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterViewForNotification<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszview: super::super::Foundation::PWSTR, pviewchangedsink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterViewForNotification(&*(&pszview as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pviewchangedsink as *const <ISearchViewChangedSink as ::windows::core::Abi>::Abi as *const <ISearchViewChangedSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsChangedSink<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisearchnotifyinlinesite: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows::core::GUID, pguidcheckpointsignature: *mut ::windows::core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemsChangedSink(
                &*(&pisearchnotifyinlinesite as *const <ISearchNotifyInlineSite as ::windows::core::Abi>::Abi as *const <ISearchNotifyInlineSite as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
                ::core::mem::transmute_copy(&pguidcatalogresetsignature),
                ::core::mem::transmute_copy(&pguidcheckpointsignature),
                ::core::mem::transmute_copy(&pdwlastcheckpointnumber),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterViewForNotification<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterViewForNotification(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionClusion<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszextension: super::super::Foundation::PWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetExtensionClusion(&*(&pszextension as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&fexclude as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateExcludedExtensions<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateExcludedExtensions(::core::mem::transmute_copy(&ppextensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryHelper<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsearchqueryhelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQueryHelper(::core::mem::transmute_copy(&ppsearchqueryhelper)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiacriticSensitivity<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDiacriticSensitivity(&*(&fdiacriticsensitive as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiacriticSensitivity<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiacriticSensitivity(::core::mem::transmute_copy(&pfdiacriticsensitive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCrawlScopeManager<Impl: ISearchCatalogManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcrawlscopemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCrawlScopeManager(::core::mem::transmute_copy(&ppcrawlscopemanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISearchCatalogManager>,
            base.5,
            Name::<Impl, OFFSET>,
            GetParameter::<Impl, OFFSET>,
            SetParameter::<Impl, OFFSET>,
            GetCatalogStatus::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            Reindex::<Impl, OFFSET>,
            ReindexMatchingURLs::<Impl, OFFSET>,
            ReindexSearchRoot::<Impl, OFFSET>,
            SetConnectTimeout::<Impl, OFFSET>,
            ConnectTimeout::<Impl, OFFSET>,
            SetDataTimeout::<Impl, OFFSET>,
            DataTimeout::<Impl, OFFSET>,
            NumberOfItems::<Impl, OFFSET>,
            NumberOfItemsToIndex::<Impl, OFFSET>,
            URLBeingIndexed::<Impl, OFFSET>,
            GetURLIndexingState::<Impl, OFFSET>,
            GetPersistentItemsChangedSink::<Impl, OFFSET>,
            RegisterViewForNotification::<Impl, OFFSET>,
            GetItemsChangedSink::<Impl, OFFSET>,
            UnregisterViewForNotification::<Impl, OFFSET>,
            SetExtensionClusion::<Impl, OFFSET>,
            EnumerateExcludedExtensions::<Impl, OFFSET>,
            GetQueryHelper::<Impl, OFFSET>,
            SetDiacriticSensitivity::<Impl, OFFSET>,
            DiacriticSensitivity::<Impl, OFFSET>,
            GetCrawlScopeManager::<Impl, OFFSET>,
        )
    }
}
pub trait ISearchCatalogManager2Impl: Sized + ISearchCatalogManagerImpl {
    fn PrioritizeMatchingURLs();
}
impl ::windows::core::RuntimeName for ISearchCatalogManager2 {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchCatalogManager2";
}
impl ISearchCatalogManager2Vtbl {
    pub const fn new<Impl: ISearchCatalogManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchCatalogManager2Vtbl {
        unsafe extern "system" fn PrioritizeMatchingURLs<Impl: ISearchCatalogManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrioritizeMatchingURLs(&*(&pszpattern as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwprioritizeflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchCatalogManager2>, base.5, PrioritizeMatchingURLs::<Impl, OFFSET>)
    }
}
pub trait ISearchCrawlScopeManagerImpl: Sized {
    fn AddDefaultScopeRule();
    fn AddRoot();
    fn RemoveRoot();
    fn EnumerateRoots();
    fn AddHierarchicalScope();
    fn AddUserScopeRule();
    fn RemoveScopeRule();
    fn EnumerateScopeRules();
    fn HasParentScopeRule();
    fn HasChildScopeRule();
    fn IncludedInCrawlScope();
    fn IncludedInCrawlScopeEx();
    fn RevertToDefaultScopes();
    fn SaveAll();
    fn GetParentScopeVersionId();
    fn RemoveDefaultScopeRule();
}
impl ::windows::core::RuntimeName for ISearchCrawlScopeManager {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchCrawlScopeManager";
}
impl ISearchCrawlScopeManagerVtbl {
    pub const fn new<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchCrawlScopeManagerVtbl {
        unsafe extern "system" fn AddDefaultScopeRule<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDefaultScopeRule(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&finclude as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ffollowflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRoot<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psearchroot: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRoot(&*(&psearchroot as *const <ISearchRoot as ::windows::core::Abi>::Abi as *const <ISearchRoot as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRoot<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveRoot(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateRoots<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsearchroots: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateRoots(::core::mem::transmute_copy(&ppsearchroots)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddHierarchicalScope<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddHierarchicalScope(
                &*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&finclude as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fdefault as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&foverridechildren as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddUserScopeRule<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddUserScopeRule(
                &*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&finclude as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&foverridechildren as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ffollowflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScopeRule<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrule: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveScopeRule(&*(&pszrule as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateScopeRules<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsearchscoperules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateScopeRules(::core::mem::transmute_copy(&ppsearchscoperules)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasParentScopeRule<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfhasparentrule: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasParentScopeRule(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfhasparentrule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildScopeRule<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfhaschildrule: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasChildScopeRule(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfhaschildrule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScope<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludedInCrawlScope(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfisincluded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScopeEx<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludedInCrawlScopeEx(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfisincluded), ::core::mem::transmute_copy(&preason)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevertToDefaultScopes<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevertToDefaultScopes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAll<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentScopeVersionId<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, plscopeid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParentScopeVersionId(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plscopeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultScopeRule<Impl: ISearchCrawlScopeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveDefaultScopeRule(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISearchCrawlScopeManager>,
            base.5,
            AddDefaultScopeRule::<Impl, OFFSET>,
            AddRoot::<Impl, OFFSET>,
            RemoveRoot::<Impl, OFFSET>,
            EnumerateRoots::<Impl, OFFSET>,
            AddHierarchicalScope::<Impl, OFFSET>,
            AddUserScopeRule::<Impl, OFFSET>,
            RemoveScopeRule::<Impl, OFFSET>,
            EnumerateScopeRules::<Impl, OFFSET>,
            HasParentScopeRule::<Impl, OFFSET>,
            HasChildScopeRule::<Impl, OFFSET>,
            IncludedInCrawlScope::<Impl, OFFSET>,
            IncludedInCrawlScopeEx::<Impl, OFFSET>,
            RevertToDefaultScopes::<Impl, OFFSET>,
            SaveAll::<Impl, OFFSET>,
            GetParentScopeVersionId::<Impl, OFFSET>,
            RemoveDefaultScopeRule::<Impl, OFFSET>,
        )
    }
}
pub trait ISearchCrawlScopeManager2Impl: Sized + ISearchCrawlScopeManagerImpl {
    fn GetVersion();
}
impl ::windows::core::RuntimeName for ISearchCrawlScopeManager2 {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchCrawlScopeManager2";
}
impl ISearchCrawlScopeManager2Vtbl {
    pub const fn new<Impl: ISearchCrawlScopeManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchCrawlScopeManager2Vtbl {
        unsafe extern "system" fn GetVersion<Impl: ISearchCrawlScopeManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVersion(plversion, &*(&phfilemapping as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchCrawlScopeManager2>, base.5, GetVersion::<Impl, OFFSET>)
    }
}
pub trait ISearchItemsChangedSinkImpl: Sized {
    fn StartedMonitoringScope();
    fn StoppedMonitoringScope();
    fn OnItemsChanged();
}
impl ::windows::core::RuntimeName for ISearchItemsChangedSink {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchItemsChangedSink";
}
impl ISearchItemsChangedSinkVtbl {
    pub const fn new<Impl: ISearchItemsChangedSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchItemsChangedSinkVtbl {
        unsafe extern "system" fn StartedMonitoringScope<Impl: ISearchItemsChangedSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartedMonitoringScope(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoppedMonitoringScope<Impl: ISearchItemsChangedSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StoppedMonitoringScope(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnItemsChanged<Impl: ISearchItemsChangedSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnItemsChanged(dwnumberofchanges, &*(&rgdatachangeentries as *const <SEARCH_ITEM_CHANGE as ::windows::core::Abi>::Abi as *const <SEARCH_ITEM_CHANGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rgdwdocids), ::core::mem::transmute_copy(&rghrcompletioncodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchItemsChangedSink>, base.5, StartedMonitoringScope::<Impl, OFFSET>, StoppedMonitoringScope::<Impl, OFFSET>, OnItemsChanged::<Impl, OFFSET>)
    }
}
pub trait ISearchLanguageSupportImpl: Sized {
    fn SetDiacriticSensitivity();
    fn GetDiacriticSensitivity();
    fn LoadWordBreaker();
    fn LoadStemmer();
    fn IsPrefixNormalized();
}
impl ::windows::core::RuntimeName for ISearchLanguageSupport {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchLanguageSupport";
}
impl ISearchLanguageSupportVtbl {
    pub const fn new<Impl: ISearchLanguageSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchLanguageSupportVtbl {
        unsafe extern "system" fn SetDiacriticSensitivity<Impl: ISearchLanguageSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDiacriticSensitivity(&*(&fdiacriticsensitive as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiacriticSensitivity<Impl: ISearchLanguageSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDiacriticSensitivity(::core::mem::transmute_copy(&pfdiacriticsensitive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadWordBreaker<Impl: ISearchLanguageSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows::core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadWordBreaker(lcid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwordbreaker), ::core::mem::transmute_copy(&plcidused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStemmer<Impl: ISearchLanguageSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows::core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadStemmer(lcid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstemmer), ::core::mem::transmute_copy(&plcidused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrefixNormalized<Impl: ISearchLanguageSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsquerytoken: super::super::Foundation::PWSTR, cwcquerytoken: u32, pwcsdocumenttoken: super::super::Foundation::PWSTR, cwcdocumenttoken: u32, pulprefixlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPrefixNormalized(&*(&pwcsquerytoken as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcquerytoken, &*(&pwcsdocumenttoken as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcdocumenttoken, ::core::mem::transmute_copy(&pulprefixlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchLanguageSupport>, base.5, SetDiacriticSensitivity::<Impl, OFFSET>, GetDiacriticSensitivity::<Impl, OFFSET>, LoadWordBreaker::<Impl, OFFSET>, LoadStemmer::<Impl, OFFSET>, IsPrefixNormalized::<Impl, OFFSET>)
    }
}
pub trait ISearchManagerImpl: Sized {
    fn GetIndexerVersionStr();
    fn GetIndexerVersion();
    fn GetParameter();
    fn SetParameter();
    fn ProxyName();
    fn BypassList();
    fn SetProxy();
    fn GetCatalog();
    fn UserAgent();
    fn SetUserAgent();
    fn UseProxy();
    fn LocalBypass();
    fn PortNumber();
}
impl ::windows::core::RuntimeName for ISearchManager {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchManager";
}
impl ISearchManagerVtbl {
    pub const fn new<Impl: ISearchManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchManagerVtbl {
        unsafe extern "system" fn GetIndexerVersionStr<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszversionstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndexerVersionStr(::core::mem::transmute_copy(&ppszversionstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexerVersion<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndexerVersion(::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParameter(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParameter(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyName<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszproxyname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyName(::core::mem::transmute_copy(&ppszproxyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BypassList<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszbypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BypassList(::core::mem::transmute_copy(&ppszbypasslist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxy<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: super::super::Foundation::PWSTR, pszbypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxy(
                suseproxy,
                &*(&flocalbypassproxy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwportnumber,
                &*(&pszproxyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszbypasslist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCatalog<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR, ppcatalogmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCatalog(&*(&pszcatalog as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAgent<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszuseragent: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserAgent(::core::mem::transmute_copy(&ppszuseragent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserAgent<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszuseragent: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUserAgent(&*(&pszuseragent as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseProxy<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puseproxy: *mut PROXY_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseProxy(::core::mem::transmute_copy(&puseproxy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalBypass<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflocalbypass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalBypass(::core::mem::transmute_copy(&pflocalbypass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PortNumber<Impl: ISearchManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwportnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PortNumber(::core::mem::transmute_copy(&pdwportnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchManager>, base.5, GetIndexerVersionStr::<Impl, OFFSET>, GetIndexerVersion::<Impl, OFFSET>, GetParameter::<Impl, OFFSET>, SetParameter::<Impl, OFFSET>, ProxyName::<Impl, OFFSET>, BypassList::<Impl, OFFSET>, SetProxy::<Impl, OFFSET>, GetCatalog::<Impl, OFFSET>, UserAgent::<Impl, OFFSET>, SetUserAgent::<Impl, OFFSET>, UseProxy::<Impl, OFFSET>, LocalBypass::<Impl, OFFSET>, PortNumber::<Impl, OFFSET>)
    }
}
pub trait ISearchManager2Impl: Sized + ISearchManagerImpl {
    fn CreateCatalog();
    fn DeleteCatalog();
}
impl ::windows::core::RuntimeName for ISearchManager2 {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchManager2";
}
impl ISearchManager2Vtbl {
    pub const fn new<Impl: ISearchManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchManager2Vtbl {
        unsafe extern "system" fn CreateCatalog<Impl: ISearchManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR, ppcatalogmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCatalog(&*(&pszcatalog as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCatalog<Impl: ISearchManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteCatalog(&*(&pszcatalog as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchManager2>, base.5, CreateCatalog::<Impl, OFFSET>, DeleteCatalog::<Impl, OFFSET>)
    }
}
pub trait ISearchNotifyInlineSiteImpl: Sized {
    fn OnItemIndexedStatusChange();
    fn OnCatalogStatusChange();
}
impl ::windows::core::RuntimeName for ISearchNotifyInlineSite {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchNotifyInlineSite";
}
impl ISearchNotifyInlineSiteVtbl {
    pub const fn new<Impl: ISearchNotifyInlineSiteImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchNotifyInlineSiteVtbl {
        unsafe extern "system" fn OnItemIndexedStatusChange<Impl: ISearchNotifyInlineSiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnItemIndexedStatusChange(sipstatus, dwnumentries, &*(&rgitemstatusentries as *const <SEARCH_ITEM_INDEXING_STATUS as ::windows::core::Abi>::Abi as *const <SEARCH_ITEM_INDEXING_STATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCatalogStatusChange<Impl: ISearchNotifyInlineSiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcatalogresetsignature: *const ::windows::core::GUID, guidcheckpointsignature: *const ::windows::core::GUID, dwlastcheckpointnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCatalogStatusChange(&*(&guidcatalogresetsignature as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidcheckpointsignature as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwlastcheckpointnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchNotifyInlineSite>, base.5, OnItemIndexedStatusChange::<Impl, OFFSET>, OnCatalogStatusChange::<Impl, OFFSET>)
    }
}
pub trait ISearchPersistentItemsChangedSinkImpl: Sized {
    fn StartedMonitoringScope();
    fn StoppedMonitoringScope();
    fn OnItemsChanged();
}
impl ::windows::core::RuntimeName for ISearchPersistentItemsChangedSink {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchPersistentItemsChangedSink";
}
impl ISearchPersistentItemsChangedSinkVtbl {
    pub const fn new<Impl: ISearchPersistentItemsChangedSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPersistentItemsChangedSinkVtbl {
        unsafe extern "system" fn StartedMonitoringScope<Impl: ISearchPersistentItemsChangedSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartedMonitoringScope(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoppedMonitoringScope<Impl: ISearchPersistentItemsChangedSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StoppedMonitoringScope(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnItemsChanged<Impl: ISearchPersistentItemsChangedSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnItemsChanged(dwnumberofchanges, &*(&datachangeentries as *const <SEARCH_ITEM_PERSISTENT_CHANGE as ::windows::core::Abi>::Abi as *const <SEARCH_ITEM_PERSISTENT_CHANGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&hrcompletioncodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPersistentItemsChangedSink>, base.5, StartedMonitoringScope::<Impl, OFFSET>, StoppedMonitoringScope::<Impl, OFFSET>, OnItemsChanged::<Impl, OFFSET>)
    }
}
pub trait ISearchProtocolImpl: Sized {
    fn Init();
    fn CreateAccessor();
    fn CloseAccessor();
    fn ShutDown();
}
impl ::windows::core::RuntimeName for ISearchProtocol {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchProtocol";
}
impl ISearchProtocolVtbl {
    pub const fn new<Impl: ISearchProtocolImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchProtocolVtbl {
        unsafe extern "system" fn Init<Impl: ISearchProtocolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimeoutinfo: *mut TIMEOUT_INFO, pprotocolhandlersite: ::windows::core::RawPtr, pproxyinfo: *mut PROXY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Init(&*(&ptimeoutinfo as *const <TIMEOUT_INFO as ::windows::core::Abi>::Abi as *const <TIMEOUT_INFO as ::windows::core::DefaultType>::DefaultType), &*(&pprotocolhandlersite as *const <IProtocolHandlerSite as ::windows::core::Abi>::Abi as *const <IProtocolHandlerSite as ::windows::core::DefaultType>::DefaultType), &*(&pproxyinfo as *const <PROXY_INFO as ::windows::core::Abi>::Abi as *const <PROXY_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccessor<Impl: ISearchProtocolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, ppaccessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAccessor(
                &*(&pcwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pauthenticationinfo as *const <AUTHENTICATION_INFO as ::windows::core::Abi>::Abi as *const <AUTHENTICATION_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&pincrementalaccessinfo as *const <INCREMENTAL_ACCESS_INFO as ::windows::core::Abi>::Abi as *const <INCREMENTAL_ACCESS_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&piteminfo as *const <ITEM_INFO as ::windows::core::Abi>::Abi as *const <ITEM_INFO as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppaccessor),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseAccessor<Impl: ISearchProtocolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paccessor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseAccessor(&*(&paccessor as *const <IUrlAccessor as ::windows::core::Abi>::Abi as *const <IUrlAccessor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutDown<Impl: ISearchProtocolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutDown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchProtocol>, base.5, Init::<Impl, OFFSET>, CreateAccessor::<Impl, OFFSET>, CloseAccessor::<Impl, OFFSET>, ShutDown::<Impl, OFFSET>)
    }
}
pub trait ISearchProtocol2Impl: Sized + ISearchProtocolImpl {
    fn CreateAccessorEx();
}
impl ::windows::core::RuntimeName for ISearchProtocol2 {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchProtocol2";
}
impl ISearchProtocol2Vtbl {
    pub const fn new<Impl: ISearchProtocol2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchProtocol2Vtbl {
        unsafe extern "system" fn CreateAccessorEx<Impl: ISearchProtocol2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAccessorEx(
                &*(&pcwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pauthenticationinfo as *const <AUTHENTICATION_INFO as ::windows::core::Abi>::Abi as *const <AUTHENTICATION_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&pincrementalaccessinfo as *const <INCREMENTAL_ACCESS_INFO as ::windows::core::Abi>::Abi as *const <INCREMENTAL_ACCESS_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&piteminfo as *const <ITEM_INFO as ::windows::core::Abi>::Abi as *const <ITEM_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&puserdata as *const <super::Com::BLOB as ::windows::core::Abi>::Abi as *const <super::Com::BLOB as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppaccessor),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchProtocol2>, base.5, CreateAccessorEx::<Impl, OFFSET>)
    }
}
pub trait ISearchProtocolThreadContextImpl: Sized {
    fn ThreadInit();
    fn ThreadShutdown();
    fn ThreadIdle();
}
impl ::windows::core::RuntimeName for ISearchProtocolThreadContext {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchProtocolThreadContext";
}
impl ISearchProtocolThreadContextVtbl {
    pub const fn new<Impl: ISearchProtocolThreadContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchProtocolThreadContextVtbl {
        unsafe extern "system" fn ThreadInit<Impl: ISearchProtocolThreadContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThreadInit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThreadShutdown<Impl: ISearchProtocolThreadContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThreadShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThreadIdle<Impl: ISearchProtocolThreadContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimeelaspedsincelastcallinms: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThreadIdle(dwtimeelaspedsincelastcallinms) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchProtocolThreadContext>, base.5, ThreadInit::<Impl, OFFSET>, ThreadShutdown::<Impl, OFFSET>, ThreadIdle::<Impl, OFFSET>)
    }
}
pub trait ISearchQueryHelperImpl: Sized {
    fn ConnectionString();
    fn SetQueryContentLocale();
    fn QueryContentLocale();
    fn SetQueryKeywordLocale();
    fn QueryKeywordLocale();
    fn SetQueryTermExpansion();
    fn QueryTermExpansion();
    fn SetQuerySyntax();
    fn QuerySyntax();
    fn SetQueryContentProperties();
    fn QueryContentProperties();
    fn SetQuerySelectColumns();
    fn QuerySelectColumns();
    fn SetQueryWhereRestrictions();
    fn QueryWhereRestrictions();
    fn SetQuerySorting();
    fn QuerySorting();
    fn GenerateSQLFromUserQuery();
    fn WriteProperties();
    fn SetQueryMaxResults();
    fn QueryMaxResults();
}
impl ::windows::core::RuntimeName for ISearchQueryHelper {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchQueryHelper";
}
impl ISearchQueryHelperVtbl {
    pub const fn new<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchQueryHelperVtbl {
        unsafe extern "system" fn ConnectionString<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszconnectionstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionString(::core::mem::transmute_copy(&pszconnectionstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentLocale<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQueryContentLocale(lcid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryContentLocale<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryContentLocale(::core::mem::transmute_copy(&plcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryKeywordLocale<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQueryKeywordLocale(lcid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryKeywordLocale<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryKeywordLocale(::core::mem::transmute_copy(&plcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryTermExpansion<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expandterms: SEARCH_TERM_EXPANSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQueryTermExpansion(expandterms) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryTermExpansion<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexpandterms: *mut SEARCH_TERM_EXPANSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryTermExpansion(::core::mem::transmute_copy(&pexpandterms)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySyntax<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuerySyntax(querysyntax) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySyntax<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquerysyntax: *mut SEARCH_QUERY_SYNTAX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuerySyntax(::core::mem::transmute_copy(&pquerysyntax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentProperties<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcontentproperties: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQueryContentProperties(&*(&pszcontentproperties as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryContentProperties<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcontentproperties: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryContentProperties(::core::mem::transmute_copy(&ppszcontentproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySelectColumns<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszselectcolumns: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuerySelectColumns(&*(&pszselectcolumns as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySelectColumns<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszselectcolumns: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuerySelectColumns(::core::mem::transmute_copy(&ppszselectcolumns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryWhereRestrictions<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrestrictions: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQueryWhereRestrictions(&*(&pszrestrictions as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryWhereRestrictions<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszrestrictions: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryWhereRestrictions(::core::mem::transmute_copy(&ppszrestrictions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySorting<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsorting: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuerySorting(&*(&pszsorting as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySorting<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszsorting: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuerySorting(::core::mem::transmute_copy(&ppszsorting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateSQLFromUserQuery<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszquery: super::super::Foundation::PWSTR, ppszsql: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateSQLFromUserQuery(&*(&pszquery as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszsql)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProperties<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteProperties(
                itemid,
                dwnumberofcolumns,
                &*(&pcolumns as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pvalues as *const <SEARCH_COLUMN_PROPERTIES as ::windows::core::Abi>::Abi as *const <SEARCH_COLUMN_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&pftgathermodifiedtime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryMaxResults<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmaxresults: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQueryMaxResults(cmaxresults) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxResults<Impl: ISearchQueryHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcmaxresults: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryMaxResults(::core::mem::transmute_copy(&pcmaxresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISearchQueryHelper>,
            base.5,
            ConnectionString::<Impl, OFFSET>,
            SetQueryContentLocale::<Impl, OFFSET>,
            QueryContentLocale::<Impl, OFFSET>,
            SetQueryKeywordLocale::<Impl, OFFSET>,
            QueryKeywordLocale::<Impl, OFFSET>,
            SetQueryTermExpansion::<Impl, OFFSET>,
            QueryTermExpansion::<Impl, OFFSET>,
            SetQuerySyntax::<Impl, OFFSET>,
            QuerySyntax::<Impl, OFFSET>,
            SetQueryContentProperties::<Impl, OFFSET>,
            QueryContentProperties::<Impl, OFFSET>,
            SetQuerySelectColumns::<Impl, OFFSET>,
            QuerySelectColumns::<Impl, OFFSET>,
            SetQueryWhereRestrictions::<Impl, OFFSET>,
            QueryWhereRestrictions::<Impl, OFFSET>,
            SetQuerySorting::<Impl, OFFSET>,
            QuerySorting::<Impl, OFFSET>,
            GenerateSQLFromUserQuery::<Impl, OFFSET>,
            WriteProperties::<Impl, OFFSET>,
            SetQueryMaxResults::<Impl, OFFSET>,
            QueryMaxResults::<Impl, OFFSET>,
        )
    }
}
pub trait ISearchQueryHitsImpl: Sized {
    fn Init();
    fn NextHitMoniker();
    fn NextHitOffset();
}
impl ::windows::core::RuntimeName for ISearchQueryHits {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchQueryHits";
}
impl ISearchQueryHitsVtbl {
    pub const fn new<Impl: ISearchQueryHitsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchQueryHitsVtbl {
        unsafe extern "system" fn Init<Impl: ISearchQueryHitsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflt: ::windows::core::RawPtr, ulflags: u32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Init(&*(&pflt as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::IFilter as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHitMoniker<Impl: ISearchQueryHitsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcmnk: *mut u32, papmnk: *mut *mut ::windows::core::RawPtr) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextHitMoniker(pcmnk, &*(&papmnk as *const <super::Com::IMoniker as ::windows::core::Abi>::Abi as *const <super::Com::IMoniker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHitOffset<Impl: ISearchQueryHitsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextHitOffset(pcregion, &*(&paregion as *const <super::super::Storage::IndexServer::FILTERREGION as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::FILTERREGION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchQueryHits>, base.5, Init::<Impl, OFFSET>, NextHitMoniker::<Impl, OFFSET>, NextHitOffset::<Impl, OFFSET>)
    }
}
pub trait ISearchRootImpl: Sized {
    fn SetSchedule();
    fn Schedule();
    fn SetRootURL();
    fn RootURL();
    fn SetIsHierarchical();
    fn IsHierarchical();
    fn SetProvidesNotifications();
    fn ProvidesNotifications();
    fn SetUseNotificationsOnly();
    fn UseNotificationsOnly();
    fn SetEnumerationDepth();
    fn EnumerationDepth();
    fn SetHostDepth();
    fn HostDepth();
    fn SetFollowDirectories();
    fn FollowDirectories();
    fn SetAuthenticationType();
    fn AuthenticationType();
    fn SetUser();
    fn User();
    fn SetPassword();
    fn Password();
}
impl ::windows::core::RuntimeName for ISearchRoot {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchRoot";
}
impl ISearchRootVtbl {
    pub const fn new<Impl: ISearchRootImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchRootVtbl {
        unsafe extern "system" fn SetSchedule<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztaskarg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSchedule(&*(&psztaskarg as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Schedule<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsztaskarg: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Schedule(::core::mem::transmute_copy(&ppsztaskarg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootURL<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRootURL(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootURL<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RootURL(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHierarchical<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fishierarchical: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsHierarchical(&*(&fishierarchical as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHierarchical<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfishierarchical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHierarchical(::core::mem::transmute_copy(&pfishierarchical)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvidesNotifications<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProvidesNotifications(&*(&fprovidesnotifications as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvidesNotifications<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprovidesnotifications: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProvidesNotifications(::core::mem::transmute_copy(&pfprovidesnotifications)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseNotificationsOnly<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUseNotificationsOnly(&*(&fusenotificationsonly as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseNotificationsOnly<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfusenotificationsonly: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseNotificationsOnly(::core::mem::transmute_copy(&pfusenotificationsonly)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumerationDepth<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnumerationDepth(dwdepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationDepth<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationDepth(::core::mem::transmute_copy(&pdwdepth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostDepth<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHostDepth(dwdepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostDepth<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HostDepth(::core::mem::transmute_copy(&pdwdepth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFollowDirectories<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFollowDirectories(&*(&ffollowdirectories as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FollowDirectories<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pffollowdirectories: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FollowDirectories(::core::mem::transmute_copy(&pffollowdirectories)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authtype: AUTH_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationType(authtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationType<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthtype: *mut AUTH_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationType(::core::mem::transmute_copy(&pauthtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUser<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszuser: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUser(&*(&pszuser as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszuser: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User(::core::mem::transmute_copy(&ppszuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPassword(&*(&pszpassword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: ISearchRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Password(::core::mem::transmute_copy(&ppszpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISearchRoot>,
            base.5,
            SetSchedule::<Impl, OFFSET>,
            Schedule::<Impl, OFFSET>,
            SetRootURL::<Impl, OFFSET>,
            RootURL::<Impl, OFFSET>,
            SetIsHierarchical::<Impl, OFFSET>,
            IsHierarchical::<Impl, OFFSET>,
            SetProvidesNotifications::<Impl, OFFSET>,
            ProvidesNotifications::<Impl, OFFSET>,
            SetUseNotificationsOnly::<Impl, OFFSET>,
            UseNotificationsOnly::<Impl, OFFSET>,
            SetEnumerationDepth::<Impl, OFFSET>,
            EnumerationDepth::<Impl, OFFSET>,
            SetHostDepth::<Impl, OFFSET>,
            HostDepth::<Impl, OFFSET>,
            SetFollowDirectories::<Impl, OFFSET>,
            FollowDirectories::<Impl, OFFSET>,
            SetAuthenticationType::<Impl, OFFSET>,
            AuthenticationType::<Impl, OFFSET>,
            SetUser::<Impl, OFFSET>,
            User::<Impl, OFFSET>,
            SetPassword::<Impl, OFFSET>,
            Password::<Impl, OFFSET>,
        )
    }
}
pub trait ISearchScopeRuleImpl: Sized {
    fn PatternOrURL();
    fn IsIncluded();
    fn IsDefault();
    fn FollowFlags();
}
impl ::windows::core::RuntimeName for ISearchScopeRule {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchScopeRule";
}
impl ISearchScopeRuleVtbl {
    pub const fn new<Impl: ISearchScopeRuleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchScopeRuleVtbl {
        unsafe extern "system" fn PatternOrURL<Impl: ISearchScopeRuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpatternorurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PatternOrURL(::core::mem::transmute_copy(&ppszpatternorurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIncluded<Impl: ISearchScopeRuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsIncluded(::core::mem::transmute_copy(&pfisincluded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: ISearchScopeRuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDefault(::core::mem::transmute_copy(&pfisdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FollowFlags<Impl: ISearchScopeRuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfollowflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FollowFlags(::core::mem::transmute_copy(&pfollowflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchScopeRule>, base.5, PatternOrURL::<Impl, OFFSET>, IsIncluded::<Impl, OFFSET>, IsDefault::<Impl, OFFSET>, FollowFlags::<Impl, OFFSET>)
    }
}
pub trait ISearchViewChangedSinkImpl: Sized {
    fn OnChange();
}
impl ::windows::core::RuntimeName for ISearchViewChangedSink {
    const NAME: &'static str = "Windows.Win32.System.Search.ISearchViewChangedSink";
}
impl ISearchViewChangedSinkVtbl {
    pub const fn new<Impl: ISearchViewChangedSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchViewChangedSinkVtbl {
        unsafe extern "system" fn OnChange<Impl: ISearchViewChangedSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChange(pdwdocid, &*(&pchange as *const <SEARCH_ITEM_CHANGE as ::windows::core::Abi>::Abi as *const <SEARCH_ITEM_CHANGE as ::windows::core::DefaultType>::DefaultType), &*(&pfinview as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchViewChangedSink>, base.5, OnChange::<Impl, OFFSET>)
    }
}
pub trait ISecurityInfoImpl: Sized {
    fn GetCurrentTrustee();
    fn GetObjectTypes();
    fn GetPermissions();
}
impl ::windows::core::RuntimeName for ISecurityInfo {
    const NAME: &'static str = "Windows.Win32.System.Search.ISecurityInfo";
}
impl ISecurityInfoVtbl {
    pub const fn new<Impl: ISecurityInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISecurityInfoVtbl {
        unsafe extern "system" fn GetCurrentTrustee<Impl: ISecurityInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentTrustee(&*(&pptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectTypes<Impl: ISecurityInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectTypes(cobjecttypes, &*(&rgobjecttypes as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPermissions<Impl: ISecurityInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objecttype: ::windows::core::GUID, ppermissions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPermissions(&*(&objecttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ppermissions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISecurityInfo>, base.5, GetCurrentTrustee::<Impl, OFFSET>, GetObjectTypes::<Impl, OFFSET>, GetPermissions::<Impl, OFFSET>)
    }
}
pub trait IServiceImpl: Sized {
    fn InvokeService();
}
impl ::windows::core::RuntimeName for IService {
    const NAME: &'static str = "Windows.Win32.System.Search.IService";
}
impl IServiceVtbl {
    pub const fn new<Impl: IServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IServiceVtbl {
        unsafe extern "system" fn InvokeService<Impl: IServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvokeService(&*(&punkinner as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IService>, base.5, InvokeService::<Impl, OFFSET>)
    }
}
pub trait ISessionPropertiesImpl: Sized {
    fn GetProperties();
    fn SetProperties();
}
impl ::windows::core::RuntimeName for ISessionProperties {
    const NAME: &'static str = "Windows.Win32.System.Search.ISessionProperties";
}
impl ISessionPropertiesVtbl {
    pub const fn new<Impl: ISessionPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISessionPropertiesVtbl {
        unsafe extern "system" fn GetProperties<Impl: ISessionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperties(cpropertyidsets, &*(&rgpropertyidsets as *const <DBPROPIDSET as ::windows::core::Abi>::Abi as *const <DBPROPIDSET as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperties<Impl: ISessionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperties(cpropertysets, &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISessionProperties>, base.5, GetProperties::<Impl, OFFSET>, SetProperties::<Impl, OFFSET>)
    }
}
pub trait ISimpleCommandCreatorImpl: Sized {
    fn CreateICommand();
    fn VerifyCatalog();
    fn GetDefaultCatalog();
}
impl ::windows::core::RuntimeName for ISimpleCommandCreator {
    const NAME: &'static str = "Windows.Win32.System.Search.ISimpleCommandCreator";
}
impl ISimpleCommandCreatorVtbl {
    pub const fn new<Impl: ISimpleCommandCreatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISimpleCommandCreatorVtbl {
        unsafe extern "system" fn CreateICommand<Impl: ISimpleCommandCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiunknown: *mut *mut ::core::ffi::c_void, pouterunk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateICommand(::core::mem::transmute_copy(&ppiunknown), &*(&pouterunk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyCatalog<Impl: ISimpleCommandCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmachine: super::super::Foundation::PWSTR, pwszcatalogname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerifyCatalog(&*(&pwszmachine as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszcatalogname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultCatalog<Impl: ISimpleCommandCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcatalogname: super::super::Foundation::PWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultCatalog(&*(&pwszcatalogname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcin, pcwcout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISimpleCommandCreator>, base.5, CreateICommand::<Impl, OFFSET>, VerifyCatalog::<Impl, OFFSET>, GetDefaultCatalog::<Impl, OFFSET>)
    }
}
pub trait ISourcesRowsetImpl: Sized {
    fn GetSourcesRowset();
}
impl ::windows::core::RuntimeName for ISourcesRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.ISourcesRowset";
}
impl ISourcesRowsetVtbl {
    pub const fn new<Impl: ISourcesRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISourcesRowsetVtbl {
        unsafe extern "system" fn GetSourcesRowset<Impl: ISourcesRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourcesRowset(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgproperties as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppsourcesrowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISourcesRowset>, base.5, GetSourcesRowset::<Impl, OFFSET>)
    }
}
pub trait IStemmerImpl: Sized {
    fn Init();
    fn GenerateWordForms();
    fn GetLicenseToUse();
}
impl ::windows::core::RuntimeName for IStemmer {
    const NAME: &'static str = "Windows.Win32.System.Search.IStemmer";
}
impl IStemmerVtbl {
    pub const fn new<Impl: IStemmerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStemmerVtbl {
        unsafe extern "system" fn Init<Impl: IStemmerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Init(ulmaxtokensize, &*(&pflicense as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateWordForms<Impl: IStemmerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32, pstemsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateWordForms(&*(&pwcinbuf as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwc, &*(&pstemsink as *const <IWordFormSink as ::windows::core::Abi>::Abi as *const <IWordFormSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseToUse<Impl: IStemmerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLicenseToUse(ppwcslicense) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStemmer>, base.5, Init::<Impl, OFFSET>, GenerateWordForms::<Impl, OFFSET>, GetLicenseToUse::<Impl, OFFSET>)
    }
}
pub trait ISubscriptionItemImpl: Sized {
    fn GetCookie();
    fn GetSubscriptionItemInfo();
    fn SetSubscriptionItemInfo();
    fn ReadProperties();
    fn WriteProperties();
    fn EnumProperties();
    fn NotifyChanged();
}
impl ::windows::core::RuntimeName for ISubscriptionItem {
    const NAME: &'static str = "Windows.Win32.System.Search.ISubscriptionItem";
}
impl ISubscriptionItemVtbl {
    pub const fn new<Impl: ISubscriptionItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISubscriptionItemVtbl {
        unsafe extern "system" fn GetCookie<Impl: ISubscriptionItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCookie(::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionItemInfo<Impl: ISubscriptionItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubscriptionItemInfo(::core::mem::transmute_copy(&psubscriptioniteminfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionItemInfo<Impl: ISubscriptionItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSubscriptionItemInfo(&*(&psubscriptioniteminfo as *const <SUBSCRIPTIONITEMINFO as ::windows::core::Abi>::Abi as *const <SUBSCRIPTIONITEMINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadProperties<Impl: ISubscriptionItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const super::super::Foundation::PWSTR, rgvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadProperties(ncount, &*(&rgwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rgvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProperties<Impl: ISubscriptionItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const super::super::Foundation::PWSTR, rgvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteProperties(ncount, &*(&rgwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&rgvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProperties<Impl: ISubscriptionItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumitemproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumProperties(::core::mem::transmute_copy(&ppenumitemproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyChanged<Impl: ISubscriptionItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotifyChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISubscriptionItem>, base.5, GetCookie::<Impl, OFFSET>, GetSubscriptionItemInfo::<Impl, OFFSET>, SetSubscriptionItemInfo::<Impl, OFFSET>, ReadProperties::<Impl, OFFSET>, WriteProperties::<Impl, OFFSET>, EnumProperties::<Impl, OFFSET>, NotifyChanged::<Impl, OFFSET>)
    }
}
pub trait ISubscriptionMgrImpl: Sized {
    fn DeleteSubscription();
    fn UpdateSubscription();
    fn UpdateAll();
    fn IsSubscribed();
    fn GetSubscriptionInfo();
    fn GetDefaultInfo();
    fn ShowSubscriptionProperties();
    fn CreateSubscription();
}
impl ::windows::core::RuntimeName for ISubscriptionMgr {
    const NAME: &'static str = "Windows.Win32.System.Search.ISubscriptionMgr";
}
impl ISubscriptionMgrVtbl {
    pub const fn new<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISubscriptionMgrVtbl {
        unsafe extern "system" fn DeleteSubscription<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteSubscription(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateSubscription<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateSubscription(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAll<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscribed<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pfsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSubscribed(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfsubscribed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionInfo<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubscriptionInfo(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultInfo<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultInfo(subtype, ::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowSubscriptionProperties<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowSubscriptionProperties(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubscription<Impl: ISubscriptionMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pwszurl: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSubscription(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                substype,
                &*(&pinfo as *const <SUBSCRIPTIONINFO as ::windows::core::Abi>::Abi as *const <SUBSCRIPTIONINFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISubscriptionMgr>, base.5, DeleteSubscription::<Impl, OFFSET>, UpdateSubscription::<Impl, OFFSET>, UpdateAll::<Impl, OFFSET>, IsSubscribed::<Impl, OFFSET>, GetSubscriptionInfo::<Impl, OFFSET>, GetDefaultInfo::<Impl, OFFSET>, ShowSubscriptionProperties::<Impl, OFFSET>, CreateSubscription::<Impl, OFFSET>)
    }
}
pub trait ISubscriptionMgr2Impl: Sized + ISubscriptionMgrImpl {
    fn GetItemFromURL();
    fn GetItemFromCookie();
    fn GetSubscriptionRunState();
    fn EnumSubscriptions();
    fn UpdateItems();
    fn AbortItems();
    fn AbortAll();
}
impl ::windows::core::RuntimeName for ISubscriptionMgr2 {
    const NAME: &'static str = "Windows.Win32.System.Search.ISubscriptionMgr2";
}
impl ISubscriptionMgr2Vtbl {
    pub const fn new<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISubscriptionMgr2Vtbl {
        unsafe extern "system" fn GetItemFromURL<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, ppsubscriptionitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemFromURL(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsubscriptionitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemFromCookie<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubscriptioncookie: *const ::windows::core::GUID, ppsubscriptionitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemFromCookie(&*(&psubscriptioncookie as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsubscriptionitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionRunState<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows::core::GUID, pdwrunstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubscriptionRunState(dwnumcookies, &*(&pcookies as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwrunstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSubscriptions<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumsubscriptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumSubscriptions(dwflags, ::core::mem::transmute_copy(&ppenumsubscriptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateItems<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateItems(dwflags, dwnumcookies, &*(&pcookies as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortItems<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbortItems(dwnumcookies, &*(&pcookies as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAll<Impl: ISubscriptionMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbortAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISubscriptionMgr2>, base.5, GetItemFromURL::<Impl, OFFSET>, GetItemFromCookie::<Impl, OFFSET>, GetSubscriptionRunState::<Impl, OFFSET>, EnumSubscriptions::<Impl, OFFSET>, UpdateItems::<Impl, OFFSET>, AbortItems::<Impl, OFFSET>, AbortAll::<Impl, OFFSET>)
    }
}
pub trait ITableCreationImpl: Sized + ITableDefinitionImpl {
    fn GetTableDefinition();
}
impl ::windows::core::RuntimeName for ITableCreation {
    const NAME: &'static str = "Windows.Win32.System.Search.ITableCreation";
}
impl ITableCreationVtbl {
    pub const fn new<Impl: ITableCreationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITableCreationVtbl {
        unsafe extern "system" fn GetTableDefinition<Impl: ITableCreationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTableDefinition(
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pccolumndescs),
                ::core::mem::transmute_copy(&prgcolumndescs),
                ::core::mem::transmute_copy(&pcpropertysets),
                ::core::mem::transmute_copy(&prgpropertysets),
                ::core::mem::transmute_copy(&pcconstraintdescs),
                ::core::mem::transmute_copy(&prgconstraintdescs),
                ::core::mem::transmute_copy(&ppwszstringbuffer),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITableCreation>, base.5, GetTableDefinition::<Impl, OFFSET>)
    }
}
pub trait ITableDefinitionImpl: Sized {
    fn CreateTable();
    fn DropTable();
    fn AddColumn();
    fn DropColumn();
}
impl ::windows::core::RuntimeName for ITableDefinition {
    const NAME: &'static str = "Windows.Win32.System.Search.ITableDefinition";
}
impl ITableDefinitionVtbl {
    pub const fn new<Impl: ITableDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITableDefinitionVtbl {
        unsafe extern "system" fn CreateTable<Impl: ITableDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTable(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                ccolumndescs,
                &*(&rgcolumndescs as *const <DBCOLUMNDESC as ::windows::core::Abi>::Abi as *const <DBCOLUMNDESC as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pptableid),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropTable<Impl: ITableDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropTable(&*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddColumn<Impl: ITableDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddColumn(&*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), &*(&pcolumndesc as *const <DBCOLUMNDESC as ::windows::core::Abi>::Abi as *const <DBCOLUMNDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcolumnid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropColumn<Impl: ITableDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropColumn(&*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), &*(&pcolumnid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITableDefinition>, base.5, CreateTable::<Impl, OFFSET>, DropTable::<Impl, OFFSET>, AddColumn::<Impl, OFFSET>, DropColumn::<Impl, OFFSET>)
    }
}
pub trait ITableDefinitionWithConstraintsImpl: Sized + ITableCreationImpl + ITableDefinitionImpl {
    fn AddConstraint();
    fn CreateTableWithConstraints();
    fn DropConstraint();
}
impl ::windows::core::RuntimeName for ITableDefinitionWithConstraints {
    const NAME: &'static str = "Windows.Win32.System.Search.ITableDefinitionWithConstraints";
}
impl ITableDefinitionWithConstraintsVtbl {
    pub const fn new<Impl: ITableDefinitionWithConstraintsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITableDefinitionWithConstraintsVtbl {
        unsafe extern "system" fn AddConstraint<Impl: ITableDefinitionWithConstraintsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintdesc: *mut DBCONSTRAINTDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddConstraint(&*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), &*(&pconstraintdesc as *const <DBCONSTRAINTDESC as ::windows::core::Abi>::Abi as *const <DBCONSTRAINTDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableWithConstraints<Impl: ITableDefinitionWithConstraintsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *mut DBCONSTRAINTDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTableWithConstraints(
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                ccolumndescs,
                &*(&rgcolumndescs as *const <DBCOLUMNDESC as ::windows::core::Abi>::Abi as *const <DBCOLUMNDESC as ::windows::core::DefaultType>::DefaultType),
                cconstraintdescs,
                &*(&rgconstraintdescs as *const <DBCONSTRAINTDESC as ::windows::core::Abi>::Abi as *const <DBCONSTRAINTDESC as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cpropertysets,
                &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
                &*(&pptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprowset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropConstraint<Impl: ITableDefinitionWithConstraintsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropConstraint(&*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType), &*(&pconstraintid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITableDefinitionWithConstraints>, base.5, AddConstraint::<Impl, OFFSET>, CreateTableWithConstraints::<Impl, OFFSET>, DropConstraint::<Impl, OFFSET>)
    }
}
pub trait ITableRenameImpl: Sized {
    fn RenameColumn();
    fn RenameTable();
}
impl ::windows::core::RuntimeName for ITableRename {
    const NAME: &'static str = "Windows.Win32.System.Search.ITableRename";
}
impl ITableRenameVtbl {
    pub const fn new<Impl: ITableRenameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITableRenameVtbl {
        unsafe extern "system" fn RenameColumn<Impl: ITableRenameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, poldcolumnid: *mut super::super::Storage::IndexServer::DBID, pnewcolumnid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenameColumn(
                &*(&ptableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&poldcolumnid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pnewcolumnid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameTable<Impl: ITableRenameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poldtableid: *mut super::super::Storage::IndexServer::DBID, poldindexid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenameTable(
                &*(&poldtableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&poldindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pnewtableid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
                &*(&pnewindexid as *const <super::super::Storage::IndexServer::DBID as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::DBID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITableRename>, base.5, RenameColumn::<Impl, OFFSET>, RenameTable::<Impl, OFFSET>)
    }
}
pub trait ITokenCollectionImpl: Sized {
    fn NumberOfTokens();
    fn GetToken();
}
impl ::windows::core::RuntimeName for ITokenCollection {
    const NAME: &'static str = "Windows.Win32.System.Search.ITokenCollection";
}
impl ITokenCollectionVtbl {
    pub const fn new<Impl: ITokenCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITokenCollectionVtbl {
        unsafe extern "system" fn NumberOfTokens<Impl: ITokenCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfTokens(pcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToken<Impl: ITokenCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetToken(i, ::core::mem::transmute_copy(&pbegin), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITokenCollection>, base.5, NumberOfTokens::<Impl, OFFSET>, GetToken::<Impl, OFFSET>)
    }
}
pub trait ITransactionJoinImpl: Sized {
    fn GetOptionsObject();
    fn JoinTransaction();
}
impl ::windows::core::RuntimeName for ITransactionJoin {
    const NAME: &'static str = "Windows.Win32.System.Search.ITransactionJoin";
}
impl ITransactionJoinVtbl {
    pub const fn new<Impl: ITransactionJoinImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionJoinVtbl {
        unsafe extern "system" fn GetOptionsObject<Impl: ITransactionJoinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionsObject(::core::mem::transmute_copy(&ppoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinTransaction<Impl: ITransactionJoinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punktransactioncoord: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JoinTransaction(&*(&punktransactioncoord as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), isolevel, isoflags, &*(&potheroptions as *const <super::DistributedTransactionCoordinator::ITransactionOptions as ::windows::core::Abi>::Abi as *const <super::DistributedTransactionCoordinator::ITransactionOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionJoin>, base.5, GetOptionsObject::<Impl, OFFSET>, JoinTransaction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionLocalImpl: Sized + ITransactionImpl {
    fn GetOptionsObject();
    fn StartTransaction();
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows::core::RuntimeName for ITransactionLocal {
    const NAME: &'static str = "Windows.Win32.System.Search.ITransactionLocal";
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionLocalVtbl {
    pub const fn new<Impl: ITransactionLocalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionLocalVtbl {
        unsafe extern "system" fn GetOptionsObject<Impl: ITransactionLocalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionsObject(::core::mem::transmute_copy(&ppoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTransaction<Impl: ITransactionLocalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: ::windows::core::RawPtr, pultransactionlevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTransaction(isolevel, isoflags, &*(&potheroptions as *const <super::DistributedTransactionCoordinator::ITransactionOptions as ::windows::core::Abi>::Abi as *const <super::DistributedTransactionCoordinator::ITransactionOptions as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pultransactionlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionLocal>, base.5, GetOptionsObject::<Impl, OFFSET>, StartTransaction::<Impl, OFFSET>)
    }
}
pub trait ITransactionObjectImpl: Sized {
    fn GetTransactionObject();
}
impl ::windows::core::RuntimeName for ITransactionObject {
    const NAME: &'static str = "Windows.Win32.System.Search.ITransactionObject";
}
impl ITransactionObjectVtbl {
    pub const fn new<Impl: ITransactionObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransactionObjectVtbl {
        unsafe extern "system" fn GetTransactionObject<Impl: ITransactionObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ultransactionlevel: u32, pptransactionobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransactionObject(ultransactionlevel, ::core::mem::transmute_copy(&pptransactionobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransactionObject>, base.5, GetTransactionObject::<Impl, OFFSET>)
    }
}
pub trait ITrusteeAdminImpl: Sized {
    fn CompareTrustees();
    fn CreateTrustee();
    fn DeleteTrustee();
    fn SetTrusteeProperties();
    fn GetTrusteeProperties();
}
impl ::windows::core::RuntimeName for ITrusteeAdmin {
    const NAME: &'static str = "Windows.Win32.System.Search.ITrusteeAdmin";
}
impl ITrusteeAdminVtbl {
    pub const fn new<Impl: ITrusteeAdminImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITrusteeAdminVtbl {
        unsafe extern "system" fn CompareTrustees<Impl: ITrusteeAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee1: *mut super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompareTrustees(&*(&ptrustee1 as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType), &*(&ptrustee2 as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrustee<Impl: ITrusteeAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTrustee(&*(&ptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType), cpropertysets, &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTrustee<Impl: ITrusteeAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteTrustee(&*(&ptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrusteeProperties<Impl: ITrusteeAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTrusteeProperties(&*(&ptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType), cpropertysets, &*(&rgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrusteeProperties<Impl: ITrusteeAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTrusteeProperties(
                &*(&ptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType),
                cpropertyidsets,
                &*(&rgpropertyidsets as *const <DBPROPIDSET as ::windows::core::Abi>::Abi as *const <DBPROPIDSET as ::windows::core::DefaultType>::DefaultType),
                pcpropertysets,
                &*(&prgpropertysets as *const <DBPROPSET as ::windows::core::Abi>::Abi as *const <DBPROPSET as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITrusteeAdmin>, base.5, CompareTrustees::<Impl, OFFSET>, CreateTrustee::<Impl, OFFSET>, DeleteTrustee::<Impl, OFFSET>, SetTrusteeProperties::<Impl, OFFSET>, GetTrusteeProperties::<Impl, OFFSET>)
    }
}
pub trait ITrusteeGroupAdminImpl: Sized {
    fn AddMember();
    fn DeleteMember();
    fn IsMember();
    fn GetMembers();
    fn GetMemberships();
}
impl ::windows::core::RuntimeName for ITrusteeGroupAdmin {
    const NAME: &'static str = "Windows.Win32.System.Search.ITrusteeGroupAdmin";
}
impl ITrusteeGroupAdminVtbl {
    pub const fn new<Impl: ITrusteeGroupAdminImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITrusteeGroupAdminVtbl {
        unsafe extern "system" fn AddMember<Impl: ITrusteeGroupAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddMember(&*(&pmembershiptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType), &*(&pmembertrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMember<Impl: ITrusteeGroupAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteMember(&*(&pmembershiptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType), &*(&pmembertrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMember<Impl: ITrusteeGroupAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMember(
                &*(&pmembershiptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType),
                &*(&pmembertrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType),
                &*(&pfstatus as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMembers<Impl: ITrusteeGroupAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMembers(&*(&pmembershiptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType), pcmembers, &*(&prgmembers as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberships<Impl: ITrusteeGroupAdminImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMemberships(&*(&ptrustee as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType), pcmemberships, &*(&prgmemberships as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::Abi>::Abi as *const <super::super::Security::Authorization::TRUSTEE_W as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITrusteeGroupAdmin>, base.5, AddMember::<Impl, OFFSET>, DeleteMember::<Impl, OFFSET>, IsMember::<Impl, OFFSET>, GetMembers::<Impl, OFFSET>, GetMemberships::<Impl, OFFSET>)
    }
}
pub trait IUMSImpl: Sized {
    fn SqlUmsSuspend();
    fn SqlUmsYield();
    fn SqlUmsSwitchPremptive();
    fn SqlUmsSwitchNonPremptive();
    fn SqlUmsFIsPremptive();
}
impl ::windows::core::RuntimeName for IUMS {
    const NAME: &'static str = "Windows.Win32.System.Search.IUMS";
}
impl IUMSVtbl {
    pub const fn new<Impl: IUMSImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUMSVtbl {
        unsafe extern "system" fn SqlUmsSuspend<Impl: IUMSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SqlUmsSuspend(ticks).into()
        }
        unsafe extern "system" fn SqlUmsYield<Impl: IUMSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SqlUmsYield(ticks).into()
        }
        unsafe extern "system" fn SqlUmsSwitchPremptive<Impl: IUMSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SqlUmsSwitchPremptive().into()
        }
        unsafe extern "system" fn SqlUmsSwitchNonPremptive<Impl: IUMSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SqlUmsSwitchNonPremptive().into()
        }
        unsafe extern "system" fn SqlUmsFIsPremptive<Impl: IUMSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SqlUmsFIsPremptive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUMS>, base.5, SqlUmsSuspend::<Impl, OFFSET>, SqlUmsYield::<Impl, OFFSET>, SqlUmsSwitchPremptive::<Impl, OFFSET>, SqlUmsSwitchNonPremptive::<Impl, OFFSET>, SqlUmsFIsPremptive::<Impl, OFFSET>)
    }
}
pub trait IUMSInitializeImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IUMSInitialize {
    const NAME: &'static str = "Windows.Win32.System.Search.IUMSInitialize";
}
impl IUMSInitializeVtbl {
    pub const fn new<Impl: IUMSInitializeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUMSInitializeVtbl {
        unsafe extern "system" fn Initialize<Impl: IUMSInitializeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pums: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pums as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUMSInitialize>, base.5, Initialize::<Impl, OFFSET>)
    }
}
pub trait IUrlAccessorImpl: Sized {
    fn AddRequestParameter();
    fn GetDocFormat();
    fn GetCLSID();
    fn GetHost();
    fn IsDirectory();
    fn GetSize();
    fn GetLastModified();
    fn GetFileName();
    fn GetSecurityDescriptor();
    fn GetRedirectedURL();
    fn GetSecurityProvider();
    fn BindToStream();
    fn BindToFilter();
}
impl ::windows::core::RuntimeName for IUrlAccessor {
    const NAME: &'static str = "Windows.Win32.System.Search.IUrlAccessor";
}
impl IUrlAccessorVtbl {
    pub const fn new<Impl: IUrlAccessorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUrlAccessorVtbl {
        unsafe extern "system" fn AddRequestParameter<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRequestParameter(&*(&pspec as *const <super::Com::StructuredStorage::PROPSPEC as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPSPEC as ::windows::core::DefaultType>::DefaultType), &*(&pvar as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocFormat<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszdocformat: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocFormat(::core::mem::transmute_copy(&wszdocformat), dwsize, ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCLSID<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCLSID(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszhost: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHost(::core::mem::transmute_copy(&wszhost), dwsize, ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirectory<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&pllsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastModified<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftlastmodified: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastModified(::core::mem::transmute_copy(&pftlastmodified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfilename: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileName(::core::mem::transmute_copy(&wszfilename), dwsize, ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&psd), dwsize, ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRedirectedURL<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszredirectedurl: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRedirectedURL(::core::mem::transmute_copy(&wszredirectedurl), dwsize, ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityProvider<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pspclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSecurityProvider(::core::mem::transmute_copy(&pspclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToStream<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BindToStream(::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFilter<Impl: IUrlAccessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BindToFilter(::core::mem::transmute_copy(&ppfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUrlAccessor>,
            base.5,
            AddRequestParameter::<Impl, OFFSET>,
            GetDocFormat::<Impl, OFFSET>,
            GetCLSID::<Impl, OFFSET>,
            GetHost::<Impl, OFFSET>,
            IsDirectory::<Impl, OFFSET>,
            GetSize::<Impl, OFFSET>,
            GetLastModified::<Impl, OFFSET>,
            GetFileName::<Impl, OFFSET>,
            GetSecurityDescriptor::<Impl, OFFSET>,
            GetRedirectedURL::<Impl, OFFSET>,
            GetSecurityProvider::<Impl, OFFSET>,
            BindToStream::<Impl, OFFSET>,
            BindToFilter::<Impl, OFFSET>,
        )
    }
}
pub trait IUrlAccessor2Impl: Sized + IUrlAccessorImpl {
    fn GetDisplayUrl();
    fn IsDocument();
    fn GetCodePage();
}
impl ::windows::core::RuntimeName for IUrlAccessor2 {
    const NAME: &'static str = "Windows.Win32.System.Search.IUrlAccessor2";
}
impl IUrlAccessor2Vtbl {
    pub const fn new<Impl: IUrlAccessor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUrlAccessor2Vtbl {
        unsafe extern "system" fn GetDisplayUrl<Impl: IUrlAccessor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszdocurl: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayUrl(::core::mem::transmute_copy(&wszdocurl), dwsize, ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDocument<Impl: IUrlAccessor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePage<Impl: IUrlAccessor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszcodepage: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodePage(::core::mem::transmute_copy(&wszcodepage), dwsize, ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUrlAccessor2>, base.5, GetDisplayUrl::<Impl, OFFSET>, IsDocument::<Impl, OFFSET>, GetCodePage::<Impl, OFFSET>)
    }
}
pub trait IUrlAccessor3Impl: Sized + IUrlAccessor2Impl + IUrlAccessorImpl {
    fn GetImpersonationSidBlobs();
}
impl ::windows::core::RuntimeName for IUrlAccessor3 {
    const NAME: &'static str = "Windows.Win32.System.Search.IUrlAccessor3";
}
impl IUrlAccessor3Vtbl {
    pub const fn new<Impl: IUrlAccessor3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUrlAccessor3Vtbl {
        unsafe extern "system" fn GetImpersonationSidBlobs<Impl: IUrlAccessor3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImpersonationSidBlobs(&*(&pcwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcsidcount), ::core::mem::transmute_copy(&ppsidblobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUrlAccessor3>, base.5, GetImpersonationSidBlobs::<Impl, OFFSET>)
    }
}
pub trait IUrlAccessor4Impl: Sized + IUrlAccessor3Impl + IUrlAccessor2Impl + IUrlAccessorImpl {
    fn ShouldIndexItemContent();
    fn ShouldIndexProperty();
}
impl ::windows::core::RuntimeName for IUrlAccessor4 {
    const NAME: &'static str = "Windows.Win32.System.Search.IUrlAccessor4";
}
impl IUrlAccessor4Vtbl {
    pub const fn new<Impl: IUrlAccessor4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUrlAccessor4Vtbl {
        unsafe extern "system" fn ShouldIndexItemContent<Impl: IUrlAccessor4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfindexcontent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldIndexItemContent(::core::mem::transmute_copy(&pfindexcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldIndexProperty<Impl: IUrlAccessor4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfindexproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldIndexProperty(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfindexproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUrlAccessor4>, base.5, ShouldIndexItemContent::<Impl, OFFSET>, ShouldIndexProperty::<Impl, OFFSET>)
    }
}
pub trait IViewChapterImpl: Sized {
    fn GetSpecification();
    fn OpenViewChapter();
}
impl ::windows::core::RuntimeName for IViewChapter {
    const NAME: &'static str = "Windows.Win32.System.Search.IViewChapter";
}
impl IViewChapterVtbl {
    pub const fn new<Impl: IViewChapterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IViewChapterVtbl {
        unsafe extern "system" fn GetSpecification<Impl: IViewChapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpecification(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprowset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewChapter<Impl: IViewChapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsource: usize, phviewchapter: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenViewChapter(hsource, ::core::mem::transmute_copy(&phviewchapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IViewChapter>, base.5, GetSpecification::<Impl, OFFSET>, OpenViewChapter::<Impl, OFFSET>)
    }
}
pub trait IViewFilterImpl: Sized {
    fn GetFilter();
    fn GetFilterBindings();
    fn SetFilter();
}
impl ::windows::core::RuntimeName for IViewFilter {
    const NAME: &'static str = "Windows.Win32.System.Search.IViewFilter";
}
impl IViewFilterVtbl {
    pub const fn new<Impl: IViewFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IViewFilterVtbl {
        unsafe extern "system" fn GetFilter<Impl: IViewFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilter(haccessor, ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&pcompareops), ::core::mem::transmute_copy(&pcriteriadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterBindings<Impl: IViewFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterBindings(::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Impl: IViewFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: usize, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFilter(haccessor, crows, compareops, &*(&pcriteriadata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IViewFilter>, base.5, GetFilter::<Impl, OFFSET>, GetFilterBindings::<Impl, OFFSET>, SetFilter::<Impl, OFFSET>)
    }
}
pub trait IViewRowsetImpl: Sized {
    fn GetSpecification();
    fn OpenViewRowset();
}
impl ::windows::core::RuntimeName for IViewRowset {
    const NAME: &'static str = "Windows.Win32.System.Search.IViewRowset";
}
impl IViewRowsetVtbl {
    pub const fn new<Impl: IViewRowsetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IViewRowsetVtbl {
        unsafe extern "system" fn GetSpecification<Impl: IViewRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpecification(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewRowset<Impl: IViewRowsetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenViewRowset(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprowset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IViewRowset>, base.5, GetSpecification::<Impl, OFFSET>, OpenViewRowset::<Impl, OFFSET>)
    }
}
pub trait IViewSortImpl: Sized {
    fn GetSortOrder();
    fn SetSortOrder();
}
impl ::windows::core::RuntimeName for IViewSort {
    const NAME: &'static str = "Windows.Win32.System.Search.IViewSort";
}
impl IViewSortVtbl {
    pub const fn new<Impl: IViewSortImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IViewSortVtbl {
        unsafe extern "system" fn GetSortOrder<Impl: IViewSortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSortOrder(::core::mem::transmute_copy(&pcvalues), ::core::mem::transmute_copy(&prgcolumns), ::core::mem::transmute_copy(&prgorders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSortOrder<Impl: IViewSortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSortOrder(cvalues, rgcolumns, rgorders) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IViewSort>, base.5, GetSortOrder::<Impl, OFFSET>, SetSortOrder::<Impl, OFFSET>)
    }
}
pub trait IWordBreakerImpl: Sized {
    fn Init();
    fn BreakText();
    fn ComposePhrase();
    fn GetLicenseToUse();
}
impl ::windows::core::RuntimeName for IWordBreaker {
    const NAME: &'static str = "Windows.Win32.System.Search.IWordBreaker";
}
impl IWordBreakerVtbl {
    pub const fn new<Impl: IWordBreakerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWordBreakerVtbl {
        unsafe extern "system" fn Init<Impl: IWordBreakerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Init(&*(&fquery as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ulmaxtokensize, &*(&pflicense as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BreakText<Impl: IWordBreakerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptextsource: *mut TEXT_SOURCE, pwordsink: ::windows::core::RawPtr, pphrasesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BreakText(
                &*(&ptextsource as *const <TEXT_SOURCE as ::windows::core::Abi>::Abi as *const <TEXT_SOURCE as ::windows::core::DefaultType>::DefaultType),
                &*(&pwordsink as *const <IWordSink as ::windows::core::Abi>::Abi as *const <IWordSink as ::windows::core::DefaultType>::DefaultType),
                &*(&pphrasesink as *const <super::super::Storage::IndexServer::IPhraseSink as ::windows::core::Abi>::Abi as *const <super::super::Storage::IndexServer::IPhraseSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComposePhrase<Impl: IWordBreakerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: super::super::Foundation::PWSTR, pcwcphrase: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComposePhrase(
                &*(&pwcnoun as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cwcnoun,
                &*(&pwcmodifier as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cwcmodifier,
                ulattachmenttype,
                &*(&pwcphrase as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                pcwcphrase,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseToUse<Impl: IWordBreakerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLicenseToUse(ppwcslicense) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWordBreaker>, base.5, Init::<Impl, OFFSET>, BreakText::<Impl, OFFSET>, ComposePhrase::<Impl, OFFSET>, GetLicenseToUse::<Impl, OFFSET>)
    }
}
pub trait IWordFormSinkImpl: Sized {
    fn PutAltWord();
    fn PutWord();
}
impl ::windows::core::RuntimeName for IWordFormSink {
    const NAME: &'static str = "Windows.Win32.System.Search.IWordFormSink";
}
impl IWordFormSinkVtbl {
    pub const fn new<Impl: IWordFormSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWordFormSinkVtbl {
        unsafe extern "system" fn PutAltWord<Impl: IWordFormSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutAltWord(&*(&pwcinbuf as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwc) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutWord<Impl: IWordFormSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutWord(&*(&pwcinbuf as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwc) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWordFormSink>, base.5, PutAltWord::<Impl, OFFSET>, PutWord::<Impl, OFFSET>)
    }
}
pub trait IWordSinkImpl: Sized {
    fn PutWord();
    fn PutAltWord();
    fn StartAltPhrase();
    fn EndAltPhrase();
    fn PutBreak();
}
impl ::windows::core::RuntimeName for IWordSink {
    const NAME: &'static str = "Windows.Win32.System.Search.IWordSink";
}
impl IWordSinkVtbl {
    pub const fn new<Impl: IWordSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWordSinkVtbl {
        unsafe extern "system" fn PutWord<Impl: IWordSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: super::super::Foundation::PWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutWord(cwc, &*(&pwcinbuf as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcsrclen, cwcsrcpos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAltWord<Impl: IWordSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: super::super::Foundation::PWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutAltWord(cwc, &*(&pwcinbuf as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcsrclen, cwcsrcpos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAltPhrase<Impl: IWordSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAltPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAltPhrase<Impl: IWordSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndAltPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutBreak<Impl: IWordSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutBreak(breaktype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWordSink>, base.5, PutWord::<Impl, OFFSET>, PutAltWord::<Impl, OFFSET>, StartAltPhrase::<Impl, OFFSET>, EndAltPhrase::<Impl, OFFSET>, PutBreak::<Impl, OFFSET>)
    }
}
pub trait OLEDBSimpleProviderImpl: Sized {
    fn getRowCount();
    fn getColumnCount();
    fn getRWStatus();
    fn getVariant();
    fn setVariant();
    fn getLocale();
    fn deleteRows();
    fn insertRows();
    fn find();
    fn addOLEDBSimpleProviderListener();
    fn removeOLEDBSimpleProviderListener();
    fn isAsync();
    fn getEstimatedRows();
    fn stopTransfer();
}
impl ::windows::core::RuntimeName for OLEDBSimpleProvider {
    const NAME: &'static str = "Windows.Win32.System.Search.OLEDBSimpleProvider";
}
impl OLEDBSimpleProviderVtbl {
    pub const fn new<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> OLEDBSimpleProviderVtbl {
        unsafe extern "system" fn getRowCount<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcrows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getRowCount(::core::mem::transmute_copy(&pcrows)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getColumnCount<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccolumns: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getColumnCount(::core::mem::transmute_copy(&pccolumns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getRWStatus<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, prwstatus: *mut OSPRW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getRWStatus(irow, icolumn, ::core::mem::transmute_copy(&prwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getVariant<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, pvar: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getVariant(irow, icolumn, format, ::core::mem::transmute_copy(&pvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setVariant<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setVariant(irow, icolumn, format, &*(&var as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocale<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlocale: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getLocale(::core::mem::transmute_copy(&pbstrlocale)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deleteRows<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsdeleted: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).deleteRows(irow, crows, ::core::mem::transmute_copy(&pcrowsdeleted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertRows<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsinserted: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).insertRows(irow, crows, ::core::mem::transmute_copy(&pcrowsinserted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn find<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irowstart: isize, icolumn: isize, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>, findflags: OSPFIND, comptype: OSPCOMP, pirowfound: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).find(irowstart, icolumn, &*(&val as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), findflags, comptype, ::core::mem::transmute_copy(&pirowfound)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addOLEDBSimpleProviderListener<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pospilistener: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addOLEDBSimpleProviderListener(&*(&pospilistener as *const <OLEDBSimpleProviderListener as ::windows::core::Abi>::Abi as *const <OLEDBSimpleProviderListener as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeOLEDBSimpleProviderListener<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pospilistener: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeOLEDBSimpleProviderListener(&*(&pospilistener as *const <OLEDBSimpleProviderListener as ::windows::core::Abi>::Abi as *const <OLEDBSimpleProviderListener as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAsync<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbasynch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).isAsync(::core::mem::transmute_copy(&pbasynch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEstimatedRows<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pirows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getEstimatedRows(::core::mem::transmute_copy(&pirows)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stopTransfer<Impl: OLEDBSimpleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).stopTransfer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<OLEDBSimpleProvider>,
            base.5,
            getRowCount::<Impl, OFFSET>,
            getColumnCount::<Impl, OFFSET>,
            getRWStatus::<Impl, OFFSET>,
            getVariant::<Impl, OFFSET>,
            setVariant::<Impl, OFFSET>,
            getLocale::<Impl, OFFSET>,
            deleteRows::<Impl, OFFSET>,
            insertRows::<Impl, OFFSET>,
            find::<Impl, OFFSET>,
            addOLEDBSimpleProviderListener::<Impl, OFFSET>,
            removeOLEDBSimpleProviderListener::<Impl, OFFSET>,
            isAsync::<Impl, OFFSET>,
            getEstimatedRows::<Impl, OFFSET>,
            stopTransfer::<Impl, OFFSET>,
        )
    }
}
pub trait OLEDBSimpleProviderListenerImpl: Sized {
    fn aboutToChangeCell();
    fn cellChanged();
    fn aboutToDeleteRows();
    fn deletedRows();
    fn aboutToInsertRows();
    fn insertedRows();
    fn rowsAvailable();
    fn transferComplete();
}
impl ::windows::core::RuntimeName for OLEDBSimpleProviderListener {
    const NAME: &'static str = "Windows.Win32.System.Search.OLEDBSimpleProviderListener";
}
impl OLEDBSimpleProviderListenerVtbl {
    pub const fn new<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> OLEDBSimpleProviderListenerVtbl {
        unsafe extern "system" fn aboutToChangeCell<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).aboutToChangeCell(irow, icolumn) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cellChanged<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).cellChanged(irow, icolumn) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn aboutToDeleteRows<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).aboutToDeleteRows(irow, crows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deletedRows<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).deletedRows(irow, crows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn aboutToInsertRows<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).aboutToInsertRows(irow, crows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertedRows<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).insertedRows(irow, crows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn rowsAvailable<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).rowsAvailable(irow, crows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transferComplete<Impl: OLEDBSimpleProviderListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xfer: OSPXFER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).transferComplete(xfer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<OLEDBSimpleProviderListener>, base.5, aboutToChangeCell::<Impl, OFFSET>, cellChanged::<Impl, OFFSET>, aboutToDeleteRows::<Impl, OFFSET>, deletedRows::<Impl, OFFSET>, aboutToInsertRows::<Impl, OFFSET>, insertedRows::<Impl, OFFSET>, rowsAvailable::<Impl, OFFSET>, transferComplete::<Impl, OFFSET>)
    }
}
