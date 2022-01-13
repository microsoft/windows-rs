#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterStaticsImpl: Sized {
    fn SetUpdateInformation(&mut self, file: &::core::option::Option<super::IStorageFile>, contentid: &::windows::core::HSTRING, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICachedFileUpdaterStatics {
    const NAME: &'static str = "Windows.Storage.Provider.ICachedFileUpdaterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICachedFileUpdaterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileUpdaterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICachedFileUpdaterStaticsVtbl {
        unsafe extern "system" fn SetUpdateInformation<Impl: ICachedFileUpdaterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateInformation(&*(&file as *const <super::IStorageFile as ::windows::core::Abi>::Abi as *const <super::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&contentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), readmode, writemode, options).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICachedFileUpdaterStatics, BASE_OFFSET>(),
            SetUpdateInformation: SetUpdateInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileUpdaterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICachedFileUpdaterUIImpl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateTarget(&mut self) -> ::windows::core::Result<CachedFileTarget>;
    fn FileUpdateRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileUpdateRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UIRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUIRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UIStatus(&mut self) -> ::windows::core::Result<UIStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICachedFileUpdaterUI {
    const NAME: &'static str = "Windows.Storage.Provider.ICachedFileUpdaterUI";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICachedFileUpdaterUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileUpdaterUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICachedFileUpdaterUIVtbl {
        unsafe extern "system" fn Title<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateTarget<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CachedFileTarget) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileUpdateRequested<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileUpdateRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFileUpdateRequested<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFileUpdateRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UIRequested<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUIRequested<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUIRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UIStatus<Impl: ICachedFileUpdaterUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UIStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICachedFileUpdaterUI, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            UpdateTarget: UpdateTarget::<Impl, IMPL_OFFSET>,
            FileUpdateRequested: FileUpdateRequested::<Impl, IMPL_OFFSET>,
            RemoveFileUpdateRequested: RemoveFileUpdateRequested::<Impl, IMPL_OFFSET>,
            UIRequested: UIRequested::<Impl, IMPL_OFFSET>,
            RemoveUIRequested: RemoveUIRequested::<Impl, IMPL_OFFSET>,
            UIStatus: UIStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileUpdaterUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICachedFileUpdaterUI2Impl: Sized + ICachedFileUpdaterUIImpl {
    fn UpdateRequest(&mut self) -> ::windows::core::Result<FileUpdateRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<FileUpdateRequestDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICachedFileUpdaterUI2 {
    const NAME: &'static str = "Windows.Storage.Provider.ICachedFileUpdaterUI2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICachedFileUpdaterUI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileUpdaterUI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICachedFileUpdaterUI2Vtbl {
        unsafe extern "system" fn UpdateRequest<Impl: ICachedFileUpdaterUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ICachedFileUpdaterUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICachedFileUpdaterUI2, BASE_OFFSET>(),
            UpdateRequest: UpdateRequest::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileUpdaterUI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequestImpl: Sized {
    fn ContentId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn File(&mut self) -> ::windows::core::Result<super::StorageFile>;
    fn Status(&mut self) -> ::windows::core::Result<FileUpdateStatus>;
    fn SetStatus(&mut self, value: FileUpdateStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<FileUpdateRequestDeferral>;
    fn UpdateLocalFile(&mut self, value: &::core::option::Option<super::IStorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileUpdateRequest {
    const NAME: &'static str = "Windows.Storage.Provider.IFileUpdateRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IFileUpdateRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileUpdateRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileUpdateRequestVtbl {
        unsafe extern "system" fn ContentId<Impl: IFileUpdateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn File<Impl: IFileUpdateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFileUpdateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FileUpdateStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IFileUpdateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FileUpdateStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IFileUpdateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateLocalFile<Impl: IFileUpdateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateLocalFile(&*(&value as *const <super::IStorageFile as ::windows::core::Abi>::Abi as *const <super::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileUpdateRequest, BASE_OFFSET>(),
            ContentId: ContentId::<Impl, IMPL_OFFSET>,
            File: File::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            UpdateLocalFile: UpdateLocalFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileUpdateRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequest2Impl: Sized + IFileUpdateRequestImpl {
    fn UserInputNeededMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserInputNeededMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileUpdateRequest2 {
    const NAME: &'static str = "Windows.Storage.Provider.IFileUpdateRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl IFileUpdateRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileUpdateRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileUpdateRequest2Vtbl {
        unsafe extern "system" fn UserInputNeededMessage<Impl: IFileUpdateRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserInputNeededMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserInputNeededMessage<Impl: IFileUpdateRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserInputNeededMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileUpdateRequest2, BASE_OFFSET>(),
            UserInputNeededMessage: UserInputNeededMessage::<Impl, IMPL_OFFSET>,
            SetUserInputNeededMessage: SetUserInputNeededMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileUpdateRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequestDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileUpdateRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Provider.IFileUpdateRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IFileUpdateRequestDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileUpdateRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileUpdateRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IFileUpdateRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFileUpdateRequestDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileUpdateRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequestedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<FileUpdateRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Provider.IFileUpdateRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFileUpdateRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileUpdateRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileUpdateRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IFileUpdateRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFileUpdateRequestedEventArgs, BASE_OFFSET>(), Request: Request::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileUpdateRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderErrorImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FilePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFilePath(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrimaryAction(&mut self) -> ::windows::core::Result<StorageProviderErrorCommand>;
    fn SetPrimaryAction(&mut self, value: &::core::option::Option<StorageProviderErrorCommand>) -> ::windows::core::Result<()>;
    fn SecondaryAction(&mut self) -> ::windows::core::Result<StorageProviderErrorCommand>;
    fn SetSecondaryAction(&mut self, value: &::core::option::Option<StorageProviderErrorCommand>) -> ::windows::core::Result<()>;
    fn InformationalLink(&mut self) -> ::windows::core::Result<StorageProviderErrorCommand>;
    fn SetInformationalLink(&mut self, value: &::core::option::Option<StorageProviderErrorCommand>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderError {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderError";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderErrorVtbl {
        unsafe extern "system" fn Id<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Title<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Message<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePath<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilePath<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilePath(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrimaryAction<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryAction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryAction<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrimaryAction(&*(&value as *const <StorageProviderErrorCommand as ::windows::core::Abi>::Abi as *const <StorageProviderErrorCommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SecondaryAction<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecondaryAction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecondaryAction<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecondaryAction(&*(&value as *const <StorageProviderErrorCommand as ::windows::core::Abi>::Abi as *const <StorageProviderErrorCommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InformationalLink<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InformationalLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationalLink<Impl: IStorageProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInformationalLink(&*(&value as *const <StorageProviderErrorCommand as ::windows::core::Abi>::Abi as *const <StorageProviderErrorCommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderError, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            FilePath: FilePath::<Impl, IMPL_OFFSET>,
            SetFilePath: SetFilePath::<Impl, IMPL_OFFSET>,
            PrimaryAction: PrimaryAction::<Impl, IMPL_OFFSET>,
            SetPrimaryAction: SetPrimaryAction::<Impl, IMPL_OFFSET>,
            SecondaryAction: SecondaryAction::<Impl, IMPL_OFFSET>,
            SetSecondaryAction: SetSecondaryAction::<Impl, IMPL_OFFSET>,
            InformationalLink: InformationalLink::<Impl, IMPL_OFFSET>,
            SetInformationalLink: SetInformationalLink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageProviderErrorCommandImpl: Sized {
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActionUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderErrorCommand {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderErrorCommand";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageProviderErrorCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderErrorCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderErrorCommandVtbl {
        unsafe extern "system" fn Label<Impl: IStorageProviderErrorCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionUri<Impl: IStorageProviderErrorCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderErrorCommand, BASE_OFFSET>(),
            Label: Label::<Impl, IMPL_OFFSET>,
            ActionUri: ActionUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderErrorCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageProviderErrorCommandFactoryImpl: Sized {
    fn CreateInstance(&mut self, label: &::windows::core::HSTRING, actionuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<StorageProviderErrorCommand>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderErrorCommandFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderErrorCommandFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageProviderErrorCommandFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderErrorCommandFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderErrorCommandFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStorageProviderErrorCommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, actionuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&label as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&actionuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderErrorCommandFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderErrorCommandFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderErrorFactoryImpl: Sized {
    fn CreateInstance(&mut self, id: &::windows::core::HSTRING, title: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderErrorFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderErrorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderErrorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderErrorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderErrorFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStorageProviderErrorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderErrorFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderErrorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderFileTypeInfoImpl: Sized {
    fn FileExtension(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IconResource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderFileTypeInfo {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderFileTypeInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderFileTypeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderFileTypeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderFileTypeInfoVtbl {
        unsafe extern "system" fn FileExtension<Impl: IStorageProviderFileTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconResource<Impl: IStorageProviderFileTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderFileTypeInfo, BASE_OFFSET>(),
            FileExtension: FileExtension::<Impl, IMPL_OFFSET>,
            IconResource: IconResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderFileTypeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderFileTypeInfoFactoryImpl: Sized {
    fn CreateInstance(&mut self, fileextension: &::windows::core::HSTRING, iconresource: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderFileTypeInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderFileTypeInfoFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderFileTypeInfoFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderFileTypeInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderFileTypeInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderFileTypeInfoFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStorageProviderFileTypeInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconresource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&fileextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&iconresource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderFileTypeInfoFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderFileTypeInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderGetContentInfoForPathResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<StorageProviderUriSourceStatus>;
    fn SetStatus(&mut self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()>;
    fn ContentUri(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentUri(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderGetContentInfoForPathResult {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderGetContentInfoForPathResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderGetContentInfoForPathResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderGetContentInfoForPathResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderGetContentInfoForPathResultVtbl {
        unsafe extern "system" fn Status<Impl: IStorageProviderGetContentInfoForPathResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IStorageProviderGetContentInfoForPathResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn ContentUri<Impl: IStorageProviderGetContentInfoForPathResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentUri<Impl: IStorageProviderGetContentInfoForPathResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentUri(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentId<Impl: IStorageProviderGetContentInfoForPathResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentId<Impl: IStorageProviderGetContentInfoForPathResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderGetContentInfoForPathResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            ContentUri: ContentUri::<Impl, IMPL_OFFSET>,
            SetContentUri: SetContentUri::<Impl, IMPL_OFFSET>,
            ContentId: ContentId::<Impl, IMPL_OFFSET>,
            SetContentId: SetContentId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderGetContentInfoForPathResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderGetPathForContentUriResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<StorageProviderUriSourceStatus>;
    fn SetStatus(&mut self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPath(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderGetPathForContentUriResult {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderGetPathForContentUriResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderGetPathForContentUriResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderGetPathForContentUriResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderGetPathForContentUriResultVtbl {
        unsafe extern "system" fn Status<Impl: IStorageProviderGetPathForContentUriResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IStorageProviderGetPathForContentUriResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn Path<Impl: IStorageProviderGetPathForContentUriResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPath<Impl: IStorageProviderGetPathForContentUriResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderGetPathForContentUriResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderGetPathForContentUriResult as ::windows::core::Interface>::IID
    }
}
pub trait IStorageProviderHandlerFactoryImpl: Sized {
    fn GetStatusSource(&mut self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<IStorageProviderStatusSource>;
}
impl ::windows::core::RuntimeName for IStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderHandlerFactory";
}
impl IStorageProviderHandlerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderHandlerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderHandlerFactoryVtbl {
        unsafe extern "system" fn GetStatusSource<Impl: IStorageProviderHandlerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusSource(&*(&syncrootid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderHandlerFactory, BASE_OFFSET>(),
            GetStatusSource: GetStatusSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderHandlerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageProviderItemPropertiesStaticsImpl: Sized {
    fn SetAsync(&mut self, item: &::core::option::Option<super::IStorageItem>, itemproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderItemPropertiesStatics {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertiesStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageProviderItemPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderItemPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderItemPropertiesStaticsVtbl {
        unsafe extern "system" fn SetAsync<Impl: IStorageProviderItemPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, itemproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAsync(&*(&item as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&itemproperties as *const <super::super::Foundation::Collections::IIterable<StorageProviderItemProperty> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<StorageProviderItemProperty> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderItemPropertiesStatics, BASE_OFFSET>(),
            SetAsync: SetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderItemPropertyImpl: Sized {
    fn SetId(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn SetValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIconResource(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconResource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderItemProperty {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemProperty";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderItemPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderItemPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderItemPropertyVtbl {
        unsafe extern "system" fn SetId<Impl: IStorageProviderItemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(value).into()
        }
        unsafe extern "system" fn Id<Impl: IStorageProviderItemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IStorageProviderItemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: IStorageProviderItemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIconResource<Impl: IStorageProviderItemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IconResource<Impl: IStorageProviderItemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderItemProperty, BASE_OFFSET>(),
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetIconResource: SetIconResource::<Impl, IMPL_OFFSET>,
            IconResource: IconResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderItemProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderItemPropertyDefinitionImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn SetId(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn DisplayNameResource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayNameResource(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderItemPropertyDefinition {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertyDefinition";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderItemPropertyDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderItemPropertyDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderItemPropertyDefinitionVtbl {
        unsafe extern "system" fn Id<Impl: IStorageProviderItemPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IStorageProviderItemPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(value).into()
        }
        unsafe extern "system" fn DisplayNameResource<Impl: IStorageProviderItemPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayNameResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayNameResource<Impl: IStorageProviderItemPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayNameResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderItemPropertyDefinition, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            DisplayNameResource: DisplayNameResource::<Impl, IMPL_OFFSET>,
            SetDisplayNameResource: SetDisplayNameResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertyDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageProviderItemPropertySourceImpl: Sized {
    fn GetItemProperties(&mut self, itempath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IStorageProviderItemPropertySource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertySource";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageProviderItemPropertySourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderItemPropertySourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderItemPropertySourceVtbl {
        unsafe extern "system" fn GetItemProperties<Impl: IStorageProviderItemPropertySourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itempath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemProperties(&*(&itempath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderItemPropertySource, BASE_OFFSET>(),
            GetItemProperties: GetItemProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertySource as ::windows::core::Interface>::IID
    }
}
pub trait IStorageProviderPropertyCapabilitiesImpl: Sized {
    fn IsPropertySupported(&mut self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageProviderPropertyCapabilities {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderPropertyCapabilities";
}
impl IStorageProviderPropertyCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderPropertyCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderPropertyCapabilitiesVtbl {
        unsafe extern "system" fn IsPropertySupported<Impl: IStorageProviderPropertyCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertySupported(&*(&propertycanonicalname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderPropertyCapabilities, BASE_OFFSET>(),
            IsPropertySupported: IsPropertySupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderPropertyCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageProviderStatusImpl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<StorageProviderState>;
    fn ErrorMessages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderError>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderStatus {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatus";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageProviderStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderStatusVtbl {
        unsafe extern "system" fn Message<Impl: IStorageProviderStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IStorageProviderStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorMessages<Impl: IStorageProviderStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderStatus, BASE_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            ErrorMessages: ErrorMessages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageProviderStatusFactoryImpl: Sized {
    fn CreateInstance(&mut self, state: StorageProviderState, message: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderStatus>;
    fn CreateInstance2(&mut self, state: StorageProviderState, message: &::windows::core::HSTRING, errormessages: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorageProviderError>>) -> ::windows::core::Result<StorageProviderStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderStatusFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageProviderStatusFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderStatusFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStorageProviderStatusFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(state, &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance2<Impl: IStorageProviderStatusFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, errormessages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance2(state, &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&errormessages as *const <super::super::Foundation::Collections::IIterable<StorageProviderError> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<StorageProviderError> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderStatusFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstance2: CreateInstance2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderStatusFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageProviderStatusSourceImpl: Sized {
    fn GetStatus(&mut self) -> ::windows::core::Result<StorageProviderStatus>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageProviderStatusSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusSource";
}
#[cfg(feature = "Foundation")]
impl IStorageProviderStatusSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderStatusSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderStatusSourceVtbl {
        unsafe extern "system" fn GetStatus<Impl: IStorageProviderStatusSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IStorageProviderStatusSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IStorageProviderStatusSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderStatusSource, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderStatusSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStorageProviderSyncRootInfoImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Context(&mut self) -> ::windows::core::Result<super::Streams::IBuffer>;
    fn SetContext(&mut self, value: &::core::option::Option<super::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::IStorageFolder>;
    fn SetPath(&mut self, value: &::core::option::Option<super::IStorageFolder>) -> ::windows::core::Result<()>;
    fn DisplayNameResource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayNameResource(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconResource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIconResource(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HydrationPolicy(&mut self) -> ::windows::core::Result<StorageProviderHydrationPolicy>;
    fn SetHydrationPolicy(&mut self, value: StorageProviderHydrationPolicy) -> ::windows::core::Result<()>;
    fn HydrationPolicyModifier(&mut self) -> ::windows::core::Result<StorageProviderHydrationPolicyModifier>;
    fn SetHydrationPolicyModifier(&mut self, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::Result<()>;
    fn PopulationPolicy(&mut self) -> ::windows::core::Result<StorageProviderPopulationPolicy>;
    fn SetPopulationPolicy(&mut self, value: StorageProviderPopulationPolicy) -> ::windows::core::Result<()>;
    fn InSyncPolicy(&mut self) -> ::windows::core::Result<StorageProviderInSyncPolicy>;
    fn SetInSyncPolicy(&mut self, value: StorageProviderInSyncPolicy) -> ::windows::core::Result<()>;
    fn HardlinkPolicy(&mut self) -> ::windows::core::Result<StorageProviderHardlinkPolicy>;
    fn SetHardlinkPolicy(&mut self, value: StorageProviderHardlinkPolicy) -> ::windows::core::Result<()>;
    fn ShowSiblingsAsGroup(&mut self) -> ::windows::core::Result<bool>;
    fn SetShowSiblingsAsGroup(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ProtectionMode(&mut self) -> ::windows::core::Result<StorageProviderProtectionMode>;
    fn SetProtectionMode(&mut self, value: StorageProviderProtectionMode) -> ::windows::core::Result<()>;
    fn AllowPinning(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowPinning(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StorageProviderItemPropertyDefinitions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>>;
    fn RecycleBinUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetRecycleBinUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderSyncRootInfo {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSyncRootInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStorageProviderSyncRootInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderSyncRootInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderSyncRootInfoVtbl {
        unsafe extern "system" fn Id<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Context<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(&*(&value as *const <super::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Path<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPath<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <super::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::IStorageFolder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayNameResource<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayNameResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayNameResource<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayNameResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IconResource<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconResource<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HydrationPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HydrationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHydrationPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHydrationPolicy(value).into()
        }
        unsafe extern "system" fn HydrationPolicyModifier<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HydrationPolicyModifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHydrationPolicyModifier<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHydrationPolicyModifier(value).into()
        }
        unsafe extern "system" fn PopulationPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderPopulationPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PopulationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPopulationPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderPopulationPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPopulationPolicy(value).into()
        }
        unsafe extern "system" fn InSyncPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderInSyncPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InSyncPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInSyncPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderInSyncPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInSyncPolicy(value).into()
        }
        unsafe extern "system" fn HardlinkPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardlinkPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHardlinkPolicy<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHardlinkPolicy(value).into()
        }
        unsafe extern "system" fn ShowSiblingsAsGroup<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowSiblingsAsGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowSiblingsAsGroup<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowSiblingsAsGroup(value).into()
        }
        unsafe extern "system" fn Version<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVersion<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProtectionMode<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderProtectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectionMode<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StorageProviderProtectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectionMode(value).into()
        }
        unsafe extern "system" fn AllowPinning<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowPinning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPinning<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowPinning(value).into()
        }
        unsafe extern "system" fn StorageProviderItemPropertyDefinitions<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageProviderItemPropertyDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleBinUri<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecycleBinUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecycleBinUri<Impl: IStorageProviderSyncRootInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecycleBinUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderSyncRootInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            DisplayNameResource: DisplayNameResource::<Impl, IMPL_OFFSET>,
            SetDisplayNameResource: SetDisplayNameResource::<Impl, IMPL_OFFSET>,
            IconResource: IconResource::<Impl, IMPL_OFFSET>,
            SetIconResource: SetIconResource::<Impl, IMPL_OFFSET>,
            HydrationPolicy: HydrationPolicy::<Impl, IMPL_OFFSET>,
            SetHydrationPolicy: SetHydrationPolicy::<Impl, IMPL_OFFSET>,
            HydrationPolicyModifier: HydrationPolicyModifier::<Impl, IMPL_OFFSET>,
            SetHydrationPolicyModifier: SetHydrationPolicyModifier::<Impl, IMPL_OFFSET>,
            PopulationPolicy: PopulationPolicy::<Impl, IMPL_OFFSET>,
            SetPopulationPolicy: SetPopulationPolicy::<Impl, IMPL_OFFSET>,
            InSyncPolicy: InSyncPolicy::<Impl, IMPL_OFFSET>,
            SetInSyncPolicy: SetInSyncPolicy::<Impl, IMPL_OFFSET>,
            HardlinkPolicy: HardlinkPolicy::<Impl, IMPL_OFFSET>,
            SetHardlinkPolicy: SetHardlinkPolicy::<Impl, IMPL_OFFSET>,
            ShowSiblingsAsGroup: ShowSiblingsAsGroup::<Impl, IMPL_OFFSET>,
            SetShowSiblingsAsGroup: SetShowSiblingsAsGroup::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            ProtectionMode: ProtectionMode::<Impl, IMPL_OFFSET>,
            SetProtectionMode: SetProtectionMode::<Impl, IMPL_OFFSET>,
            AllowPinning: AllowPinning::<Impl, IMPL_OFFSET>,
            SetAllowPinning: SetAllowPinning::<Impl, IMPL_OFFSET>,
            StorageProviderItemPropertyDefinitions: StorageProviderItemPropertyDefinitions::<Impl, IMPL_OFFSET>,
            RecycleBinUri: RecycleBinUri::<Impl, IMPL_OFFSET>,
            SetRecycleBinUri: SetRecycleBinUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderSyncRootInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderSyncRootInfo2Impl: Sized {
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetProviderId(&mut self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderSyncRootInfo2 {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSyncRootInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderSyncRootInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderSyncRootInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderSyncRootInfo2Vtbl {
        unsafe extern "system" fn ProviderId<Impl: IStorageProviderSyncRootInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderId<Impl: IStorageProviderSyncRootInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderSyncRootInfo2, BASE_OFFSET>(),
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            SetProviderId: SetProviderId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderSyncRootInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageProviderSyncRootInfo3Impl: Sized {
    fn FallbackFileTypeInfo(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderSyncRootInfo3 {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSyncRootInfo3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageProviderSyncRootInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderSyncRootInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderSyncRootInfo3Vtbl {
        unsafe extern "system" fn FallbackFileTypeInfo<Impl: IStorageProviderSyncRootInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FallbackFileTypeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderSyncRootInfo3, BASE_OFFSET>(),
            FallbackFileTypeInfo: FallbackFileTypeInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderSyncRootInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageProviderSyncRootManagerStaticsImpl: Sized {
    fn Register(&mut self, syncrootinformation: &::core::option::Option<StorageProviderSyncRootInfo>) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetSyncRootInformationForFolder(&mut self, folder: &::core::option::Option<super::IStorageFolder>) -> ::windows::core::Result<StorageProviderSyncRootInfo>;
    fn GetSyncRootInformationForId(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderSyncRootInfo>;
    fn GetCurrentSyncRoots(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProviderSyncRootManagerStatics {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSyncRootManagerStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageProviderSyncRootManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderSyncRootManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderSyncRootManagerStaticsVtbl {
        unsafe extern "system" fn Register<Impl: IStorageProviderSyncRootManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootinformation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register(&*(&syncrootinformation as *const <StorageProviderSyncRootInfo as ::windows::core::Abi>::Abi as *const <StorageProviderSyncRootInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unregister<Impl: IStorageProviderSyncRootManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSyncRootInformationForFolder<Impl: IStorageProviderSyncRootManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncRootInformationForFolder(&*(&folder as *const <super::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::IStorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncRootInformationForId<Impl: IStorageProviderSyncRootManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncRootInformationForId(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSyncRoots<Impl: IStorageProviderSyncRootManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSyncRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderSyncRootManagerStatics, BASE_OFFSET>(),
            Register: Register::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
            GetSyncRootInformationForFolder: GetSyncRootInformationForFolder::<Impl, IMPL_OFFSET>,
            GetSyncRootInformationForId: GetSyncRootInformationForId::<Impl, IMPL_OFFSET>,
            GetCurrentSyncRoots: GetCurrentSyncRoots::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderSyncRootManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderSyncRootManagerStatics2Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProviderSyncRootManagerStatics2 {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSyncRootManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderSyncRootManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderSyncRootManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderSyncRootManagerStatics2Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IStorageProviderSyncRootManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderSyncRootManagerStatics2, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderSyncRootManagerStatics2 as ::windows::core::Interface>::IID
    }
}
pub trait IStorageProviderUriSourceImpl: Sized {
    fn GetPathForContentUri(&mut self, contenturi: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetPathForContentUriResult>) -> ::windows::core::Result<()>;
    fn GetContentInfoForPath(&mut self, path: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetContentInfoForPathResult>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStorageProviderUriSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUriSource";
}
impl IStorageProviderUriSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderUriSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderUriSourceVtbl {
        unsafe extern "system" fn GetPathForContentUri<Impl: IStorageProviderUriSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenturi: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPathForContentUri(&*(&contenturi as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&result as *const <StorageProviderGetPathForContentUriResult as ::windows::core::Abi>::Abi as *const <StorageProviderGetPathForContentUriResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetContentInfoForPath<Impl: IStorageProviderUriSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentInfoForPath(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&result as *const <StorageProviderGetContentInfoForPathResult as ::windows::core::Abi>::Abi as *const <StorageProviderGetContentInfoForPathResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProviderUriSource, BASE_OFFSET>(),
            GetPathForContentUri: GetPathForContentUri::<Impl, IMPL_OFFSET>,
            GetContentInfoForPath: GetContentInfoForPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderUriSource as ::windows::core::Interface>::IID
    }
}
