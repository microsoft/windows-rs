#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IEmailAttachmentImpl: Sized {
    fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailAttachment";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IEmailAttachmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAttachmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailAttachmentVtbl {
        unsafe extern "system" fn FileName<Impl: IEmailAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileName<Impl: IEmailAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Data<Impl: IEmailAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IEmailAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailAttachment>, ::windows::core::GetTrustLevel, FileName::<Impl, IMPL_OFFSET>, SetFileName::<Impl, IMPL_OFFSET>, Data::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAttachment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailAttachment2Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentLocation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DownloadState(&self) -> ::windows::core::Result<EmailAttachmentDownloadState>;
    fn SetDownloadState(&self, value: EmailAttachmentDownloadState) -> ::windows::core::Result<()>;
    fn EstimatedDownloadSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SetEstimatedDownloadSizeInBytes(&self, value: u64) -> ::windows::core::Result<()>;
    fn IsFromBaseMessage(&self) -> ::windows::core::Result<bool>;
    fn IsInline(&self) -> ::windows::core::Result<bool>;
    fn SetIsInline(&self, value: bool) -> ::windows::core::Result<()>;
    fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMimeType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailAttachment2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailAttachment2";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailAttachment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAttachment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailAttachment2Vtbl {
        unsafe extern "system" fn Id<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentId<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentId<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentLocation<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentLocation<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentLocation(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadState<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailAttachmentDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadState<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailAttachmentDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDownloadState(value).into()
        }
        unsafe extern "system" fn EstimatedDownloadSizeInBytes<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EstimatedDownloadSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEstimatedDownloadSizeInBytes<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEstimatedDownloadSizeInBytes(value).into()
        }
        unsafe extern "system" fn IsFromBaseMessage<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFromBaseMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInline<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInline<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInline(value).into()
        }
        unsafe extern "system" fn MimeType<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMimeType<Impl: IEmailAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMimeType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailAttachment2>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            ContentId::<Impl, IMPL_OFFSET>,
            SetContentId::<Impl, IMPL_OFFSET>,
            ContentLocation::<Impl, IMPL_OFFSET>,
            SetContentLocation::<Impl, IMPL_OFFSET>,
            DownloadState::<Impl, IMPL_OFFSET>,
            SetDownloadState::<Impl, IMPL_OFFSET>,
            EstimatedDownloadSizeInBytes::<Impl, IMPL_OFFSET>,
            SetEstimatedDownloadSizeInBytes::<Impl, IMPL_OFFSET>,
            IsFromBaseMessage::<Impl, IMPL_OFFSET>,
            IsInline::<Impl, IMPL_OFFSET>,
            SetIsInline::<Impl, IMPL_OFFSET>,
            MimeType::<Impl, IMPL_OFFSET>,
            SetMimeType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAttachment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IEmailAttachmentFactoryImpl: Sized {
    fn Create(&self, filename: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<EmailAttachment>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailAttachmentFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailAttachmentFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IEmailAttachmentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAttachmentFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailAttachmentFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IEmailAttachmentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailAttachmentFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAttachmentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IEmailAttachmentFactory2Impl: Sized {
    fn Create(&self, filename: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<EmailAttachment>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailAttachmentFactory2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailAttachmentFactory2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IEmailAttachmentFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAttachmentFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailAttachmentFactory2Vtbl {
        unsafe extern "system" fn Create<Impl: IEmailAttachmentFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType),
                &*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailAttachmentFactory2>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAttachmentFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailConversationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FlagState(&self) -> ::windows::core::Result<EmailFlagState>;
    fn HasAttachment(&self) -> ::windows::core::Result<bool>;
    fn Importance(&self) -> ::windows::core::Result<EmailImportance>;
    fn LastEmailResponseKind(&self) -> ::windows::core::Result<EmailMessageResponseKind>;
    fn MessageCount(&self) -> ::windows::core::Result<u32>;
    fn MostRecentMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MostRecentMessageTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Preview(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LatestSender(&self) -> ::windows::core::Result<EmailRecipient>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnreadMessageCount(&self) -> ::windows::core::Result<u32>;
    fn FindMessagesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>;
    fn FindMessagesWithCountAsync(&self, count: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailConversation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailConversationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailConversationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailConversationVtbl {
        unsafe extern "system" fn Id<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MailboxId<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlagState<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailFlagState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlagState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasAttachment<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasAttachment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Importance<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailImportance) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Importance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastEmailResponseKind<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageResponseKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastEmailResponseKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MostRecentMessageId<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MostRecentMessageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MostRecentMessageTime<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MostRecentMessageTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preview<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Preview() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LatestSender<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LatestSender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadMessageCount<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnreadMessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindMessagesAsync<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindMessagesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindMessagesWithCountAsync<Impl: IEmailConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindMessagesWithCountAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailConversation>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            MailboxId::<Impl, IMPL_OFFSET>,
            FlagState::<Impl, IMPL_OFFSET>,
            HasAttachment::<Impl, IMPL_OFFSET>,
            Importance::<Impl, IMPL_OFFSET>,
            LastEmailResponseKind::<Impl, IMPL_OFFSET>,
            MessageCount::<Impl, IMPL_OFFSET>,
            MostRecentMessageId::<Impl, IMPL_OFFSET>,
            MostRecentMessageTime::<Impl, IMPL_OFFSET>,
            Preview::<Impl, IMPL_OFFSET>,
            LatestSender::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            UnreadMessageCount::<Impl, IMPL_OFFSET>,
            FindMessagesAsync::<Impl, IMPL_OFFSET>,
            FindMessagesWithCountAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailConversation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailConversationBatchImpl: Sized {
    fn Conversations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmailConversation>>;
    fn Status(&self) -> ::windows::core::Result<EmailBatchStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailConversationBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailConversationBatch";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailConversationBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailConversationBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailConversationBatchVtbl {
        unsafe extern "system" fn Conversations<Impl: IEmailConversationBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Conversations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IEmailConversationBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailBatchStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailConversationBatch>, ::windows::core::GetTrustLevel, Conversations::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailConversationBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailConversationReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversationBatch>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailConversationReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailConversationReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailConversationReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailConversationReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IEmailConversationReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailConversationReader>, ::windows::core::GetTrustLevel, ReadBatchAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailConversationReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailFolderImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<EmailSpecialFolderKind>;
    fn CreateFolderAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FindChildFoldersAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailFolder>>>;
    fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader>;
    fn GetConversationReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailConversationReader>;
    fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageCountsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailItemCounts>>;
    fn TryMoveAsync(&self, newparentfolder: &::core::option::Option<EmailFolder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryMoveWithNewNameAsync(&self, newparentfolder: &::core::option::Option<EmailFolder>, newfoldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SaveMessageAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailFolder {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailFolder";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailFolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailFolderVtbl {
        unsafe extern "system" fn Id<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteId<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MailboxId<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentFolderId<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentFolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSyncEnabled<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSyncEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSyncEnabled<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSyncEnabled(value).into()
        }
        unsafe extern "system" fn LastSuccessfulSyncTime<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSuccessfulSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastSuccessfulSyncTime<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastSuccessfulSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Kind<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailSpecialFolderKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindChildFoldersAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindChildFoldersAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationReader<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationReaderWithOptions<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationReaderWithOptions(&*(&options as *const <EmailQueryOptions as ::windows::core::Abi>::Abi as *const <EmailQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReader<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReaderWithOptions<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReaderWithOptions(&*(&options as *const <EmailQueryOptions as ::windows::core::Abi>::Abi as *const <EmailQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageCountsAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageCountsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveAsync(&*(&newparentfolder as *const <EmailFolder as ::windows::core::Abi>::Abi as *const <EmailFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveWithNewNameAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentfolder: ::windows::core::RawPtr, newfoldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveWithNewNameAsync(&*(&newparentfolder as *const <EmailFolder as ::windows::core::Abi>::Abi as *const <EmailFolder as ::windows::core::DefaultType>::DefaultType), &*(&newfoldername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySaveAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveMessageAsync<Impl: IEmailFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveMessageAsync(&*(&message as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailFolder>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId::<Impl, IMPL_OFFSET>,
            MailboxId::<Impl, IMPL_OFFSET>,
            ParentFolderId::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            IsSyncEnabled::<Impl, IMPL_OFFSET>,
            SetIsSyncEnabled::<Impl, IMPL_OFFSET>,
            LastSuccessfulSyncTime::<Impl, IMPL_OFFSET>,
            SetLastSuccessfulSyncTime::<Impl, IMPL_OFFSET>,
            Kind::<Impl, IMPL_OFFSET>,
            CreateFolderAsync::<Impl, IMPL_OFFSET>,
            DeleteAsync::<Impl, IMPL_OFFSET>,
            FindChildFoldersAsync::<Impl, IMPL_OFFSET>,
            GetConversationReader::<Impl, IMPL_OFFSET>,
            GetConversationReaderWithOptions::<Impl, IMPL_OFFSET>,
            GetMessageAsync::<Impl, IMPL_OFFSET>,
            GetMessageReader::<Impl, IMPL_OFFSET>,
            GetMessageReaderWithOptions::<Impl, IMPL_OFFSET>,
            GetMessageCountsAsync::<Impl, IMPL_OFFSET>,
            TryMoveAsync::<Impl, IMPL_OFFSET>,
            TryMoveWithNewNameAsync::<Impl, IMPL_OFFSET>,
            TrySaveAsync::<Impl, IMPL_OFFSET>,
            SaveMessageAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailFolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailIrmInfoImpl: Sized {
    fn CanEdit(&self) -> ::windows::core::Result<bool>;
    fn SetCanEdit(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanExtractData(&self) -> ::windows::core::Result<bool>;
    fn SetCanExtractData(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanForward(&self) -> ::windows::core::Result<bool>;
    fn SetCanForward(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanModifyRecipientsOnResponse(&self) -> ::windows::core::Result<bool>;
    fn SetCanModifyRecipientsOnResponse(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanPrintData(&self) -> ::windows::core::Result<bool>;
    fn SetCanPrintData(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanRemoveIrmOnResponse(&self) -> ::windows::core::Result<bool>;
    fn SetCanRemoveIrmOnResponse(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanReply(&self) -> ::windows::core::Result<bool>;
    fn SetCanReply(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanReplyAll(&self) -> ::windows::core::Result<bool>;
    fn SetCanReplyAll(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetExpirationDate(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn IsIrmOriginator(&self) -> ::windows::core::Result<bool>;
    fn SetIsIrmOriginator(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsProgramaticAccessAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetIsProgramaticAccessAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn Template(&self) -> ::windows::core::Result<EmailIrmTemplate>;
    fn SetTemplate(&self, value: &::core::option::Option<EmailIrmTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailIrmInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailIrmInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailIrmInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailIrmInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailIrmInfoVtbl {
        unsafe extern "system" fn CanEdit<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanEdit<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanEdit(value).into()
        }
        unsafe extern "system" fn CanExtractData<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanExtractData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanExtractData<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanExtractData(value).into()
        }
        unsafe extern "system" fn CanForward<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanForward() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanForward<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanForward(value).into()
        }
        unsafe extern "system" fn CanModifyRecipientsOnResponse<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanModifyRecipientsOnResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanModifyRecipientsOnResponse<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanModifyRecipientsOnResponse(value).into()
        }
        unsafe extern "system" fn CanPrintData<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPrintData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanPrintData<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanPrintData(value).into()
        }
        unsafe extern "system" fn CanRemoveIrmOnResponse<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRemoveIrmOnResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanRemoveIrmOnResponse<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanRemoveIrmOnResponse(value).into()
        }
        unsafe extern "system" fn CanReply<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanReply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanReply<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanReply(value).into()
        }
        unsafe extern "system" fn CanReplyAll<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanReplyAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanReplyAll<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanReplyAll(value).into()
        }
        unsafe extern "system" fn ExpirationDate<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationDate<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationDate(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsIrmOriginator<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIrmOriginator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsIrmOriginator<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsIrmOriginator(value).into()
        }
        unsafe extern "system" fn IsProgramaticAccessAllowed<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProgramaticAccessAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsProgramaticAccessAllowed<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsProgramaticAccessAllowed(value).into()
        }
        unsafe extern "system" fn Template<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemplate<Impl: IEmailIrmInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTemplate(&*(&value as *const <EmailIrmTemplate as ::windows::core::Abi>::Abi as *const <EmailIrmTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailIrmInfo>,
            ::windows::core::GetTrustLevel,
            CanEdit::<Impl, IMPL_OFFSET>,
            SetCanEdit::<Impl, IMPL_OFFSET>,
            CanExtractData::<Impl, IMPL_OFFSET>,
            SetCanExtractData::<Impl, IMPL_OFFSET>,
            CanForward::<Impl, IMPL_OFFSET>,
            SetCanForward::<Impl, IMPL_OFFSET>,
            CanModifyRecipientsOnResponse::<Impl, IMPL_OFFSET>,
            SetCanModifyRecipientsOnResponse::<Impl, IMPL_OFFSET>,
            CanPrintData::<Impl, IMPL_OFFSET>,
            SetCanPrintData::<Impl, IMPL_OFFSET>,
            CanRemoveIrmOnResponse::<Impl, IMPL_OFFSET>,
            SetCanRemoveIrmOnResponse::<Impl, IMPL_OFFSET>,
            CanReply::<Impl, IMPL_OFFSET>,
            SetCanReply::<Impl, IMPL_OFFSET>,
            CanReplyAll::<Impl, IMPL_OFFSET>,
            SetCanReplyAll::<Impl, IMPL_OFFSET>,
            ExpirationDate::<Impl, IMPL_OFFSET>,
            SetExpirationDate::<Impl, IMPL_OFFSET>,
            IsIrmOriginator::<Impl, IMPL_OFFSET>,
            SetIsIrmOriginator::<Impl, IMPL_OFFSET>,
            IsProgramaticAccessAllowed::<Impl, IMPL_OFFSET>,
            SetIsProgramaticAccessAllowed::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
            SetTemplate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailIrmInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailIrmInfoFactoryImpl: Sized {
    fn Create(&self, expiration: &super::super::Foundation::DateTime, irmtemplate: &::core::option::Option<EmailIrmTemplate>) -> ::windows::core::Result<EmailIrmInfo>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailIrmInfoFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailIrmInfoFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailIrmInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailIrmInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailIrmInfoFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IEmailIrmInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expiration: super::super::Foundation::DateTime, irmtemplate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&expiration as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&irmtemplate as *const <EmailIrmTemplate as ::windows::core::Abi>::Abi as *const <EmailIrmTemplate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailIrmInfoFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailIrmInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailIrmTemplateImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailIrmTemplate {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailIrmTemplate";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailIrmTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailIrmTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailIrmTemplateVtbl {
        unsafe extern "system" fn Id<Impl: IEmailIrmTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IEmailIrmTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IEmailIrmTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IEmailIrmTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IEmailIrmTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IEmailIrmTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailIrmTemplate>, ::windows::core::GetTrustLevel, Id::<Impl, IMPL_OFFSET>, SetId::<Impl, IMPL_OFFSET>, Description::<Impl, IMPL_OFFSET>, SetDescription::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailIrmTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailIrmTemplateFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, name: &::windows::core::HSTRING, description: &::windows::core::HSTRING) -> ::windows::core::Result<EmailIrmTemplate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailIrmTemplateFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailIrmTemplateFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailIrmTemplateFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailIrmTemplateFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailIrmTemplateFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IEmailIrmTemplateFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&description as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailIrmTemplateFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailIrmTemplateFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailItemCountsImpl: Sized {
    fn Flagged(&self) -> ::windows::core::Result<u32>;
    fn Important(&self) -> ::windows::core::Result<u32>;
    fn Total(&self) -> ::windows::core::Result<u32>;
    fn Unread(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailItemCounts {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailItemCounts";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailItemCountsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailItemCountsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailItemCountsVtbl {
        unsafe extern "system" fn Flagged<Impl: IEmailItemCountsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flagged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Important<Impl: IEmailItemCountsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Important() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Total<Impl: IEmailItemCountsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Total() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unread<Impl: IEmailItemCountsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailItemCounts>, ::windows::core::GetTrustLevel, Flagged::<Impl, IMPL_OFFSET>, Important::<Impl, IMPL_OFFSET>, Total::<Impl, IMPL_OFFSET>, Unread::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailItemCounts as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMailboxImpl: Sized {
    fn Capabilities(&self) -> ::windows::core::Result<EmailMailboxCapabilities>;
    fn ChangeTracker(&self) -> ::windows::core::Result<EmailMailboxChangeTracker>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOwnedByCurrentApp(&self) -> ::windows::core::Result<bool>;
    fn IsDataEncryptedUnderLock(&self) -> ::windows::core::Result<bool>;
    fn MailAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMailAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MailAddressAliases(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<EmailMailboxOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: EmailMailboxOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&self) -> ::windows::core::Result<EmailMailboxOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&self, value: EmailMailboxOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn Policies(&self) -> ::windows::core::Result<EmailMailboxPolicies>;
    fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SyncManager(&self) -> ::windows::core::Result<EmailMailboxSyncManager>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader>;
    fn GetConversationReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailConversationReader>;
    fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailMessageReader>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetConversationAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>>;
    fn GetFolderAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn GetSpecialFolderAsync(&self, foldertype: EmailSpecialFolderKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkMessageAsSeenAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkFolderAsSeenAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkMessageReadAsync(&self, messageid: &::windows::core::HSTRING, isread: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ChangeMessageFlagStateAsync(&self, messageid: &::windows::core::HSTRING, flagstate: EmailFlagState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TryMoveMessageAsync(&self, messageid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryMoveFolderAsync(&self, folderid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryMoveFolderWithNewNameAsync(&self, folderid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING, newfoldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DeleteMessageAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkFolderSyncEnabledAsync(&self, folderid: &::windows::core::HSTRING, issyncenabled: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SendMessageAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveDraftAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DownloadMessageAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DownloadAttachmentAsync(&self, attachmentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateResponseMessageAsync(&self, messageid: &::windows::core::HSTRING, responsetype: EmailMessageResponseKind, subject: &::windows::core::HSTRING, responseheadertype: EmailMessageBodyKind, responseheader: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn TryUpdateMeetingResponseAsync(&self, meeting: &::core::option::Option<EmailMessage>, response: EmailMeetingResponseType, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING, sendupdate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryForwardMeetingAsync(&self, meeting: &::core::option::Option<EmailMessage>, recipients: &::core::option::Option<super::super::Foundation::Collections::IIterable<EmailRecipient>>, subject: &::windows::core::HSTRING, forwardheadertype: EmailMessageBodyKind, forwardheader: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryProposeNewTimeForMeetingAsync(&self, meeting: &::core::option::Option<EmailMessage>, newstarttime: &super::super::Foundation::DateTime, newduration: &super::super::Foundation::TimeSpan, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn MailboxChanged(&self, phandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EmailMailbox, EmailMailboxChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMailboxChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SmartSendMessageAsync(&self, message: &::core::option::Option<EmailMessage>, smartsend: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrySetAutoReplySettingsAsync(&self, autoreplysettings: &::core::option::Option<EmailMailboxAutoReplySettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryGetAutoReplySettingsAsync(&self, requestedformat: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxAutoReplySettings>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailbox {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailbox";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMailboxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxVtbl {
        unsafe extern "system" fn Capabilities<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeTracker<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOwnedByCurrentApp<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOwnedByCurrentApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataEncryptedUnderLock<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataEncryptedUnderLock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MailAddress<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailAddress<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MailAddressAliases<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailAddressAliases() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherAppReadAccess<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAppReadAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherAppReadAccess<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMailboxOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppReadAccess(value).into()
        }
        unsafe extern "system" fn OtherAppWriteAccess<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxOtherAppWriteAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAppWriteAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherAppWriteAccess<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMailboxOtherAppWriteAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppWriteAccess(value).into()
        }
        unsafe extern "system" fn Policies<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Policies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceDisplayName<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncManager<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDataAccountId<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDataAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationReader<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationReaderWithOptions<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationReaderWithOptions(&*(&options as *const <EmailQueryOptions as ::windows::core::Abi>::Abi as *const <EmailQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReader<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReaderWithOptions<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReaderWithOptions(&*(&options as *const <EmailQueryOptions as ::windows::core::Abi>::Abi as *const <EmailQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecialFolderAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldertype: EmailSpecialFolderKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpecialFolderAsync(foldertype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MarkMessageAsSeenAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkMessageAsSeenAsync(&*(&messageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkFolderAsSeenAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkFolderAsSeenAsync(&*(&folderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkMessageReadAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, isread: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkMessageReadAsync(&*(&messageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), isread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeMessageFlagStateAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flagstate: EmailFlagState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeMessageFlagStateAsync(&*(&messageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), flagstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveMessageAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newparentfolderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveMessageAsync(&*(&messageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&newparentfolderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveFolderAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newparentfolderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveFolderAsync(&*(&folderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&newparentfolderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveFolderWithNewNameAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newparentfolderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newfoldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveFolderWithNewNameAsync(
                &*(&folderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&newparentfolderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&newfoldername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMessageAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMessageAsync(&*(&messageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkFolderSyncEnabledAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, issyncenabled: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkFolderSyncEnabledAsync(&*(&folderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), issyncenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessageAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessageAsync(&*(&message as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveDraftAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveDraftAsync(&*(&message as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMessageAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMessageAsync(&*(&messageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadAttachmentAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadAttachmentAsync(&*(&attachmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponseMessageAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, responsetype: EmailMessageResponseKind, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, responseheadertype: EmailMessageBodyKind, responseheader: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResponseMessageAsync(
                &*(&messageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                responsetype,
                &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                responseheadertype,
                &*(&responseheader as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateMeetingResponseAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, response: EmailMeetingResponseType, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sendupdate: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdateMeetingResponseAsync(
                &*(&meeting as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType),
                response,
                &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&comment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                sendupdate,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryForwardMeetingAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, recipients: ::windows::core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, forwardheadertype: EmailMessageBodyKind, forwardheader: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryForwardMeetingAsync(
                &*(&meeting as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType),
                &*(&recipients as *const <super::super::Foundation::Collections::IIterable<EmailRecipient> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<EmailRecipient> as ::windows::core::DefaultType>::DefaultType),
                &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                forwardheadertype,
                &*(&forwardheader as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&comment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryProposeNewTimeForMeetingAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meeting: ::windows::core::RawPtr, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryProposeNewTimeForMeetingAsync(
                &*(&meeting as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType),
                &*(&newstarttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&newduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&comment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MailboxChanged<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailboxChanged(&*(&phandler as *const <super::super::Foundation::TypedEventHandler<EmailMailbox, EmailMailboxChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EmailMailbox, EmailMailboxChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMailboxChanged<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMailboxChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SmartSendMessageAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, smartsend: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartSendMessageAsync(&*(&message as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType), smartsend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetAutoReplySettingsAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoreplysettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetAutoReplySettingsAsync(&*(&autoreplysettings as *const <EmailMailboxAutoReplySettings as ::windows::core::Abi>::Abi as *const <EmailMailboxAutoReplySettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetAutoReplySettingsAsync<Impl: IEmailMailboxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedformat: EmailMailboxAutoReplyMessageResponseKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetAutoReplySettingsAsync(requestedformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailbox>,
            ::windows::core::GetTrustLevel,
            Capabilities::<Impl, IMPL_OFFSET>,
            ChangeTracker::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            IsOwnedByCurrentApp::<Impl, IMPL_OFFSET>,
            IsDataEncryptedUnderLock::<Impl, IMPL_OFFSET>,
            MailAddress::<Impl, IMPL_OFFSET>,
            SetMailAddress::<Impl, IMPL_OFFSET>,
            MailAddressAliases::<Impl, IMPL_OFFSET>,
            OtherAppReadAccess::<Impl, IMPL_OFFSET>,
            SetOtherAppReadAccess::<Impl, IMPL_OFFSET>,
            OtherAppWriteAccess::<Impl, IMPL_OFFSET>,
            SetOtherAppWriteAccess::<Impl, IMPL_OFFSET>,
            Policies::<Impl, IMPL_OFFSET>,
            SourceDisplayName::<Impl, IMPL_OFFSET>,
            SyncManager::<Impl, IMPL_OFFSET>,
            UserDataAccountId::<Impl, IMPL_OFFSET>,
            GetConversationReader::<Impl, IMPL_OFFSET>,
            GetConversationReaderWithOptions::<Impl, IMPL_OFFSET>,
            GetMessageReader::<Impl, IMPL_OFFSET>,
            GetMessageReaderWithOptions::<Impl, IMPL_OFFSET>,
            DeleteAsync::<Impl, IMPL_OFFSET>,
            GetConversationAsync::<Impl, IMPL_OFFSET>,
            GetFolderAsync::<Impl, IMPL_OFFSET>,
            GetMessageAsync::<Impl, IMPL_OFFSET>,
            GetSpecialFolderAsync::<Impl, IMPL_OFFSET>,
            SaveAsync::<Impl, IMPL_OFFSET>,
            MarkMessageAsSeenAsync::<Impl, IMPL_OFFSET>,
            MarkFolderAsSeenAsync::<Impl, IMPL_OFFSET>,
            MarkMessageReadAsync::<Impl, IMPL_OFFSET>,
            ChangeMessageFlagStateAsync::<Impl, IMPL_OFFSET>,
            TryMoveMessageAsync::<Impl, IMPL_OFFSET>,
            TryMoveFolderAsync::<Impl, IMPL_OFFSET>,
            TryMoveFolderWithNewNameAsync::<Impl, IMPL_OFFSET>,
            DeleteMessageAsync::<Impl, IMPL_OFFSET>,
            MarkFolderSyncEnabledAsync::<Impl, IMPL_OFFSET>,
            SendMessageAsync::<Impl, IMPL_OFFSET>,
            SaveDraftAsync::<Impl, IMPL_OFFSET>,
            DownloadMessageAsync::<Impl, IMPL_OFFSET>,
            DownloadAttachmentAsync::<Impl, IMPL_OFFSET>,
            CreateResponseMessageAsync::<Impl, IMPL_OFFSET>,
            TryUpdateMeetingResponseAsync::<Impl, IMPL_OFFSET>,
            TryForwardMeetingAsync::<Impl, IMPL_OFFSET>,
            TryProposeNewTimeForMeetingAsync::<Impl, IMPL_OFFSET>,
            MailboxChanged::<Impl, IMPL_OFFSET>,
            RemoveMailboxChanged::<Impl, IMPL_OFFSET>,
            SmartSendMessageAsync::<Impl, IMPL_OFFSET>,
            TrySetAutoReplySettingsAsync::<Impl, IMPL_OFFSET>,
            TryGetAutoReplySettingsAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailbox as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMailbox2Impl: Sized + IEmailMailboxImpl {
    fn LinkedMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NetworkId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailbox2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailbox2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMailbox2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailbox2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailbox2Vtbl {
        unsafe extern "system" fn LinkedMailboxId<Impl: IEmailMailbox2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkedMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAccountId<Impl: IEmailMailbox2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkId<Impl: IEmailMailbox2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailbox2>, ::windows::core::GetTrustLevel, LinkedMailboxId::<Impl, IMPL_OFFSET>, NetworkAccountId::<Impl, IMPL_OFFSET>, NetworkId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailbox2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMailbox3Impl: Sized + IEmailMailboxImpl + IEmailMailbox2Impl {
    fn ResolveRecipientsAsync(&self, recipients: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailRecipientResolutionResult>>>;
    fn ValidateCertificatesAsync(&self, certificates: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailCertificateValidationStatus>>>;
    fn TryEmptyFolderAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxEmptyFolderStatus>>;
    fn TryCreateFolderAsync(&self, parentfolderid: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxCreateFolderResult>>;
    fn TryDeleteFolderAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxDeleteFolderStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailbox3 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailbox3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMailbox3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailbox3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailbox3Vtbl {
        unsafe extern "system" fn ResolveRecipientsAsync<Impl: IEmailMailbox3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recipients: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveRecipientsAsync(&*(&recipients as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateCertificatesAsync<Impl: IEmailMailbox3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateCertificatesAsync(&*(&certificates as *const <super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEmptyFolderAsync<Impl: IEmailMailbox3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryEmptyFolderAsync(&*(&folderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFolderAsync<Impl: IEmailMailbox3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentfolderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFolderAsync(&*(&parentfolderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDeleteFolderAsync<Impl: IEmailMailbox3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDeleteFolderAsync(&*(&folderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailbox3>,
            ::windows::core::GetTrustLevel,
            ResolveRecipientsAsync::<Impl, IMPL_OFFSET>,
            ValidateCertificatesAsync::<Impl, IMPL_OFFSET>,
            TryEmptyFolderAsync::<Impl, IMPL_OFFSET>,
            TryCreateFolderAsync::<Impl, IMPL_OFFSET>,
            TryDeleteFolderAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailbox3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMailbox4Impl: Sized {
    fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailbox4 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailbox4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMailbox4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailbox4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailbox4Vtbl {
        unsafe extern "system" fn RegisterSyncManagerAsync<Impl: IEmailMailbox4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterSyncManagerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailbox4>, ::windows::core::GetTrustLevel, RegisterSyncManagerAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailbox4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailbox5Impl: Sized {
    fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<EmailMailboxChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailbox5 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailbox5";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailbox5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailbox5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailbox5Vtbl {
        unsafe extern "system" fn GetChangeTracker<Impl: IEmailMailbox5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeTracker(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailbox5>, ::windows::core::GetTrustLevel, GetChangeTracker::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailbox5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxActionImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<EmailMailboxActionKind>;
    fn ChangeNumber(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxAction {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxAction";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxActionVtbl {
        unsafe extern "system" fn Kind<Impl: IEmailMailboxActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxActionKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeNumber<Impl: IEmailMailboxActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxAction>, ::windows::core::GetTrustLevel, Kind::<Impl, IMPL_OFFSET>, ChangeNumber::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxAutoReplyImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Response(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetResponse(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxAutoReply {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxAutoReply";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxAutoReplyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxAutoReplyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxAutoReplyVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IEmailMailboxAutoReplyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IEmailMailboxAutoReplyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn Response<Impl: IEmailMailboxAutoReplyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResponse<Impl: IEmailMailboxAutoReplyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResponse(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxAutoReply>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, IMPL_OFFSET>, SetIsEnabled::<Impl, IMPL_OFFSET>, Response::<Impl, IMPL_OFFSET>, SetResponse::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxAutoReply as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMailboxAutoReplySettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ResponseKind(&self) -> ::windows::core::Result<EmailMailboxAutoReplyMessageResponseKind>;
    fn SetResponseKind(&self, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetEndTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn InternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply>;
    fn KnownExternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply>;
    fn UnknownExternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailboxAutoReplySettings {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxAutoReplySettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMailboxAutoReplySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxAutoReplySettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxAutoReplySettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn ResponseKind<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResponseKind<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResponseKind(value).into()
        }
        unsafe extern "system" fn StartTime<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndTime<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndTime<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InternalReply<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalReply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnownExternalReply<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KnownExternalReply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnknownExternalReply<Impl: IEmailMailboxAutoReplySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnknownExternalReply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxAutoReplySettings>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled::<Impl, IMPL_OFFSET>,
            ResponseKind::<Impl, IMPL_OFFSET>,
            SetResponseKind::<Impl, IMPL_OFFSET>,
            StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime::<Impl, IMPL_OFFSET>,
            EndTime::<Impl, IMPL_OFFSET>,
            SetEndTime::<Impl, IMPL_OFFSET>,
            InternalReply::<Impl, IMPL_OFFSET>,
            KnownExternalReply::<Impl, IMPL_OFFSET>,
            UnknownExternalReply::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxAutoReplySettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCapabilitiesImpl: Sized {
    fn CanForwardMeetings(&self) -> ::windows::core::Result<bool>;
    fn CanGetAndSetExternalAutoReplies(&self) -> ::windows::core::Result<bool>;
    fn CanGetAndSetInternalAutoReplies(&self) -> ::windows::core::Result<bool>;
    fn CanUpdateMeetingResponses(&self) -> ::windows::core::Result<bool>;
    fn CanServerSearchFolders(&self) -> ::windows::core::Result<bool>;
    fn CanServerSearchMailbox(&self) -> ::windows::core::Result<bool>;
    fn CanProposeNewTimeForMeetings(&self) -> ::windows::core::Result<bool>;
    fn CanSmartSend(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxCapabilitiesVtbl {
        unsafe extern "system" fn CanForwardMeetings<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanForwardMeetings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGetAndSetExternalAutoReplies<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanGetAndSetExternalAutoReplies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGetAndSetInternalAutoReplies<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanGetAndSetInternalAutoReplies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanUpdateMeetingResponses<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanUpdateMeetingResponses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanServerSearchFolders<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanServerSearchFolders() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanServerSearchMailbox<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanServerSearchMailbox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanProposeNewTimeForMeetings<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanProposeNewTimeForMeetings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSmartSend<Impl: IEmailMailboxCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSmartSend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxCapabilities>,
            ::windows::core::GetTrustLevel,
            CanForwardMeetings::<Impl, IMPL_OFFSET>,
            CanGetAndSetExternalAutoReplies::<Impl, IMPL_OFFSET>,
            CanGetAndSetInternalAutoReplies::<Impl, IMPL_OFFSET>,
            CanUpdateMeetingResponses::<Impl, IMPL_OFFSET>,
            CanServerSearchFolders::<Impl, IMPL_OFFSET>,
            CanServerSearchMailbox::<Impl, IMPL_OFFSET>,
            CanProposeNewTimeForMeetings::<Impl, IMPL_OFFSET>,
            CanSmartSend::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCapabilities2Impl: Sized {
    fn CanResolveRecipients(&self) -> ::windows::core::Result<bool>;
    fn CanValidateCertificates(&self) -> ::windows::core::Result<bool>;
    fn CanEmptyFolder(&self) -> ::windows::core::Result<bool>;
    fn CanCreateFolder(&self) -> ::windows::core::Result<bool>;
    fn CanDeleteFolder(&self) -> ::windows::core::Result<bool>;
    fn CanMoveFolder(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxCapabilities2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxCapabilities2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxCapabilities2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxCapabilities2Vtbl {
        unsafe extern "system" fn CanResolveRecipients<Impl: IEmailMailboxCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanResolveRecipients() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanValidateCertificates<Impl: IEmailMailboxCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanValidateCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanEmptyFolder<Impl: IEmailMailboxCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanEmptyFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCreateFolder<Impl: IEmailMailboxCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCreateFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDeleteFolder<Impl: IEmailMailboxCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDeleteFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMoveFolder<Impl: IEmailMailboxCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMoveFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxCapabilities2>,
            ::windows::core::GetTrustLevel,
            CanResolveRecipients::<Impl, IMPL_OFFSET>,
            CanValidateCertificates::<Impl, IMPL_OFFSET>,
            CanEmptyFolder::<Impl, IMPL_OFFSET>,
            CanCreateFolder::<Impl, IMPL_OFFSET>,
            CanDeleteFolder::<Impl, IMPL_OFFSET>,
            CanMoveFolder::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxCapabilities2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCapabilities3Impl: Sized {
    fn SetCanForwardMeetings(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanGetAndSetExternalAutoReplies(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanGetAndSetInternalAutoReplies(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanServerSearchFolders(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanServerSearchMailbox(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanSmartSend(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanResolveRecipients(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanValidateCertificates(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanEmptyFolder(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanCreateFolder(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanDeleteFolder(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanMoveFolder(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxCapabilities3 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxCapabilities3";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxCapabilities3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxCapabilities3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxCapabilities3Vtbl {
        unsafe extern "system" fn SetCanForwardMeetings<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanForwardMeetings(value).into()
        }
        unsafe extern "system" fn SetCanGetAndSetExternalAutoReplies<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanGetAndSetExternalAutoReplies(value).into()
        }
        unsafe extern "system" fn SetCanGetAndSetInternalAutoReplies<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanGetAndSetInternalAutoReplies(value).into()
        }
        unsafe extern "system" fn SetCanUpdateMeetingResponses<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanUpdateMeetingResponses(value).into()
        }
        unsafe extern "system" fn SetCanServerSearchFolders<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanServerSearchFolders(value).into()
        }
        unsafe extern "system" fn SetCanServerSearchMailbox<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanServerSearchMailbox(value).into()
        }
        unsafe extern "system" fn SetCanProposeNewTimeForMeetings<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanProposeNewTimeForMeetings(value).into()
        }
        unsafe extern "system" fn SetCanSmartSend<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanSmartSend(value).into()
        }
        unsafe extern "system" fn SetCanResolveRecipients<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanResolveRecipients(value).into()
        }
        unsafe extern "system" fn SetCanValidateCertificates<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanValidateCertificates(value).into()
        }
        unsafe extern "system" fn SetCanEmptyFolder<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanEmptyFolder(value).into()
        }
        unsafe extern "system" fn SetCanCreateFolder<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanCreateFolder(value).into()
        }
        unsafe extern "system" fn SetCanDeleteFolder<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanDeleteFolder(value).into()
        }
        unsafe extern "system" fn SetCanMoveFolder<Impl: IEmailMailboxCapabilities3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanMoveFolder(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxCapabilities3>,
            ::windows::core::GetTrustLevel,
            SetCanForwardMeetings::<Impl, IMPL_OFFSET>,
            SetCanGetAndSetExternalAutoReplies::<Impl, IMPL_OFFSET>,
            SetCanGetAndSetInternalAutoReplies::<Impl, IMPL_OFFSET>,
            SetCanUpdateMeetingResponses::<Impl, IMPL_OFFSET>,
            SetCanServerSearchFolders::<Impl, IMPL_OFFSET>,
            SetCanServerSearchMailbox::<Impl, IMPL_OFFSET>,
            SetCanProposeNewTimeForMeetings::<Impl, IMPL_OFFSET>,
            SetCanSmartSend::<Impl, IMPL_OFFSET>,
            SetCanResolveRecipients::<Impl, IMPL_OFFSET>,
            SetCanValidateCertificates::<Impl, IMPL_OFFSET>,
            SetCanEmptyFolder::<Impl, IMPL_OFFSET>,
            SetCanCreateFolder::<Impl, IMPL_OFFSET>,
            SetCanDeleteFolder::<Impl, IMPL_OFFSET>,
            SetCanMoveFolder::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxCapabilities3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMailboxChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<EmailMailboxChangeType>;
    fn MailboxActions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailMailboxAction>>;
    fn Message(&self) -> ::windows::core::Result<EmailMessage>;
    fn Folder(&self) -> ::windows::core::Result<EmailFolder>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailboxChange {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxChange";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMailboxChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxChangeVtbl {
        unsafe extern "system" fn ChangeType<Impl: IEmailMailboxChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxChangeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MailboxActions<Impl: IEmailMailboxChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailboxActions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IEmailMailboxChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Folder<Impl: IEmailMailboxChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxChange>, ::windows::core::GetTrustLevel, ChangeType::<Impl, IMPL_OFFSET>, MailboxActions::<Impl, IMPL_OFFSET>, Message::<Impl, IMPL_OFFSET>, Folder::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMailboxChangeReaderImpl: Sized {
    fn AcceptChanges(&self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&self, lastchangetoacknowledge: &::core::option::Option<EmailMailboxChange>) -> ::windows::core::Result<()>;
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailboxChange>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailboxChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxChangeReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMailboxChangeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxChangeReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxChangeReaderVtbl {
        unsafe extern "system" fn AcceptChanges<Impl: IEmailMailboxChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChanges().into()
        }
        unsafe extern "system" fn AcceptChangesThrough<Impl: IEmailMailboxChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchangetoacknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChangesThrough(&*(&lastchangetoacknowledge as *const <EmailMailboxChange as ::windows::core::Abi>::Abi as *const <EmailMailboxChange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadBatchAsync<Impl: IEmailMailboxChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxChangeReader>, ::windows::core::GetTrustLevel, AcceptChanges::<Impl, IMPL_OFFSET>, AcceptChangesThrough::<Impl, IMPL_OFFSET>, ReadBatchAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxChangeReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangeTrackerImpl: Sized {
    fn IsTracking(&self) -> ::windows::core::Result<bool>;
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn GetChangeReader(&self) -> ::windows::core::Result<EmailMailboxChangeReader>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxChangeTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxChangeTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxChangeTrackerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxChangeTrackerVtbl {
        unsafe extern "system" fn IsTracking<Impl: IEmailMailboxChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTracking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IEmailMailboxChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn GetChangeReader<Impl: IEmailMailboxChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEmailMailboxChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxChangeTracker>, ::windows::core::GetTrustLevel, IsTracking::<Impl, IMPL_OFFSET>, Enable::<Impl, IMPL_OFFSET>, GetChangeReader::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxChangeTracker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxChangedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxChangedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxChangedDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxChangedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IEmailMailboxChangedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxChangedDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxChangedDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<EmailMailboxChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxChangedEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCreateFolderResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<EmailMailboxCreateFolderStatus>;
    fn Folder(&self) -> ::windows::core::Result<EmailFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxCreateFolderResult {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxCreateFolderResult";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxCreateFolderResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxCreateFolderResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxCreateFolderResultVtbl {
        unsafe extern "system" fn Status<Impl: IEmailMailboxCreateFolderResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxCreateFolderStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Folder<Impl: IEmailMailboxCreateFolderResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxCreateFolderResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, Folder::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxCreateFolderResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMailboxPoliciesImpl: Sized {
    fn AllowedSmimeEncryptionAlgorithmNegotiation(&self) -> ::windows::core::Result<EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation>;
    fn AllowSmimeSoftCertificates(&self) -> ::windows::core::Result<bool>;
    fn RequiredSmimeEncryptionAlgorithm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>;
    fn RequiredSmimeSigningAlgorithm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailboxPolicies {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxPolicies";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMailboxPoliciesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxPoliciesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxPoliciesVtbl {
        unsafe extern "system" fn AllowedSmimeEncryptionAlgorithmNegotiation<Impl: IEmailMailboxPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedSmimeEncryptionAlgorithmNegotiation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSmimeSoftCertificates<Impl: IEmailMailboxPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowSmimeSoftCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiredSmimeEncryptionAlgorithm<Impl: IEmailMailboxPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredSmimeEncryptionAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiredSmimeSigningAlgorithm<Impl: IEmailMailboxPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredSmimeSigningAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxPolicies>,
            ::windows::core::GetTrustLevel,
            AllowedSmimeEncryptionAlgorithmNegotiation::<Impl, IMPL_OFFSET>,
            AllowSmimeSoftCertificates::<Impl, IMPL_OFFSET>,
            RequiredSmimeEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            RequiredSmimeSigningAlgorithm::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxPolicies as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxPolicies2Impl: Sized {
    fn MustEncryptSmimeMessages(&self) -> ::windows::core::Result<bool>;
    fn MustSignSmimeMessages(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxPolicies2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxPolicies2";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxPolicies2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxPolicies2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxPolicies2Vtbl {
        unsafe extern "system" fn MustEncryptSmimeMessages<Impl: IEmailMailboxPolicies2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MustEncryptSmimeMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MustSignSmimeMessages<Impl: IEmailMailboxPolicies2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MustSignSmimeMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxPolicies2>, ::windows::core::GetTrustLevel, MustEncryptSmimeMessages::<Impl, IMPL_OFFSET>, MustSignSmimeMessages::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxPolicies2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMailboxPolicies3Impl: Sized {
    fn SetAllowedSmimeEncryptionAlgorithmNegotiation(&self, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows::core::Result<()>;
    fn SetAllowSmimeSoftCertificates(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetRequiredSmimeEncryptionAlgorithm(&self, value: &::core::option::Option<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>) -> ::windows::core::Result<()>;
    fn SetRequiredSmimeSigningAlgorithm(&self, value: &::core::option::Option<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>) -> ::windows::core::Result<()>;
    fn SetMustEncryptSmimeMessages(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetMustSignSmimeMessages(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailboxPolicies3 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxPolicies3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMailboxPolicies3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxPolicies3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxPolicies3Vtbl {
        unsafe extern "system" fn SetAllowedSmimeEncryptionAlgorithmNegotiation<Impl: IEmailMailboxPolicies3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedSmimeEncryptionAlgorithmNegotiation(value).into()
        }
        unsafe extern "system" fn SetAllowSmimeSoftCertificates<Impl: IEmailMailboxPolicies3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowSmimeSoftCertificates(value).into()
        }
        unsafe extern "system" fn SetRequiredSmimeEncryptionAlgorithm<Impl: IEmailMailboxPolicies3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiredSmimeEncryptionAlgorithm(&*(&value as *const <super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetRequiredSmimeSigningAlgorithm<Impl: IEmailMailboxPolicies3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiredSmimeSigningAlgorithm(&*(&value as *const <super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMustEncryptSmimeMessages<Impl: IEmailMailboxPolicies3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMustEncryptSmimeMessages(value).into()
        }
        unsafe extern "system" fn SetMustSignSmimeMessages<Impl: IEmailMailboxPolicies3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMustSignSmimeMessages(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxPolicies3>,
            ::windows::core::GetTrustLevel,
            SetAllowedSmimeEncryptionAlgorithmNegotiation::<Impl, IMPL_OFFSET>,
            SetAllowSmimeSoftCertificates::<Impl, IMPL_OFFSET>,
            SetRequiredSmimeEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetRequiredSmimeSigningAlgorithm::<Impl, IMPL_OFFSET>,
            SetMustEncryptSmimeMessages::<Impl, IMPL_OFFSET>,
            SetMustSignSmimeMessages::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxPolicies3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMailboxSyncManagerImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<EmailMailboxSyncStatus>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EmailMailboxSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailboxSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxSyncManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMailboxSyncManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxSyncManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxSyncManagerVtbl {
        unsafe extern "system" fn Status<Impl: IEmailMailboxSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxSyncStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LastSuccessfulSyncTime<Impl: IEmailMailboxSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSuccessfulSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastAttemptedSyncTime<Impl: IEmailMailboxSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastAttemptedSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncAsync<Impl: IEmailMailboxSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncStatusChanged<Impl: IEmailMailboxSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<EmailMailboxSyncManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EmailMailboxSyncManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSyncStatusChanged<Impl: IEmailMailboxSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSyncStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxSyncManager>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, IMPL_OFFSET>,
            LastSuccessfulSyncTime::<Impl, IMPL_OFFSET>,
            LastAttemptedSyncTime::<Impl, IMPL_OFFSET>,
            SyncAsync::<Impl, IMPL_OFFSET>,
            SyncStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveSyncStatusChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxSyncManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMailboxSyncManager2Impl: Sized {
    fn SetStatus(&self, value: EmailMailboxSyncStatus) -> ::windows::core::Result<()>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetLastAttemptedSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMailboxSyncManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMailboxSyncManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMailboxSyncManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxSyncManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMailboxSyncManager2Vtbl {
        unsafe extern "system" fn SetStatus<Impl: IEmailMailboxSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMailboxSyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn SetLastSuccessfulSyncTime<Impl: IEmailMailboxSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastSuccessfulSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetLastAttemptedSyncTime<Impl: IEmailMailboxSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastAttemptedSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxSyncManager2>, ::windows::core::GetTrustLevel, SetStatus::<Impl, IMPL_OFFSET>, SetLastSuccessfulSyncTime::<Impl, IMPL_OFFSET>, SetLastAttemptedSyncTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMailboxSyncManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IEmailManagerForUserImpl: Sized {
    fn ShowComposeNewEmailAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestStoreAsync(&self, accesstype: EmailStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailStore>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailManagerForUser";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IEmailManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailManagerForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailManagerForUserVtbl {
        unsafe extern "system" fn ShowComposeNewEmailAsync<Impl: IEmailManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowComposeNewEmailAsync(&*(&message as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStoreAsync<Impl: IEmailManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: EmailStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IEmailManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailManagerForUser>, ::windows::core::GetTrustLevel, ShowComposeNewEmailAsync::<Impl, IMPL_OFFSET>, RequestStoreAsync::<Impl, IMPL_OFFSET>, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailManagerStaticsImpl: Sized {
    fn ShowComposeNewEmailAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailManagerStaticsVtbl {
        unsafe extern "system" fn ShowComposeNewEmailAsync<Impl: IEmailManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowComposeNewEmailAsync(&*(&message as *const <EmailMessage as ::windows::core::Abi>::Abi as *const <EmailMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailManagerStatics>, ::windows::core::GetTrustLevel, ShowComposeNewEmailAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailManagerStatics2Impl: Sized {
    fn RequestStoreAsync(&self, accesstype: EmailStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailManagerStatics2Vtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IEmailManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: EmailStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailManagerStatics2>, ::windows::core::GetTrustLevel, RequestStoreAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IEmailManagerStatics3Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<EmailManagerForUser>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailManagerStatics3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IEmailManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailManagerStatics3Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IEmailManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailManagerStatics3>, ::windows::core::GetTrustLevel, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMeetingInfoImpl: Sized {
    fn AllowNewTimeProposal(&self) -> ::windows::core::Result<bool>;
    fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows::core::Result<()>;
    fn AppointmentRoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppointmentRoamingId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetAppointmentOriginalStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsAllDay(&self) -> ::windows::core::Result<bool>;
    fn SetIsAllDay(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsResponseRequested(&self) -> ::windows::core::Result<bool>;
    fn SetIsResponseRequested(&self, value: bool) -> ::windows::core::Result<()>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ProposedStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetProposedStartTime(&self, proposedstarttime: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ProposedDuration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetProposedDuration(&self, duration: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn RecurrenceStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetRecurrenceStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Recurrence(&self) -> ::windows::core::Result<super::Appointments::AppointmentRecurrence>;
    fn SetRecurrence(&self, value: &::core::option::Option<super::Appointments::AppointmentRecurrence>) -> ::windows::core::Result<()>;
    fn RemoteChangeNumber(&self) -> ::windows::core::Result<u64>;
    fn SetRemoteChangeNumber(&self, value: u64) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMeetingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMeetingInfo";
}
#[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMeetingInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMeetingInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMeetingInfoVtbl {
        unsafe extern "system" fn AllowNewTimeProposal<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowNewTimeProposal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowNewTimeProposal<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowNewTimeProposal(value).into()
        }
        unsafe extern "system" fn AppointmentRoamingId<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppointmentRoamingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppointmentRoamingId<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppointmentRoamingId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppointmentOriginalStartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppointmentOriginalStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppointmentOriginalStartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppointmentOriginalStartTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsAllDay<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAllDay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAllDay<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAllDay(value).into()
        }
        unsafe extern "system" fn IsResponseRequested<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResponseRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsResponseRequested<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsResponseRequested(value).into()
        }
        unsafe extern "system" fn Location<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProposedStartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProposedStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProposedStartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proposedstarttime: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProposedStartTime(&*(&proposedstarttime as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProposedDuration<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProposedDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProposedDuration<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProposedDuration(&*(&duration as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecurrenceStartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecurrenceStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecurrenceStartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecurrenceStartTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Recurrence<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recurrence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecurrence<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecurrence(&*(&value as *const <super::Appointments::AppointmentRecurrence as ::windows::core::Abi>::Abi as *const <super::Appointments::AppointmentRecurrence as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteChangeNumber<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteChangeNumber<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteChangeNumber(value).into()
        }
        unsafe extern "system" fn StartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IEmailMeetingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMeetingInfo>,
            ::windows::core::GetTrustLevel,
            AllowNewTimeProposal::<Impl, IMPL_OFFSET>,
            SetAllowNewTimeProposal::<Impl, IMPL_OFFSET>,
            AppointmentRoamingId::<Impl, IMPL_OFFSET>,
            SetAppointmentRoamingId::<Impl, IMPL_OFFSET>,
            AppointmentOriginalStartTime::<Impl, IMPL_OFFSET>,
            SetAppointmentOriginalStartTime::<Impl, IMPL_OFFSET>,
            Duration::<Impl, IMPL_OFFSET>,
            SetDuration::<Impl, IMPL_OFFSET>,
            IsAllDay::<Impl, IMPL_OFFSET>,
            SetIsAllDay::<Impl, IMPL_OFFSET>,
            IsResponseRequested::<Impl, IMPL_OFFSET>,
            SetIsResponseRequested::<Impl, IMPL_OFFSET>,
            Location::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            ProposedStartTime::<Impl, IMPL_OFFSET>,
            SetProposedStartTime::<Impl, IMPL_OFFSET>,
            ProposedDuration::<Impl, IMPL_OFFSET>,
            SetProposedDuration::<Impl, IMPL_OFFSET>,
            RecurrenceStartTime::<Impl, IMPL_OFFSET>,
            SetRecurrenceStartTime::<Impl, IMPL_OFFSET>,
            Recurrence::<Impl, IMPL_OFFSET>,
            SetRecurrence::<Impl, IMPL_OFFSET>,
            RemoteChangeNumber::<Impl, IMPL_OFFSET>,
            SetRemoteChangeNumber::<Impl, IMPL_OFFSET>,
            StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMeetingInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMeetingInfo2Impl: Sized {
    fn IsReportedOutOfDateByServer(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMeetingInfo2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMeetingInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMeetingInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMeetingInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMeetingInfo2Vtbl {
        unsafe extern "system" fn IsReportedOutOfDateByServer<Impl: IEmailMeetingInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReportedOutOfDateByServer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMeetingInfo2>, ::windows::core::GetTrustLevel, IsReportedOutOfDateByServer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMeetingInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMessageImpl: Sized {
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn CC(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn Bcc(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn Attachments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailAttachment>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMessage";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMessageVtbl {
        unsafe extern "system" fn Subject<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Body<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CC<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CC() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bcc<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bcc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attachments<Impl: IEmailMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attachments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMessage>,
            ::windows::core::GetTrustLevel,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            Body::<Impl, IMPL_OFFSET>,
            SetBody::<Impl, IMPL_OFFSET>,
            To::<Impl, IMPL_OFFSET>,
            CC::<Impl, IMPL_OFFSET>,
            Bcc::<Impl, IMPL_OFFSET>,
            Attachments::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IEmailMessage2Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConversationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowInternetImages(&self) -> ::windows::core::Result<bool>;
    fn SetAllowInternetImages(&self, value: bool) -> ::windows::core::Result<()>;
    fn ChangeNumber(&self) -> ::windows::core::Result<u64>;
    fn DownloadState(&self) -> ::windows::core::Result<EmailMessageDownloadState>;
    fn SetDownloadState(&self, value: EmailMessageDownloadState) -> ::windows::core::Result<()>;
    fn EstimatedDownloadSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetEstimatedDownloadSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn FlagState(&self) -> ::windows::core::Result<EmailFlagState>;
    fn SetFlagState(&self, value: EmailFlagState) -> ::windows::core::Result<()>;
    fn HasPartialBodies(&self) -> ::windows::core::Result<bool>;
    fn Importance(&self) -> ::windows::core::Result<EmailImportance>;
    fn SetImportance(&self, value: EmailImportance) -> ::windows::core::Result<()>;
    fn InResponseToMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IrmInfo(&self) -> ::windows::core::Result<EmailIrmInfo>;
    fn SetIrmInfo(&self, value: &::core::option::Option<EmailIrmInfo>) -> ::windows::core::Result<()>;
    fn IsDraftMessage(&self) -> ::windows::core::Result<bool>;
    fn IsRead(&self) -> ::windows::core::Result<bool>;
    fn SetIsRead(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeen(&self) -> ::windows::core::Result<bool>;
    fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsServerSearchMessage(&self) -> ::windows::core::Result<bool>;
    fn IsSmartSendable(&self) -> ::windows::core::Result<bool>;
    fn MessageClass(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessageClass(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NormalizedSubject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OriginalCodePage(&self) -> ::windows::core::Result<i32>;
    fn SetOriginalCodePage(&self, value: i32) -> ::windows::core::Result<()>;
    fn Preview(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreview(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LastResponseKind(&self) -> ::windows::core::Result<EmailMessageResponseKind>;
    fn SetLastResponseKind(&self, value: EmailMessageResponseKind) -> ::windows::core::Result<()>;
    fn Sender(&self) -> ::windows::core::Result<EmailRecipient>;
    fn SetSender(&self, value: &::core::option::Option<EmailRecipient>) -> ::windows::core::Result<()>;
    fn SentTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetSentTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn MeetingInfo(&self) -> ::windows::core::Result<EmailMeetingInfo>;
    fn SetMeetingInfo(&self, value: &::core::option::Option<EmailMeetingInfo>) -> ::windows::core::Result<()>;
    fn GetBodyStream(&self, r#type: EmailMessageBodyKind) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetBodyStream(&self, r#type: EmailMessageBodyKind, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMessage2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMessage2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IEmailMessage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMessage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMessage2Vtbl {
        unsafe extern "system" fn Id<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteId<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MailboxId<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConversationId<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConversationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderId<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInternetImages<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInternetImages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInternetImages<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInternetImages(value).into()
        }
        unsafe extern "system" fn ChangeNumber<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadState<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadState<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMessageDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDownloadState(value).into()
        }
        unsafe extern "system" fn EstimatedDownloadSizeInBytes<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EstimatedDownloadSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEstimatedDownloadSizeInBytes<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEstimatedDownloadSizeInBytes(value).into()
        }
        unsafe extern "system" fn FlagState<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailFlagState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlagState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlagState<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailFlagState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlagState(value).into()
        }
        unsafe extern "system" fn HasPartialBodies<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasPartialBodies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Importance<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailImportance) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Importance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImportance<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailImportance) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImportance(value).into()
        }
        unsafe extern "system" fn InResponseToMessageId<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InResponseToMessageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IrmInfo<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IrmInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIrmInfo<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIrmInfo(&*(&value as *const <EmailIrmInfo as ::windows::core::Abi>::Abi as *const <EmailIrmInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsDraftMessage<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDraftMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRead(value).into()
        }
        unsafe extern "system" fn IsSeen<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSeen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSeen<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSeen(value).into()
        }
        unsafe extern "system" fn IsServerSearchMessage<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsServerSearchMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSmartSendable<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSmartSendable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageClass<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageClass<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageClass(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NormalizedSubject<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedSubject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalCodePage<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalCodePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginalCodePage<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginalCodePage(value).into()
        }
        unsafe extern "system" fn Preview<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Preview() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreview<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreview(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LastResponseKind<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageResponseKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastResponseKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastResponseKind<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMessageResponseKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastResponseKind(value).into()
        }
        unsafe extern "system" fn Sender<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSender<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSender(&*(&value as *const <EmailRecipient as ::windows::core::Abi>::Abi as *const <EmailRecipient as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSentTime<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSentTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MeetingInfo<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeetingInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeetingInfo<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMeetingInfo(&*(&value as *const <EmailMeetingInfo as ::windows::core::Abi>::Abi as *const <EmailMeetingInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBodyStream<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: EmailMessageBodyKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBodyStream(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodyStream<Impl: IEmailMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: EmailMessageBodyKind, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBodyStream(r#type, &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMessage2>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId::<Impl, IMPL_OFFSET>,
            MailboxId::<Impl, IMPL_OFFSET>,
            ConversationId::<Impl, IMPL_OFFSET>,
            FolderId::<Impl, IMPL_OFFSET>,
            AllowInternetImages::<Impl, IMPL_OFFSET>,
            SetAllowInternetImages::<Impl, IMPL_OFFSET>,
            ChangeNumber::<Impl, IMPL_OFFSET>,
            DownloadState::<Impl, IMPL_OFFSET>,
            SetDownloadState::<Impl, IMPL_OFFSET>,
            EstimatedDownloadSizeInBytes::<Impl, IMPL_OFFSET>,
            SetEstimatedDownloadSizeInBytes::<Impl, IMPL_OFFSET>,
            FlagState::<Impl, IMPL_OFFSET>,
            SetFlagState::<Impl, IMPL_OFFSET>,
            HasPartialBodies::<Impl, IMPL_OFFSET>,
            Importance::<Impl, IMPL_OFFSET>,
            SetImportance::<Impl, IMPL_OFFSET>,
            InResponseToMessageId::<Impl, IMPL_OFFSET>,
            IrmInfo::<Impl, IMPL_OFFSET>,
            SetIrmInfo::<Impl, IMPL_OFFSET>,
            IsDraftMessage::<Impl, IMPL_OFFSET>,
            IsRead::<Impl, IMPL_OFFSET>,
            SetIsRead::<Impl, IMPL_OFFSET>,
            IsSeen::<Impl, IMPL_OFFSET>,
            SetIsSeen::<Impl, IMPL_OFFSET>,
            IsServerSearchMessage::<Impl, IMPL_OFFSET>,
            IsSmartSendable::<Impl, IMPL_OFFSET>,
            MessageClass::<Impl, IMPL_OFFSET>,
            SetMessageClass::<Impl, IMPL_OFFSET>,
            NormalizedSubject::<Impl, IMPL_OFFSET>,
            OriginalCodePage::<Impl, IMPL_OFFSET>,
            SetOriginalCodePage::<Impl, IMPL_OFFSET>,
            Preview::<Impl, IMPL_OFFSET>,
            SetPreview::<Impl, IMPL_OFFSET>,
            LastResponseKind::<Impl, IMPL_OFFSET>,
            SetLastResponseKind::<Impl, IMPL_OFFSET>,
            Sender::<Impl, IMPL_OFFSET>,
            SetSender::<Impl, IMPL_OFFSET>,
            SentTime::<Impl, IMPL_OFFSET>,
            SetSentTime::<Impl, IMPL_OFFSET>,
            MeetingInfo::<Impl, IMPL_OFFSET>,
            SetMeetingInfo::<Impl, IMPL_OFFSET>,
            GetBodyStream::<Impl, IMPL_OFFSET>,
            SetBodyStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMessage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IEmailMessage3Impl: Sized {
    fn SmimeData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetSmimeData(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn SmimeKind(&self) -> ::windows::core::Result<EmailMessageSmimeKind>;
    fn SetSmimeKind(&self, value: EmailMessageSmimeKind) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMessage3 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMessage3";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IEmailMessage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMessage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMessage3Vtbl {
        unsafe extern "system" fn SmimeData<Impl: IEmailMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmimeData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmimeData<Impl: IEmailMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmimeData(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SmimeKind<Impl: IEmailMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageSmimeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmimeKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmimeKind<Impl: IEmailMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailMessageSmimeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmimeKind(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMessage3>, ::windows::core::GetTrustLevel, SmimeData::<Impl, IMPL_OFFSET>, SetSmimeData::<Impl, IMPL_OFFSET>, SmimeKind::<Impl, IMPL_OFFSET>, SetSmimeKind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMessage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMessage4Impl: Sized {
    fn ReplyTo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn SentRepresenting(&self) -> ::windows::core::Result<EmailRecipient>;
    fn SetSentRepresenting(&self, value: &::core::option::Option<EmailRecipient>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMessage4 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMessage4";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMessage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMessage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMessage4Vtbl {
        unsafe extern "system" fn ReplyTo<Impl: IEmailMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplyTo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SentRepresenting<Impl: IEmailMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentRepresenting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSentRepresenting<Impl: IEmailMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSentRepresenting(&*(&value as *const <EmailRecipient as ::windows::core::Abi>::Abi as *const <EmailRecipient as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMessage4>, ::windows::core::GetTrustLevel, ReplyTo::<Impl, IMPL_OFFSET>, SentRepresenting::<Impl, IMPL_OFFSET>, SetSentRepresenting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMessage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailMessageBatchImpl: Sized {
    fn Messages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmailMessage>>;
    fn Status(&self) -> ::windows::core::Result<EmailBatchStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMessageBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMessageBatch";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailMessageBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMessageBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMessageBatchVtbl {
        unsafe extern "system" fn Messages<Impl: IEmailMessageBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Messages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IEmailMessageBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailBatchStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMessageBatch>, ::windows::core::GetTrustLevel, Messages::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMessageBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailMessageReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessageBatch>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailMessageReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailMessageReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMessageReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailMessageReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IEmailMessageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMessageReader>, ::windows::core::GetTrustLevel, ReadBatchAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailMessageReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailQueryOptionsImpl: Sized {
    fn TextSearch(&self) -> ::windows::core::Result<EmailQueryTextSearch>;
    fn SortDirection(&self) -> ::windows::core::Result<EmailQuerySortDirection>;
    fn SetSortDirection(&self, value: EmailQuerySortDirection) -> ::windows::core::Result<()>;
    fn SortProperty(&self) -> ::windows::core::Result<EmailQuerySortProperty>;
    fn SetSortProperty(&self, value: EmailQuerySortProperty) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<EmailQueryKind>;
    fn SetKind(&self, value: EmailQueryKind) -> ::windows::core::Result<()>;
    fn FolderIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailQueryOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailQueryOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailQueryOptionsVtbl {
        unsafe extern "system" fn TextSearch<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextSearch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SortDirection<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySortDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSortDirection<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailQuerySortDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSortDirection(value).into()
        }
        unsafe extern "system" fn SortProperty<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySortProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSortProperty<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailQuerySortProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSortProperty(value).into()
        }
        unsafe extern "system" fn Kind<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailQueryKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKind<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailQueryKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        unsafe extern "system" fn FolderIds<Impl: IEmailQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailQueryOptions>,
            ::windows::core::GetTrustLevel,
            TextSearch::<Impl, IMPL_OFFSET>,
            SortDirection::<Impl, IMPL_OFFSET>,
            SetSortDirection::<Impl, IMPL_OFFSET>,
            SortProperty::<Impl, IMPL_OFFSET>,
            SetSortProperty::<Impl, IMPL_OFFSET>,
            Kind::<Impl, IMPL_OFFSET>,
            SetKind::<Impl, IMPL_OFFSET>,
            FolderIds::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailQueryOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailQueryOptionsFactoryImpl: Sized {
    fn CreateWithText(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<EmailQueryOptions>;
    fn CreateWithTextAndFields(&self, text: &::windows::core::HSTRING, fields: EmailQuerySearchFields) -> ::windows::core::Result<EmailQueryOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailQueryOptionsFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailQueryOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailQueryOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailQueryOptionsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailQueryOptionsFactoryVtbl {
        unsafe extern "system" fn CreateWithText<Impl: IEmailQueryOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithText(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTextAndFields<Impl: IEmailQueryOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: EmailQuerySearchFields, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithTextAndFields(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), fields) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailQueryOptionsFactory>, ::windows::core::GetTrustLevel, CreateWithText::<Impl, IMPL_OFFSET>, CreateWithTextAndFields::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailQueryOptionsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailQueryTextSearchImpl: Sized {
    fn Fields(&self) -> ::windows::core::Result<EmailQuerySearchFields>;
    fn SetFields(&self, value: EmailQuerySearchFields) -> ::windows::core::Result<()>;
    fn SearchScope(&self) -> ::windows::core::Result<EmailQuerySearchScope>;
    fn SetSearchScope(&self, value: EmailQuerySearchScope) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailQueryTextSearch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailQueryTextSearch";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailQueryTextSearchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailQueryTextSearchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailQueryTextSearchVtbl {
        unsafe extern "system" fn Fields<Impl: IEmailQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySearchFields) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fields() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFields<Impl: IEmailQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailQuerySearchFields) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFields(value).into()
        }
        unsafe extern "system" fn SearchScope<Impl: IEmailQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchScope<Impl: IEmailQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailQuerySearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchScope(value).into()
        }
        unsafe extern "system" fn Text<Impl: IEmailQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: IEmailQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailQueryTextSearch>, ::windows::core::GetTrustLevel, Fields::<Impl, IMPL_OFFSET>, SetFields::<Impl, IMPL_OFFSET>, SearchScope::<Impl, IMPL_OFFSET>, SetSearchScope::<Impl, IMPL_OFFSET>, Text::<Impl, IMPL_OFFSET>, SetText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailQueryTextSearch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailRecipientImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailRecipient {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailRecipient";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailRecipientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailRecipientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailRecipientVtbl {
        unsafe extern "system" fn Name<Impl: IEmailRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IEmailRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Address<Impl: IEmailRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Impl: IEmailRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailRecipient>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, SetAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailRecipient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailRecipientFactoryImpl: Sized {
    fn Create(&self, address: &::windows::core::HSTRING) -> ::windows::core::Result<EmailRecipient>;
    fn CreateWithName(&self, address: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<EmailRecipient>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailRecipientFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailRecipientFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailRecipientFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailRecipientFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailRecipientFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IEmailRecipientFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&address as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithName<Impl: IEmailRecipientFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithName(&*(&address as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailRecipientFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailRecipientFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IEmailRecipientResolutionResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<EmailRecipientResolutionStatus>;
    fn PublicKeys(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailRecipientResolutionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailRecipientResolutionResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IEmailRecipientResolutionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailRecipientResolutionResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailRecipientResolutionResultVtbl {
        unsafe extern "system" fn Status<Impl: IEmailRecipientResolutionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EmailRecipientResolutionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PublicKeys<Impl: IEmailRecipientResolutionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailRecipientResolutionResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, PublicKeys::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailRecipientResolutionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IEmailRecipientResolutionResult2Impl: Sized {
    fn SetStatus(&self, value: EmailRecipientResolutionStatus) -> ::windows::core::Result<()>;
    fn SetPublicKeys(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailRecipientResolutionResult2 {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailRecipientResolutionResult2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IEmailRecipientResolutionResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailRecipientResolutionResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailRecipientResolutionResult2Vtbl {
        unsafe extern "system" fn SetStatus<Impl: IEmailRecipientResolutionResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EmailRecipientResolutionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn SetPublicKeys<Impl: IEmailRecipientResolutionResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublicKeys(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailRecipientResolutionResult2>, ::windows::core::GetTrustLevel, SetStatus::<Impl, IMPL_OFFSET>, SetPublicKeys::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailRecipientResolutionResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEmailStoreImpl: Sized {
    fn FindMailboxesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailbox>>>;
    fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader>;
    fn GetConversationReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailConversationReader>;
    fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMailboxAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>>;
    fn GetConversationAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>>;
    fn GetFolderAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn CreateMailboxAsync(&self, accountname: &::windows::core::HSTRING, accountaddress: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>>;
    fn CreateMailboxInAccountAsync(&self, accountname: &::windows::core::HSTRING, accountaddress: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailStore {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailStore";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEmailStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailStoreVtbl {
        unsafe extern "system" fn FindMailboxesAsync<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindMailboxesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationReader<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationReaderWithOptions<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationReaderWithOptions(&*(&options as *const <EmailQueryOptions as ::windows::core::Abi>::Abi as *const <EmailQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReader<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReaderWithOptions<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReaderWithOptions(&*(&options as *const <EmailQueryOptions as ::windows::core::Abi>::Abi as *const <EmailQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMailboxAsync<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMailboxAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationAsync<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageAsync<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMailboxAsync<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accountaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMailboxAsync(&*(&accountname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&accountaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMailboxInAccountAsync<Impl: IEmailStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accountaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMailboxInAccountAsync(
                &*(&accountname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&accountaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&userdataaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailStore>,
            ::windows::core::GetTrustLevel,
            FindMailboxesAsync::<Impl, IMPL_OFFSET>,
            GetConversationReader::<Impl, IMPL_OFFSET>,
            GetConversationReaderWithOptions::<Impl, IMPL_OFFSET>,
            GetMessageReader::<Impl, IMPL_OFFSET>,
            GetMessageReaderWithOptions::<Impl, IMPL_OFFSET>,
            GetMailboxAsync::<Impl, IMPL_OFFSET>,
            GetConversationAsync::<Impl, IMPL_OFFSET>,
            GetFolderAsync::<Impl, IMPL_OFFSET>,
            GetMessageAsync::<Impl, IMPL_OFFSET>,
            CreateMailboxAsync::<Impl, IMPL_OFFSET>,
            CreateMailboxInAccountAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailStoreNotificationTriggerDetailsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.IEmailStoreNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailStoreNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailStoreNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailStoreNotificationTriggerDetailsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailStoreNotificationTriggerDetails>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailStoreNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
