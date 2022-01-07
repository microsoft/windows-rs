#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerImpl: Sized {
    fn AddAsync(&self, indexablecontent: &::core::option::Option<IIndexableContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateAsync(&self, indexablecontent: &::core::option::Option<IIndexableContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self, contentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteMultipleAsync(&self, contentids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetrievePropertiesAsync(&self, contentid: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn Revision(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentIndexer {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexer";
}
#[cfg(feature = "implement_exclusive")]
impl IContentIndexerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerImpl, const OFFSET: isize>() -> IContentIndexerVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentIndexer>, ::windows::core::GetTrustLevel, AddAsync::<Impl, OFFSET>, UpdateAsync::<Impl, OFFSET>, DeleteAsync::<Impl, OFFSET>, DeleteMultipleAsync::<Impl, OFFSET>, DeleteAllAsync::<Impl, OFFSET>, RetrievePropertiesAsync::<Impl, OFFSET>, Revision::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerQueryImpl: Sized {
    fn GetCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>;
    fn GetPropertiesRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>;
    fn GetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>;
    fn GetRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>;
    fn QueryFolder(&self) -> ::windows::core::Result<super::StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentIndexerQuery {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexerQuery";
}
#[cfg(feature = "implement_exclusive")]
impl IContentIndexerQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerQueryImpl, const OFFSET: isize>() -> IContentIndexerQueryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentIndexerQuery>, ::windows::core::GetTrustLevel, GetCountAsync::<Impl, OFFSET>, GetPropertiesAsync::<Impl, OFFSET>, GetPropertiesRangeAsync::<Impl, OFFSET>, GetAsync::<Impl, OFFSET>, GetRangeAsync::<Impl, OFFSET>, QueryFolder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerQueryOperationsImpl: Sized {
    fn CreateQueryWithSortOrderAndLanguage(&self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, sortorder: &::core::option::Option<super::super::Foundation::Collections::IIterable<SortEntry>>, searchfilterlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<ContentIndexerQuery>;
    fn CreateQueryWithSortOrder(&self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, sortorder: &::core::option::Option<super::super::Foundation::Collections::IIterable<SortEntry>>) -> ::windows::core::Result<ContentIndexerQuery>;
    fn CreateQuery(&self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<ContentIndexerQuery>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentIndexerQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexerQueryOperations";
}
#[cfg(feature = "implement_exclusive")]
impl IContentIndexerQueryOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerQueryOperationsImpl, const OFFSET: isize>() -> IContentIndexerQueryOperationsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentIndexerQueryOperations>, ::windows::core::GetTrustLevel, CreateQueryWithSortOrderAndLanguage::<Impl, OFFSET>, CreateQueryWithSortOrder::<Impl, OFFSET>, CreateQuery::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerStaticsImpl: Sized {
    fn GetIndexerWithName(&self, indexname: &::windows::core::HSTRING) -> ::windows::core::Result<ContentIndexer>;
    fn GetIndexer(&self) -> ::windows::core::Result<ContentIndexer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentIndexerStatics {
    const NAME: &'static str = "Windows.Storage.Search.IContentIndexerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContentIndexerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentIndexerStaticsImpl, const OFFSET: isize>() -> IContentIndexerStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentIndexerStatics>, ::windows::core::GetTrustLevel, GetIndexerWithName::<Impl, OFFSET>, GetIndexer::<Impl, OFFSET>)
    }
}
pub trait IIndexableContentImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Stream(&self) -> ::windows::core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&self, value: &::core::option::Option<super::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StreamContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IIndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IIndexableContent";
}
impl IIndexableContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContentImpl, const OFFSET: isize>() -> IIndexableContentVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIndexableContent>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, SetId::<Impl, OFFSET>, Properties::<Impl, OFFSET>, Stream::<Impl, OFFSET>, SetStream::<Impl, OFFSET>, StreamContentType::<Impl, OFFSET>, SetStreamContentType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQueryOptionsImpl: Sized {
    fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn FolderDepth(&self) -> ::windows::core::Result<FolderDepth>;
    fn SetFolderDepth(&self, value: FolderDepth) -> ::windows::core::Result<()>;
    fn ApplicationSearchFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetApplicationSearchFilter(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserSearchFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserSearchFilter(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IndexerOption(&self) -> ::windows::core::Result<IndexerOption>;
    fn SetIndexerOption(&self, value: IndexerOption) -> ::windows::core::Result<()>;
    fn SortOrder(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SortEntry>>;
    fn GroupPropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateStackOption(&self) -> ::windows::core::Result<DateStackOption>;
    fn SaveToString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LoadFromString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetThumbnailPrefetch(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<()>;
    fn SetPropertyPrefetch(&self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQueryOptions {
    const NAME: &'static str = "Windows.Storage.Search.IQueryOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOptionsImpl, const OFFSET: isize>() -> IQueryOptionsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IQueryOptions>,
            ::windows::core::GetTrustLevel,
            FileTypeFilter::<Impl, OFFSET>,
            FolderDepth::<Impl, OFFSET>,
            SetFolderDepth::<Impl, OFFSET>,
            ApplicationSearchFilter::<Impl, OFFSET>,
            SetApplicationSearchFilter::<Impl, OFFSET>,
            UserSearchFilter::<Impl, OFFSET>,
            SetUserSearchFilter::<Impl, OFFSET>,
            Language::<Impl, OFFSET>,
            SetLanguage::<Impl, OFFSET>,
            IndexerOption::<Impl, OFFSET>,
            SetIndexerOption::<Impl, OFFSET>,
            SortOrder::<Impl, OFFSET>,
            GroupPropertyName::<Impl, OFFSET>,
            DateStackOption::<Impl, OFFSET>,
            SaveToString::<Impl, OFFSET>,
            LoadFromString::<Impl, OFFSET>,
            SetThumbnailPrefetch::<Impl, OFFSET>,
            SetPropertyPrefetch::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQueryOptionsFactoryImpl: Sized {
    fn CreateCommonFileQuery(&self, query: CommonFileQuery, filetypefilter: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<QueryOptions>;
    fn CreateCommonFolderQuery(&self, query: CommonFolderQuery) -> ::windows::core::Result<QueryOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQueryOptionsFactory {
    const NAME: &'static str = "Windows.Storage.Search.IQueryOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IQueryOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOptionsFactoryImpl, const OFFSET: isize>() -> IQueryOptionsFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQueryOptionsFactory>, ::windows::core::GetTrustLevel, CreateCommonFileQuery::<Impl, OFFSET>, CreateCommonFolderQuery::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQueryOptionsWithProviderFilterImpl: Sized {
    fn StorageProviderIdFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQueryOptionsWithProviderFilter {
    const NAME: &'static str = "Windows.Storage.Search.IQueryOptionsWithProviderFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IQueryOptionsWithProviderFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOptionsWithProviderFilterImpl, const OFFSET: isize>() -> IQueryOptionsWithProviderFilterVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQueryOptionsWithProviderFilter>, ::windows::core::GetTrustLevel, StorageProviderIdFilter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetFilesAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFileQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFileQueryResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFileQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileQueryResultImpl, const OFFSET: isize>() -> IStorageFileQueryResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFileQueryResult>, ::windows::core::GetTrustLevel, GetFilesAsync::<Impl, OFFSET>, GetFilesAsyncDefaultStartAndCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileQueryResult2Impl: Sized + IStorageQueryResultBaseImpl {
    fn GetMatchingPropertiesWithRanges(&self, file: &::core::option::Option<super::StorageFile>) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFileQueryResult2 {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFileQueryResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFileQueryResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileQueryResult2Impl, const OFFSET: isize>() -> IStorageFileQueryResult2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFileQueryResult2>, ::windows::core::GetTrustLevel, GetMatchingPropertiesWithRanges::<Impl, OFFSET>)
    }
}
pub trait IStorageFolderQueryOperationsImpl: Sized {
    fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&self, query: CommonFileQuery) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&self, query: CommonFolderQuery) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&self) -> ::windows::core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<bool>;
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> ::windows::core::Result<bool>;
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
impl IStorageFolderQueryOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperationsImpl, const OFFSET: isize>() -> IStorageFolderQueryOperationsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageFolderQueryOperations>,
            ::windows::core::GetTrustLevel,
            GetIndexedStateAsync::<Impl, OFFSET>,
            CreateFileQueryOverloadDefault::<Impl, OFFSET>,
            CreateFileQuery::<Impl, OFFSET>,
            CreateFileQueryWithOptions::<Impl, OFFSET>,
            CreateFolderQueryOverloadDefault::<Impl, OFFSET>,
            CreateFolderQuery::<Impl, OFFSET>,
            CreateFolderQueryWithOptions::<Impl, OFFSET>,
            CreateItemQuery::<Impl, OFFSET>,
            CreateItemQueryWithOptions::<Impl, OFFSET>,
            GetFilesAsync::<Impl, OFFSET>,
            GetFilesAsyncOverloadDefaultStartAndCount::<Impl, OFFSET>,
            GetFoldersAsync::<Impl, OFFSET>,
            GetFoldersAsyncOverloadDefaultStartAndCount::<Impl, OFFSET>,
            GetItemsAsync::<Impl, OFFSET>,
            AreQueryOptionsSupported::<Impl, OFFSET>,
            IsCommonFolderQuerySupported::<Impl, OFFSET>,
            IsCommonFileQuerySupported::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolderQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetFoldersAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFolderQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFolderQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryResultImpl, const OFFSET: isize>() -> IStorageFolderQueryResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFolderQueryResult>, ::windows::core::GetTrustLevel, GetFoldersAsync::<Impl, OFFSET>, GetFoldersAsyncDefaultStartAndCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageItemQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetItemsAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageItemQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.IStorageItemQueryResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageItemQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemQueryResultImpl, const OFFSET: isize>() -> IStorageItemQueryResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageItemQueryResult>, ::windows::core::GetTrustLevel, GetItemsAsync::<Impl, OFFSET>, GetItemsAsyncDefaultStartAndCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerTriggerDetailsImpl: Sized {
    fn Folder(&self) -> ::windows::core::Result<super::StorageFolder>;
    fn ChangeTracker(&self) -> ::windows::core::Result<super::StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTrackerTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.IStorageLibraryChangeTrackerTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTrackerTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTrackerTriggerDetailsImpl, const OFFSET: isize>() -> IStorageLibraryChangeTrackerTriggerDetailsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryChangeTrackerTriggerDetails>, ::windows::core::GetTrustLevel, Folder::<Impl, OFFSET>, ChangeTracker::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryContentChangedTriggerDetailsImpl: Sized {
    fn Folder(&self) -> ::windows::core::Result<super::StorageFolder>;
    fn CreateModifiedSinceQuery(&self, lastquerytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<StorageItemQueryResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryContentChangedTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.IStorageLibraryContentChangedTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryContentChangedTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryContentChangedTriggerDetailsImpl, const OFFSET: isize>() -> IStorageLibraryContentChangedTriggerDetailsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryContentChangedTriggerDetails>, ::windows::core::GetTrustLevel, Folder::<Impl, OFFSET>, CreateModifiedSinceQuery::<Impl, OFFSET>)
    }
}
pub trait IStorageQueryResultBaseImpl: Sized {
    fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(&self) -> ::windows::core::Result<super::StorageFolder>;
    fn ContentsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OptionsChanged(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindStartIndexAsync(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&self, newqueryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStorageQueryResultBase {
    const NAME: &'static str = "Windows.Storage.Search.IStorageQueryResultBase";
}
impl IStorageQueryResultBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBaseImpl, const OFFSET: isize>() -> IStorageQueryResultBaseVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageQueryResultBase>,
            ::windows::core::GetTrustLevel,
            GetItemCountAsync::<Impl, OFFSET>,
            Folder::<Impl, OFFSET>,
            ContentsChanged::<Impl, OFFSET>,
            RemoveContentsChanged::<Impl, OFFSET>,
            OptionsChanged::<Impl, OFFSET>,
            RemoveOptionsChanged::<Impl, OFFSET>,
            FindStartIndexAsync::<Impl, OFFSET>,
            GetCurrentQueryOptions::<Impl, OFFSET>,
            ApplyNewQueryOptions::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IValueAndLanguageImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IValueAndLanguage {
    const NAME: &'static str = "Windows.Storage.Search.IValueAndLanguage";
}
#[cfg(feature = "implement_exclusive")]
impl IValueAndLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueAndLanguageImpl, const OFFSET: isize>() -> IValueAndLanguageVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IValueAndLanguage>, ::windows::core::GetTrustLevel, Language::<Impl, OFFSET>, SetLanguage::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
