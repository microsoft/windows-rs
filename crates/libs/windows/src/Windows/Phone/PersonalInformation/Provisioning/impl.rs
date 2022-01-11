#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IContactPartnerProvisioningManagerStaticsImpl: Sized {
    fn AssociateNetworkAccountAsync(&self, store: &::core::option::Option<super::ContactStore>, networkname: &::windows::core::HSTRING, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportVcardToSystemAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactPartnerProvisioningManagerStatics {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.IContactPartnerProvisioningManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IContactPartnerProvisioningManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPartnerProvisioningManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPartnerProvisioningManagerStaticsVtbl {
        unsafe extern "system" fn AssociateNetworkAccountAsync<Impl: IContactPartnerProvisioningManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, store: ::windows::core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateNetworkAccountAsync(
                &*(&store as *const <super::ContactStore as ::windows::core::Abi>::Abi as *const <super::ContactStore as ::windows::core::DefaultType>::DefaultType),
                &*(&networkname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportVcardToSystemAsync<Impl: IContactPartnerProvisioningManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportVcardToSystemAsync(&*(&stream as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactPartnerProvisioningManagerStatics, BASE_OFFSET>(),
            AssociateNetworkAccountAsync: AssociateNetworkAccountAsync::<Impl, IMPL_OFFSET>,
            ImportVcardToSystemAsync: ImportVcardToSystemAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPartnerProvisioningManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactPartnerProvisioningManagerStatics2Impl: Sized {
    fn AssociateSocialNetworkAccountAsync(&self, store: &::core::option::Option<super::ContactStore>, networkname: &::windows::core::HSTRING, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactPartnerProvisioningManagerStatics2 {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.IContactPartnerProvisioningManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactPartnerProvisioningManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPartnerProvisioningManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPartnerProvisioningManagerStatics2Vtbl {
        unsafe extern "system" fn AssociateSocialNetworkAccountAsync<Impl: IContactPartnerProvisioningManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, store: ::windows::core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateSocialNetworkAccountAsync(
                &*(&store as *const <super::ContactStore as ::windows::core::Abi>::Abi as *const <super::ContactStore as ::windows::core::DefaultType>::DefaultType),
                &*(&networkname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactPartnerProvisioningManagerStatics2, BASE_OFFSET>(),
            AssociateSocialNetworkAccountAsync: AssociateSocialNetworkAccountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPartnerProvisioningManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMessagePartnerProvisioningManagerStaticsImpl: Sized {
    fn ImportSmsToSystemAsync(&self, incoming: bool, read: bool, body: &::windows::core::HSTRING, sender: &::windows::core::HSTRING, recipients: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, deliverytime: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportMmsToSystemAsync(&self, incoming: bool, read: bool, subject: &::windows::core::HSTRING, sender: &::windows::core::HSTRING, recipients: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, deliverytime: &super::super::super::Foundation::DateTime, attachments: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessagePartnerProvisioningManagerStatics {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.IMessagePartnerProvisioningManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMessagePartnerProvisioningManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessagePartnerProvisioningManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessagePartnerProvisioningManagerStaticsVtbl {
        unsafe extern "system" fn ImportSmsToSystemAsync<Impl: IMessagePartnerProvisioningManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, body: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recipients: ::windows::core::RawPtr, deliverytime: super::super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportSmsToSystemAsync(
                incoming,
                read,
                &*(&body as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&sender as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&recipients as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&deliverytime as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportMmsToSystemAsync<Impl: IMessagePartnerProvisioningManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recipients: ::windows::core::RawPtr, deliverytime: super::super::super::Foundation::DateTime, attachments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportMmsToSystemAsync(
                incoming,
                read,
                &*(&subject as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&sender as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&recipients as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&deliverytime as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&attachments as *const <super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessagePartnerProvisioningManagerStatics, BASE_OFFSET>(),
            ImportSmsToSystemAsync: ImportSmsToSystemAsync::<Impl, IMPL_OFFSET>,
            ImportMmsToSystemAsync: ImportMmsToSystemAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessagePartnerProvisioningManagerStatics as ::windows::core::Interface>::IID
    }
}
