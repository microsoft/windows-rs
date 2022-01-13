#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhotoImportDeleteImportedItemsFromSourceResultImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<PhotoImportSession>;
    fn HasSucceeded(&mut self) -> ::windows::core::Result<bool>;
    fn DeletedItems(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>;
    fn PhotosCount(&mut self) -> ::windows::core::Result<u32>;
    fn PhotosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn VideosCount(&mut self) -> ::windows::core::Result<u32>;
    fn VideosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SidecarsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SidecarsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SiblingsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SiblingsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn TotalCount(&mut self) -> ::windows::core::Result<u32>;
    fn TotalSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportDeleteImportedItemsFromSourceResult {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportDeleteImportedItemsFromSourceResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhotoImportDeleteImportedItemsFromSourceResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportDeleteImportedItemsFromSourceResultVtbl {
        unsafe extern "system" fn Session<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasSucceeded<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasSucceeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletedItems<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotosCount<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotosSizeInBytes<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideosCount<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideosSizeInBytes<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidecarsCount<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidecarsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidecarsSizeInBytes<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidecarsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiblingsCount<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SiblingsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiblingsSizeInBytes<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SiblingsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCount<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSizeInBytes<Impl: IPhotoImportDeleteImportedItemsFromSourceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportDeleteImportedItemsFromSourceResult, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            HasSucceeded: HasSucceeded::<Impl, IMPL_OFFSET>,
            DeletedItems: DeletedItems::<Impl, IMPL_OFFSET>,
            PhotosCount: PhotosCount::<Impl, IMPL_OFFSET>,
            PhotosSizeInBytes: PhotosSizeInBytes::<Impl, IMPL_OFFSET>,
            VideosCount: VideosCount::<Impl, IMPL_OFFSET>,
            VideosSizeInBytes: VideosSizeInBytes::<Impl, IMPL_OFFSET>,
            SidecarsCount: SidecarsCount::<Impl, IMPL_OFFSET>,
            SidecarsSizeInBytes: SidecarsSizeInBytes::<Impl, IMPL_OFFSET>,
            SiblingsCount: SiblingsCount::<Impl, IMPL_OFFSET>,
            SiblingsSizeInBytes: SiblingsSizeInBytes::<Impl, IMPL_OFFSET>,
            TotalCount: TotalCount::<Impl, IMPL_OFFSET>,
            TotalSizeInBytes: TotalSizeInBytes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportDeleteImportedItemsFromSourceResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhotoImportFindItemsResultImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<PhotoImportSession>;
    fn HasSucceeded(&mut self) -> ::windows::core::Result<bool>;
    fn FoundItems(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>;
    fn PhotosCount(&mut self) -> ::windows::core::Result<u32>;
    fn PhotosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn VideosCount(&mut self) -> ::windows::core::Result<u32>;
    fn VideosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SidecarsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SidecarsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SiblingsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SiblingsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn TotalCount(&mut self) -> ::windows::core::Result<u32>;
    fn TotalSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SelectAll(&mut self) -> ::windows::core::Result<()>;
    fn SelectNone(&mut self) -> ::windows::core::Result<()>;
    fn SelectNewAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetImportMode(&mut self, value: PhotoImportImportMode) -> ::windows::core::Result<()>;
    fn ImportMode(&mut self) -> ::windows::core::Result<PhotoImportImportMode>;
    fn SelectedPhotosCount(&mut self) -> ::windows::core::Result<u32>;
    fn SelectedPhotosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SelectedVideosCount(&mut self) -> ::windows::core::Result<u32>;
    fn SelectedVideosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SelectedSidecarsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SelectedSidecarsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SelectedSiblingsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SelectedSiblingsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SelectedTotalCount(&mut self) -> ::windows::core::Result<u32>;
    fn SelectedTotalSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SelectionChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportSelectionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImportItemsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>;
    fn ItemImported(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportItemImportedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemImported(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportFindItemsResult {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportFindItemsResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhotoImportFindItemsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportFindItemsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportFindItemsResultVtbl {
        unsafe extern "system" fn Session<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasSucceeded<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasSucceeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FoundItems<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FoundItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotosCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotosSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideosCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideosSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidecarsCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidecarsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidecarsSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidecarsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiblingsCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SiblingsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiblingsSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SiblingsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectAll<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectAll().into()
        }
        unsafe extern "system" fn SelectNone<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectNone().into()
        }
        unsafe extern "system" fn SelectNewAsync<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectNewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImportMode<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhotoImportImportMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImportMode(value).into()
        }
        unsafe extern "system" fn ImportMode<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportImportMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPhotosCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPhotosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPhotosSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPhotosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedVideosCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedVideosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedVideosSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedVideosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedSidecarsCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedSidecarsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedSidecarsSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedSidecarsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedSiblingsCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedSiblingsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedSiblingsSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedSiblingsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedTotalCount<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedTotalCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedTotalSizeInBytes<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedTotalSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionChanged<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportSelectionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportSelectionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionChanged<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSelectionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImportItemsAsync<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemImported<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemImported(&*(&value as *const <super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportItemImportedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportItemImportedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemImported<Impl: IPhotoImportFindItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemImported(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportFindItemsResult, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            HasSucceeded: HasSucceeded::<Impl, IMPL_OFFSET>,
            FoundItems: FoundItems::<Impl, IMPL_OFFSET>,
            PhotosCount: PhotosCount::<Impl, IMPL_OFFSET>,
            PhotosSizeInBytes: PhotosSizeInBytes::<Impl, IMPL_OFFSET>,
            VideosCount: VideosCount::<Impl, IMPL_OFFSET>,
            VideosSizeInBytes: VideosSizeInBytes::<Impl, IMPL_OFFSET>,
            SidecarsCount: SidecarsCount::<Impl, IMPL_OFFSET>,
            SidecarsSizeInBytes: SidecarsSizeInBytes::<Impl, IMPL_OFFSET>,
            SiblingsCount: SiblingsCount::<Impl, IMPL_OFFSET>,
            SiblingsSizeInBytes: SiblingsSizeInBytes::<Impl, IMPL_OFFSET>,
            TotalCount: TotalCount::<Impl, IMPL_OFFSET>,
            TotalSizeInBytes: TotalSizeInBytes::<Impl, IMPL_OFFSET>,
            SelectAll: SelectAll::<Impl, IMPL_OFFSET>,
            SelectNone: SelectNone::<Impl, IMPL_OFFSET>,
            SelectNewAsync: SelectNewAsync::<Impl, IMPL_OFFSET>,
            SetImportMode: SetImportMode::<Impl, IMPL_OFFSET>,
            ImportMode: ImportMode::<Impl, IMPL_OFFSET>,
            SelectedPhotosCount: SelectedPhotosCount::<Impl, IMPL_OFFSET>,
            SelectedPhotosSizeInBytes: SelectedPhotosSizeInBytes::<Impl, IMPL_OFFSET>,
            SelectedVideosCount: SelectedVideosCount::<Impl, IMPL_OFFSET>,
            SelectedVideosSizeInBytes: SelectedVideosSizeInBytes::<Impl, IMPL_OFFSET>,
            SelectedSidecarsCount: SelectedSidecarsCount::<Impl, IMPL_OFFSET>,
            SelectedSidecarsSizeInBytes: SelectedSidecarsSizeInBytes::<Impl, IMPL_OFFSET>,
            SelectedSiblingsCount: SelectedSiblingsCount::<Impl, IMPL_OFFSET>,
            SelectedSiblingsSizeInBytes: SelectedSiblingsSizeInBytes::<Impl, IMPL_OFFSET>,
            SelectedTotalCount: SelectedTotalCount::<Impl, IMPL_OFFSET>,
            SelectedTotalSizeInBytes: SelectedTotalSizeInBytes::<Impl, IMPL_OFFSET>,
            SelectionChanged: SelectionChanged::<Impl, IMPL_OFFSET>,
            RemoveSelectionChanged: RemoveSelectionChanged::<Impl, IMPL_OFFSET>,
            ImportItemsAsync: ImportItemsAsync::<Impl, IMPL_OFFSET>,
            ItemImported: ItemImported::<Impl, IMPL_OFFSET>,
            RemoveItemImported: RemoveItemImported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportFindItemsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhotoImportFindItemsResult2Impl: Sized {
    fn AddItemsInDateRangeToSelection(&mut self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportFindItemsResult2 {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportFindItemsResult2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhotoImportFindItemsResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportFindItemsResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportFindItemsResult2Vtbl {
        unsafe extern "system" fn AddItemsInDateRangeToSelection<Impl: IPhotoImportFindItemsResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItemsInDateRangeToSelection(&*(&rangestart as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&rangelength as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportFindItemsResult2, BASE_OFFSET>(),
            AddItemsInDateRangeToSelection: AddItemsInDateRangeToSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportFindItemsResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhotoImportImportItemsResultImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<PhotoImportSession>;
    fn HasSucceeded(&mut self) -> ::windows::core::Result<bool>;
    fn ImportedItems(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>;
    fn PhotosCount(&mut self) -> ::windows::core::Result<u32>;
    fn PhotosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn VideosCount(&mut self) -> ::windows::core::Result<u32>;
    fn VideosSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SidecarsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SidecarsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn SiblingsCount(&mut self) -> ::windows::core::Result<u32>;
    fn SiblingsSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn TotalCount(&mut self) -> ::windows::core::Result<u32>;
    fn TotalSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn DeleteImportedItemsFromSourceAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportImportItemsResult {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportImportItemsResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhotoImportImportItemsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportImportItemsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportImportItemsResultVtbl {
        unsafe extern "system" fn Session<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasSucceeded<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasSucceeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportedItems<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotosCount<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotosSizeInBytes<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideosCount<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideosCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideosSizeInBytes<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideosSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidecarsCount<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidecarsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidecarsSizeInBytes<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidecarsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiblingsCount<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SiblingsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiblingsSizeInBytes<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SiblingsSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCount<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSizeInBytes<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteImportedItemsFromSourceAsync<Impl: IPhotoImportImportItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteImportedItemsFromSourceAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportImportItemsResult, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            HasSucceeded: HasSucceeded::<Impl, IMPL_OFFSET>,
            ImportedItems: ImportedItems::<Impl, IMPL_OFFSET>,
            PhotosCount: PhotosCount::<Impl, IMPL_OFFSET>,
            PhotosSizeInBytes: PhotosSizeInBytes::<Impl, IMPL_OFFSET>,
            VideosCount: VideosCount::<Impl, IMPL_OFFSET>,
            VideosSizeInBytes: VideosSizeInBytes::<Impl, IMPL_OFFSET>,
            SidecarsCount: SidecarsCount::<Impl, IMPL_OFFSET>,
            SidecarsSizeInBytes: SidecarsSizeInBytes::<Impl, IMPL_OFFSET>,
            SiblingsCount: SiblingsCount::<Impl, IMPL_OFFSET>,
            SiblingsSizeInBytes: SiblingsSizeInBytes::<Impl, IMPL_OFFSET>,
            TotalCount: TotalCount::<Impl, IMPL_OFFSET>,
            TotalSizeInBytes: TotalSizeInBytes::<Impl, IMPL_OFFSET>,
            DeleteImportedItemsFromSourceAsync: DeleteImportedItemsFromSourceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportImportItemsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPhotoImportItemImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ItemKey(&mut self) -> ::windows::core::Result<u64>;
    fn ContentType(&mut self) -> ::windows::core::Result<PhotoImportContentType>;
    fn SizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn Date(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Sibling(&mut self) -> ::windows::core::Result<PhotoImportSidecar>;
    fn Sidecars(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>;
    fn VideoSegments(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportVideoSegment>>;
    fn IsSelected(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ImportedFileNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DeletedFileNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportItem {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPhotoImportItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportItemVtbl {
        unsafe extern "system" fn Name<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemKey<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentType<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportContentType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInBytes<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sibling<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sibling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sidecars<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sidecars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoSegments<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoSegments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelected<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSelected<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSelected(value).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportedFileNames<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportedFileNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletedFileNames<Impl: IPhotoImportItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletedFileNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportItem, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            ItemKey: ItemKey::<Impl, IMPL_OFFSET>,
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            SizeInBytes: SizeInBytes::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
            Sibling: Sibling::<Impl, IMPL_OFFSET>,
            Sidecars: Sidecars::<Impl, IMPL_OFFSET>,
            VideoSegments: VideoSegments::<Impl, IMPL_OFFSET>,
            IsSelected: IsSelected::<Impl, IMPL_OFFSET>,
            SetIsSelected: SetIsSelected::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            ImportedFileNames: ImportedFileNames::<Impl, IMPL_OFFSET>,
            DeletedFileNames: DeletedFileNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportItem2Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoImportItem2 {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportItem2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoImportItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportItem2Vtbl {
        unsafe extern "system" fn Path<Impl: IPhotoImportItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportItem2, BASE_OFFSET>(), Path: Path::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportItemImportedEventArgsImpl: Sized {
    fn ImportedItem(&mut self) -> ::windows::core::Result<PhotoImportItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoImportItemImportedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportItemImportedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoImportItemImportedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportItemImportedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportItemImportedEventArgsVtbl {
        unsafe extern "system" fn ImportedItem<Impl: IPhotoImportItemImportedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportItemImportedEventArgs, BASE_OFFSET>(),
            ImportedItem: ImportedItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportItemImportedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhotoImportManagerStaticsImpl: Sized {
    fn IsSupportedAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn FindAllSourcesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhotoImportSource>>>;
    fn GetPendingOperations(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportOperation>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportManagerStatics {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhotoImportManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportManagerStaticsVtbl {
        unsafe extern "system" fn IsSupportedAsync<Impl: IPhotoImportManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllSourcesAsync<Impl: IPhotoImportManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllSourcesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPendingOperations<Impl: IPhotoImportManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPendingOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportManagerStatics, BASE_OFFSET>(),
            IsSupportedAsync: IsSupportedAsync::<Impl, IMPL_OFFSET>,
            FindAllSourcesAsync: FindAllSourcesAsync::<Impl, IMPL_OFFSET>,
            GetPendingOperations: GetPendingOperations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhotoImportOperationImpl: Sized {
    fn Stage(&mut self) -> ::windows::core::Result<PhotoImportStage>;
    fn Session(&mut self) -> ::windows::core::Result<PhotoImportSession>;
    fn ContinueFindingItemsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>;
    fn ContinueImportingItemsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>;
    fn ContinueDeletingImportedItemsFromSourceAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportOperation {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportOperation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhotoImportOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportOperationVtbl {
        unsafe extern "system" fn Stage<Impl: IPhotoImportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportStage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IPhotoImportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContinueFindingItemsAsync<Impl: IPhotoImportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContinueFindingItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContinueImportingItemsAsync<Impl: IPhotoImportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContinueImportingItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContinueDeletingImportedItemsFromSourceAsync<Impl: IPhotoImportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContinueDeletingImportedItemsFromSourceAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportOperation, BASE_OFFSET>(),
            Stage: Stage::<Impl, IMPL_OFFSET>,
            Session: Session::<Impl, IMPL_OFFSET>,
            ContinueFindingItemsAsync: ContinueFindingItemsAsync::<Impl, IMPL_OFFSET>,
            ContinueImportingItemsAsync: ContinueImportingItemsAsync::<Impl, IMPL_OFFSET>,
            ContinueDeletingImportedItemsFromSourceAsync: ContinueDeletingImportedItemsFromSourceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportSelectionChangedEventArgsImpl: Sized {
    fn IsSelectionEmpty(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoImportSelectionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportSelectionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoImportSelectionChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportSelectionChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportSelectionChangedEventArgsVtbl {
        unsafe extern "system" fn IsSelectionEmpty<Impl: IPhotoImportSelectionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectionEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportSelectionChangedEventArgs, BASE_OFFSET>(),
            IsSelectionEmpty: IsSelectionEmpty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportSelectionChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IPhotoImportSessionImpl: Sized + IClosableImpl {
    fn Source(&mut self) -> ::windows::core::Result<PhotoImportSource>;
    fn SessionId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDestinationFolder(&mut self, value: &::core::option::Option<super::super::Storage::IStorageFolder>) -> ::windows::core::Result<()>;
    fn DestinationFolder(&mut self) -> ::windows::core::Result<super::super::Storage::IStorageFolder>;
    fn SetAppendSessionDateToDestinationFolder(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AppendSessionDateToDestinationFolder(&mut self) -> ::windows::core::Result<bool>;
    fn SetSubfolderCreationMode(&mut self, value: PhotoImportSubfolderCreationMode) -> ::windows::core::Result<()>;
    fn SubfolderCreationMode(&mut self) -> ::windows::core::Result<PhotoImportSubfolderCreationMode>;
    fn SetDestinationFileNamePrefix(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DestinationFileNamePrefix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FindItemsAsync(&mut self, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportSession {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportSession";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IPhotoImportSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportSessionVtbl {
        unsafe extern "system" fn Source<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionId<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationFolder<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationFolder(&*(&value as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DestinationFolder<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppendSessionDateToDestinationFolder<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppendSessionDateToDestinationFolder(value).into()
        }
        unsafe extern "system" fn AppendSessionDateToDestinationFolder<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendSessionDateToDestinationFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubfolderCreationMode<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhotoImportSubfolderCreationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubfolderCreationMode(value).into()
        }
        unsafe extern "system" fn SubfolderCreationMode<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSubfolderCreationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubfolderCreationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationFileNamePrefix<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationFileNamePrefix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DestinationFileNamePrefix<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationFileNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemsAsync<Impl: IPhotoImportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemsAsync(contenttypefilter, itemselectionmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportSession, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            SetDestinationFolder: SetDestinationFolder::<Impl, IMPL_OFFSET>,
            DestinationFolder: DestinationFolder::<Impl, IMPL_OFFSET>,
            SetAppendSessionDateToDestinationFolder: SetAppendSessionDateToDestinationFolder::<Impl, IMPL_OFFSET>,
            AppendSessionDateToDestinationFolder: AppendSessionDateToDestinationFolder::<Impl, IMPL_OFFSET>,
            SetSubfolderCreationMode: SetSubfolderCreationMode::<Impl, IMPL_OFFSET>,
            SubfolderCreationMode: SubfolderCreationMode::<Impl, IMPL_OFFSET>,
            SetDestinationFileNamePrefix: SetDestinationFileNamePrefix::<Impl, IMPL_OFFSET>,
            DestinationFileNamePrefix: DestinationFileNamePrefix::<Impl, IMPL_OFFSET>,
            FindItemsAsync: FindItemsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportSession2Impl: Sized {
    fn SetSubfolderDateFormat(&mut self, value: PhotoImportSubfolderDateFormat) -> ::windows::core::Result<()>;
    fn SubfolderDateFormat(&mut self) -> ::windows::core::Result<PhotoImportSubfolderDateFormat>;
    fn SetRememberDeselectedItems(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RememberDeselectedItems(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoImportSession2 {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportSession2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoImportSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportSession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportSession2Vtbl {
        unsafe extern "system" fn SetSubfolderDateFormat<Impl: IPhotoImportSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhotoImportSubfolderDateFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubfolderDateFormat(value).into()
        }
        unsafe extern "system" fn SubfolderDateFormat<Impl: IPhotoImportSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSubfolderDateFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubfolderDateFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRememberDeselectedItems<Impl: IPhotoImportSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRememberDeselectedItems(value).into()
        }
        unsafe extern "system" fn RememberDeselectedItems<Impl: IPhotoImportSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RememberDeselectedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportSession2, BASE_OFFSET>(),
            SetSubfolderDateFormat: SetSubfolderDateFormat::<Impl, IMPL_OFFSET>,
            SubfolderDateFormat: SubfolderDateFormat::<Impl, IMPL_OFFSET>,
            SetRememberDeselectedItems: SetRememberDeselectedItems::<Impl, IMPL_OFFSET>,
            RememberDeselectedItems: RememberDeselectedItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhotoImportSidecarImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn Date(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportSidecar {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportSidecar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhotoImportSidecarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportSidecarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportSidecarVtbl {
        unsafe extern "system" fn Name<Impl: IPhotoImportSidecarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInBytes<Impl: IPhotoImportSidecarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IPhotoImportSidecarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportSidecar, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SizeInBytes: SizeInBytes::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportSidecar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPhotoImportSourceImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Manufacturer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Model(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SerialNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionProtocol(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionTransport(&mut self) -> ::windows::core::Result<PhotoImportConnectionTransport>;
    fn Type(&mut self) -> ::windows::core::Result<PhotoImportSourceType>;
    fn PowerSource(&mut self) -> ::windows::core::Result<PhotoImportPowerSource>;
    fn BatteryLevelPercent(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn DateTime(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn StorageMedia(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportStorageMedium>>;
    fn IsLocked(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn IsMassStorage(&mut self) -> ::windows::core::Result<bool>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn CreateImportSession(&mut self) -> ::windows::core::Result<PhotoImportSession>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportSource {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPhotoImportSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportSourceVtbl {
        unsafe extern "system" fn Id<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Model<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionProtocol<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionTransport<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportConnectionTransport) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionTransport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSourceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSource<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportPowerSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatteryLevelPercent<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatteryLevelPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageMedia<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocked<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMassStorage<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMassStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImportSession<Impl: IPhotoImportSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImportSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportSource, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Manufacturer: Manufacturer::<Impl, IMPL_OFFSET>,
            Model: Model::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            ConnectionProtocol: ConnectionProtocol::<Impl, IMPL_OFFSET>,
            ConnectionTransport: ConnectionTransport::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            PowerSource: PowerSource::<Impl, IMPL_OFFSET>,
            BatteryLevelPercent: BatteryLevelPercent::<Impl, IMPL_OFFSET>,
            DateTime: DateTime::<Impl, IMPL_OFFSET>,
            StorageMedia: StorageMedia::<Impl, IMPL_OFFSET>,
            IsLocked: IsLocked::<Impl, IMPL_OFFSET>,
            IsMassStorage: IsMassStorage::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            CreateImportSession: CreateImportSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IPhotoImportSourceStaticsImpl: Sized {
    fn FromIdAsync(&mut self, sourceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>>;
    fn FromFolderAsync(&mut self, sourcerootfolder: &::core::option::Option<super::super::Storage::IStorageFolder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportSourceStatics {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IPhotoImportSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportSourceStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IPhotoImportSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&sourceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromFolderAsync<Impl: IPhotoImportSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcerootfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromFolderAsync(&*(&sourcerootfolder as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportSourceStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            FromFolderAsync: FromFolderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportStorageMediumImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SerialNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageMediumType(&mut self) -> ::windows::core::Result<PhotoImportStorageMediumType>;
    fn SupportedAccessMode(&mut self) -> ::windows::core::Result<PhotoImportAccessMode>;
    fn CapacityInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn AvailableSpaceInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoImportStorageMedium {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportStorageMedium";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoImportStorageMediumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportStorageMediumImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportStorageMediumVtbl {
        unsafe extern "system" fn Name<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageMediumType<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportStorageMediumType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageMediumType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedAccessMode<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportAccessMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedAccessMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapacityInBytes<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapacityInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableSpaceInBytes<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableSpaceInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IPhotoImportStorageMediumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportStorageMedium, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            StorageMediumType: StorageMediumType::<Impl, IMPL_OFFSET>,
            SupportedAccessMode: SupportedAccessMode::<Impl, IMPL_OFFSET>,
            CapacityInBytes: CapacityInBytes::<Impl, IMPL_OFFSET>,
            AvailableSpaceInBytes: AvailableSpaceInBytes::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportStorageMedium as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhotoImportVideoSegmentImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn Date(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Sibling(&mut self) -> ::windows::core::Result<PhotoImportSidecar>;
    fn Sidecars(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoImportVideoSegment {
    const NAME: &'static str = "Windows.Media.Import.IPhotoImportVideoSegment";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhotoImportVideoSegmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoImportVideoSegmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoImportVideoSegmentVtbl {
        unsafe extern "system" fn Name<Impl: IPhotoImportVideoSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInBytes<Impl: IPhotoImportVideoSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IPhotoImportVideoSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sibling<Impl: IPhotoImportVideoSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sibling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sidecars<Impl: IPhotoImportVideoSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sidecars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoImportVideoSegment, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SizeInBytes: SizeInBytes::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
            Sibling: Sibling::<Impl, IMPL_OFFSET>,
            Sidecars: Sidecars::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoImportVideoSegment as ::windows::core::Interface>::IID
    }
}
