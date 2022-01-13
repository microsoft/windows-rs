#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrint3DManagerImpl: Sized {
    fn TaskRequested(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTaskRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrint3DManager {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrint3DManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DManagerVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DManager, BASE_OFFSET>(),
            TaskRequested: TaskRequested::<Impl, IMPL_OFFSET>,
            RemoveTaskRequested: RemoveTaskRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrint3DManagerStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<Print3DManager>;
    fn ShowPrintUIAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrint3DManagerStatics {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrint3DManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DManagerStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DManagerStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            ShowPrintUIAsync: ShowPrintUIAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrint3DTaskImpl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<Printing3D3MFPackage>;
    fn Submitting(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitting(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChanged(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrint3DTask {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTask";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrint3DTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DTaskVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DTask, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            Submitting: Submitting::<Impl, IMPL_OFFSET>,
            RemoveSubmitting: RemoveSubmitting::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
            SourceChanged: SourceChanged::<Impl, IMPL_OFFSET>,
            RemoveSourceChanged: RemoveSourceChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskCompletedEventArgsImpl: Sized {
    fn Completion(&mut self) -> ::windows::core::Result<Print3DTaskCompletion>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<Print3DTaskDetail>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DTaskCompletedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DTaskCompletedEventArgs, BASE_OFFSET>(),
            Completion: Completion::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DTaskCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskRequestImpl: Sized {
    fn CreateTask(&mut self, title: &::windows::core::HSTRING, printerid: &::windows::core::HSTRING, handler: &::core::option::Option<Print3DTaskSourceRequestedHandler>) -> ::windows::core::Result<Print3DTask>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DTaskRequestVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DTaskRequest, BASE_OFFSET>(), CreateTask: CreateTask::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DTaskRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskRequestedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<Print3DTaskRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DTaskRequestedEventArgsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DTaskRequestedEventArgs, BASE_OFFSET>(), Request: Request::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DTaskRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskSourceChangedEventArgsImpl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<Printing3D3MFPackage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskSourceChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskSourceChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskSourceChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DTaskSourceChangedEventArgsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DTaskSourceChangedEventArgs, BASE_OFFSET>(), Source: Source::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DTaskSourceChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskSourceRequestedArgsImpl: Sized {
    fn SetSource(&mut self, source: &::core::option::Option<Printing3D3MFPackage>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DTaskSourceRequestedArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DTaskSourceRequestedArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrint3DTaskSourceRequestedArgsVtbl {
        unsafe extern "system" fn SetSource<Impl: IPrint3DTaskSourceRequestedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&source as *const <Printing3D3MFPackage as ::windows::core::Abi>::Abi as *const <Printing3D3MFPackage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrint3DTaskSourceRequestedArgs, BASE_OFFSET>(),
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrint3DTaskSourceRequestedArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrinting3D3MFPackageImpl: Sized {
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn PrintTicket(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetPrintTicket(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn ModelPart(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetModelPart(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn Textures(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTextureResource>>;
    fn LoadModelFromPackageAsync(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DModel>>;
    fn SaveModelToPackageAsync(&mut self, value: &::core::option::Option<Printing3DModel>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3D3MFPackage {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3D3MFPackage";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrinting3D3MFPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3D3MFPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3D3MFPackageVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3D3MFPackage, BASE_OFFSET>(),
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            PrintTicket: PrintTicket::<Impl, IMPL_OFFSET>,
            SetPrintTicket: SetPrintTicket::<Impl, IMPL_OFFSET>,
            ModelPart: ModelPart::<Impl, IMPL_OFFSET>,
            SetModelPart: SetModelPart::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            Textures: Textures::<Impl, IMPL_OFFSET>,
            LoadModelFromPackageAsync: LoadModelFromPackageAsync::<Impl, IMPL_OFFSET>,
            SaveModelToPackageAsync: SaveModelToPackageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3D3MFPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3D3MFPackage2Impl: Sized {
    fn Compression(&mut self) -> ::windows::core::Result<Printing3DPackageCompression>;
    fn SetCompression(&mut self, value: Printing3DPackageCompression) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3D3MFPackage2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3D3MFPackage2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3D3MFPackage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3D3MFPackage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3D3MFPackage2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3D3MFPackage2, BASE_OFFSET>(),
            Compression: Compression::<Impl, IMPL_OFFSET>,
            SetCompression: SetCompression::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3D3MFPackage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrinting3D3MFPackageStaticsImpl: Sized {
    fn LoadAsync(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3D3MFPackageStatics {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrinting3D3MFPackageStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3D3MFPackageStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3D3MFPackageStaticsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3D3MFPackageStatics, BASE_OFFSET>(), LoadAsync: LoadAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3D3MFPackageStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Color(&mut self) -> ::windows::core::Result<Printing3DColorMaterial>;
    fn SetColor(&mut self, value: &::core::option::Option<Printing3DColorMaterial>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DBaseMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DBaseMaterialVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DBaseMaterial, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DBaseMaterial as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DBaseMaterialGroupImpl: Sized {
    fn Bases(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>>;
    fn MaterialGroupId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DBaseMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DBaseMaterialGroupVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DBaseMaterialGroup, BASE_OFFSET>(),
            Bases: Bases::<Impl, IMPL_OFFSET>,
            MaterialGroupId: MaterialGroupId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DBaseMaterialGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialGroupFactoryImpl: Sized {
    fn Create(&mut self, materialgroupid: u32) -> ::windows::core::Result<Printing3DBaseMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DBaseMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialGroupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DBaseMaterialGroupFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DBaseMaterialGroupFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DBaseMaterialGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialStaticsImpl: Sized {
    fn Abs(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pla(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DBaseMaterialStatics {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DBaseMaterialStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DBaseMaterialStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DBaseMaterialStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DBaseMaterialStatics, BASE_OFFSET>(),
            Abs: Abs::<Impl, IMPL_OFFSET>,
            Pla: Pla::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DBaseMaterialStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialImpl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<u32>;
    fn SetValue(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DColorMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DColorMaterialVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DColorMaterial, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DColorMaterial as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait IPrinting3DColorMaterial2Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterial2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterial2";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IPrinting3DColorMaterial2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterial2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DColorMaterial2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DColorMaterial2, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DColorMaterial2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DColorMaterialGroupImpl: Sized {
    fn Colors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>>;
    fn MaterialGroupId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DColorMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterialGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DColorMaterialGroupVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DColorMaterialGroup, BASE_OFFSET>(),
            Colors: Colors::<Impl, IMPL_OFFSET>,
            MaterialGroupId: MaterialGroupId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DColorMaterialGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialGroupFactoryImpl: Sized {
    fn Create(&mut self, materialgroupid: u32) -> ::windows::core::Result<Printing3DColorMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DColorMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DColorMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DColorMaterialGroupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DColorMaterialGroupFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DColorMaterialGroupFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DColorMaterialGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DComponentImpl: Sized {
    fn Mesh(&mut self) -> ::windows::core::Result<Printing3DMesh>;
    fn SetMesh(&mut self, value: &::core::option::Option<Printing3DMesh>) -> ::windows::core::Result<()>;
    fn Components(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<Printing3DObjectType>;
    fn SetType(&mut self, value: Printing3DObjectType) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PartNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPartNumber(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DComponent {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DComponent";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DComponentVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DComponent, BASE_OFFSET>(),
            Mesh: Mesh::<Impl, IMPL_OFFSET>,
            SetMesh: SetMesh::<Impl, IMPL_OFFSET>,
            Components: Components::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            PartNumber: PartNumber::<Impl, IMPL_OFFSET>,
            SetPartNumber: SetPartNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IPrinting3DComponentWithMatrixImpl: Sized {
    fn Component(&mut self) -> ::windows::core::Result<Printing3DComponent>;
    fn SetComponent(&mut self, value: &::core::option::Option<Printing3DComponent>) -> ::windows::core::Result<()>;
    fn Matrix(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetMatrix(&mut self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DComponentWithMatrix {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IPrinting3DComponentWithMatrixVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DComponentWithMatrixImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DComponentWithMatrixVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DComponentWithMatrix, BASE_OFFSET>(),
            Component: Component::<Impl, IMPL_OFFSET>,
            SetComponent: SetComponent::<Impl, IMPL_OFFSET>,
            Matrix: Matrix::<Impl, IMPL_OFFSET>,
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DComponentWithMatrix as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DCompositeMaterialImpl: Sized {
    fn Values(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<f64>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DCompositeMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DCompositeMaterialVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DCompositeMaterial, BASE_OFFSET>(), Values: Values::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DCompositeMaterial as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DCompositeMaterialGroupImpl: Sized {
    fn Composites(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>>;
    fn MaterialGroupId(&mut self) -> ::windows::core::Result<u32>;
    fn MaterialIndices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DCompositeMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DCompositeMaterialGroupVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DCompositeMaterialGroup, BASE_OFFSET>(),
            Composites: Composites::<Impl, IMPL_OFFSET>,
            MaterialGroupId: MaterialGroupId::<Impl, IMPL_OFFSET>,
            MaterialIndices: MaterialIndices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DCompositeMaterialGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroup2Impl: Sized {
    fn BaseMaterialGroup(&mut self) -> ::windows::core::Result<Printing3DBaseMaterialGroup>;
    fn SetBaseMaterialGroup(&mut self, value: &::core::option::Option<Printing3DBaseMaterialGroup>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterialGroup2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DCompositeMaterialGroup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialGroup2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DCompositeMaterialGroup2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DCompositeMaterialGroup2, BASE_OFFSET>(),
            BaseMaterialGroup: BaseMaterialGroup::<Impl, IMPL_OFFSET>,
            SetBaseMaterialGroup: SetBaseMaterialGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DCompositeMaterialGroup2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroupFactoryImpl: Sized {
    fn Create(&mut self, materialgroupid: u32) -> ::windows::core::Result<Printing3DCompositeMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DCompositeMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DCompositeMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DCompositeMaterialGroupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DCompositeMaterialGroupFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DCompositeMaterialGroupFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DCompositeMaterialGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DFaceReductionOptionsImpl: Sized {
    fn MaxReductionArea(&mut self) -> ::windows::core::Result<f64>;
    fn SetMaxReductionArea(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn TargetTriangleCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetTargetTriangleCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn MaxEdgeLength(&mut self) -> ::windows::core::Result<f64>;
    fn SetMaxEdgeLength(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DFaceReductionOptions {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DFaceReductionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DFaceReductionOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DFaceReductionOptionsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DFaceReductionOptions, BASE_OFFSET>(),
            MaxReductionArea: MaxReductionArea::<Impl, IMPL_OFFSET>,
            SetMaxReductionArea: SetMaxReductionArea::<Impl, IMPL_OFFSET>,
            TargetTriangleCount: TargetTriangleCount::<Impl, IMPL_OFFSET>,
            SetTargetTriangleCount: SetTargetTriangleCount::<Impl, IMPL_OFFSET>,
            MaxEdgeLength: MaxEdgeLength::<Impl, IMPL_OFFSET>,
            SetMaxEdgeLength: SetMaxEdgeLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DFaceReductionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DMaterialImpl: Sized {
    fn BaseGroups(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>>;
    fn ColorGroups(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>>;
    fn Texture2CoordGroups(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>>;
    fn CompositeGroups(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>>;
    fn MultiplePropertyGroups(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMaterial";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMaterialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DMaterialVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DMaterial, BASE_OFFSET>(),
            BaseGroups: BaseGroups::<Impl, IMPL_OFFSET>,
            ColorGroups: ColorGroups::<Impl, IMPL_OFFSET>,
            Texture2CoordGroups: Texture2CoordGroups::<Impl, IMPL_OFFSET>,
            CompositeGroups: CompositeGroups::<Impl, IMPL_OFFSET>,
            MultiplePropertyGroups: MultiplePropertyGroups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DMaterial as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrinting3DMeshImpl: Sized {
    fn VertexCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetVertexCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn IndexCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetIndexCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn VertexPositionsDescription(&mut self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetVertexPositionsDescription(&mut self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn VertexNormalsDescription(&mut self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetVertexNormalsDescription(&mut self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn TriangleIndicesDescription(&mut self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetTriangleIndicesDescription(&mut self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn TriangleMaterialIndicesDescription(&mut self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetTriangleMaterialIndicesDescription(&mut self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn GetVertexPositions(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateVertexPositions(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn GetVertexNormals(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateVertexNormals(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn GetTriangleIndices(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateTriangleIndices(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn GetTriangleMaterialIndices(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateTriangleMaterialIndices(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn BufferDescriptionSet(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn BufferSet(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn VerifyAsync(&mut self, value: Printing3DMeshVerificationMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DMesh {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMesh";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrinting3DMeshVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMeshImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DMeshVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DMesh, BASE_OFFSET>(),
            VertexCount: VertexCount::<Impl, IMPL_OFFSET>,
            SetVertexCount: SetVertexCount::<Impl, IMPL_OFFSET>,
            IndexCount: IndexCount::<Impl, IMPL_OFFSET>,
            SetIndexCount: SetIndexCount::<Impl, IMPL_OFFSET>,
            VertexPositionsDescription: VertexPositionsDescription::<Impl, IMPL_OFFSET>,
            SetVertexPositionsDescription: SetVertexPositionsDescription::<Impl, IMPL_OFFSET>,
            VertexNormalsDescription: VertexNormalsDescription::<Impl, IMPL_OFFSET>,
            SetVertexNormalsDescription: SetVertexNormalsDescription::<Impl, IMPL_OFFSET>,
            TriangleIndicesDescription: TriangleIndicesDescription::<Impl, IMPL_OFFSET>,
            SetTriangleIndicesDescription: SetTriangleIndicesDescription::<Impl, IMPL_OFFSET>,
            TriangleMaterialIndicesDescription: TriangleMaterialIndicesDescription::<Impl, IMPL_OFFSET>,
            SetTriangleMaterialIndicesDescription: SetTriangleMaterialIndicesDescription::<Impl, IMPL_OFFSET>,
            GetVertexPositions: GetVertexPositions::<Impl, IMPL_OFFSET>,
            CreateVertexPositions: CreateVertexPositions::<Impl, IMPL_OFFSET>,
            GetVertexNormals: GetVertexNormals::<Impl, IMPL_OFFSET>,
            CreateVertexNormals: CreateVertexNormals::<Impl, IMPL_OFFSET>,
            GetTriangleIndices: GetTriangleIndices::<Impl, IMPL_OFFSET>,
            CreateTriangleIndices: CreateTriangleIndices::<Impl, IMPL_OFFSET>,
            GetTriangleMaterialIndices: GetTriangleMaterialIndices::<Impl, IMPL_OFFSET>,
            CreateTriangleMaterialIndices: CreateTriangleMaterialIndices::<Impl, IMPL_OFFSET>,
            BufferDescriptionSet: BufferDescriptionSet::<Impl, IMPL_OFFSET>,
            BufferSet: BufferSet::<Impl, IMPL_OFFSET>,
            VerifyAsync: VerifyAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DMesh as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DMeshVerificationResultImpl: Sized {
    fn IsValid(&mut self) -> ::windows::core::Result<bool>;
    fn NonmanifoldTriangles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn ReversedNormalTriangles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DMeshVerificationResult {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DMeshVerificationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMeshVerificationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DMeshVerificationResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DMeshVerificationResult, BASE_OFFSET>(),
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            NonmanifoldTriangles: NonmanifoldTriangles::<Impl, IMPL_OFFSET>,
            ReversedNormalTriangles: ReversedNormalTriangles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DMeshVerificationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DModelImpl: Sized {
    fn Unit(&mut self) -> ::windows::core::Result<Printing3DModelUnit>;
    fn SetUnit(&mut self, value: Printing3DModelUnit) -> ::windows::core::Result<()>;
    fn Textures(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DModelTexture>>;
    fn Meshes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMesh>>;
    fn Components(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponent>>;
    fn Material(&mut self) -> ::windows::core::Result<Printing3DMaterial>;
    fn SetMaterial(&mut self, value: &::core::option::Option<Printing3DMaterial>) -> ::windows::core::Result<()>;
    fn Build(&mut self) -> ::windows::core::Result<Printing3DComponent>;
    fn SetBuild(&mut self, value: &::core::option::Option<Printing3DComponent>) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RequiredExtensions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Metadata(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn RepairAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Clone(&mut self) -> ::windows::core::Result<Printing3DModel>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DModel {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DModel";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DModelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DModelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DModelVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DModel, BASE_OFFSET>(),
            Unit: Unit::<Impl, IMPL_OFFSET>,
            SetUnit: SetUnit::<Impl, IMPL_OFFSET>,
            Textures: Textures::<Impl, IMPL_OFFSET>,
            Meshes: Meshes::<Impl, IMPL_OFFSET>,
            Components: Components::<Impl, IMPL_OFFSET>,
            Material: Material::<Impl, IMPL_OFFSET>,
            SetMaterial: SetMaterial::<Impl, IMPL_OFFSET>,
            Build: Build::<Impl, IMPL_OFFSET>,
            SetBuild: SetBuild::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            RequiredExtensions: RequiredExtensions::<Impl, IMPL_OFFSET>,
            Metadata: Metadata::<Impl, IMPL_OFFSET>,
            RepairAsync: RepairAsync::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DModel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrinting3DModel2Impl: Sized {
    fn TryPartialRepairAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryPartialRepairWithTimeAsync(&mut self, maxwaittime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryReduceFacesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn TryReduceFacesWithOptionsAsync(&mut self, printing3dfacereductionoptions: &::core::option::Option<Printing3DFaceReductionOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn TryReduceFacesWithOptionsAndTimeAsync(&mut self, printing3dfacereductionoptions: &::core::option::Option<Printing3DFaceReductionOptions>, maxwait: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn RepairWithProgressAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DModel2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DModel2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrinting3DModel2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DModel2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DModel2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DModel2, BASE_OFFSET>(),
            TryPartialRepairAsync: TryPartialRepairAsync::<Impl, IMPL_OFFSET>,
            TryPartialRepairWithTimeAsync: TryPartialRepairWithTimeAsync::<Impl, IMPL_OFFSET>,
            TryReduceFacesAsync: TryReduceFacesAsync::<Impl, IMPL_OFFSET>,
            TryReduceFacesWithOptionsAsync: TryReduceFacesWithOptionsAsync::<Impl, IMPL_OFFSET>,
            TryReduceFacesWithOptionsAndTimeAsync: TryReduceFacesWithOptionsAndTimeAsync::<Impl, IMPL_OFFSET>,
            RepairWithProgressAsync: RepairWithProgressAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DModel2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DModelTextureImpl: Sized {
    fn TextureResource(&mut self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetTextureResource(&mut self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn TileStyleU(&mut self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior>;
    fn SetTileStyleU(&mut self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()>;
    fn TileStyleV(&mut self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior>;
    fn SetTileStyleV(&mut self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DModelTexture {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DModelTexture";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DModelTextureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DModelTextureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DModelTextureVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DModelTexture, BASE_OFFSET>(),
            TextureResource: TextureResource::<Impl, IMPL_OFFSET>,
            SetTextureResource: SetTextureResource::<Impl, IMPL_OFFSET>,
            TileStyleU: TileStyleU::<Impl, IMPL_OFFSET>,
            SetTileStyleU: SetTileStyleU::<Impl, IMPL_OFFSET>,
            TileStyleV: TileStyleV::<Impl, IMPL_OFFSET>,
            SetTileStyleV: SetTileStyleV::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DModelTexture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DMultiplePropertyMaterialImpl: Sized {
    fn MaterialIndices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DMultiplePropertyMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DMultiplePropertyMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMultiplePropertyMaterialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DMultiplePropertyMaterialVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DMultiplePropertyMaterial, BASE_OFFSET>(),
            MaterialIndices: MaterialIndices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DMultiplePropertyMaterial as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DMultiplePropertyMaterialGroupImpl: Sized {
    fn MultipleProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>>;
    fn MaterialGroupIndices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
    fn MaterialGroupId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DMultiplePropertyMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DMultiplePropertyMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMultiplePropertyMaterialGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DMultiplePropertyMaterialGroupVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DMultiplePropertyMaterialGroup, BASE_OFFSET>(),
            MultipleProperties: MultipleProperties::<Impl, IMPL_OFFSET>,
            MaterialGroupIndices: MaterialGroupIndices::<Impl, IMPL_OFFSET>,
            MaterialGroupId: MaterialGroupId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DMultiplePropertyMaterialGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMultiplePropertyMaterialGroupFactoryImpl: Sized {
    fn Create(&mut self, materialgroupid: u32) -> ::windows::core::Result<Printing3DMultiplePropertyMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DMultiplePropertyMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DMultiplePropertyMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DMultiplePropertyMaterialGroupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DMultiplePropertyMaterialGroupFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DMultiplePropertyMaterialGroupFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DMultiplePropertyMaterialGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialImpl: Sized {
    fn Texture(&mut self) -> ::windows::core::Result<Printing3DModelTexture>;
    fn SetTexture(&mut self, value: &::core::option::Option<Printing3DModelTexture>) -> ::windows::core::Result<()>;
    fn U(&mut self) -> ::windows::core::Result<f64>;
    fn SetU(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn V(&mut self) -> ::windows::core::Result<f64>;
    fn SetV(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTexture2CoordMaterialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DTexture2CoordMaterial, BASE_OFFSET>(),
            Texture: Texture::<Impl, IMPL_OFFSET>,
            SetTexture: SetTexture::<Impl, IMPL_OFFSET>,
            U: U::<Impl, IMPL_OFFSET>,
            SetU: SetU::<Impl, IMPL_OFFSET>,
            V: V::<Impl, IMPL_OFFSET>,
            SetV: SetV::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DTexture2CoordMaterial as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrinting3DTexture2CoordMaterialGroupImpl: Sized {
    fn Texture2Coords(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>>;
    fn MaterialGroupId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrinting3DTexture2CoordMaterialGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialGroupVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DTexture2CoordMaterialGroup, BASE_OFFSET>(),
            Texture2Coords: Texture2Coords::<Impl, IMPL_OFFSET>,
            MaterialGroupId: MaterialGroupId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DTexture2CoordMaterialGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroup2Impl: Sized {
    fn Texture(&mut self) -> ::windows::core::Result<Printing3DModelTexture>;
    fn SetTexture(&mut self, value: &::core::option::Option<Printing3DModelTexture>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterialGroup2 {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTexture2CoordMaterialGroup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialGroup2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialGroup2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DTexture2CoordMaterialGroup2, BASE_OFFSET>(),
            Texture: Texture::<Impl, IMPL_OFFSET>,
            SetTexture: SetTexture::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DTexture2CoordMaterialGroup2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroupFactoryImpl: Sized {
    fn Create(&mut self, materialgroupid: u32) -> ::windows::core::Result<Printing3DTexture2CoordMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrinting3DTexture2CoordMaterialGroupFactory {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrinting3DTexture2CoordMaterialGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTexture2CoordMaterialGroupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DTexture2CoordMaterialGroupFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DTexture2CoordMaterialGroupFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DTexture2CoordMaterialGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrinting3DTextureResourceImpl: Sized {
    fn TextureData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn SetTextureData(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrinting3DTextureResource {
    const NAME: &'static str = "Windows.Graphics.Printing3D.IPrinting3DTextureResource";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrinting3DTextureResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DTextureResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DTextureResourceVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DTextureResource, BASE_OFFSET>(),
            TextureData: TextureData::<Impl, IMPL_OFFSET>,
            SetTextureData: SetTextureData::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DTextureResource as ::windows::core::Interface>::IID
    }
}
