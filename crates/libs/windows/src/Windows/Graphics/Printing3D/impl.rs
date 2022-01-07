#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DManagerImpl: Sized {
    fn TaskRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTaskRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DManager {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DManager";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DManagerImpl, const OFFSET: isize>() -> IPrint3DManagerVtbl {
        unsafe extern "system" fn TaskRequested<Impl: IPrint3DManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskRequested(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTaskRequested<Impl: IPrint3DManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTaskRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DManager>, ::windows::core::GetTrustLevel, TaskRequested::<Impl, OFFSET>, RemoveTaskRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<Print3DManager>;
    fn ShowPrintUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DManagerStatics {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DManagerStaticsImpl, const OFFSET: isize>() -> IPrint3DManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IPrint3DManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPrintUIAsync<Impl: IPrint3DManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowPrintUIAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DManagerStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, OFFSET>, ShowPrintUIAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<Printing3D3MFPackage>;
    fn Submitting(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitting(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChanged(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTask {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTask";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskImpl, const OFFSET: isize>() -> IPrint3DTaskVtbl {
        unsafe extern "system" fn Source<Impl: IPrint3DTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Submitting<Impl: IPrint3DTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submitting(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<Print3DTask, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Print3DTask, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubmitting<Impl: IPrint3DTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubmitting(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IPrint3DTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IPrint3DTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceChanged<Impl: IPrint3DTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceChanged(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceChanged<Impl: IPrint3DTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DTask>, ::windows::core::GetTrustLevel, Source::<Impl, OFFSET>, Submitting::<Impl, OFFSET>, RemoveSubmitting::<Impl, OFFSET>, Completed::<Impl, OFFSET>, RemoveCompleted::<Impl, OFFSET>, SourceChanged::<Impl, OFFSET>, RemoveSourceChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskCompletedEventArgsImpl: Sized {
    fn Completion(&self) -> ::windows::core::Result<Print3DTaskCompletion>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<Print3DTaskDetail>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskCompletedEventArgsImpl, const OFFSET: isize>() -> IPrint3DTaskCompletedEventArgsVtbl {
        unsafe extern "system" fn Completion<Impl: IPrint3DTaskCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Print3DTaskCompletion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IPrint3DTaskCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Print3DTaskDetail) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DTaskCompletedEventArgs>, ::windows::core::GetTrustLevel, Completion::<Impl, OFFSET>, ExtendedStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskRequestImpl: Sized {
    fn CreateTask(&self, title: &::windows::core::HSTRING, printerid: &::windows::core::HSTRING, handler: &::core::option::Option<Print3DTaskSourceRequestedHandler>) -> ::windows::core::Result<Print3DTask>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskRequestImpl, const OFFSET: isize>() -> IPrint3DTaskRequestVtbl {
        unsafe extern "system" fn CreateTask<Impl: IPrint3DTaskRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTask(
                &*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&printerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <Print3DTaskSourceRequestedHandler as ::windows::core::Abi>::Abi as *const <Print3DTaskSourceRequestedHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DTaskRequest>, ::windows::core::GetTrustLevel, CreateTask::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<Print3DTaskRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskRequestedEventArgsImpl, const OFFSET: isize>() -> IPrint3DTaskRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IPrint3DTaskRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DTaskRequestedEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskSourceChangedEventArgsImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<Printing3D3MFPackage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskSourceChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskSourceChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskSourceChangedEventArgsImpl, const OFFSET: isize>() -> IPrint3DTaskSourceChangedEventArgsVtbl {
        unsafe extern "system" fn Source<Impl: IPrint3DTaskSourceChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DTaskSourceChangedEventArgs>, ::windows::core::GetTrustLevel, Source::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskSourceRequestedArgsImpl: Sized {
    fn SetSource(&self, source: &::core::option::Option<Printing3D3MFPackage>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskSourceRequestedArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskSourceRequestedArgsImpl, const OFFSET: isize>() -> IPrint3DTaskSourceRequestedArgsVtbl {
        unsafe extern "system" fn SetSource<Impl: IPrint3DTaskSourceRequestedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&source as *const <Printing3D3MFPackage as ::windows::core::Abi>::Abi as *const <Printing3D3MFPackage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DTaskSourceRequestedArgs>, ::windows::core::GetTrustLevel, SetSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3D3MFPackageImpl: Sized {
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn PrintTicket(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetPrintTicket(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn ModelPart(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetModelPart(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetThumbnail(&self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn Textures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTextureResource>>;
    fn LoadModelFromPackageAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DModel>>;
    fn SaveModelToPackageAsync(&self, value: &::core::option::Option<Printing3DModel>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3D3MFPackage {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3D3MFPackage";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3D3MFPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>() -> IPrinting3D3MFPackageVtbl {
        unsafe extern "system" fn SaveAsync<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintTicket<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintTicket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicket<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicket(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ModelPart<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelPart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelPart<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelPart(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <Printing3DTextureResource as ::windows::core::Abi>::Abi as *const <Printing3DTextureResource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Textures<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Textures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadModelFromPackageAsync<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadModelFromPackageAsync(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveModelToPackageAsync<Impl: IPrinting3D3MFPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveModelToPackageAsync(&*(&value as *const <Printing3DModel as ::windows::core::Abi>::Abi as *const <Printing3DModel as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IPrinting3D3MFPackage>,
            ::windows::core::GetTrustLevel,
            SaveAsync::<Impl, OFFSET>,
            PrintTicket::<Impl, OFFSET>,
            SetPrintTicket::<Impl, OFFSET>,
            ModelPart::<Impl, OFFSET>,
            SetModelPart::<Impl, OFFSET>,
            Thumbnail::<Impl, OFFSET>,
            SetThumbnail::<Impl, OFFSET>,
            Textures::<Impl, OFFSET>,
            LoadModelFromPackageAsync::<Impl, OFFSET>,
            SaveModelToPackageAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3D3MFPackage2Impl: Sized {
    fn Compression(&self) -> ::windows::core::Result<Printing3DPackageCompression>;
    fn SetCompression(&self, value: Printing3DPackageCompression) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3D3MFPackage2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3D3MFPackage2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3D3MFPackage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3D3MFPackage2Impl, const OFFSET: isize>() -> IPrinting3D3MFPackage2Vtbl {
        unsafe extern "system" fn Compression<Impl: IPrinting3D3MFPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DPackageCompression) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compression() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompression<Impl: IPrinting3D3MFPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DPackageCompression) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompression(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3D3MFPackage2>, ::windows::core::GetTrustLevel, Compression::<Impl, OFFSET>, SetCompression::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3D3MFPackageStaticsImpl: Sized {
    fn LoadAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3D3MFPackageStatics {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3D3MFPackageStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3D3MFPackageStaticsImpl, const OFFSET: isize>() -> IPrinting3D3MFPackageStaticsVtbl {
        unsafe extern "system" fn LoadAsync<Impl: IPrinting3D3MFPackageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadAsync(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3D3MFPackageStatics>, ::windows::core::GetTrustLevel, LoadAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Color(&self) -> ::windows::core::Result<Printing3DColorMaterial>;
    fn SetColor(&self, value: &::core::option::Option<Printing3DColorMaterial>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DBaseMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialImpl, const OFFSET: isize>() -> IPrinting3DBaseMaterialVtbl {
        unsafe extern "system" fn Name<Impl: IPrinting3DBaseMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IPrinting3DBaseMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Color<Impl: IPrinting3DBaseMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IPrinting3DBaseMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <Printing3DColorMaterial as ::windows::core::Abi>::Abi as *const <Printing3DColorMaterial as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DBaseMaterial>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>, Color::<Impl, OFFSET>, SetColor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialGroupImpl: Sized {
    fn Bases(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DBaseMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialGroupImpl, const OFFSET: isize>() -> IPrinting3DBaseMaterialGroupVtbl {
        unsafe extern "system" fn Bases<Impl: IPrinting3DBaseMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bases() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialGroupId<Impl: IPrinting3DBaseMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialGroupId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DBaseMaterialGroup>, ::windows::core::GetTrustLevel, Bases::<Impl, OFFSET>, MaterialGroupId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DBaseMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DBaseMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialGroupFactoryImpl, const OFFSET: isize>() -> IPrinting3DBaseMaterialGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPrinting3DBaseMaterialGroupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(materialgroupid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DBaseMaterialGroupFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialStaticsImpl: Sized {
    fn Abs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pla(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterialStatics {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DBaseMaterialStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialStaticsImpl, const OFFSET: isize>() -> IPrinting3DBaseMaterialStaticsVtbl {
        unsafe extern "system" fn Abs<Impl: IPrinting3DBaseMaterialStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pla<Impl: IPrinting3DBaseMaterialStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pla() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DBaseMaterialStatics>, ::windows::core::GetTrustLevel, Abs::<Impl, OFFSET>, Pla::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValue(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DColorMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterialImpl, const OFFSET: isize>() -> IPrinting3DColorMaterialVtbl {
        unsafe extern "system" fn Value<Impl: IPrinting3DColorMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IPrinting3DColorMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DColorMaterial>, ::windows::core::GetTrustLevel, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterial2Impl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterial2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterial2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DColorMaterial2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterial2Impl, const OFFSET: isize>() -> IPrinting3DColorMaterial2Vtbl {
        unsafe extern "system" fn Color<Impl: IPrinting3DColorMaterial2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IPrinting3DColorMaterial2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DColorMaterial2>, ::windows::core::GetTrustLevel, Color::<Impl, OFFSET>, SetColor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialGroupImpl: Sized {
    fn Colors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DColorMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterialGroupImpl, const OFFSET: isize>() -> IPrinting3DColorMaterialGroupVtbl {
        unsafe extern "system" fn Colors<Impl: IPrinting3DColorMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Colors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialGroupId<Impl: IPrinting3DColorMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialGroupId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DColorMaterialGroup>, ::windows::core::GetTrustLevel, Colors::<Impl, OFFSET>, MaterialGroupId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DColorMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DColorMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterialGroupFactoryImpl, const OFFSET: isize>() -> IPrinting3DColorMaterialGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPrinting3DColorMaterialGroupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(materialgroupid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DColorMaterialGroupFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DComponentImpl: Sized {
    fn Mesh(&self) -> ::windows::core::Result<Printing3DMesh>;
    fn SetMesh(&self, value: &::core::option::Option<Printing3DMesh>) -> ::windows::core::Result<()>;
    fn Components(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>>;
    fn Thumbnail(&self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetThumbnail(&self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<Printing3DObjectType>;
    fn SetType(&self, value: Printing3DObjectType) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PartNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPartNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DComponent {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DComponent";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DComponentImpl, const OFFSET: isize>() -> IPrinting3DComponentVtbl {
        unsafe extern "system" fn Mesh<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mesh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMesh<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMesh(&*(&value as *const <Printing3DMesh as ::windows::core::Abi>::Abi as *const <Printing3DMesh as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Components<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <Printing3DTextureResource as ::windows::core::Abi>::Abi as *const <Printing3DTextureResource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Type<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DObjectType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DObjectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Name<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PartNumber<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartNumber<Impl: IPrinting3DComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPartNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrinting3DComponent>,
            ::windows::core::GetTrustLevel,
            Mesh::<Impl, OFFSET>,
            SetMesh::<Impl, OFFSET>,
            Components::<Impl, OFFSET>,
            Thumbnail::<Impl, OFFSET>,
            SetThumbnail::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            SetType::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            PartNumber::<Impl, OFFSET>,
            SetPartNumber::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DComponentWithMatrixImpl: Sized {
    fn Component(&self) -> ::windows::core::Result<Printing3DComponent>;
    fn SetComponent(&self, value: &::core::option::Option<Printing3DComponent>) -> ::windows::core::Result<()>;
    fn Matrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetMatrix(&self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DComponentWithMatrix {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DComponentWithMatrixVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DComponentWithMatrixImpl, const OFFSET: isize>() -> IPrinting3DComponentWithMatrixVtbl {
        unsafe extern "system" fn Component<Impl: IPrinting3DComponentWithMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Component() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponent<Impl: IPrinting3DComponentWithMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComponent(&*(&value as *const <Printing3DComponent as ::windows::core::Abi>::Abi as *const <Printing3DComponent as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Matrix<Impl: IPrinting3DComponentWithMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Matrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Impl: IPrinting3DComponentWithMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DComponentWithMatrix>, ::windows::core::GetTrustLevel, Component::<Impl, OFFSET>, SetComponent::<Impl, OFFSET>, Matrix::<Impl, OFFSET>, SetMatrix::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialImpl: Sized {
    fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DCompositeMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialImpl, const OFFSET: isize>() -> IPrinting3DCompositeMaterialVtbl {
        unsafe extern "system" fn Values<Impl: IPrinting3DCompositeMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DCompositeMaterial>, ::windows::core::GetTrustLevel, Values::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroupImpl: Sized {
    fn Composites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
    fn MaterialIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DCompositeMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialGroupImpl, const OFFSET: isize>() -> IPrinting3DCompositeMaterialGroupVtbl {
        unsafe extern "system" fn Composites<Impl: IPrinting3DCompositeMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Composites() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialGroupId<Impl: IPrinting3DCompositeMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialGroupId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialIndices<Impl: IPrinting3DCompositeMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialIndices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DCompositeMaterialGroup>, ::windows::core::GetTrustLevel, Composites::<Impl, OFFSET>, MaterialGroupId::<Impl, OFFSET>, MaterialIndices::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroup2Impl: Sized {
    fn BaseMaterialGroup(&self) -> ::windows::core::Result<Printing3DBaseMaterialGroup>;
    fn SetBaseMaterialGroup(&self, value: &::core::option::Option<Printing3DBaseMaterialGroup>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterialGroup2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DCompositeMaterialGroup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialGroup2Impl, const OFFSET: isize>() -> IPrinting3DCompositeMaterialGroup2Vtbl {
        unsafe extern "system" fn BaseMaterialGroup<Impl: IPrinting3DCompositeMaterialGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseMaterialGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseMaterialGroup<Impl: IPrinting3DCompositeMaterialGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseMaterialGroup(&*(&value as *const <Printing3DBaseMaterialGroup as ::windows::core::Abi>::Abi as *const <Printing3DBaseMaterialGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DCompositeMaterialGroup2>, ::windows::core::GetTrustLevel, BaseMaterialGroup::<Impl, OFFSET>, SetBaseMaterialGroup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DCompositeMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DCompositeMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialGroupFactoryImpl, const OFFSET: isize>() -> IPrinting3DCompositeMaterialGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPrinting3DCompositeMaterialGroupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(materialgroupid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DCompositeMaterialGroupFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DFaceReductionOptionsImpl: Sized {
    fn MaxReductionArea(&self) -> ::windows::core::Result<f64>;
    fn SetMaxReductionArea(&self, value: f64) -> ::windows::core::Result<()>;
    fn TargetTriangleCount(&self) -> ::windows::core::Result<u32>;
    fn SetTargetTriangleCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxEdgeLength(&self) -> ::windows::core::Result<f64>;
    fn SetMaxEdgeLength(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DFaceReductionOptions {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DFaceReductionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DFaceReductionOptionsImpl, const OFFSET: isize>() -> IPrinting3DFaceReductionOptionsVtbl {
        unsafe extern "system" fn MaxReductionArea<Impl: IPrinting3DFaceReductionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxReductionArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxReductionArea<Impl: IPrinting3DFaceReductionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxReductionArea(value).into()
        }
        unsafe extern "system" fn TargetTriangleCount<Impl: IPrinting3DFaceReductionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetTriangleCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetTriangleCount<Impl: IPrinting3DFaceReductionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetTriangleCount(value).into()
        }
        unsafe extern "system" fn MaxEdgeLength<Impl: IPrinting3DFaceReductionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxEdgeLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxEdgeLength<Impl: IPrinting3DFaceReductionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxEdgeLength(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrinting3DFaceReductionOptions>,
            ::windows::core::GetTrustLevel,
            MaxReductionArea::<Impl, OFFSET>,
            SetMaxReductionArea::<Impl, OFFSET>,
            TargetTriangleCount::<Impl, OFFSET>,
            SetTargetTriangleCount::<Impl, OFFSET>,
            MaxEdgeLength::<Impl, OFFSET>,
            SetMaxEdgeLength::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMaterialImpl: Sized {
    fn BaseGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>>;
    fn ColorGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>>;
    fn Texture2CoordGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>>;
    fn CompositeGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>>;
    fn MultiplePropertyGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMaterialImpl, const OFFSET: isize>() -> IPrinting3DMaterialVtbl {
        unsafe extern "system" fn BaseGroups<Impl: IPrinting3DMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorGroups<Impl: IPrinting3DMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Texture2CoordGroups<Impl: IPrinting3DMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Texture2CoordGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompositeGroups<Impl: IPrinting3DMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositeGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultiplePropertyGroups<Impl: IPrinting3DMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultiplePropertyGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DMaterial>, ::windows::core::GetTrustLevel, BaseGroups::<Impl, OFFSET>, ColorGroups::<Impl, OFFSET>, Texture2CoordGroups::<Impl, OFFSET>, CompositeGroups::<Impl, OFFSET>, MultiplePropertyGroups::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMeshImpl: Sized {
    fn VertexCount(&self) -> ::windows::core::Result<u32>;
    fn SetVertexCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn IndexCount(&self) -> ::windows::core::Result<u32>;
    fn SetIndexCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn VertexPositionsDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetVertexPositionsDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn VertexNormalsDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetVertexNormalsDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn TriangleIndicesDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetTriangleIndicesDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn TriangleMaterialIndicesDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetTriangleMaterialIndicesDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn GetVertexPositions(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateVertexPositions(&self, value: u32) -> ::windows::core::Result<()>;
    fn GetVertexNormals(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateVertexNormals(&self, value: u32) -> ::windows::core::Result<()>;
    fn GetTriangleIndices(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateTriangleIndices(&self, value: u32) -> ::windows::core::Result<()>;
    fn GetTriangleMaterialIndices(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateTriangleMaterialIndices(&self, value: u32) -> ::windows::core::Result<()>;
    fn BufferDescriptionSet(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn BufferSet(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn VerifyAsync(&self, value: Printing3DMeshVerificationMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DMesh {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMesh";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DMeshVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMeshImpl, const OFFSET: isize>() -> IPrinting3DMeshVtbl {
        unsafe extern "system" fn VertexCount<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexCount<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVertexCount(value).into()
        }
        unsafe extern "system" fn IndexCount<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexCount<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndexCount(value).into()
        }
        unsafe extern "system" fn VertexPositionsDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexPositionsDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexPositionsDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVertexPositionsDescription(&*(&value as *const <Printing3DBufferDescription as ::windows::core::Abi>::Abi as *const <Printing3DBufferDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VertexNormalsDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexNormalsDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexNormalsDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVertexNormalsDescription(&*(&value as *const <Printing3DBufferDescription as ::windows::core::Abi>::Abi as *const <Printing3DBufferDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TriangleIndicesDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriangleIndicesDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriangleIndicesDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTriangleIndicesDescription(&*(&value as *const <Printing3DBufferDescription as ::windows::core::Abi>::Abi as *const <Printing3DBufferDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TriangleMaterialIndicesDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriangleMaterialIndicesDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriangleMaterialIndicesDescription<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTriangleMaterialIndicesDescription(&*(&value as *const <Printing3DBufferDescription as ::windows::core::Abi>::Abi as *const <Printing3DBufferDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetVertexPositions<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVertexPositions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexPositions<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateVertexPositions(value).into()
        }
        unsafe extern "system" fn GetVertexNormals<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVertexNormals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexNormals<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateVertexNormals(value).into()
        }
        unsafe extern "system" fn GetTriangleIndices<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTriangleIndices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTriangleIndices<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateTriangleIndices(value).into()
        }
        unsafe extern "system" fn GetTriangleMaterialIndices<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTriangleMaterialIndices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTriangleMaterialIndices<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateTriangleMaterialIndices(value).into()
        }
        unsafe extern "system" fn BufferDescriptionSet<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferDescriptionSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferSet<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyAsync<Impl: IPrinting3DMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DMeshVerificationMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyAsync(value) {
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
            ::windows::core::GetRuntimeClassName::<IPrinting3DMesh>,
            ::windows::core::GetTrustLevel,
            VertexCount::<Impl, OFFSET>,
            SetVertexCount::<Impl, OFFSET>,
            IndexCount::<Impl, OFFSET>,
            SetIndexCount::<Impl, OFFSET>,
            VertexPositionsDescription::<Impl, OFFSET>,
            SetVertexPositionsDescription::<Impl, OFFSET>,
            VertexNormalsDescription::<Impl, OFFSET>,
            SetVertexNormalsDescription::<Impl, OFFSET>,
            TriangleIndicesDescription::<Impl, OFFSET>,
            SetTriangleIndicesDescription::<Impl, OFFSET>,
            TriangleMaterialIndicesDescription::<Impl, OFFSET>,
            SetTriangleMaterialIndicesDescription::<Impl, OFFSET>,
            GetVertexPositions::<Impl, OFFSET>,
            CreateVertexPositions::<Impl, OFFSET>,
            GetVertexNormals::<Impl, OFFSET>,
            CreateVertexNormals::<Impl, OFFSET>,
            GetTriangleIndices::<Impl, OFFSET>,
            CreateTriangleIndices::<Impl, OFFSET>,
            GetTriangleMaterialIndices::<Impl, OFFSET>,
            CreateTriangleMaterialIndices::<Impl, OFFSET>,
            BufferDescriptionSet::<Impl, OFFSET>,
            BufferSet::<Impl, OFFSET>,
            VerifyAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMeshVerificationResultImpl: Sized {
    fn IsValid(&self) -> ::windows::core::Result<bool>;
    fn NonmanifoldTriangles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn ReversedNormalTriangles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DMeshVerificationResult {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DMeshVerificationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMeshVerificationResultImpl, const OFFSET: isize>() -> IPrinting3DMeshVerificationResultVtbl {
        unsafe extern "system" fn IsValid<Impl: IPrinting3DMeshVerificationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonmanifoldTriangles<Impl: IPrinting3DMeshVerificationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonmanifoldTriangles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReversedNormalTriangles<Impl: IPrinting3DMeshVerificationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReversedNormalTriangles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DMeshVerificationResult>, ::windows::core::GetTrustLevel, IsValid::<Impl, OFFSET>, NonmanifoldTriangles::<Impl, OFFSET>, ReversedNormalTriangles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DModelImpl: Sized {
    fn Unit(&self) -> ::windows::core::Result<Printing3DModelUnit>;
    fn SetUnit(&self, value: Printing3DModelUnit) -> ::windows::core::Result<()>;
    fn Textures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DModelTexture>>;
    fn Meshes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMesh>>;
    fn Components(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponent>>;
    fn Material(&self) -> ::windows::core::Result<Printing3DMaterial>;
    fn SetMaterial(&self, value: &::core::option::Option<Printing3DMaterial>) -> ::windows::core::Result<()>;
    fn Build(&self) -> ::windows::core::Result<Printing3DComponent>;
    fn SetBuild(&self, value: &::core::option::Option<Printing3DComponent>) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RequiredExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Metadata(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn RepairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Clone(&self) -> ::windows::core::Result<Printing3DModel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DModel {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DModel";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DModelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DModelImpl, const OFFSET: isize>() -> IPrinting3DModelVtbl {
        unsafe extern "system" fn Unit<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DModelUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnit<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DModelUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnit(value).into()
        }
        unsafe extern "system" fn Textures<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Textures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Meshes<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Meshes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Components<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Material<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Material() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaterial<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaterial(&*(&value as *const <Printing3DMaterial as ::windows::core::Abi>::Abi as *const <Printing3DMaterial as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Build<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Build() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuild<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBuild(&*(&value as *const <Printing3DComponent as ::windows::core::Abi>::Abi as *const <Printing3DComponent as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Version<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequiredExtensions<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Metadata<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Metadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RepairAsync<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepairAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IPrinting3DModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
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
            ::windows::core::GetRuntimeClassName::<IPrinting3DModel>,
            ::windows::core::GetTrustLevel,
            Unit::<Impl, OFFSET>,
            SetUnit::<Impl, OFFSET>,
            Textures::<Impl, OFFSET>,
            Meshes::<Impl, OFFSET>,
            Components::<Impl, OFFSET>,
            Material::<Impl, OFFSET>,
            SetMaterial::<Impl, OFFSET>,
            Build::<Impl, OFFSET>,
            SetBuild::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            SetVersion::<Impl, OFFSET>,
            RequiredExtensions::<Impl, OFFSET>,
            Metadata::<Impl, OFFSET>,
            RepairAsync::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DModel2Impl: Sized {
    fn TryPartialRepairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryPartialRepairWithTimeAsync(&self, maxwaittime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryReduceFacesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn TryReduceFacesWithOptionsAsync(&self, printing3dfacereductionoptions: &::core::option::Option<Printing3DFaceReductionOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn TryReduceFacesWithOptionsAndTimeAsync(&self, printing3dfacereductionoptions: &::core::option::Option<Printing3DFaceReductionOptions>, maxwait: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn RepairWithProgressAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DModel2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DModel2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DModel2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DModel2Impl, const OFFSET: isize>() -> IPrinting3DModel2Vtbl {
        unsafe extern "system" fn TryPartialRepairAsync<Impl: IPrinting3DModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryPartialRepairAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryPartialRepairWithTimeAsync<Impl: IPrinting3DModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxwaittime: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryPartialRepairWithTimeAsync(&*(&maxwaittime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryReduceFacesAsync<Impl: IPrinting3DModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryReduceFacesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryReduceFacesWithOptionsAsync<Impl: IPrinting3DModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printing3dfacereductionoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryReduceFacesWithOptionsAsync(&*(&printing3dfacereductionoptions as *const <Printing3DFaceReductionOptions as ::windows::core::Abi>::Abi as *const <Printing3DFaceReductionOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryReduceFacesWithOptionsAndTimeAsync<Impl: IPrinting3DModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printing3dfacereductionoptions: ::windows::core::RawPtr, maxwait: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryReduceFacesWithOptionsAndTimeAsync(&*(&printing3dfacereductionoptions as *const <Printing3DFaceReductionOptions as ::windows::core::Abi>::Abi as *const <Printing3DFaceReductionOptions as ::windows::core::DefaultType>::DefaultType), &*(&maxwait as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RepairWithProgressAsync<Impl: IPrinting3DModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepairWithProgressAsync() {
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
            ::windows::core::GetRuntimeClassName::<IPrinting3DModel2>,
            ::windows::core::GetTrustLevel,
            TryPartialRepairAsync::<Impl, OFFSET>,
            TryPartialRepairWithTimeAsync::<Impl, OFFSET>,
            TryReduceFacesAsync::<Impl, OFFSET>,
            TryReduceFacesWithOptionsAsync::<Impl, OFFSET>,
            TryReduceFacesWithOptionsAndTimeAsync::<Impl, OFFSET>,
            RepairWithProgressAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DModelTextureImpl: Sized {
    fn TextureResource(&self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetTextureResource(&self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn TileStyleU(&self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior>;
    fn SetTileStyleU(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()>;
    fn TileStyleV(&self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior>;
    fn SetTileStyleV(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DModelTexture {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DModelTexture";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DModelTextureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DModelTextureImpl, const OFFSET: isize>() -> IPrinting3DModelTextureVtbl {
        unsafe extern "system" fn TextureResource<Impl: IPrinting3DModelTextureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextureResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureResource<Impl: IPrinting3DModelTextureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextureResource(&*(&value as *const <Printing3DTextureResource as ::windows::core::Abi>::Abi as *const <Printing3DTextureResource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TileStyleU<Impl: IPrinting3DModelTextureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileStyleU() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileStyleU<Impl: IPrinting3DModelTextureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTileStyleU(value).into()
        }
        unsafe extern "system" fn TileStyleV<Impl: IPrinting3DModelTextureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileStyleV() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileStyleV<Impl: IPrinting3DModelTextureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTileStyleV(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DModelTexture>, ::windows::core::GetTrustLevel, TextureResource::<Impl, OFFSET>, SetTextureResource::<Impl, OFFSET>, TileStyleU::<Impl, OFFSET>, SetTileStyleU::<Impl, OFFSET>, TileStyleV::<Impl, OFFSET>, SetTileStyleV::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMultiplePropertyMaterialImpl: Sized {
    fn MaterialIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DMultiplePropertyMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DMultiplePropertyMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMultiplePropertyMaterialImpl, const OFFSET: isize>() -> IPrinting3DMultiplePropertyMaterialVtbl {
        unsafe extern "system" fn MaterialIndices<Impl: IPrinting3DMultiplePropertyMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialIndices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DMultiplePropertyMaterial>, ::windows::core::GetTrustLevel, MaterialIndices::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMultiplePropertyMaterialGroupImpl: Sized {
    fn MultipleProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>>;
    fn MaterialGroupIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DMultiplePropertyMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DMultiplePropertyMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMultiplePropertyMaterialGroupImpl, const OFFSET: isize>() -> IPrinting3DMultiplePropertyMaterialGroupVtbl {
        unsafe extern "system" fn MultipleProperties<Impl: IPrinting3DMultiplePropertyMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultipleProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialGroupIndices<Impl: IPrinting3DMultiplePropertyMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialGroupIndices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialGroupId<Impl: IPrinting3DMultiplePropertyMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialGroupId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DMultiplePropertyMaterialGroup>, ::windows::core::GetTrustLevel, MultipleProperties::<Impl, OFFSET>, MaterialGroupIndices::<Impl, OFFSET>, MaterialGroupId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMultiplePropertyMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DMultiplePropertyMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DMultiplePropertyMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DMultiplePropertyMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMultiplePropertyMaterialGroupFactoryImpl, const OFFSET: isize>() -> IPrinting3DMultiplePropertyMaterialGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPrinting3DMultiplePropertyMaterialGroupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(materialgroupid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DMultiplePropertyMaterialGroupFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialImpl: Sized {
    fn Texture(&self) -> ::windows::core::Result<Printing3DModelTexture>;
    fn SetTexture(&self, value: &::core::option::Option<Printing3DModelTexture>) -> ::windows::core::Result<()>;
    fn U(&self) -> ::windows::core::Result<f64>;
    fn SetU(&self, value: f64) -> ::windows::core::Result<()>;
    fn V(&self) -> ::windows::core::Result<f64>;
    fn SetV(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTexture2CoordMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialImpl, const OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialVtbl {
        unsafe extern "system" fn Texture<Impl: IPrinting3DTexture2CoordMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Texture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTexture<Impl: IPrinting3DTexture2CoordMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTexture(&*(&value as *const <Printing3DModelTexture as ::windows::core::Abi>::Abi as *const <Printing3DModelTexture as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn U<Impl: IPrinting3DTexture2CoordMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).U() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetU<Impl: IPrinting3DTexture2CoordMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetU(value).into()
        }
        unsafe extern "system" fn V<Impl: IPrinting3DTexture2CoordMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).V() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetV<Impl: IPrinting3DTexture2CoordMaterialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetV(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DTexture2CoordMaterial>, ::windows::core::GetTrustLevel, Texture::<Impl, OFFSET>, SetTexture::<Impl, OFFSET>, U::<Impl, OFFSET>, SetU::<Impl, OFFSET>, V::<Impl, OFFSET>, SetV::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroupImpl: Sized {
    fn Texture2Coords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTexture2CoordMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialGroupImpl, const OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialGroupVtbl {
        unsafe extern "system" fn Texture2Coords<Impl: IPrinting3DTexture2CoordMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Texture2Coords() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialGroupId<Impl: IPrinting3DTexture2CoordMaterialGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialGroupId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DTexture2CoordMaterialGroup>, ::windows::core::GetTrustLevel, Texture2Coords::<Impl, OFFSET>, MaterialGroupId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroup2Impl: Sized {
    fn Texture(&self) -> ::windows::core::Result<Printing3DModelTexture>;
    fn SetTexture(&self, value: &::core::option::Option<Printing3DModelTexture>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterialGroup2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTexture2CoordMaterialGroup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialGroup2Impl, const OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialGroup2Vtbl {
        unsafe extern "system" fn Texture<Impl: IPrinting3DTexture2CoordMaterialGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Texture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTexture<Impl: IPrinting3DTexture2CoordMaterialGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTexture(&*(&value as *const <Printing3DModelTexture as ::windows::core::Abi>::Abi as *const <Printing3DModelTexture as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DTexture2CoordMaterialGroup2>, ::windows::core::GetTrustLevel, Texture::<Impl, OFFSET>, SetTexture::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DTexture2CoordMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTexture2CoordMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialGroupFactoryImpl, const OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPrinting3DTexture2CoordMaterialGroupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(materialgroupid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DTexture2CoordMaterialGroupFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTextureResourceImpl: Sized {
    fn TextureData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn SetTextureData(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTextureResource {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTextureResource";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTextureResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTextureResourceImpl, const OFFSET: isize>() -> IPrinting3DTextureResourceVtbl {
        unsafe extern "system" fn TextureData<Impl: IPrinting3DTextureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextureData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureData<Impl: IPrinting3DTextureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextureData(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IPrinting3DTextureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IPrinting3DTextureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrinting3DTextureResource>, ::windows::core::GetTrustLevel, TextureData::<Impl, OFFSET>, SetTextureData::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>)
    }
}
