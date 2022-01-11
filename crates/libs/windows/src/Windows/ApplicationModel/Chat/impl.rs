#[cfg(feature = "implement_exclusive")]
pub trait IChatCapabilitiesImpl: Sized {
    fn IsOnline(&self) -> ::windows::core::Result<bool>;
    fn IsChatCapable(&self) -> ::windows::core::Result<bool>;
    fn IsFileTransferCapable(&self) -> ::windows::core::Result<bool>;
    fn IsGeoLocationPushCapable(&self) -> ::windows::core::Result<bool>;
    fn IsIntegratedMessagingCapable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IChatCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatCapabilitiesVtbl {
        unsafe extern "system" fn IsOnline<Impl: IChatCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOnline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsChatCapable<Impl: IChatCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsChatCapable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileTransferCapable<Impl: IChatCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFileTransferCapable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGeoLocationPushCapable<Impl: IChatCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGeoLocationPushCapable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIntegratedMessagingCapable<Impl: IChatCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIntegratedMessagingCapable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatCapabilities, BASE_OFFSET>(),
            IsOnline: IsOnline::<Impl, IMPL_OFFSET>,
            IsChatCapable: IsChatCapable::<Impl, IMPL_OFFSET>,
            IsFileTransferCapable: IsFileTransferCapable::<Impl, IMPL_OFFSET>,
            IsGeoLocationPushCapable: IsGeoLocationPushCapable::<Impl, IMPL_OFFSET>,
            IsIntegratedMessagingCapable: IsIntegratedMessagingCapable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatCapabilitiesManagerStaticsImpl: Sized {
    fn GetCachedCapabilitiesAsync(&self, address: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
    fn GetCapabilitiesFromNetworkAsync(&self, address: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatCapabilitiesManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatCapabilitiesManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatCapabilitiesManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatCapabilitiesManagerStaticsVtbl {
        unsafe extern "system" fn GetCachedCapabilitiesAsync<Impl: IChatCapabilitiesManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedCapabilitiesAsync(&*(&address as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilitiesFromNetworkAsync<Impl: IChatCapabilitiesManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilitiesFromNetworkAsync(&*(&address as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatCapabilitiesManagerStatics, BASE_OFFSET>(),
            GetCachedCapabilitiesAsync: GetCachedCapabilitiesAsync::<Impl, IMPL_OFFSET>,
            GetCapabilitiesFromNetworkAsync: GetCapabilitiesFromNetworkAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatCapabilitiesManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatCapabilitiesManagerStatics2Impl: Sized {
    fn GetCachedCapabilitiesForTransportAsync(&self, address: &::windows::core::HSTRING, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
    fn GetCapabilitiesFromNetworkForTransportAsync(&self, address: &::windows::core::HSTRING, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatCapabilitiesManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatCapabilitiesManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatCapabilitiesManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatCapabilitiesManagerStatics2Vtbl {
        unsafe extern "system" fn GetCachedCapabilitiesForTransportAsync<Impl: IChatCapabilitiesManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedCapabilitiesForTransportAsync(&*(&address as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&transportid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilitiesFromNetworkForTransportAsync<Impl: IChatCapabilitiesManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilitiesFromNetworkForTransportAsync(&*(&address as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&transportid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatCapabilitiesManagerStatics2, BASE_OFFSET>(),
            GetCachedCapabilitiesForTransportAsync: GetCachedCapabilitiesForTransportAsync::<Impl, IMPL_OFFSET>,
            GetCapabilitiesFromNetworkForTransportAsync: GetCapabilitiesFromNetworkForTransportAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatCapabilitiesManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatConversationImpl: Sized {
    fn HasUnreadMessages(&self) -> ::windows::core::Result<bool>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsConversationMuted(&self) -> ::windows::core::Result<bool>;
    fn SetIsConversationMuted(&self, value: bool) -> ::windows::core::Result<()>;
    fn MostRecentMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMessageReader(&self) -> ::windows::core::Result<ChatMessageReader>;
    fn MarkAllMessagesAsReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkMessagesAsReadAsync(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn NotifyLocalParticipantComposing(&self, transportid: &::windows::core::HSTRING, participantaddress: &::windows::core::HSTRING, iscomposing: bool) -> ::windows::core::Result<()>;
    fn NotifyRemoteParticipantComposing(&self, transportid: &::windows::core::HSTRING, participantaddress: &::windows::core::HSTRING, iscomposing: bool) -> ::windows::core::Result<()>;
    fn RemoteParticipantComposingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteParticipantComposingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatConversation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatConversationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatConversationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatConversationVtbl {
        unsafe extern "system" fn HasUnreadMessages<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasUnreadMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Subject<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSubject<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsConversationMuted<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConversationMuted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsConversationMuted<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsConversationMuted(value).into()
        }
        unsafe extern "system" fn MostRecentMessageId<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Participants<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participants() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThreadingInfo<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThreadingInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMessageReader<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MarkAllMessagesAsReadAsync<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkAllMessagesAsReadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkMessagesAsReadAsync<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkMessagesAsReadAsync(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NotifyLocalParticipantComposing<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, participantaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iscomposing: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyLocalParticipantComposing(&*(&transportid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&participantaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), iscomposing).into()
        }
        unsafe extern "system" fn NotifyRemoteParticipantComposing<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, participantaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iscomposing: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyRemoteParticipantComposing(&*(&transportid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&participantaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), iscomposing).into()
        }
        unsafe extern "system" fn RemoteParticipantComposingChanged<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteParticipantComposingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoteParticipantComposingChanged<Impl: IChatConversationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoteParticipantComposingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatConversation, BASE_OFFSET>(),
            HasUnreadMessages: HasUnreadMessages::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            IsConversationMuted: IsConversationMuted::<Impl, IMPL_OFFSET>,
            SetIsConversationMuted: SetIsConversationMuted::<Impl, IMPL_OFFSET>,
            MostRecentMessageId: MostRecentMessageId::<Impl, IMPL_OFFSET>,
            Participants: Participants::<Impl, IMPL_OFFSET>,
            ThreadingInfo: ThreadingInfo::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            GetMessageReader: GetMessageReader::<Impl, IMPL_OFFSET>,
            MarkAllMessagesAsReadAsync: MarkAllMessagesAsReadAsync::<Impl, IMPL_OFFSET>,
            MarkMessagesAsReadAsync: MarkMessagesAsReadAsync::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            NotifyLocalParticipantComposing: NotifyLocalParticipantComposing::<Impl, IMPL_OFFSET>,
            NotifyRemoteParticipantComposing: NotifyRemoteParticipantComposing::<Impl, IMPL_OFFSET>,
            RemoteParticipantComposingChanged: RemoteParticipantComposingChanged::<Impl, IMPL_OFFSET>,
            RemoveRemoteParticipantComposingChanged: RemoveRemoteParticipantComposingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatConversation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatConversation2Impl: Sized {
    fn CanModifyParticipants(&self) -> ::windows::core::Result<bool>;
    fn SetCanModifyParticipants(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatConversation2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatConversation2";
}
#[cfg(feature = "implement_exclusive")]
impl IChatConversation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatConversation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatConversation2Vtbl {
        unsafe extern "system" fn CanModifyParticipants<Impl: IChatConversation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanModifyParticipants() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanModifyParticipants<Impl: IChatConversation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanModifyParticipants(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatConversation2, BASE_OFFSET>(),
            CanModifyParticipants: CanModifyParticipants::<Impl, IMPL_OFFSET>,
            SetCanModifyParticipants: SetCanModifyParticipants::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatConversation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatConversationReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>;
    fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatConversationReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatConversationReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatConversationReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatConversationReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IChatConversationReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadBatchWithCountAsync<Impl: IChatConversationReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchWithCountAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatConversationReader, BASE_OFFSET>(),
            ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET>,
            ReadBatchWithCountAsync: ReadBatchWithCountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatConversationReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatConversationThreadingInfoImpl: Sized {
    fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Custom(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCustom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ConversationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetConversationId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Kind(&self) -> ::windows::core::Result<ChatConversationThreadingKind>;
    fn SetKind(&self, value: ChatConversationThreadingKind) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatConversationThreadingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatConversationThreadingInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatConversationThreadingInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatConversationThreadingInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatConversationThreadingInfoVtbl {
        unsafe extern "system" fn ContactId<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContactId<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Custom<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Custom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustom<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustom(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConversationId<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetConversationId<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConversationId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Participants<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participants() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatConversationThreadingKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKind<Impl: IChatConversationThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ChatConversationThreadingKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatConversationThreadingInfo, BASE_OFFSET>(),
            ContactId: ContactId::<Impl, IMPL_OFFSET>,
            SetContactId: SetContactId::<Impl, IMPL_OFFSET>,
            Custom: Custom::<Impl, IMPL_OFFSET>,
            SetCustom: SetCustom::<Impl, IMPL_OFFSET>,
            ConversationId: ConversationId::<Impl, IMPL_OFFSET>,
            SetConversationId: SetConversationId::<Impl, IMPL_OFFSET>,
            Participants: Participants::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            SetKind: SetKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatConversationThreadingInfo as ::windows::core::Interface>::IID
    }
}
pub trait IChatItemImpl: Sized {
    fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind>;
}
impl ::windows::core::RuntimeName for IChatItem {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatItem";
}
impl IChatItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatItemVtbl {
        unsafe extern "system" fn ItemKind<Impl: IChatItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatItemKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IChatItem, BASE_OFFSET>(), ItemKind: ItemKind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessageImpl: Sized {
    fn Attachments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatMessageAttachment>>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsForwardingDisabled(&self) -> ::windows::core::Result<bool>;
    fn IsIncoming(&self) -> ::windows::core::Result<bool>;
    fn IsRead(&self) -> ::windows::core::Result<bool>;
    fn LocalTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn NetworkTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Recipients(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RecipientSendStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ChatMessageStatus>>;
    fn Status(&self) -> ::windows::core::Result<ChatMessageStatus>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTransportId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessage";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageVtbl {
        unsafe extern "system" fn Attachments<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Body<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBody<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn From<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsForwardingDisabled<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsForwardingDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIncoming<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalTimestamp<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkTimestamp<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipients() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecipientSendStatuses<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecipientSendStatuses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Subject<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransportFriendlyName<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportId<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportId<Impl: IChatMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessage, BASE_OFFSET>(),
            Attachments: Attachments::<Impl, IMPL_OFFSET>,
            Body: Body::<Impl, IMPL_OFFSET>,
            SetBody: SetBody::<Impl, IMPL_OFFSET>,
            From: From::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            IsForwardingDisabled: IsForwardingDisabled::<Impl, IMPL_OFFSET>,
            IsIncoming: IsIncoming::<Impl, IMPL_OFFSET>,
            IsRead: IsRead::<Impl, IMPL_OFFSET>,
            LocalTimestamp: LocalTimestamp::<Impl, IMPL_OFFSET>,
            NetworkTimestamp: NetworkTimestamp::<Impl, IMPL_OFFSET>,
            Recipients: Recipients::<Impl, IMPL_OFFSET>,
            RecipientSendStatuses: RecipientSendStatuses::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            TransportFriendlyName: TransportFriendlyName::<Impl, IMPL_OFFSET>,
            TransportId: TransportId::<Impl, IMPL_OFFSET>,
            SetTransportId: SetTransportId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessage2Impl: Sized + IChatMessageImpl + IChatMessage3Impl {
    fn EstimatedDownloadSize(&self) -> ::windows::core::Result<u64>;
    fn SetEstimatedDownloadSize(&self, value: u64) -> ::windows::core::Result<()>;
    fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsAutoReply(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutoReply(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetIsForwardingDisabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsReplyDisabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsIncoming(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetIsRead(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeen(&self) -> ::windows::core::Result<bool>;
    fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSimMessage(&self) -> ::windows::core::Result<bool>;
    fn SetLocalTimestamp(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn MessageKind(&self) -> ::windows::core::Result<ChatMessageKind>;
    fn SetMessageKind(&self, value: ChatMessageKind) -> ::windows::core::Result<()>;
    fn MessageOperatorKind(&self) -> ::windows::core::Result<ChatMessageOperatorKind>;
    fn SetMessageOperatorKind(&self, value: ChatMessageOperatorKind) -> ::windows::core::Result<()>;
    fn SetNetworkTimestamp(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn IsReceivedDuringQuietHours(&self) -> ::windows::core::Result<bool>;
    fn SetIsReceivedDuringQuietHours(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetStatus(&self, value: ChatMessageStatus) -> ::windows::core::Result<()>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ShouldSuppressNotification(&self) -> ::windows::core::Result<bool>;
    fn SetShouldSuppressNotification(&self, value: bool) -> ::windows::core::Result<()>;
    fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo>;
    fn SetThreadingInfo(&self, value: &::core::option::Option<ChatConversationThreadingInfo>) -> ::windows::core::Result<()>;
    fn RecipientsDeliveryInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessage2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessage2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessage2Vtbl {
        unsafe extern "system" fn EstimatedDownloadSize<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EstimatedDownloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEstimatedDownloadSize<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEstimatedDownloadSize(value).into()
        }
        unsafe extern "system" fn SetFrom<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsAutoReply<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAutoReply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAutoReply<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAutoReply(value).into()
        }
        unsafe extern "system" fn SetIsForwardingDisabled<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsForwardingDisabled(value).into()
        }
        unsafe extern "system" fn IsReplyDisabled<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReplyDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsIncoming<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsIncoming(value).into()
        }
        unsafe extern "system" fn SetIsRead<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRead(value).into()
        }
        unsafe extern "system" fn IsSeen<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSeen<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSeen(value).into()
        }
        unsafe extern "system" fn IsSimMessage<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSimMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalTimestamp<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalTimestamp(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MessageKind<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageKind<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ChatMessageKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageKind(value).into()
        }
        unsafe extern "system" fn MessageOperatorKind<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageOperatorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageOperatorKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageOperatorKind<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ChatMessageOperatorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageOperatorKind(value).into()
        }
        unsafe extern "system" fn SetNetworkTimestamp<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkTimestamp(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsReceivedDuringQuietHours<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReceivedDuringQuietHours() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsReceivedDuringQuietHours<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsReceivedDuringQuietHours(value).into()
        }
        unsafe extern "system" fn SetRemoteId<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStatus<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ChatMessageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn SetSubject<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShouldSuppressNotification<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldSuppressNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldSuppressNotification<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldSuppressNotification(value).into()
        }
        unsafe extern "system" fn ThreadingInfo<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThreadingInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThreadingInfo<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThreadingInfo(&*(&value as *const <ChatConversationThreadingInfo as ::windows::core::Abi>::Abi as *const <ChatConversationThreadingInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecipientsDeliveryInfos<Impl: IChatMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecipientsDeliveryInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessage2, BASE_OFFSET>(),
            EstimatedDownloadSize: EstimatedDownloadSize::<Impl, IMPL_OFFSET>,
            SetEstimatedDownloadSize: SetEstimatedDownloadSize::<Impl, IMPL_OFFSET>,
            SetFrom: SetFrom::<Impl, IMPL_OFFSET>,
            IsAutoReply: IsAutoReply::<Impl, IMPL_OFFSET>,
            SetIsAutoReply: SetIsAutoReply::<Impl, IMPL_OFFSET>,
            SetIsForwardingDisabled: SetIsForwardingDisabled::<Impl, IMPL_OFFSET>,
            IsReplyDisabled: IsReplyDisabled::<Impl, IMPL_OFFSET>,
            SetIsIncoming: SetIsIncoming::<Impl, IMPL_OFFSET>,
            SetIsRead: SetIsRead::<Impl, IMPL_OFFSET>,
            IsSeen: IsSeen::<Impl, IMPL_OFFSET>,
            SetIsSeen: SetIsSeen::<Impl, IMPL_OFFSET>,
            IsSimMessage: IsSimMessage::<Impl, IMPL_OFFSET>,
            SetLocalTimestamp: SetLocalTimestamp::<Impl, IMPL_OFFSET>,
            MessageKind: MessageKind::<Impl, IMPL_OFFSET>,
            SetMessageKind: SetMessageKind::<Impl, IMPL_OFFSET>,
            MessageOperatorKind: MessageOperatorKind::<Impl, IMPL_OFFSET>,
            SetMessageOperatorKind: SetMessageOperatorKind::<Impl, IMPL_OFFSET>,
            SetNetworkTimestamp: SetNetworkTimestamp::<Impl, IMPL_OFFSET>,
            IsReceivedDuringQuietHours: IsReceivedDuringQuietHours::<Impl, IMPL_OFFSET>,
            SetIsReceivedDuringQuietHours: SetIsReceivedDuringQuietHours::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            ShouldSuppressNotification: ShouldSuppressNotification::<Impl, IMPL_OFFSET>,
            SetShouldSuppressNotification: SetShouldSuppressNotification::<Impl, IMPL_OFFSET>,
            ThreadingInfo: ThreadingInfo::<Impl, IMPL_OFFSET>,
            SetThreadingInfo: SetThreadingInfo::<Impl, IMPL_OFFSET>,
            RecipientsDeliveryInfos: RecipientsDeliveryInfos::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessage3Impl: Sized + IChatMessageImpl {
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessage3 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessage3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessage3Vtbl {
        unsafe extern "system" fn RemoteId<Impl: IChatMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessage3, BASE_OFFSET>(), RemoteId: RemoteId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessage4Impl: Sized + IChatMessageImpl {
    fn SyncId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSyncId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessage4 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessage4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessage4Vtbl {
        unsafe extern "system" fn SyncId<Impl: IChatMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncId<Impl: IChatMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessage4, BASE_OFFSET>(),
            SyncId: SyncId::<Impl, IMPL_OFFSET>,
            SetSyncId: SetSyncId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IChatMessageAttachmentImpl: Sized {
    fn DataStreamReference(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetDataStreamReference(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn GroupId(&self) -> ::windows::core::Result<u32>;
    fn SetGroupId(&self, value: u32) -> ::windows::core::Result<()>;
    fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMimeType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageAttachment";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IChatMessageAttachmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageAttachmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageAttachmentVtbl {
        unsafe extern "system" fn DataStreamReference<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataStreamReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataStreamReference<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataStreamReference(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GroupId<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupId<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupId(value).into()
        }
        unsafe extern "system" fn MimeType<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMimeType<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMimeType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Text<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetText<Impl: IChatMessageAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageAttachment, BASE_OFFSET>(),
            DataStreamReference: DataStreamReference::<Impl, IMPL_OFFSET>,
            SetDataStreamReference: SetDataStreamReference::<Impl, IMPL_OFFSET>,
            GroupId: GroupId::<Impl, IMPL_OFFSET>,
            SetGroupId: SetGroupId::<Impl, IMPL_OFFSET>,
            MimeType: MimeType::<Impl, IMPL_OFFSET>,
            SetMimeType: SetMimeType::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageAttachment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IChatMessageAttachment2Impl: Sized + IChatMessageAttachmentImpl {
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn TransferProgress(&self) -> ::windows::core::Result<f64>;
    fn SetTransferProgress(&self, value: f64) -> ::windows::core::Result<()>;
    fn OriginalFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOriginalFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageAttachment2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageAttachment2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IChatMessageAttachment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageAttachment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageAttachment2Vtbl {
        unsafe extern "system" fn Thumbnail<Impl: IChatMessageAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: IChatMessageAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferProgress<Impl: IChatMessageAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransferProgress<Impl: IChatMessageAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferProgress(value).into()
        }
        unsafe extern "system" fn OriginalFileName<Impl: IChatMessageAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginalFileName<Impl: IChatMessageAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginalFileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageAttachment2, BASE_OFFSET>(),
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            TransferProgress: TransferProgress::<Impl, IMPL_OFFSET>,
            SetTransferProgress: SetTransferProgress::<Impl, IMPL_OFFSET>,
            OriginalFileName: OriginalFileName::<Impl, IMPL_OFFSET>,
            SetOriginalFileName: SetOriginalFileName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageAttachment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IChatMessageAttachmentFactoryImpl: Sized {
    fn CreateChatMessageAttachment(&self, mimetype: &::windows::core::HSTRING, datastreamreference: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<ChatMessageAttachment>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageAttachmentFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageAttachmentFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IChatMessageAttachmentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageAttachmentFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageAttachmentFactoryVtbl {
        unsafe extern "system" fn CreateChatMessageAttachment<Impl: IChatMessageAttachmentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datastreamreference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateChatMessageAttachment(&*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&datastreamreference as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageAttachmentFactory, BASE_OFFSET>(),
            CreateChatMessageAttachment: CreateChatMessageAttachment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageAttachmentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageBlockingStaticImpl: Sized {
    fn MarkMessageAsBlockedAsync(&self, localchatmessageid: &::windows::core::HSTRING, blocked: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageBlockingStatic {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageBlockingStatic";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageBlockingStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageBlockingStaticImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageBlockingStaticVtbl {
        unsafe extern "system" fn MarkMessageAsBlockedAsync<Impl: IChatMessageBlockingStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, blocked: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkMessageAsBlockedAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), blocked) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageBlockingStatic, BASE_OFFSET>(),
            MarkMessageAsBlockedAsync: MarkMessageAsBlockedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageBlockingStatic as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<ChatMessageChangeType>;
    fn Message(&self) -> ::windows::core::Result<ChatMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageChange {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageChange";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageChangeVtbl {
        unsafe extern "system" fn ChangeType<Impl: IChatMessageChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageChangeType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Message<Impl: IChatMessageChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageChange, BASE_OFFSET>(),
            ChangeType: ChangeType::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessageChangeReaderImpl: Sized {
    fn AcceptChanges(&self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&self, lastchangetoacknowledge: &::core::option::Option<ChatMessageChange>) -> ::windows::core::Result<()>;
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageChangeReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessageChangeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageChangeReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageChangeReaderVtbl {
        unsafe extern "system" fn AcceptChanges<Impl: IChatMessageChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChanges().into()
        }
        unsafe extern "system" fn AcceptChangesThrough<Impl: IChatMessageChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchangetoacknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChangesThrough(&*(&lastchangetoacknowledge as *const <ChatMessageChange as ::windows::core::Abi>::Abi as *const <ChatMessageChange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadBatchAsync<Impl: IChatMessageChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageChangeReader, BASE_OFFSET>(),
            AcceptChanges: AcceptChanges::<Impl, IMPL_OFFSET>,
            AcceptChangesThrough: AcceptChangesThrough::<Impl, IMPL_OFFSET>,
            ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageChangeReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangeTrackerImpl: Sized {
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn GetChangeReader(&self) -> ::windows::core::Result<ChatMessageChangeReader>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageChangeTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageChangeTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageChangeTrackerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageChangeTrackerVtbl {
        unsafe extern "system" fn Enable<Impl: IChatMessageChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn GetChangeReader<Impl: IChatMessageChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Reset<Impl: IChatMessageChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageChangeTracker, BASE_OFFSET>(),
            Enable: Enable::<Impl, IMPL_OFFSET>,
            GetChangeReader: GetChangeReader::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageChangeTracker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageChangedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageChangedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageChangedDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageChangedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IChatMessageChangedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageChangedDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageChangedDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<ChatMessageChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IChatMessageChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageChangedEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageManager2StaticsImpl: Sized + IChatMessageManagerStaticImpl {
    fn RegisterTransportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetTransportAsync(&self, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageTransport>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageManager2Statics {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageManager2Statics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageManager2StaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageManager2StaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageManager2StaticsVtbl {
        unsafe extern "system" fn RegisterTransportAsync<Impl: IChatMessageManager2StaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterTransportAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAsync<Impl: IChatMessageManager2StaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportAsync(&*(&transportid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageManager2Statics, BASE_OFFSET>(),
            RegisterTransportAsync: RegisterTransportAsync::<Impl, IMPL_OFFSET>,
            GetTransportAsync: GetTransportAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageManager2Statics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessageManagerStaticImpl: Sized {
    fn GetTransportsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>>;
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageStore>>;
    fn ShowComposeSmsMessageAsync(&self, message: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowSmsSettings(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageManagerStatic {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageManagerStatic";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessageManagerStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageManagerStaticImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageManagerStaticVtbl {
        unsafe extern "system" fn GetTransportsAsync<Impl: IChatMessageManagerStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStoreAsync<Impl: IChatMessageManagerStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowComposeSmsMessageAsync<Impl: IChatMessageManagerStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowComposeSmsMessageAsync(&*(&message as *const <ChatMessage as ::windows::core::Abi>::Abi as *const <ChatMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowSmsSettings<Impl: IChatMessageManagerStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowSmsSettings().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageManagerStatic, BASE_OFFSET>(),
            GetTransportsAsync: GetTransportsAsync::<Impl, IMPL_OFFSET>,
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
            ShowComposeSmsMessageAsync: ShowComposeSmsMessageAsync::<Impl, IMPL_OFFSET>,
            ShowSmsSettings: ShowSmsSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageManagerStatic as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageManagerStatics3Impl: Sized + IChatMessageManagerStaticImpl {
    fn RequestSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatSyncManager>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageManagerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageManagerStatics3Vtbl {
        unsafe extern "system" fn RequestSyncManagerAsync<Impl: IChatMessageManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSyncManagerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageManagerStatics3, BASE_OFFSET>(),
            RequestSyncManagerAsync: RequestSyncManagerAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageNotificationTriggerDetailsImpl: Sized {
    fn ChatMessage(&self) -> ::windows::core::Result<ChatMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn ChatMessage<Impl: IChatMessageNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChatMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageNotificationTriggerDetails, BASE_OFFSET>(),
            ChatMessage: ChatMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageNotificationTriggerDetails2Impl: Sized + IChatMessageNotificationTriggerDetailsImpl {
    fn ShouldDisplayToast(&self) -> ::windows::core::Result<bool>;
    fn ShouldUpdateDetailText(&self) -> ::windows::core::Result<bool>;
    fn ShouldUpdateBadge(&self) -> ::windows::core::Result<bool>;
    fn ShouldUpdateActionCenter(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageNotificationTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageNotificationTriggerDetails2Vtbl {
        unsafe extern "system" fn ShouldDisplayToast<Impl: IChatMessageNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldDisplayToast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldUpdateDetailText<Impl: IChatMessageNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldUpdateDetailText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldUpdateBadge<Impl: IChatMessageNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldUpdateBadge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldUpdateActionCenter<Impl: IChatMessageNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldUpdateActionCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageNotificationTriggerDetails2, BASE_OFFSET>(),
            ShouldDisplayToast: ShouldDisplayToast::<Impl, IMPL_OFFSET>,
            ShouldUpdateDetailText: ShouldUpdateDetailText::<Impl, IMPL_OFFSET>,
            ShouldUpdateBadge: ShouldUpdateBadge::<Impl, IMPL_OFFSET>,
            ShouldUpdateActionCenter: ShouldUpdateActionCenter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageNotificationTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessageReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessageReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IChatMessageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageReader, BASE_OFFSET>(), ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessageReader2Impl: Sized {
    fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageReader2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageReader2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessageReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageReader2Vtbl {
        unsafe extern "system" fn ReadBatchWithCountAsync<Impl: IChatMessageReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchWithCountAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageReader2, BASE_OFFSET>(),
            ReadBatchWithCountAsync: ReadBatchWithCountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageStoreImpl: Sized {
    fn ChangeTracker(&self) -> ::windows::core::Result<ChatMessageChangeTracker>;
    fn DeleteMessageAsync(&self, localmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DownloadMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
    fn GetMessageReader1(&self) -> ::windows::core::Result<ChatMessageReader>;
    fn GetMessageReader2(&self, recenttimelimit: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ChatMessageReader>;
    fn MarkMessageReadAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetrySendMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SendMessageAsync(&self, chatmessage: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ValidateMessage(&self, chatmessage: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<ChatMessageValidationResult>;
    fn MessageChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageChanged(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageStore {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageStore";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageStoreVtbl {
        unsafe extern "system" fn ChangeTracker<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteMessageAsync<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMessageAsync(&*(&localmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMessageAsync<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMessageAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageAsync<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReader1<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReader1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageReader2<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recenttimelimit: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageReader2(&*(&recenttimelimit as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkMessageReadAsync<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkMessageReadAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrySendMessageAsync<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrySendMessageAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessageAsync<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessageAsync(&*(&chatmessage as *const <ChatMessage as ::windows::core::Abi>::Abi as *const <ChatMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateMessage<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateMessage(&*(&chatmessage as *const <ChatMessage as ::windows::core::Abi>::Abi as *const <ChatMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageChanged<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageChanged<Impl: IChatMessageStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageChanged(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageStore, BASE_OFFSET>(),
            ChangeTracker: ChangeTracker::<Impl, IMPL_OFFSET>,
            DeleteMessageAsync: DeleteMessageAsync::<Impl, IMPL_OFFSET>,
            DownloadMessageAsync: DownloadMessageAsync::<Impl, IMPL_OFFSET>,
            GetMessageAsync: GetMessageAsync::<Impl, IMPL_OFFSET>,
            GetMessageReader1: GetMessageReader1::<Impl, IMPL_OFFSET>,
            GetMessageReader2: GetMessageReader2::<Impl, IMPL_OFFSET>,
            MarkMessageReadAsync: MarkMessageReadAsync::<Impl, IMPL_OFFSET>,
            RetrySendMessageAsync: RetrySendMessageAsync::<Impl, IMPL_OFFSET>,
            SendMessageAsync: SendMessageAsync::<Impl, IMPL_OFFSET>,
            ValidateMessage: ValidateMessage::<Impl, IMPL_OFFSET>,
            MessageChanged: MessageChanged::<Impl, IMPL_OFFSET>,
            RemoveMessageChanged: RemoveMessageChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatMessageStore2Impl: Sized + IChatMessageStoreImpl {
    fn ForwardMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING, addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
    fn GetConversationAsync(&self, conversationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>;
    fn GetConversationForTransportsAsync(&self, conversationid: &::windows::core::HSTRING, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>;
    fn GetConversationFromThreadingInfoAsync(&self, threadinginfo: &::core::option::Option<ChatConversationThreadingInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>;
    fn GetConversationReader(&self) -> ::windows::core::Result<ChatConversationReader>;
    fn GetConversationForTransportsReader(&self, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<ChatConversationReader>;
    fn GetMessageByRemoteIdAsync(&self, transportid: &::windows::core::HSTRING, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
    fn GetUnseenCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn GetUnseenCountForTransportsReaderAsync(&self, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn MarkAsSeenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkAsSeenForTransportsAsync(&self, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetSearchReader(&self, value: &::core::option::Option<ChatQueryOptions>) -> ::windows::core::Result<ChatSearchReader>;
    fn SaveMessageAsync(&self, chatmessage: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TryCancelDownloadMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryCancelSendMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn StoreChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStoreChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageStore2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageStore2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatMessageStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageStore2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageStore2Vtbl {
        unsafe extern "system" fn ForwardMessageAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, addresses: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardMessageAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&addresses as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conversationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationAsync(&*(&conversationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationForTransportsAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conversationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationForTransportsAsync(&*(&conversationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&transportids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationFromThreadingInfoAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadinginfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationFromThreadingInfoAsync(&*(&threadinginfo as *const <ChatConversationThreadingInfo as ::windows::core::Abi>::Abi as *const <ChatConversationThreadingInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversationReader<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetConversationForTransportsReader<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversationForTransportsReader(&*(&transportids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageByRemoteIdAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageByRemoteIdAsync(&*(&transportid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&remoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnseenCountAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnseenCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnseenCountForTransportsReaderAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnseenCountForTransportsReaderAsync(&*(&transportids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAsSeenAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkAsSeenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAsSeenForTransportsAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkAsSeenForTransportsAsync(&*(&transportids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSearchReader<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSearchReader(&*(&value as *const <ChatQueryOptions as ::windows::core::Abi>::Abi as *const <ChatQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveMessageAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveMessageAsync(&*(&chatmessage as *const <ChatMessage as ::windows::core::Abi>::Abi as *const <ChatMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCancelDownloadMessageAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCancelDownloadMessageAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCancelSendMessageAsync<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCancelSendMessageAsync(&*(&localchatmessageid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreChanged<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStoreChanged<Impl: IChatMessageStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStoreChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageStore2, BASE_OFFSET>(),
            ForwardMessageAsync: ForwardMessageAsync::<Impl, IMPL_OFFSET>,
            GetConversationAsync: GetConversationAsync::<Impl, IMPL_OFFSET>,
            GetConversationForTransportsAsync: GetConversationForTransportsAsync::<Impl, IMPL_OFFSET>,
            GetConversationFromThreadingInfoAsync: GetConversationFromThreadingInfoAsync::<Impl, IMPL_OFFSET>,
            GetConversationReader: GetConversationReader::<Impl, IMPL_OFFSET>,
            GetConversationForTransportsReader: GetConversationForTransportsReader::<Impl, IMPL_OFFSET>,
            GetMessageByRemoteIdAsync: GetMessageByRemoteIdAsync::<Impl, IMPL_OFFSET>,
            GetUnseenCountAsync: GetUnseenCountAsync::<Impl, IMPL_OFFSET>,
            GetUnseenCountForTransportsReaderAsync: GetUnseenCountForTransportsReaderAsync::<Impl, IMPL_OFFSET>,
            MarkAsSeenAsync: MarkAsSeenAsync::<Impl, IMPL_OFFSET>,
            MarkAsSeenForTransportsAsync: MarkAsSeenForTransportsAsync::<Impl, IMPL_OFFSET>,
            GetSearchReader: GetSearchReader::<Impl, IMPL_OFFSET>,
            SaveMessageAsync: SaveMessageAsync::<Impl, IMPL_OFFSET>,
            TryCancelDownloadMessageAsync: TryCancelDownloadMessageAsync::<Impl, IMPL_OFFSET>,
            TryCancelSendMessageAsync: TryCancelSendMessageAsync::<Impl, IMPL_OFFSET>,
            StoreChanged: StoreChanged::<Impl, IMPL_OFFSET>,
            RemoveStoreChanged: RemoveStoreChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageStore3Impl: Sized + IChatMessageStoreImpl {
    fn GetMessageBySyncIdAsync(&self, syncid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageStore3 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageStore3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageStore3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageStore3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageStore3Vtbl {
        unsafe extern "system" fn GetMessageBySyncIdAsync<Impl: IChatMessageStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageBySyncIdAsync(&*(&syncid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageStore3, BASE_OFFSET>(),
            GetMessageBySyncIdAsync: GetMessageBySyncIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageStore3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageStoreChangedEventArgsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<ChatStoreChangedEventKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageStoreChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageStoreChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageStoreChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageStoreChangedEventArgsVtbl {
        unsafe extern "system" fn Id<Impl: IChatMessageStoreChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IChatMessageStoreChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatStoreChangedEventKind) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageStoreChangedEventArgs, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageStoreChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageTransportImpl: Sized {
    fn IsAppSetAsNotificationProvider(&self) -> ::windows::core::Result<bool>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestSetAsNotificationProviderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageTransport";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageTransportVtbl {
        unsafe extern "system" fn IsAppSetAsNotificationProvider<Impl: IChatMessageTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAppSetAsNotificationProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IChatMessageTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportFriendlyName<Impl: IChatMessageTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportId<Impl: IChatMessageTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSetAsNotificationProviderAsync<Impl: IChatMessageTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSetAsNotificationProviderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageTransport, BASE_OFFSET>(),
            IsAppSetAsNotificationProvider: IsAppSetAsNotificationProvider::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            TransportFriendlyName: TransportFriendlyName::<Impl, IMPL_OFFSET>,
            TransportId: TransportId::<Impl, IMPL_OFFSET>,
            RequestSetAsNotificationProviderAsync: RequestSetAsNotificationProviderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageTransport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageTransport2Impl: Sized + IChatMessageTransportImpl {
    fn Configuration(&self) -> ::windows::core::Result<ChatMessageTransportConfiguration>;
    fn TransportKind(&self) -> ::windows::core::Result<ChatMessageTransportKind>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageTransport2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageTransport2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageTransport2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageTransport2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageTransport2Vtbl {
        unsafe extern "system" fn Configuration<Impl: IChatMessageTransport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportKind<Impl: IChatMessageTransport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageTransportKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageTransport2, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            TransportKind: TransportKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageTransport2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IChatMessageTransportConfigurationImpl: Sized {
    fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32>;
    fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn MaxRecipientCount(&self) -> ::windows::core::Result<i32>;
    fn SupportedVideoFormat(&self) -> ::windows::core::Result<super::super::Media::MediaProperties::MediaEncodingProfile>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageTransportConfiguration";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IChatMessageTransportConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageTransportConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageTransportConfigurationVtbl {
        unsafe extern "system" fn MaxAttachmentCount<Impl: IChatMessageTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAttachmentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxMessageSizeInKilobytes<Impl: IChatMessageTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxMessageSizeInKilobytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxRecipientCount<Impl: IChatMessageTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxRecipientCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedVideoFormat<Impl: IChatMessageTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedVideoFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IChatMessageTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageTransportConfiguration, BASE_OFFSET>(),
            MaxAttachmentCount: MaxAttachmentCount::<Impl, IMPL_OFFSET>,
            MaxMessageSizeInKilobytes: MaxMessageSizeInKilobytes::<Impl, IMPL_OFFSET>,
            MaxRecipientCount: MaxRecipientCount::<Impl, IMPL_OFFSET>,
            SupportedVideoFormat: SupportedVideoFormat::<Impl, IMPL_OFFSET>,
            ExtendedProperties: ExtendedProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageTransportConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatMessageValidationResultImpl: Sized {
    fn MaxPartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn RemainingCharacterCountInPart(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Status(&self) -> ::windows::core::Result<ChatMessageValidationStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatMessageValidationResult {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatMessageValidationResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatMessageValidationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageValidationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageValidationResultVtbl {
        unsafe extern "system" fn MaxPartCount<Impl: IChatMessageValidationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPartCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartCount<Impl: IChatMessageValidationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingCharacterCountInPart<Impl: IChatMessageValidationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingCharacterCountInPart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IChatMessageValidationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageValidationStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageValidationResult, BASE_OFFSET>(),
            MaxPartCount: MaxPartCount::<Impl, IMPL_OFFSET>,
            PartCount: PartCount::<Impl, IMPL_OFFSET>,
            RemainingCharacterCountInPart: RemainingCharacterCountInPart::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageValidationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatQueryOptionsImpl: Sized {
    fn SearchString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSearchString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatQueryOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IChatQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatQueryOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatQueryOptionsVtbl {
        unsafe extern "system" fn SearchString<Impl: IChatQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchString<Impl: IChatQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatQueryOptions, BASE_OFFSET>(),
            SearchString: SearchString::<Impl, IMPL_OFFSET>,
            SetSearchString: SetSearchString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatQueryOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChatRecipientDeliveryInfoImpl: Sized {
    fn TransportAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTransportAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDeliveryTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ReadTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetReadTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn TransportErrorCodeCategory(&self) -> ::windows::core::Result<ChatTransportErrorCodeCategory>;
    fn TransportInterpretedErrorCode(&self) -> ::windows::core::Result<ChatTransportInterpretedErrorCode>;
    fn TransportErrorCode(&self) -> ::windows::core::Result<i32>;
    fn IsErrorPermanent(&self) -> ::windows::core::Result<bool>;
    fn Status(&self) -> ::windows::core::Result<ChatMessageStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatRecipientDeliveryInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatRecipientDeliveryInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChatRecipientDeliveryInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatRecipientDeliveryInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatRecipientDeliveryInfoVtbl {
        unsafe extern "system" fn TransportAddress<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportAddress<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeliveryTime<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeliveryTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeliveryTime<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeliveryTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadTime<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadTime<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransportErrorCodeCategory<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportErrorCodeCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportErrorCodeCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportInterpretedErrorCode<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportInterpretedErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportInterpretedErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportErrorCode<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsErrorPermanent<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsErrorPermanent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IChatRecipientDeliveryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatRecipientDeliveryInfo, BASE_OFFSET>(),
            TransportAddress: TransportAddress::<Impl, IMPL_OFFSET>,
            SetTransportAddress: SetTransportAddress::<Impl, IMPL_OFFSET>,
            DeliveryTime: DeliveryTime::<Impl, IMPL_OFFSET>,
            SetDeliveryTime: SetDeliveryTime::<Impl, IMPL_OFFSET>,
            ReadTime: ReadTime::<Impl, IMPL_OFFSET>,
            SetReadTime: SetReadTime::<Impl, IMPL_OFFSET>,
            TransportErrorCodeCategory: TransportErrorCodeCategory::<Impl, IMPL_OFFSET>,
            TransportInterpretedErrorCode: TransportInterpretedErrorCode::<Impl, IMPL_OFFSET>,
            TransportErrorCode: TransportErrorCode::<Impl, IMPL_OFFSET>,
            IsErrorPermanent: IsErrorPermanent::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatRecipientDeliveryInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChatSearchReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>;
    fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatSearchReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatSearchReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChatSearchReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatSearchReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatSearchReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IChatSearchReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadBatchWithCountAsync<Impl: IChatSearchReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchWithCountAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatSearchReader, BASE_OFFSET>(),
            ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET>,
            ReadBatchWithCountAsync: ReadBatchWithCountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatSearchReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatSyncConfigurationImpl: Sized {
    fn IsSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestoreHistorySpan(&self) -> ::windows::core::Result<ChatRestoreHistorySpan>;
    fn SetRestoreHistorySpan(&self, value: ChatRestoreHistorySpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatSyncConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatSyncConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IChatSyncConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatSyncConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatSyncConfigurationVtbl {
        unsafe extern "system" fn IsSyncEnabled<Impl: IChatSyncConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSyncEnabled<Impl: IChatSyncConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSyncEnabled(value).into()
        }
        unsafe extern "system" fn RestoreHistorySpan<Impl: IChatSyncConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChatRestoreHistorySpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreHistorySpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestoreHistorySpan<Impl: IChatSyncConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ChatRestoreHistorySpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestoreHistorySpan(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatSyncConfiguration, BASE_OFFSET>(),
            IsSyncEnabled: IsSyncEnabled::<Impl, IMPL_OFFSET>,
            SetIsSyncEnabled: SetIsSyncEnabled::<Impl, IMPL_OFFSET>,
            RestoreHistorySpan: RestoreHistorySpan::<Impl, IMPL_OFFSET>,
            SetRestoreHistorySpan: SetRestoreHistorySpan::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatSyncConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IChatSyncManagerImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<ChatSyncConfiguration>;
    fn AssociateAccountAsync(&self, webaccount: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UnassociateAccountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn IsAccountAssociated(&self, webaccount: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<bool>;
    fn StartSync(&self) -> ::windows::core::Result<()>;
    fn SetConfigurationAsync(&self, configuration: &::core::option::Option<ChatSyncConfiguration>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChatSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatSyncManager";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IChatSyncManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatSyncManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatSyncManagerVtbl {
        unsafe extern "system" fn Configuration<Impl: IChatSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociateAccountAsync<Impl: IChatSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateAccountAsync(&*(&webaccount as *const <super::super::Security::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnassociateAccountAsync<Impl: IChatSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnassociateAccountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAccountAssociated<Impl: IChatSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAccountAssociated(&*(&webaccount as *const <super::super::Security::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartSync<Impl: IChatSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartSync().into()
        }
        unsafe extern "system" fn SetConfigurationAsync<Impl: IChatSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConfigurationAsync(&*(&configuration as *const <ChatSyncConfiguration as ::windows::core::Abi>::Abi as *const <ChatSyncConfiguration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChatSyncManager, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            AssociateAccountAsync: AssociateAccountAsync::<Impl, IMPL_OFFSET>,
            UnassociateAccountAsync: UnassociateAccountAsync::<Impl, IMPL_OFFSET>,
            IsAccountAssociated: IsAccountAssociated::<Impl, IMPL_OFFSET>,
            StartSync: StartSync::<Impl, IMPL_OFFSET>,
            SetConfigurationAsync: SetConfigurationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatSyncManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRcsEndUserMessageImpl: Sized {
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsPinRequired(&self) -> ::windows::core::Result<bool>;
    fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>>;
    fn SendResponseAsync(&self, action: &::core::option::Option<RcsEndUserMessageAction>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SendResponseWithPinAsync(&self, action: &::core::option::Option<RcsEndUserMessageAction>, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRcsEndUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsEndUserMessage";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRcsEndUserMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsEndUserMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsEndUserMessageVtbl {
        unsafe extern "system" fn TransportId<Impl: IRcsEndUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IRcsEndUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IRcsEndUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPinRequired<Impl: IRcsEndUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Actions<Impl: IRcsEndUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendResponseAsync<Impl: IRcsEndUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendResponseAsync(&*(&action as *const <RcsEndUserMessageAction as ::windows::core::Abi>::Abi as *const <RcsEndUserMessageAction as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendResponseWithPinAsync<Impl: IRcsEndUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendResponseWithPinAsync(&*(&action as *const <RcsEndUserMessageAction as ::windows::core::Abi>::Abi as *const <RcsEndUserMessageAction as ::windows::core::DefaultType>::DefaultType), &*(&pin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsEndUserMessage, BASE_OFFSET>(),
            TransportId: TransportId::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            IsPinRequired: IsPinRequired::<Impl, IMPL_OFFSET>,
            Actions: Actions::<Impl, IMPL_OFFSET>,
            SendResponseAsync: SendResponseAsync::<Impl, IMPL_OFFSET>,
            SendResponseWithPinAsync: SendResponseWithPinAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsEndUserMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageActionImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRcsEndUserMessageAction {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsEndUserMessageAction";
}
#[cfg(feature = "implement_exclusive")]
impl IRcsEndUserMessageActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsEndUserMessageActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsEndUserMessageActionVtbl {
        unsafe extern "system" fn Label<Impl: IRcsEndUserMessageActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsEndUserMessageAction, BASE_OFFSET>(), Label: Label::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsEndUserMessageAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageAvailableEventArgsImpl: Sized {
    fn IsMessageAvailable(&self) -> ::windows::core::Result<bool>;
    fn Message(&self) -> ::windows::core::Result<RcsEndUserMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRcsEndUserMessageAvailableEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRcsEndUserMessageAvailableEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsEndUserMessageAvailableEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsEndUserMessageAvailableEventArgsVtbl {
        unsafe extern "system" fn IsMessageAvailable<Impl: IRcsEndUserMessageAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMessageAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IRcsEndUserMessageAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsEndUserMessageAvailableEventArgs, BASE_OFFSET>(),
            IsMessageAvailable: IsMessageAvailable::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsEndUserMessageAvailableEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageAvailableTriggerDetailsImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRcsEndUserMessageAvailableTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IRcsEndUserMessageAvailableTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsEndUserMessageAvailableTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsEndUserMessageAvailableTriggerDetailsVtbl {
        unsafe extern "system" fn Title<Impl: IRcsEndUserMessageAvailableTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IRcsEndUserMessageAvailableTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsEndUserMessageAvailableTriggerDetails, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsEndUserMessageAvailableTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRcsEndUserMessageManagerImpl: Sized {
    fn MessageAvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRcsEndUserMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsEndUserMessageManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRcsEndUserMessageManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsEndUserMessageManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsEndUserMessageManagerVtbl {
        unsafe extern "system" fn MessageAvailableChanged<Impl: IRcsEndUserMessageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageAvailableChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageAvailableChanged<Impl: IRcsEndUserMessageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageAvailableChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsEndUserMessageManager, BASE_OFFSET>(),
            MessageAvailableChanged: MessageAvailableChanged::<Impl, IMPL_OFFSET>,
            RemoveMessageAvailableChanged: RemoveMessageAvailableChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsEndUserMessageManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRcsManagerStaticsImpl: Sized {
    fn GetEndUserMessageManager(&self) -> ::windows::core::Result<RcsEndUserMessageManager>;
    fn GetTransportsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>>;
    fn GetTransportAsync(&self, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RcsTransport>>;
    fn LeaveConversationAsync(&self, conversation: &::core::option::Option<ChatConversation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRcsManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRcsManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsManagerStaticsVtbl {
        unsafe extern "system" fn GetEndUserMessageManager<Impl: IRcsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndUserMessageManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportsAsync<Impl: IRcsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAsync<Impl: IRcsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportAsync(&*(&transportid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaveConversationAsync<Impl: IRcsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conversation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaveConversationAsync(&*(&conversation as *const <ChatConversation as ::windows::core::Abi>::Abi as *const <ChatConversation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsManagerStatics, BASE_OFFSET>(),
            GetEndUserMessageManager: GetEndUserMessageManager::<Impl, IMPL_OFFSET>,
            GetTransportsAsync: GetTransportsAsync::<Impl, IMPL_OFFSET>,
            GetTransportAsync: GetTransportAsync::<Impl, IMPL_OFFSET>,
            LeaveConversationAsync: LeaveConversationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRcsManagerStatics2Impl: Sized {
    fn TransportListChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransportListChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRcsManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRcsManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsManagerStatics2Vtbl {
        unsafe extern "system" fn TransportListChanged<Impl: IRcsManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportListChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransportListChanged<Impl: IRcsManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransportListChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsManagerStatics2, BASE_OFFSET>(),
            TransportListChanged: TransportListChanged::<Impl, IMPL_OFFSET>,
            RemoveTransportListChanged: RemoveTransportListChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsServiceKindSupportedChangedEventArgsImpl: Sized {
    fn ServiceKind(&self) -> ::windows::core::Result<RcsServiceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRcsServiceKindSupportedChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsServiceKindSupportedChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRcsServiceKindSupportedChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsServiceKindSupportedChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsServiceKindSupportedChangedEventArgsVtbl {
        unsafe extern "system" fn ServiceKind<Impl: IRcsServiceKindSupportedChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RcsServiceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsServiceKindSupportedChangedEventArgs, BASE_OFFSET>(),
            ServiceKind: ServiceKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsServiceKindSupportedChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRcsTransportImpl: Sized {
    fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Configuration(&self) -> ::windows::core::Result<RcsTransportConfiguration>;
    fn IsStoreAndForwardEnabled(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool>;
    fn IsServiceKindSupported(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool>;
    fn ServiceKindSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceKindSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRcsTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsTransport";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRcsTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsTransportVtbl {
        unsafe extern "system" fn ExtendedProperties<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportFriendlyName<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportId<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configuration<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStoreAndForwardEnabled<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreAndForwardEnabled(servicekind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsServiceKindSupported<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsServiceKindSupported(servicekind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceKindSupportedChanged<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceKindSupportedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServiceKindSupportedChanged<Impl: IRcsTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServiceKindSupportedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsTransport, BASE_OFFSET>(),
            ExtendedProperties: ExtendedProperties::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            TransportFriendlyName: TransportFriendlyName::<Impl, IMPL_OFFSET>,
            TransportId: TransportId::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            IsStoreAndForwardEnabled: IsStoreAndForwardEnabled::<Impl, IMPL_OFFSET>,
            IsServiceKindSupported: IsServiceKindSupported::<Impl, IMPL_OFFSET>,
            ServiceKindSupportedChanged: ServiceKindSupportedChanged::<Impl, IMPL_OFFSET>,
            RemoveServiceKindSupportedChanged: RemoveServiceKindSupportedChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsTransport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsTransportConfigurationImpl: Sized {
    fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32>;
    fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn MaxGroupMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn MaxRecipientCount(&self) -> ::windows::core::Result<i32>;
    fn MaxFileSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn WarningFileSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRcsTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRcsTransportConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IRcsTransportConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsTransportConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsTransportConfigurationVtbl {
        unsafe extern "system" fn MaxAttachmentCount<Impl: IRcsTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAttachmentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxMessageSizeInKilobytes<Impl: IRcsTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxMessageSizeInKilobytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxGroupMessageSizeInKilobytes<Impl: IRcsTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxGroupMessageSizeInKilobytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxRecipientCount<Impl: IRcsTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxRecipientCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxFileSizeInKilobytes<Impl: IRcsTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxFileSizeInKilobytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WarningFileSizeInKilobytes<Impl: IRcsTransportConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningFileSizeInKilobytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsTransportConfiguration, BASE_OFFSET>(),
            MaxAttachmentCount: MaxAttachmentCount::<Impl, IMPL_OFFSET>,
            MaxMessageSizeInKilobytes: MaxMessageSizeInKilobytes::<Impl, IMPL_OFFSET>,
            MaxGroupMessageSizeInKilobytes: MaxGroupMessageSizeInKilobytes::<Impl, IMPL_OFFSET>,
            MaxRecipientCount: MaxRecipientCount::<Impl, IMPL_OFFSET>,
            MaxFileSizeInKilobytes: MaxFileSizeInKilobytes::<Impl, IMPL_OFFSET>,
            WarningFileSizeInKilobytes: WarningFileSizeInKilobytes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsTransportConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteParticipantComposingChangedEventArgsImpl: Sized {
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParticipantAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsComposing(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteParticipantComposingChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IRemoteParticipantComposingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteParticipantComposingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteParticipantComposingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteParticipantComposingChangedEventArgsVtbl {
        unsafe extern "system" fn TransportId<Impl: IRemoteParticipantComposingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParticipantAddress<Impl: IRemoteParticipantComposingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParticipantAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComposing<Impl: IRemoteParticipantComposingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComposing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteParticipantComposingChangedEventArgs, BASE_OFFSET>(),
            TransportId: TransportId::<Impl, IMPL_OFFSET>,
            ParticipantAddress: ParticipantAddress::<Impl, IMPL_OFFSET>,
            IsComposing: IsComposing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteParticipantComposingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
