#[cfg(feature = "implement_exclusive")]
pub trait IQuickLinkImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportedDataFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SupportedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuickLink {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.IQuickLink";
}
#[cfg(feature = "implement_exclusive")]
impl IQuickLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuickLinkImpl, const OFFSET: isize>() -> IQuickLinkVtbl {
        unsafe extern "system" fn Title<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedDataFormats<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedDataFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFileTypes<Impl: IQuickLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQuickLink>, ::windows::core::GetTrustLevel, Title::<Impl, OFFSET>, SetTitle::<Impl, OFFSET>, Thumbnail::<Impl, OFFSET>, SetThumbnail::<Impl, OFFSET>, Id::<Impl, OFFSET>, SetId::<Impl, OFFSET>, SupportedDataFormats::<Impl, OFFSET>, SupportedFileTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareOperationImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::DataPackageView>;
    fn QuickLinkId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoveThisQuickLink(&self) -> ::windows::core::Result<()>;
    fn ReportStarted(&self) -> ::windows::core::Result<()>;
    fn ReportDataRetrieved(&self) -> ::windows::core::Result<()>;
    fn ReportSubmittedBackgroundTask(&self) -> ::windows::core::Result<()>;
    fn ReportCompletedWithQuickLink(&self, quicklink: &::core::option::Option<QuickLink>) -> ::windows::core::Result<()>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.IShareOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IShareOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareOperationImpl, const OFFSET: isize>() -> IShareOperationVtbl {
        unsafe extern "system" fn Data<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuickLinkId<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuickLinkId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveThisQuickLink<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveThisQuickLink().into()
        }
        unsafe extern "system" fn ReportStarted<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportStarted().into()
        }
        unsafe extern "system" fn ReportDataRetrieved<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportDataRetrieved().into()
        }
        unsafe extern "system" fn ReportSubmittedBackgroundTask<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportSubmittedBackgroundTask().into()
        }
        unsafe extern "system" fn ReportCompletedWithQuickLink<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quicklink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompletedWithQuickLink(&*(&quicklink as *const <QuickLink as ::windows::core::Abi>::Abi as *const <QuickLink as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReportCompleted<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        unsafe extern "system" fn ReportError<Impl: IShareOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IShareOperation>,
            ::windows::core::GetTrustLevel,
            Data::<Impl, OFFSET>,
            QuickLinkId::<Impl, OFFSET>,
            RemoveThisQuickLink::<Impl, OFFSET>,
            ReportStarted::<Impl, OFFSET>,
            ReportDataRetrieved::<Impl, OFFSET>,
            ReportSubmittedBackgroundTask::<Impl, OFFSET>,
            ReportCompletedWithQuickLink::<Impl, OFFSET>,
            ReportCompleted::<Impl, OFFSET>,
            ReportError::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareOperation2Impl: Sized {
    fn DismissUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareOperation2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.IShareOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl IShareOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareOperation2Impl, const OFFSET: isize>() -> IShareOperation2Vtbl {
        unsafe extern "system" fn DismissUI<Impl: IShareOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissUI().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareOperation2>, ::windows::core::GetTrustLevel, DismissUI::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareOperation3Impl: Sized {
    fn Contacts(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::Contacts::Contact>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareOperation3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTarget.IShareOperation3";
}
#[cfg(feature = "implement_exclusive")]
impl IShareOperation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareOperation3Impl, const OFFSET: isize>() -> IShareOperation3Vtbl {
        unsafe extern "system" fn Contacts<Impl: IShareOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contacts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareOperation3>, ::windows::core::GetTrustLevel, Contacts::<Impl, OFFSET>)
    }
}
