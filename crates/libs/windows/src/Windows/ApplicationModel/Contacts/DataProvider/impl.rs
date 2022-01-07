#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderConnectionImpl: Sized {
    fn SyncRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ServerSearchReadBatchRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerSearchReadBatchRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IContactDataProviderConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactDataProviderConnectionImpl, const OFFSET: isize>() -> IContactDataProviderConnectionVtbl {
        unsafe extern "system" fn SyncRequested<Impl: IContactDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSyncRequested<Impl: IContactDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSyncRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerSearchReadBatchRequested<Impl: IContactDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerSearchReadBatchRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServerSearchReadBatchRequested<Impl: IContactDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServerSearchReadBatchRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IContactDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactDataProviderConnection>, ::windows::core::GetTrustLevel, SyncRequested::<Impl, OFFSET>, RemoveSyncRequested::<Impl, OFFSET>, ServerSearchReadBatchRequested::<Impl, OFFSET>, RemoveServerSearchReadBatchRequested::<Impl, OFFSET>, Start::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderConnection2Impl: Sized {
    fn CreateOrUpdateContactRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCreateOrUpdateContactRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteContactRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeleteContactRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactDataProviderConnection2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactDataProviderConnection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactDataProviderConnection2Impl, const OFFSET: isize>() -> IContactDataProviderConnection2Vtbl {
        unsafe extern "system" fn CreateOrUpdateContactRequested<Impl: IContactDataProviderConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrUpdateContactRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCreateOrUpdateContactRequested<Impl: IContactDataProviderConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCreateOrUpdateContactRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeleteContactRequested<Impl: IContactDataProviderConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteContactRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDeleteContactRequested<Impl: IContactDataProviderConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDeleteContactRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactDataProviderConnection2>, ::windows::core::GetTrustLevel, CreateOrUpdateContactRequested::<Impl, OFFSET>, RemoveCreateOrUpdateContactRequested::<Impl, OFFSET>, DeleteContactRequested::<Impl, OFFSET>, RemoveDeleteContactRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<ContactDataProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IContactDataProviderTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactDataProviderTriggerDetailsImpl, const OFFSET: isize>() -> IContactDataProviderTriggerDetailsVtbl {
        unsafe extern "system" fn Connection<Impl: IContactDataProviderTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactDataProviderTriggerDetails>, ::windows::core::GetTrustLevel, Connection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListCreateOrUpdateContactRequestImpl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contact>;
    fn ReportCompletedAsync(&self, createdorupdatedcontact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListCreateOrUpdateContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListCreateOrUpdateContactRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListCreateOrUpdateContactRequestImpl, const OFFSET: isize>() -> IContactListCreateOrUpdateContactRequestVtbl {
        unsafe extern "system" fn ContactListId<Impl: IContactListCreateOrUpdateContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactListId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IContactListCreateOrUpdateContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IContactListCreateOrUpdateContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, createdorupdatedcontact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync(&*(&createdorupdatedcontact as *const <super::Contact as ::windows::core::Abi>::Abi as *const <super::Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IContactListCreateOrUpdateContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListCreateOrUpdateContactRequest>, ::windows::core::GetTrustLevel, ContactListId::<Impl, OFFSET>, Contact::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListCreateOrUpdateContactRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListCreateOrUpdateContactRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListCreateOrUpdateContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListCreateOrUpdateContactRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListCreateOrUpdateContactRequestEventArgsImpl, const OFFSET: isize>() -> IContactListCreateOrUpdateContactRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IContactListCreateOrUpdateContactRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IContactListCreateOrUpdateContactRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListCreateOrUpdateContactRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListDeleteContactRequestImpl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListDeleteContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListDeleteContactRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListDeleteContactRequestImpl, const OFFSET: isize>() -> IContactListDeleteContactRequestVtbl {
        unsafe extern "system" fn ContactListId<Impl: IContactListDeleteContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactListId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactId<Impl: IContactListDeleteContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportCompletedAsync<Impl: IContactListDeleteContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IContactListDeleteContactRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListDeleteContactRequest>, ::windows::core::GetTrustLevel, ContactListId::<Impl, OFFSET>, ContactId::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListDeleteContactRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListDeleteContactRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListDeleteContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListDeleteContactRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListDeleteContactRequestEventArgsImpl, const OFFSET: isize>() -> IContactListDeleteContactRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IContactListDeleteContactRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IContactListDeleteContactRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListDeleteContactRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListServerSearchReadBatchRequestImpl: Sized {
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Options(&self) -> ::windows::core::Result<super::ContactQueryOptions>;
    fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32>;
    fn SaveContactAsync(&self, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, batchstatus: super::ContactBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListServerSearchReadBatchRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>() -> IContactListServerSearchReadBatchRequestVtbl {
        unsafe extern "system" fn SessionId<Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContactListId<Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactListId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuggestedBatchSize<Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveContactAsync<Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveContactAsync(&*(&contact as *const <super::Contact as ::windows::core::Abi>::Abi as *const <super::Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IContactListServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, batchstatus: super::ContactBatchStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync(batchstatus) {
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
            ::windows::core::GetRuntimeClassName::<IContactListServerSearchReadBatchRequest>,
            ::windows::core::GetTrustLevel,
            SessionId::<Impl, OFFSET>,
            ContactListId::<Impl, OFFSET>,
            Options::<Impl, OFFSET>,
            SuggestedBatchSize::<Impl, OFFSET>,
            SaveContactAsync::<Impl, OFFSET>,
            ReportCompletedAsync::<Impl, OFFSET>,
            ReportFailedAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListServerSearchReadBatchRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListServerSearchReadBatchRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListServerSearchReadBatchRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListServerSearchReadBatchRequestEventArgsImpl, const OFFSET: isize>() -> IContactListServerSearchReadBatchRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IContactListServerSearchReadBatchRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IContactListServerSearchReadBatchRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListServerSearchReadBatchRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerSyncRequestImpl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListSyncManagerSyncRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListSyncManagerSyncRequestImpl, const OFFSET: isize>() -> IContactListSyncManagerSyncRequestVtbl {
        unsafe extern "system" fn ContactListId<Impl: IContactListSyncManagerSyncRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactListId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IContactListSyncManagerSyncRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IContactListSyncManagerSyncRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListSyncManagerSyncRequest>, ::windows::core::GetTrustLevel, ContactListId::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerSyncRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListSyncManagerSyncRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListSyncManagerSyncRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListSyncManagerSyncRequestEventArgsImpl, const OFFSET: isize>() -> IContactListSyncManagerSyncRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IContactListSyncManagerSyncRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IContactListSyncManagerSyncRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListSyncManagerSyncRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
