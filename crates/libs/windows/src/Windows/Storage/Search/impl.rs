#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContentIndexerImpl: Sized {
    fn AddAsync(&mut self, indexablecontent: &::core::option::Option<IIndexableContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateAsync(&mut self, indexablecontent: &::core::option::Option<IIndexableContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&mut self, contentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteMultipleAsync(&mut self, contentids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAllAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetrievePropertiesAsync(&mut self, contentid: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn Revision(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentIndexer {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContentIndexerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentIndexerVtbl {
        unsafe extern "system" fn AddAsync<Impl: IContentIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexablecontent: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAsync(&*(&indexablecontent as *const <IIndexableContent as ::windows::core::Abi>::Abi as *const <IIndexableContent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAsync<Impl: IContentIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexablecontent: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAsync(&*(&indexablecontent as *const <IIndexableContent as ::windows::core::Abi>::Abi as *const <IIndexableContent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IContentIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(&*(&contentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMultipleAsync<Impl: IContentIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMultipleAsync(&*(&contentids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAllAsync<Impl: IContentIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrievePropertiesAsync<Impl: IContentIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrievePropertiesAsync(&*(&contentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revision<Impl: IContentIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentIndexer, BASE_OFFSET>(),
            AddAsync: AddAsync::<Impl, IMPL_OFFSET>,
            UpdateAsync: UpdateAsync::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            DeleteMultipleAsync: DeleteMultipleAsync::<Impl, IMPL_OFFSET>,
            DeleteAllAsync: DeleteAllAsync::<Impl, IMPL_OFFSET>,
            RetrievePropertiesAsync: RetrievePropertiesAsync::<Impl, IMPL_OFFSET>,
            Revision: Revision::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentIndexer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContentIndexerQueryImpl: Sized {
    fn GetCountAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetPropertiesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>;
    fn GetPropertiesRangeAsync(&mut self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>;
    fn GetAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>;
    fn GetRangeAsync(&mut self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>;
    fn QueryFolder(&mut self) -> ::windows::core::Result<super::StorageFolder>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentIndexerQuery {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexerQuery";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContentIndexerQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentIndexerQueryVtbl {
        unsafe extern "system" fn GetCountAsync<Impl: IContentIndexerQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesAsync<Impl: IContentIndexerQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesRangeAsync<Impl: IContentIndexerQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertiesRangeAsync(startindex, maxitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Impl: IContentIndexerQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeAsync<Impl: IContentIndexerQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeAsync(startindex, maxitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryFolder<Impl: IContentIndexerQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentIndexerQuery, BASE_OFFSET>(),
            GetCountAsync: GetCountAsync::<Impl, IMPL_OFFSET>,
            GetPropertiesAsync: GetPropertiesAsync::<Impl, IMPL_OFFSET>,
            GetPropertiesRangeAsync: GetPropertiesRangeAsync::<Impl, IMPL_OFFSET>,
            GetAsync: GetAsync::<Impl, IMPL_OFFSET>,
            GetRangeAsync: GetRangeAsync::<Impl, IMPL_OFFSET>,
            QueryFolder: QueryFolder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentIndexerQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContentIndexerQueryOperationsImpl: Sized {
    fn CreateQueryWithSortOrderAndLanguage(&mut self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, sortorder: &::core::option::Option<super::super::Foundation::Collections::IIterable<SortEntry>>, searchfilterlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<ContentIndexerQuery>;
    fn CreateQueryWithSortOrder(&mut self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, sortorder: &::core::option::Option<super::super::Foundation::Collections::IIterable<SortEntry>>) -> ::windows::core::Result<ContentIndexerQuery>;
    fn CreateQuery(&mut self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<ContentIndexerQuery>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentIndexerQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexerQueryOperations";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContentIndexerQueryOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerQueryOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentIndexerQueryOperationsVtbl {
        unsafe extern "system" fn CreateQueryWithSortOrderAndLanguage<Impl: IContentIndexerQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, sortorder: ::windows::core::RawPtr, searchfilterlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWithSortOrderAndLanguage(
                &*(&searchfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&sortorder as *const <super::super::Foundation::Collections::IIterable<SortEntry> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SortEntry> as ::windows::core::DefaultType>::DefaultType),
                &*(&searchfilterlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWithSortOrder<Impl: IContentIndexerQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, sortorder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWithSortOrder(
                &*(&searchfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&sortorder as *const <super::super::Foundation::Collections::IIterable<SortEntry> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SortEntry> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Impl: IContentIndexerQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery(&*(&searchfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentIndexerQueryOperations, BASE_OFFSET>(),
            CreateQueryWithSortOrderAndLanguage: CreateQueryWithSortOrderAndLanguage::<Impl, IMPL_OFFSET>,
            CreateQueryWithSortOrder: CreateQueryWithSortOrder::<Impl, IMPL_OFFSET>,
            CreateQuery: CreateQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentIndexerQueryOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerStaticsImpl: Sized {
    fn GetIndexerWithName(&mut self, indexname: &::windows::core::HSTRING) -> ::windows::core::Result<ContentIndexer>;
    fn GetIndexer(&mut self) -> ::windows::core::Result<ContentIndexer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentIndexerStatics {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContentIndexerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentIndexerStaticsVtbl {
        unsafe extern "system" fn GetIndexerWithName<Impl: IContentIndexerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndexerWithName(&*(&indexname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexer<Impl: IContentIndexerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndexer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentIndexerStatics, BASE_OFFSET>(),
            GetIndexerWithName: GetIndexerWithName::<Impl, IMPL_OFFSET>,
            GetIndexer: GetIndexer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentIndexerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IIndexableContentImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Stream(&mut self) -> ::windows::core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&mut self, value: &::core::option::Option<super::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StreamContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamContentType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IIndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IIndexableContent";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IIndexableContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIndexableContentVtbl {
        unsafe extern "system" fn Id<Impl: IIndexableContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IIndexableContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IIndexableContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: IIndexableContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStream<Impl: IIndexableContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStream(&*(&value as *const <super::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StreamContentType<Impl: IIndexableContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamContentType<Impl: IIndexableContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIndexableContent, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Stream: Stream::<Impl, IMPL_OFFSET>,
            SetStream: SetStream::<Impl, IMPL_OFFSET>,
            StreamContentType: StreamContentType::<Impl, IMPL_OFFSET>,
            SetStreamContentType: SetStreamContentType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndexableContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties", feature = "implement_exclusive"))]
pub trait IQueryOptionsImpl: Sized {
    fn FileTypeFilter(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn FolderDepth(&mut self) -> ::windows::core::Result<FolderDepth>;
    fn SetFolderDepth(&mut self, value: FolderDepth) -> ::windows::core::Result<()>;
    fn ApplicationSearchFilter(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetApplicationSearchFilter(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserSearchFilter(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserSearchFilter(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IndexerOption(&mut self) -> ::windows::core::Result<IndexerOption>;
    fn SetIndexerOption(&mut self, value: IndexerOption) -> ::windows::core::Result<()>;
    fn SortOrder(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SortEntry>>;
    fn GroupPropertyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateStackOption(&mut self) -> ::windows::core::Result<DateStackOption>;
    fn SaveToString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LoadFromString(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetThumbnailPrefetch(&mut self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<()>;
    fn SetPropertyPrefetch(&mut self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IQueryOptions {
    const NAME: &'static str = "Windows.Storage.Search.IQueryOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties", feature = "implement_exclusive"))]
impl IQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryOptionsVtbl {
        unsafe extern "system" fn FileTypeFilter<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileTypeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderDepth<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FolderDepth) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFolderDepth<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FolderDepth) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFolderDepth(value).into()
        }
        unsafe extern "system" fn ApplicationSearchFilter<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationSearchFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationSearchFilter<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationSearchFilter(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserSearchFilter<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSearchFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserSearchFilter<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserSearchFilter(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IndexerOption<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IndexerOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexerOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexerOption<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IndexerOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndexerOption(value).into()
        }
        unsafe extern "system" fn SortOrder<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupPropertyName<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupPropertyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateStackOption<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DateStackOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateStackOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToString<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromString<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadFromString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetThumbnailPrefetch<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailPrefetch(mode, requestedsize, options).into()
        }
        unsafe extern "system" fn SetPropertyPrefetch<Impl: IQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyPrefetch(options, &*(&propertiestoretrieve as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IQueryOptions, BASE_OFFSET>(),
            FileTypeFilter: FileTypeFilter::<Impl, IMPL_OFFSET>,
            FolderDepth: FolderDepth::<Impl, IMPL_OFFSET>,
            SetFolderDepth: SetFolderDepth::<Impl, IMPL_OFFSET>,
            ApplicationSearchFilter: ApplicationSearchFilter::<Impl, IMPL_OFFSET>,
            SetApplicationSearchFilter: SetApplicationSearchFilter::<Impl, IMPL_OFFSET>,
            UserSearchFilter: UserSearchFilter::<Impl, IMPL_OFFSET>,
            SetUserSearchFilter: SetUserSearchFilter::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            IndexerOption: IndexerOption::<Impl, IMPL_OFFSET>,
            SetIndexerOption: SetIndexerOption::<Impl, IMPL_OFFSET>,
            SortOrder: SortOrder::<Impl, IMPL_OFFSET>,
            GroupPropertyName: GroupPropertyName::<Impl, IMPL_OFFSET>,
            DateStackOption: DateStackOption::<Impl, IMPL_OFFSET>,
            SaveToString: SaveToString::<Impl, IMPL_OFFSET>,
            LoadFromString: LoadFromString::<Impl, IMPL_OFFSET>,
            SetThumbnailPrefetch: SetThumbnailPrefetch::<Impl, IMPL_OFFSET>,
            SetPropertyPrefetch: SetPropertyPrefetch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IQueryOptionsFactoryImpl: Sized {
    fn CreateCommonFileQuery(&mut self, query: CommonFileQuery, filetypefilter: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<QueryOptions>;
    fn CreateCommonFolderQuery(&mut self, query: CommonFolderQuery) -> ::windows::core::Result<QueryOptions>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IQueryOptionsFactory {
    const NAME: &'static str = "Windows.Storage.Search.IQueryOptionsFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IQueryOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOptionsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryOptionsFactoryVtbl {
        unsafe extern "system" fn CreateCommonFileQuery<Impl: IQueryOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, filetypefilter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommonFileQuery(query, &*(&filetypefilter as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommonFolderQuery<Impl: IQueryOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCommonFolderQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IQueryOptionsFactory, BASE_OFFSET>(),
            CreateCommonFileQuery: CreateCommonFileQuery::<Impl, IMPL_OFFSET>,
            CreateCommonFolderQuery: CreateCommonFolderQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryOptionsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IQueryOptionsWithProviderFilterImpl: Sized {
    fn StorageProviderIdFilter(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IQueryOptionsWithProviderFilter {
    const NAME: &'static str = "Windows.Storage.Search.IQueryOptionsWithProviderFilter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IQueryOptionsWithProviderFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOptionsWithProviderFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryOptionsWithProviderFilterVtbl {
        unsafe extern "system" fn StorageProviderIdFilter<Impl: IQueryOptionsWithProviderFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageProviderIdFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IQueryOptionsWithProviderFilter, BASE_OFFSET>(),
            StorageProviderIdFilter: StorageProviderIdFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryOptionsWithProviderFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageFileQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetFilesAsync(&mut self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncDefaultStartAndCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageFileQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFileQueryResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageFileQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileQueryResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFileQueryResultVtbl {
        unsafe extern "system" fn GetFilesAsync<Impl: IStorageFileQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsync(startindex, maxnumberofitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncDefaultStartAndCount<Impl: IStorageFileQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsyncDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFileQueryResult, BASE_OFFSET>(),
            GetFilesAsync: GetFilesAsync::<Impl, IMPL_OFFSET>,
            GetFilesAsyncDefaultStartAndCount: GetFilesAsyncDefaultStartAndCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFileQueryResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Text", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageFileQueryResult2Impl: Sized + IStorageQueryResultBaseImpl {
    fn GetMatchingPropertiesWithRanges(&mut self, file: &::core::option::Option<super::StorageFile>) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>>;
}
#[cfg(all(feature = "Data_Text", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageFileQueryResult2 {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFileQueryResult2";
}
#[cfg(all(feature = "Data_Text", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageFileQueryResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileQueryResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFileQueryResult2Vtbl {
        unsafe extern "system" fn GetMatchingPropertiesWithRanges<Impl: IStorageFileQueryResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingPropertiesWithRanges(&*(&file as *const <super::StorageFile as ::windows::core::Abi>::Abi as *const <super::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFileQueryResult2, BASE_OFFSET>(),
            GetMatchingPropertiesWithRanges: GetMatchingPropertiesWithRanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFileQueryResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageFolderQueryOperationsImpl: Sized {
    fn GetIndexedStateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&mut self) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&mut self, query: CommonFileQuery) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&mut self) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&mut self, query: CommonFolderQuery) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&mut self) -> ::windows::core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&mut self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&mut self, query: CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&mut self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&mut self, query: CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&mut self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<bool>;
    fn IsCommonFolderQuerySupported(&mut self, query: CommonFolderQuery) -> ::windows::core::Result<bool>;
    fn IsCommonFileQuerySupported(&mut self, query: CommonFileQuery) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
#[cfg(feature = "Foundation")]
impl IStorageFolderQueryOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderQueryOperationsVtbl {
        unsafe extern "system" fn GetIndexedStateAsync<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndexedStateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryOverloadDefault<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQuery<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryWithOptions<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileQueryWithOptions(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryOverloadDefault<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQuery<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryWithOptions<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderQueryWithOptions(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQuery<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItemQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQueryWithOptions<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItemQueryWithOptions(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsync<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultStartAndCount<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsync<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultStartAndCount<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsync<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsAsync(startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreQueryOptionsSupported<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreQueryOptionsSupported(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFolderQuerySupported<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCommonFolderQuerySupported(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFileQuerySupported<Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCommonFileQuerySupported(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolderQueryOperations, BASE_OFFSET>(),
            GetIndexedStateAsync: GetIndexedStateAsync::<Impl, IMPL_OFFSET>,
            CreateFileQueryOverloadDefault: CreateFileQueryOverloadDefault::<Impl, IMPL_OFFSET>,
            CreateFileQuery: CreateFileQuery::<Impl, IMPL_OFFSET>,
            CreateFileQueryWithOptions: CreateFileQueryWithOptions::<Impl, IMPL_OFFSET>,
            CreateFolderQueryOverloadDefault: CreateFolderQueryOverloadDefault::<Impl, IMPL_OFFSET>,
            CreateFolderQuery: CreateFolderQuery::<Impl, IMPL_OFFSET>,
            CreateFolderQueryWithOptions: CreateFolderQueryWithOptions::<Impl, IMPL_OFFSET>,
            CreateItemQuery: CreateItemQuery::<Impl, IMPL_OFFSET>,
            CreateItemQueryWithOptions: CreateItemQueryWithOptions::<Impl, IMPL_OFFSET>,
            GetFilesAsync: GetFilesAsync::<Impl, IMPL_OFFSET>,
            GetFilesAsyncOverloadDefaultStartAndCount: GetFilesAsyncOverloadDefaultStartAndCount::<Impl, IMPL_OFFSET>,
            GetFoldersAsync: GetFoldersAsync::<Impl, IMPL_OFFSET>,
            GetFoldersAsyncOverloadDefaultStartAndCount: GetFoldersAsyncOverloadDefaultStartAndCount::<Impl, IMPL_OFFSET>,
            GetItemsAsync: GetItemsAsync::<Impl, IMPL_OFFSET>,
            AreQueryOptionsSupported: AreQueryOptionsSupported::<Impl, IMPL_OFFSET>,
            IsCommonFolderQuerySupported: IsCommonFolderQuerySupported::<Impl, IMPL_OFFSET>,
            IsCommonFileQuerySupported: IsCommonFileQuerySupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderQueryOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageFolderQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetFoldersAsync(&mut self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncDefaultStartAndCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageFolderQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageFolderQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderQueryResultVtbl {
        unsafe extern "system" fn GetFoldersAsync<Impl: IStorageFolderQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsync(startindex, maxnumberofitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncDefaultStartAndCount<Impl: IStorageFolderQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsyncDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolderQueryResult, BASE_OFFSET>(),
            GetFoldersAsync: GetFoldersAsync::<Impl, IMPL_OFFSET>,
            GetFoldersAsyncDefaultStartAndCount: GetFoldersAsyncDefaultStartAndCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderQueryResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageItemQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetItemsAsync(&mut self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn GetItemsAsyncDefaultStartAndCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageItemQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.IStorageItemQueryResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageItemQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemQueryResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemQueryResultVtbl {
        unsafe extern "system" fn GetItemsAsync<Impl: IStorageItemQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsAsync(startindex, maxnumberofitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsyncDefaultStartAndCount<Impl: IStorageItemQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsAsyncDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemQueryResult, BASE_OFFSET>(),
            GetItemsAsync: GetItemsAsync::<Impl, IMPL_OFFSET>,
            GetItemsAsyncDefaultStartAndCount: GetItemsAsyncDefaultStartAndCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemQueryResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerTriggerDetailsImpl: Sized {
    fn Folder(&mut self) -> ::windows::core::Result<super::StorageFolder>;
    fn ChangeTracker(&mut self) -> ::windows::core::Result<super::StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTrackerTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.IStorageLibraryChangeTrackerTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTrackerTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTrackerTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeTrackerTriggerDetailsVtbl {
        unsafe extern "system" fn Folder<Impl: IStorageLibraryChangeTrackerTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeTracker<Impl: IStorageLibraryChangeTrackerTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeTracker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChangeTrackerTriggerDetails, BASE_OFFSET>(),
            Folder: Folder::<Impl, IMPL_OFFSET>,
            ChangeTracker: ChangeTracker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChangeTrackerTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageLibraryContentChangedTriggerDetailsImpl: Sized {
    fn Folder(&mut self) -> ::windows::core::Result<super::StorageFolder>;
    fn CreateModifiedSinceQuery(&mut self, lastquerytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<StorageItemQueryResult>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibraryContentChangedTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.IStorageLibraryContentChangedTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageLibraryContentChangedTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryContentChangedTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryContentChangedTriggerDetailsVtbl {
        unsafe extern "system" fn Folder<Impl: IStorageLibraryContentChangedTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateModifiedSinceQuery<Impl: IStorageLibraryContentChangedTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastquerytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateModifiedSinceQuery(&*(&lastquerytime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryContentChangedTriggerDetails, BASE_OFFSET>(),
            Folder: Folder::<Impl, IMPL_OFFSET>,
            CreateModifiedSinceQuery: CreateModifiedSinceQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryContentChangedTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageQueryResultBaseImpl: Sized {
    fn GetItemCountAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(&mut self) -> ::windows::core::Result<super::StorageFolder>;
    fn ContentsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OptionsChanged(&mut self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindStartIndexAsync(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&mut self) -> ::windows::core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&mut self, newqueryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageQueryResultBase {
    const NAME: &'static str = "Windows.Storage.Search.IStorageQueryResultBase";
}
#[cfg(feature = "Foundation")]
impl IStorageQueryResultBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageQueryResultBaseVtbl {
        unsafe extern "system" fn GetItemCountAsync<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folder<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentsChanged<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentsChanged<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContentsChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionsChanged<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionsChanged(&*(&changedhandler as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionsChanged<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOptionsChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindStartIndexAsync<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStartIndexAsync(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentQueryOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyNewQueryOptions<Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newqueryoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyNewQueryOptions(&*(&newqueryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageQueryResultBase, BASE_OFFSET>(),
            GetItemCountAsync: GetItemCountAsync::<Impl, IMPL_OFFSET>,
            Folder: Folder::<Impl, IMPL_OFFSET>,
            ContentsChanged: ContentsChanged::<Impl, IMPL_OFFSET>,
            RemoveContentsChanged: RemoveContentsChanged::<Impl, IMPL_OFFSET>,
            OptionsChanged: OptionsChanged::<Impl, IMPL_OFFSET>,
            RemoveOptionsChanged: RemoveOptionsChanged::<Impl, IMPL_OFFSET>,
            FindStartIndexAsync: FindStartIndexAsync::<Impl, IMPL_OFFSET>,
            GetCurrentQueryOptions: GetCurrentQueryOptions::<Impl, IMPL_OFFSET>,
            ApplyNewQueryOptions: ApplyNewQueryOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageQueryResultBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IValueAndLanguageImpl: Sized {
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IValueAndLanguage {
    const NAME: &'static str = "Windows.Storage.Search.IValueAndLanguage";
}
#[cfg(feature = "implement_exclusive")]
impl IValueAndLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueAndLanguageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValueAndLanguageVtbl {
        unsafe extern "system" fn Language<Impl: IValueAndLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IValueAndLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: IValueAndLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IValueAndLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IValueAndLanguage, BASE_OFFSET>(),
            Language: Language::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueAndLanguage as ::windows::core::Interface>::IID
    }
}
