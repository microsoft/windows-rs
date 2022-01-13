#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactDataProviderConnectionImpl: Sized {
    fn SyncRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ServerSearchReadBatchRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerSearchReadBatchRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactDataProviderConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactDataProviderConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactDataProviderConnectionVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactDataProviderConnection, BASE_OFFSET>(),
            SyncRequested: SyncRequested::<Impl, IMPL_OFFSET>,
            RemoveSyncRequested: RemoveSyncRequested::<Impl, IMPL_OFFSET>,
            ServerSearchReadBatchRequested: ServerSearchReadBatchRequested::<Impl, IMPL_OFFSET>,
            RemoveServerSearchReadBatchRequested: RemoveServerSearchReadBatchRequested::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactDataProviderConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactDataProviderConnection2Impl: Sized {
    fn CreateOrUpdateContactRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCreateOrUpdateContactRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteContactRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeleteContactRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactDataProviderConnection2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactDataProviderConnection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactDataProviderConnection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactDataProviderConnection2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactDataProviderConnection2, BASE_OFFSET>(),
            CreateOrUpdateContactRequested: CreateOrUpdateContactRequested::<Impl, IMPL_OFFSET>,
            RemoveCreateOrUpdateContactRequested: RemoveCreateOrUpdateContactRequested::<Impl, IMPL_OFFSET>,
            DeleteContactRequested: DeleteContactRequested::<Impl, IMPL_OFFSET>,
            RemoveDeleteContactRequested: RemoveDeleteContactRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactDataProviderConnection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderTriggerDetailsImpl: Sized {
    fn Connection(&mut self) -> ::windows::core::Result<ContactDataProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IContactDataProviderTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactDataProviderTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactDataProviderTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactDataProviderTriggerDetails, BASE_OFFSET>(),
            Connection: Connection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactDataProviderTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListCreateOrUpdateContactRequestImpl: Sized {
    fn ContactListId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contact>;
    fn ReportCompletedAsync(&mut self, createdorupdatedcontact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListCreateOrUpdateContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListCreateOrUpdateContactRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListCreateOrUpdateContactRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListCreateOrUpdateContactRequestVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListCreateOrUpdateContactRequest, BASE_OFFSET>(),
            ContactListId: ContactListId::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
            ReportCompletedAsync: ReportCompletedAsync::<Impl, IMPL_OFFSET>,
            ReportFailedAsync: ReportFailedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListCreateOrUpdateContactRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListCreateOrUpdateContactRequestEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<ContactListCreateOrUpdateContactRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListCreateOrUpdateContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListCreateOrUpdateContactRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListCreateOrUpdateContactRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListCreateOrUpdateContactRequestEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListCreateOrUpdateContactRequestEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListCreateOrUpdateContactRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListDeleteContactRequestImpl: Sized {
    fn ContactListId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListDeleteContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListDeleteContactRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListDeleteContactRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListDeleteContactRequestVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListDeleteContactRequest, BASE_OFFSET>(),
            ContactListId: ContactListId::<Impl, IMPL_OFFSET>,
            ContactId: ContactId::<Impl, IMPL_OFFSET>,
            ReportCompletedAsync: ReportCompletedAsync::<Impl, IMPL_OFFSET>,
            ReportFailedAsync: ReportFailedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListDeleteContactRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListDeleteContactRequestEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<ContactListDeleteContactRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListDeleteContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListDeleteContactRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListDeleteContactRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListDeleteContactRequestEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListDeleteContactRequestEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListDeleteContactRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListServerSearchReadBatchRequestImpl: Sized {
    fn SessionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactListId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Options(&mut self) -> ::windows::core::Result<super::ContactQueryOptions>;
    fn SuggestedBatchSize(&mut self) -> ::windows::core::Result<u32>;
    fn SaveContactAsync(&mut self, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportCompletedAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&mut self, batchstatus: super::ContactBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListServerSearchReadBatchRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListServerSearchReadBatchRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListServerSearchReadBatchRequestVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListServerSearchReadBatchRequest, BASE_OFFSET>(),
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            ContactListId: ContactListId::<Impl, IMPL_OFFSET>,
            Options: Options::<Impl, IMPL_OFFSET>,
            SuggestedBatchSize: SuggestedBatchSize::<Impl, IMPL_OFFSET>,
            SaveContactAsync: SaveContactAsync::<Impl, IMPL_OFFSET>,
            ReportCompletedAsync: ReportCompletedAsync::<Impl, IMPL_OFFSET>,
            ReportFailedAsync: ReportFailedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListServerSearchReadBatchRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListServerSearchReadBatchRequestEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<ContactListServerSearchReadBatchRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListServerSearchReadBatchRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListServerSearchReadBatchRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListServerSearchReadBatchRequestEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListServerSearchReadBatchRequestEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListServerSearchReadBatchRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListSyncManagerSyncRequestImpl: Sized {
    fn ContactListId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListSyncManagerSyncRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListSyncManagerSyncRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListSyncManagerSyncRequestVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListSyncManagerSyncRequest, BASE_OFFSET>(),
            ContactListId: ContactListId::<Impl, IMPL_OFFSET>,
            ReportCompletedAsync: ReportCompletedAsync::<Impl, IMPL_OFFSET>,
            ReportFailedAsync: ReportFailedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListSyncManagerSyncRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactListSyncManagerSyncRequestEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<ContactListSyncManagerSyncRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactListSyncManagerSyncRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListSyncManagerSyncRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactListSyncManagerSyncRequestEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactListSyncManagerSyncRequestEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactListSyncManagerSyncRequestEventArgs as ::windows::core::Interface>::IID
    }
}
