#[cfg(feature = "implement_exclusive")]
pub trait IAggregateContactManagerImpl: Sized {
    fn FindRawContactsAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>;
    fn TryLinkContactsAsync(&self, primarycontact: &::core::option::Option<Contact>, secondarycontact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn UnlinkRawContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrySetPreferredSourceForPictureAsync(&self, aggregatecontact: &::core::option::Option<Contact>, rawcontact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAggregateContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IAggregateContactManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAggregateContactManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAggregateContactManagerImpl, const OFFSET: isize>() -> IAggregateContactManagerVtbl {
        unsafe extern "system" fn FindRawContactsAsync<Impl: IAggregateContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindRawContactsAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryLinkContactsAsync<Impl: IAggregateContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primarycontact: ::windows::core::RawPtr, secondarycontact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryLinkContactsAsync(&*(&primarycontact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), &*(&secondarycontact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlinkRawContactAsync<Impl: IAggregateContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlinkRawContactAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetPreferredSourceForPictureAsync<Impl: IAggregateContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregatecontact: ::windows::core::RawPtr, rawcontact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetPreferredSourceForPictureAsync(&*(&aggregatecontact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), &*(&rawcontact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAggregateContactManager>, ::windows::core::GetTrustLevel, FindRawContactsAsync::<Impl, OFFSET>, TryLinkContactsAsync::<Impl, OFFSET>, UnlinkRawContactAsync::<Impl, OFFSET>, TrySetPreferredSourceForPictureAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAggregateContactManager2Impl: Sized {
    fn SetRemoteIdentificationInformationAsync(&self, contactlistid: &::windows::core::HSTRING, remotesourceid: &::windows::core::HSTRING, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAggregateContactManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IAggregateContactManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IAggregateContactManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAggregateContactManager2Impl, const OFFSET: isize>() -> IAggregateContactManager2Vtbl {
        unsafe extern "system" fn SetRemoteIdentificationInformationAsync<Impl: IAggregateContactManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactlistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotesourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteIdentificationInformationAsync(
                &*(&contactlistid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&remotesourceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&accountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAggregateContactManager2>, ::windows::core::GetTrustLevel, SetRemoteIdentificationInformationAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Fields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IContactField>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContact {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContact";
}
#[cfg(feature = "implement_exclusive")]
impl IContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactImpl, const OFFSET: isize>() -> IContactVtbl {
        unsafe extern "system" fn Name<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Fields<Impl: IContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContact>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>, Thumbnail::<Impl, OFFSET>, SetThumbnail::<Impl, OFFSET>, Fields::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContact2Impl: Sized + IContactImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Notes(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNotes(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Phones(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactPhone>>;
    fn Emails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactEmail>>;
    fn Addresses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactAddress>>;
    fn ConnectedServiceAccounts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactConnectedServiceAccount>>;
    fn ImportantDates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactDate>>;
    fn DataSuppliers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn JobInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactJobInfo>>;
    fn SignificantOthers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactSignificantOther>>;
    fn Websites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactWebsite>>;
    fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContact2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContact2";
}
#[cfg(feature = "implement_exclusive")]
impl IContact2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContact2Impl, const OFFSET: isize>() -> IContact2Vtbl {
        unsafe extern "system" fn Id<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Notes<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotes<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotes(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Phones<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phones() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Emails<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Emails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedServiceAccounts<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectedServiceAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportantDates<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportantDates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSuppliers<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSuppliers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobInfo<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignificantOthers<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignificantOthers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Websites<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Websites() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderProperties<Impl: IContact2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderProperties() {
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
            ::windows::core::GetRuntimeClassName::<IContact2>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            SetId::<Impl, OFFSET>,
            Notes::<Impl, OFFSET>,
            SetNotes::<Impl, OFFSET>,
            Phones::<Impl, OFFSET>,
            Emails::<Impl, OFFSET>,
            Addresses::<Impl, OFFSET>,
            ConnectedServiceAccounts::<Impl, OFFSET>,
            ImportantDates::<Impl, OFFSET>,
            DataSuppliers::<Impl, OFFSET>,
            JobInfo::<Impl, OFFSET>,
            SignificantOthers::<Impl, OFFSET>,
            Websites::<Impl, OFFSET>,
            ProviderProperties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContact3Impl: Sized + IContactImpl + IContact2Impl {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayPictureUserUpdateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureUserUpdateTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn IsMe(&self) -> ::windows::core::Result<bool>;
    fn AggregateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RingToneToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRingToneToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsDisplayPictureManuallySet(&self) -> ::windows::core::Result<bool>;
    fn LargeDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SmallDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SourceDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetSourceDisplayPicture(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn TextToneToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextToneToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsAggregate(&self) -> ::windows::core::Result<bool>;
    fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayNameOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayNameOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Nickname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNickname(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SortName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContact3 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContact3";
}
#[cfg(feature = "implement_exclusive")]
impl IContact3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContact3Impl, const OFFSET: isize>() -> IContact3Vtbl {
        unsafe extern "system" fn ContactListId<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayPictureUserUpdateTime<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPictureUserUpdateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureUserUpdateTime<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayPictureUserUpdateTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsMe<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AggregateId<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AggregateId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteId<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteId<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RingToneToken<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingToneToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingToneToken<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRingToneToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsDisplayPictureManuallySet<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisplayPictureManuallySet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeDisplayPicture<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeDisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallDisplayPicture<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallDisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceDisplayPicture<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceDisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceDisplayPicture<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceDisplayPicture(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextToneToken<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextToneToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextToneToken<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextToneToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsAggregate<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAggregate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullName<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayNameOverride<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayNameOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayNameOverride<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayNameOverride(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Nickname<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nickname() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNickname<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNickname(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SortName<Impl: IContact3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortName() {
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
            ::windows::core::GetRuntimeClassName::<IContact3>,
            ::windows::core::GetTrustLevel,
            ContactListId::<Impl, OFFSET>,
            DisplayPictureUserUpdateTime::<Impl, OFFSET>,
            SetDisplayPictureUserUpdateTime::<Impl, OFFSET>,
            IsMe::<Impl, OFFSET>,
            AggregateId::<Impl, OFFSET>,
            RemoteId::<Impl, OFFSET>,
            SetRemoteId::<Impl, OFFSET>,
            RingToneToken::<Impl, OFFSET>,
            SetRingToneToken::<Impl, OFFSET>,
            IsDisplayPictureManuallySet::<Impl, OFFSET>,
            LargeDisplayPicture::<Impl, OFFSET>,
            SmallDisplayPicture::<Impl, OFFSET>,
            SourceDisplayPicture::<Impl, OFFSET>,
            SetSourceDisplayPicture::<Impl, OFFSET>,
            TextToneToken::<Impl, OFFSET>,
            SetTextToneToken::<Impl, OFFSET>,
            IsAggregate::<Impl, OFFSET>,
            FullName::<Impl, OFFSET>,
            DisplayNameOverride::<Impl, OFFSET>,
            SetDisplayNameOverride::<Impl, OFFSET>,
            Nickname::<Impl, OFFSET>,
            SetNickname::<Impl, OFFSET>,
            SortName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAddressImpl: Sized {
    fn StreetAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Locality(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocality(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCountry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPostalCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactAddressKind>;
    fn SetKind(&self, value: ContactAddressKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactAddress";
}
#[cfg(feature = "implement_exclusive")]
impl IContactAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAddressImpl, const OFFSET: isize>() -> IContactAddressVtbl {
        unsafe extern "system" fn StreetAddress<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreetAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreetAddress<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreetAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Locality<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocality<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocality(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Region<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Region() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegion<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Country<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Country() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCountry<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCountry(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PostalCode<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalCode<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalCode(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Kind<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactAddressKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKind<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactAddressKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        unsafe extern "system" fn Description<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IContactAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContactAddress>,
            ::windows::core::GetTrustLevel,
            StreetAddress::<Impl, OFFSET>,
            SetStreetAddress::<Impl, OFFSET>,
            Locality::<Impl, OFFSET>,
            SetLocality::<Impl, OFFSET>,
            Region::<Impl, OFFSET>,
            SetRegion::<Impl, OFFSET>,
            Country::<Impl, OFFSET>,
            SetCountry::<Impl, OFFSET>,
            PostalCode::<Impl, OFFSET>,
            SetPostalCode::<Impl, OFFSET>,
            Kind::<Impl, OFFSET>,
            SetKind::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AnnotationListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportedOperations(&self) -> ::windows::core::Result<ContactAnnotationOperations>;
    fn SetSupportedOperations(&self, value: ContactAnnotationOperations) -> ::windows::core::Result<()>;
    fn IsDisabled(&self) -> ::windows::core::Result<bool>;
    fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactAnnotation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactAnnotation";
}
#[cfg(feature = "implement_exclusive")]
impl IContactAnnotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAnnotationImpl, const OFFSET: isize>() -> IContactAnnotationVtbl {
        unsafe extern "system" fn Id<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AnnotationListId<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationListId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactId<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContactId<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteId<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteId<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedOperations<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactAnnotationOperations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedOperations<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactAnnotationOperations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportedOperations(value).into()
        }
        unsafe extern "system" fn IsDisabled<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderProperties<Impl: IContactAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderProperties() {
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
            ::windows::core::GetRuntimeClassName::<IContactAnnotation>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            AnnotationListId::<Impl, OFFSET>,
            ContactId::<Impl, OFFSET>,
            SetContactId::<Impl, OFFSET>,
            RemoteId::<Impl, OFFSET>,
            SetRemoteId::<Impl, OFFSET>,
            SupportedOperations::<Impl, OFFSET>,
            SetSupportedOperations::<Impl, OFFSET>,
            IsDisabled::<Impl, OFFSET>,
            ProviderProperties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotation2Impl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactListId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactAnnotation2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactAnnotation2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactAnnotation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAnnotation2Impl, const OFFSET: isize>() -> IContactAnnotation2Vtbl {
        unsafe extern "system" fn ContactListId<Impl: IContactAnnotation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContactListId<Impl: IContactAnnotation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactListId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactAnnotation2>, ::windows::core::GetTrustLevel, ContactListId::<Impl, OFFSET>, SetContactListId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationListImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrySaveAnnotationAsync(&self, annotation: &::core::option::Option<ContactAnnotation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetAnnotationAsync(&self, annotationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotation>>;
    fn FindAnnotationsByRemoteIdAsync(&self, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
    fn FindAnnotationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
    fn DeleteAnnotationAsync(&self, annotation: &::core::option::Option<ContactAnnotation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactAnnotationList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactAnnotationList";
}
#[cfg(feature = "implement_exclusive")]
impl IContactAnnotationListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAnnotationListImpl, const OFFSET: isize>() -> IContactAnnotationListVtbl {
        unsafe extern "system" fn Id<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderPackageFamilyName<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDataAccountId<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteAsync<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySaveAnnotationAsync<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySaveAnnotationAsync(&*(&annotation as *const <ContactAnnotation as ::windows::core::Abi>::Abi as *const <ContactAnnotation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationAsync<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationAsync(&*(&annotationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAnnotationsByRemoteIdAsync<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAnnotationsByRemoteIdAsync(&*(&remoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAnnotationsAsync<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAnnotationsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAnnotationAsync<Impl: IContactAnnotationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAnnotationAsync(&*(&annotation as *const <ContactAnnotation as ::windows::core::Abi>::Abi as *const <ContactAnnotation as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IContactAnnotationList>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            ProviderPackageFamilyName::<Impl, OFFSET>,
            UserDataAccountId::<Impl, OFFSET>,
            DeleteAsync::<Impl, OFFSET>,
            TrySaveAnnotationAsync::<Impl, OFFSET>,
            GetAnnotationAsync::<Impl, OFFSET>,
            FindAnnotationsByRemoteIdAsync::<Impl, OFFSET>,
            FindAnnotationsAsync::<Impl, OFFSET>,
            DeleteAnnotationAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationStoreImpl: Sized {
    fn FindContactIdsByEmailAsync(&self, emailaddress: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn FindContactIdsByPhoneNumberAsync(&self, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn FindAnnotationsForContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
    fn DisableAnnotationAsync(&self, annotation: &::core::option::Option<ContactAnnotation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateAnnotationListAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>;
    fn CreateAnnotationListInAccountAsync(&self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>;
    fn GetAnnotationListAsync(&self, annotationlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>;
    fn FindAnnotationListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotationList>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactAnnotationStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactAnnotationStore";
}
#[cfg(feature = "implement_exclusive")]
impl IContactAnnotationStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAnnotationStoreImpl, const OFFSET: isize>() -> IContactAnnotationStoreVtbl {
        unsafe extern "system" fn FindContactIdsByEmailAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emailaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContactIdsByEmailAsync(&*(&emailaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindContactIdsByPhoneNumberAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContactIdsByPhoneNumberAsync(&*(&phonenumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAnnotationsForContactAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAnnotationsForContactAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAnnotationAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableAnnotationAsync(&*(&annotation as *const <ContactAnnotation as ::windows::core::Abi>::Abi as *const <ContactAnnotation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnnotationListAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnnotationListAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnnotationListInAccountAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnnotationListInAccountAsync(&*(&userdataaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationListAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationlistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationListAsync(&*(&annotationlistid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAnnotationListsAsync<Impl: IContactAnnotationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAnnotationListsAsync() {
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
            ::windows::core::GetRuntimeClassName::<IContactAnnotationStore>,
            ::windows::core::GetTrustLevel,
            FindContactIdsByEmailAsync::<Impl, OFFSET>,
            FindContactIdsByPhoneNumberAsync::<Impl, OFFSET>,
            FindAnnotationsForContactAsync::<Impl, OFFSET>,
            DisableAnnotationAsync::<Impl, OFFSET>,
            CreateAnnotationListAsync::<Impl, OFFSET>,
            CreateAnnotationListInAccountAsync::<Impl, OFFSET>,
            GetAnnotationListAsync::<Impl, OFFSET>,
            FindAnnotationListsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationStore2Impl: Sized {
    fn FindAnnotationsForContactListAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactAnnotationStore2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactAnnotationStore2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactAnnotationStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAnnotationStore2Impl, const OFFSET: isize>() -> IContactAnnotationStore2Vtbl {
        unsafe extern "system" fn FindAnnotationsForContactListAsync<Impl: IContactAnnotationStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactlistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAnnotationsForContactListAsync(&*(&contactlistid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactAnnotationStore2>, ::windows::core::GetTrustLevel, FindAnnotationsForContactListAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactBatchImpl: Sized {
    fn Contacts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Contact>>;
    fn Status(&self) -> ::windows::core::Result<ContactBatchStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactBatch";
}
#[cfg(feature = "implement_exclusive")]
impl IContactBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactBatchImpl, const OFFSET: isize>() -> IContactBatchVtbl {
        unsafe extern "system" fn Contacts<Impl: IContactBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IContactBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactBatchStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactBatch>, ::windows::core::GetTrustLevel, Contacts::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactCardDelayedDataLoaderImpl: Sized + IClosableImpl {
    fn SetData(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactCardDelayedDataLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactCardDelayedDataLoader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactCardDelayedDataLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactCardDelayedDataLoaderImpl, const OFFSET: isize>() -> IContactCardDelayedDataLoaderVtbl {
        unsafe extern "system" fn SetData<Impl: IContactCardDelayedDataLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactCardDelayedDataLoader>, ::windows::core::GetTrustLevel, SetData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactCardOptionsImpl: Sized {
    fn HeaderKind(&self) -> ::windows::core::Result<ContactCardHeaderKind>;
    fn SetHeaderKind(&self, value: ContactCardHeaderKind) -> ::windows::core::Result<()>;
    fn InitialTabKind(&self) -> ::windows::core::Result<ContactCardTabKind>;
    fn SetInitialTabKind(&self, value: ContactCardTabKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactCardOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IContactCardOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactCardOptionsImpl, const OFFSET: isize>() -> IContactCardOptionsVtbl {
        unsafe extern "system" fn HeaderKind<Impl: IContactCardOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactCardHeaderKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderKind<Impl: IContactCardOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactCardHeaderKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeaderKind(value).into()
        }
        unsafe extern "system" fn InitialTabKind<Impl: IContactCardOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactCardTabKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialTabKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialTabKind<Impl: IContactCardOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactCardTabKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialTabKind(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactCardOptions>, ::windows::core::GetTrustLevel, HeaderKind::<Impl, OFFSET>, SetHeaderKind::<Impl, OFFSET>, InitialTabKind::<Impl, OFFSET>, SetInitialTabKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactCardOptions2Impl: Sized + IContactCardOptionsImpl {
    fn ServerSearchContactListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactCardOptions2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactCardOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactCardOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactCardOptions2Impl, const OFFSET: isize>() -> IContactCardOptions2Vtbl {
        unsafe extern "system" fn ServerSearchContactListIds<Impl: IContactCardOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerSearchContactListIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactCardOptions2>, ::windows::core::GetTrustLevel, ServerSearchContactListIds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<ContactChangeType>;
    fn Contact(&self) -> ::windows::core::Result<Contact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactChange {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactChange";
}
#[cfg(feature = "implement_exclusive")]
impl IContactChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactChangeImpl, const OFFSET: isize>() -> IContactChangeVtbl {
        unsafe extern "system" fn ChangeType<Impl: IContactChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactChangeType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Contact<Impl: IContactChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactChange>, ::windows::core::GetTrustLevel, ChangeType::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeReaderImpl: Sized {
    fn AcceptChanges(&self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&self, lastchangetoaccept: &::core::option::Option<ContactChange>) -> ::windows::core::Result<()>;
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChange>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactChangeReader";
}
#[cfg(feature = "implement_exclusive")]
impl IContactChangeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactChangeReaderImpl, const OFFSET: isize>() -> IContactChangeReaderVtbl {
        unsafe extern "system" fn AcceptChanges<Impl: IContactChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChanges().into()
        }
        unsafe extern "system" fn AcceptChangesThrough<Impl: IContactChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchangetoaccept: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptChangesThrough(&*(&lastchangetoaccept as *const <ContactChange as ::windows::core::Abi>::Abi as *const <ContactChange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadBatchAsync<Impl: IContactChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactChangeReader>, ::windows::core::GetTrustLevel, AcceptChanges::<Impl, OFFSET>, AcceptChangesThrough::<Impl, OFFSET>, ReadBatchAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeTrackerImpl: Sized {
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn GetChangeReader(&self) -> ::windows::core::Result<ContactChangeReader>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactChangeTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IContactChangeTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactChangeTrackerImpl, const OFFSET: isize>() -> IContactChangeTrackerVtbl {
        unsafe extern "system" fn Enable<Impl: IContactChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn GetChangeReader<Impl: IContactChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Reset<Impl: IContactChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactChangeTracker>, ::windows::core::GetTrustLevel, Enable::<Impl, OFFSET>, GetChangeReader::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeTracker2Impl: Sized {
    fn IsTracking(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactChangeTracker2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactChangeTracker2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactChangeTracker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactChangeTracker2Impl, const OFFSET: isize>() -> IContactChangeTracker2Vtbl {
        unsafe extern "system" fn IsTracking<Impl: IContactChangeTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactChangeTracker2>, ::windows::core::GetTrustLevel, IsTracking::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactChangedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IContactChangedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactChangedDeferralImpl, const OFFSET: isize>() -> IContactChangedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IContactChangedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactChangedDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<ContactChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactChangedEventArgsImpl, const OFFSET: isize>() -> IContactChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IContactChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactChangedEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactConnectedServiceAccountImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactConnectedServiceAccount {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactConnectedServiceAccount";
}
#[cfg(feature = "implement_exclusive")]
impl IContactConnectedServiceAccountVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactConnectedServiceAccountImpl, const OFFSET: isize>() -> IContactConnectedServiceAccountVtbl {
        unsafe extern "system" fn Id<Impl: IContactConnectedServiceAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IContactConnectedServiceAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceName<Impl: IContactConnectedServiceAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceName<Impl: IContactConnectedServiceAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactConnectedServiceAccount>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, SetId::<Impl, OFFSET>, ServiceName::<Impl, OFFSET>, SetServiceName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDateImpl: Sized {
    fn Day(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetDay(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Month(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetMonth(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Year(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetYear(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactDateKind>;
    fn SetKind(&self, value: ContactDateKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactDate {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactDate";
}
#[cfg(feature = "implement_exclusive")]
impl IContactDateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactDateImpl, const OFFSET: isize>() -> IContactDateVtbl {
        unsafe extern "system" fn Day<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Day() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDay(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Month<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Month() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonth(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Year<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Year() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYear<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYear(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Kind<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactDateKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKind<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactDateKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        unsafe extern "system" fn Description<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IContactDateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContactDate>,
            ::windows::core::GetTrustLevel,
            Day::<Impl, OFFSET>,
            SetDay::<Impl, OFFSET>,
            Month::<Impl, OFFSET>,
            SetMonth::<Impl, OFFSET>,
            Year::<Impl, OFFSET>,
            SetYear::<Impl, OFFSET>,
            Kind::<Impl, OFFSET>,
            SetKind::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactEmailImpl: Sized {
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactEmailKind>;
    fn SetKind(&self, value: ContactEmailKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactEmail {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactEmail";
}
#[cfg(feature = "implement_exclusive")]
impl IContactEmailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactEmailImpl, const OFFSET: isize>() -> IContactEmailVtbl {
        unsafe extern "system" fn Address<Impl: IContactEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAddress<Impl: IContactEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Kind<Impl: IContactEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactEmailKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKind<Impl: IContactEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactEmailKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        unsafe extern "system" fn Description<Impl: IContactEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IContactEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactEmail>, ::windows::core::GetTrustLevel, Address::<Impl, OFFSET>, SetAddress::<Impl, OFFSET>, Kind::<Impl, OFFSET>, SetKind::<Impl, OFFSET>, Description::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>)
    }
}
pub trait IContactFieldImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<ContactFieldType>;
    fn Category(&self) -> ::windows::core::Result<ContactFieldCategory>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactField";
}
impl IContactFieldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactFieldImpl, const OFFSET: isize>() -> IContactFieldVtbl {
        unsafe extern "system" fn Type<Impl: IContactFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Category<Impl: IContactFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IContactFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IContactFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactField>, ::windows::core::GetTrustLevel, Type::<Impl, OFFSET>, Category::<Impl, OFFSET>, Name::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
pub trait IContactFieldFactoryImpl: Sized {
    fn CreateField_Default(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType) -> ::windows::core::Result<ContactField>;
    fn CreateField_Category(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField>;
    fn CreateField_Custom(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField>;
}
impl ::windows::core::RuntimeName for IContactFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactFieldFactory";
}
impl IContactFieldFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactFieldFactoryImpl, const OFFSET: isize>() -> IContactFieldFactoryVtbl {
        unsafe extern "system" fn CreateField_Default<Impl: IContactFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ContactFieldType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateField_Default(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateField_Category<Impl: IContactFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateField_Category(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), r#type, category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateField_Custom<Impl: IContactFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateField_Custom(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), r#type, category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactFieldFactory>, ::windows::core::GetTrustLevel, CreateField_Default::<Impl, OFFSET>, CreateField_Category::<Impl, OFFSET>, CreateField_Custom::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactGroupImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IContactGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactGroupImpl, const OFFSET: isize>() -> IContactGroupVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactGroup>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactInformationImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
    fn Emails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
    fn PhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
    fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactLocationField>>;
    fn InstantMessages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactInstantMessageField>>;
    fn CustomFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
    fn QueryCustomFields(&self, customname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IContactInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformationImpl, const OFFSET: isize>() -> IContactInformationVtbl {
        unsafe extern "system" fn Name<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetThumbnailAsync<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Emails<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Emails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneNumbers<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstantMessages<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstantMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomFields<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomFields() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCustomFields<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCustomFields(&*(&customname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IContactInformation>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            GetThumbnailAsync::<Impl, OFFSET>,
            Emails::<Impl, OFFSET>,
            PhoneNumbers::<Impl, OFFSET>,
            Locations::<Impl, OFFSET>,
            InstantMessages::<Impl, OFFSET>,
            CustomFields::<Impl, OFFSET>,
            QueryCustomFields::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactInstantMessageFieldImpl: Sized + IContactFieldImpl {
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactInstantMessageField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactInstantMessageField";
}
#[cfg(feature = "implement_exclusive")]
impl IContactInstantMessageFieldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInstantMessageFieldImpl, const OFFSET: isize>() -> IContactInstantMessageFieldVtbl {
        unsafe extern "system" fn UserName<Impl: IContactInstantMessageFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Impl: IContactInstantMessageFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayText<Impl: IContactInstantMessageFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUri<Impl: IContactInstantMessageFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactInstantMessageField>, ::windows::core::GetTrustLevel, UserName::<Impl, OFFSET>, Service::<Impl, OFFSET>, DisplayText::<Impl, OFFSET>, LaunchUri::<Impl, OFFSET>)
    }
}
pub trait IContactInstantMessageFieldFactoryImpl: Sized {
    fn CreateInstantMessage_Default(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_Category(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_All(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory, service: &::windows::core::HSTRING, displaytext: &::windows::core::HSTRING, verb: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<ContactInstantMessageField>;
}
impl ::windows::core::RuntimeName for IContactInstantMessageFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactInstantMessageFieldFactory";
}
impl IContactInstantMessageFieldFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInstantMessageFieldFactoryImpl, const OFFSET: isize>() -> IContactInstantMessageFieldFactoryVtbl {
        unsafe extern "system" fn CreateInstantMessage_Default<Impl: IContactInstantMessageFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstantMessage_Default(&*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantMessage_Category<Impl: IContactInstantMessageFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstantMessage_Category(&*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantMessage_All<Impl: IContactInstantMessageFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, verb: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstantMessage_All(
                &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                category,
                &*(&service as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displaytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&verb as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactInstantMessageFieldFactory>, ::windows::core::GetTrustLevel, CreateInstantMessage_Default::<Impl, OFFSET>, CreateInstantMessage_Category::<Impl, OFFSET>, CreateInstantMessage_All::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactJobInfoImpl: Sized {
    fn CompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CompanyYomiName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyYomiName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Department(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDepartment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Manager(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetManager(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Office(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOffice(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CompanyAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactJobInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactJobInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IContactJobInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactJobInfoImpl, const OFFSET: isize>() -> IContactJobInfoVtbl {
        unsafe extern "system" fn CompanyName<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompanyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompanyName<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompanyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompanyYomiName<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompanyYomiName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompanyYomiName<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompanyYomiName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Department<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Department() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepartment(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Manager<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManager<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManager(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Office<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Office() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffice<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffice(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompanyAddress<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompanyAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompanyAddress<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompanyAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IContactJobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContactJobInfo>,
            ::windows::core::GetTrustLevel,
            CompanyName::<Impl, OFFSET>,
            SetCompanyName::<Impl, OFFSET>,
            CompanyYomiName::<Impl, OFFSET>,
            SetCompanyYomiName::<Impl, OFFSET>,
            Department::<Impl, OFFSET>,
            SetDepartment::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            Manager::<Impl, OFFSET>,
            SetManager::<Impl, OFFSET>,
            Office::<Impl, OFFSET>,
            SetOffice::<Impl, OFFSET>,
            CompanyAddress::<Impl, OFFSET>,
            SetCompanyAddress::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactLaunchActionVerbsStaticsImpl: Sized {
    fn Call(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Map(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Post(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoCall(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactLaunchActionVerbsStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactLaunchActionVerbsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContactLaunchActionVerbsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactLaunchActionVerbsStaticsImpl, const OFFSET: isize>() -> IContactLaunchActionVerbsStaticsVtbl {
        unsafe extern "system" fn Call<Impl: IContactLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IContactLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Map<Impl: IContactLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Post<Impl: IContactLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Post() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoCall<Impl: IContactLaunchActionVerbsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoCall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactLaunchActionVerbsStatics>, ::windows::core::GetTrustLevel, Call::<Impl, OFFSET>, Message::<Impl, OFFSET>, Map::<Impl, OFFSET>, Post::<Impl, OFFSET>, VideoCall::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsHidden(&self) -> ::windows::core::Result<bool>;
    fn SetIsHidden(&self, value: bool) -> ::windows::core::Result<()>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<ContactListOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: ContactListOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&self) -> ::windows::core::Result<ContactListOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&self, value: ContactListOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn ChangeTracker(&self) -> ::windows::core::Result<ContactChangeTracker>;
    fn SyncManager(&self) -> ::windows::core::Result<ContactListSyncManager>;
    fn SupportsServerSearch(&self) -> ::windows::core::Result<bool>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactList, ContactChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContactChanged(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetContactFromRemoteIdAsync(&self, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn GetMeContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn GetContactReader(&self) -> ::windows::core::Result<ContactReader>;
    fn GetContactReaderWithOptions(&self, options: &::core::option::Option<ContactQueryOptions>) -> ::windows::core::Result<ContactReader>;
    fn SaveContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactList";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListImpl, const OFFSET: isize>() -> IContactListVtbl {
        unsafe extern "system" fn Id<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceDisplayName<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsHidden<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHidden(value).into()
        }
        unsafe extern "system" fn OtherAppReadAccess<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactListOtherAppReadAccess) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOtherAppReadAccess<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactListOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppReadAccess(value).into()
        }
        unsafe extern "system" fn OtherAppWriteAccess<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactListOtherAppWriteAccess) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOtherAppWriteAccess<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactListOtherAppWriteAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppWriteAccess(value).into()
        }
        unsafe extern "system" fn ChangeTracker<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SyncManager<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportsServerSearch<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsServerSearch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDataAccountId<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContactChanged<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<ContactList, ContactChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ContactList, ContactChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContactChanged<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContactChanged(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SaveAsync<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteAsync<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetContactFromRemoteIdAsync<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactFromRemoteIdAsync(&*(&remoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeContactAsync<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeContactAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactReader<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactReaderWithOptions<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactReaderWithOptions(&*(&options as *const <ContactQueryOptions as ::windows::core::Abi>::Abi as *const <ContactQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveContactAsync<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveContactAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteContactAsync<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteContactAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactAsync<Impl: IContactListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactAsync(&*(&contactid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IContactList>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            DisplayName::<Impl, OFFSET>,
            SetDisplayName::<Impl, OFFSET>,
            SourceDisplayName::<Impl, OFFSET>,
            IsHidden::<Impl, OFFSET>,
            SetIsHidden::<Impl, OFFSET>,
            OtherAppReadAccess::<Impl, OFFSET>,
            SetOtherAppReadAccess::<Impl, OFFSET>,
            OtherAppWriteAccess::<Impl, OFFSET>,
            SetOtherAppWriteAccess::<Impl, OFFSET>,
            ChangeTracker::<Impl, OFFSET>,
            SyncManager::<Impl, OFFSET>,
            SupportsServerSearch::<Impl, OFFSET>,
            UserDataAccountId::<Impl, OFFSET>,
            ContactChanged::<Impl, OFFSET>,
            RemoveContactChanged::<Impl, OFFSET>,
            SaveAsync::<Impl, OFFSET>,
            DeleteAsync::<Impl, OFFSET>,
            GetContactFromRemoteIdAsync::<Impl, OFFSET>,
            GetMeContactAsync::<Impl, OFFSET>,
            GetContactReader::<Impl, OFFSET>,
            GetContactReaderWithOptions::<Impl, OFFSET>,
            SaveContactAsync::<Impl, OFFSET>,
            DeleteContactAsync::<Impl, OFFSET>,
            GetContactAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactList2Impl: Sized {
    fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetSupportsServerSearch(&self, value: bool) -> ::windows::core::Result<()>;
    fn SyncConstraints(&self) -> ::windows::core::Result<ContactListSyncConstraints>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactList2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactList2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactList2Impl, const OFFSET: isize>() -> IContactList2Vtbl {
        unsafe extern "system" fn RegisterSyncManagerAsync<Impl: IContactList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSupportsServerSearch<Impl: IContactList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportsServerSearch(value).into()
        }
        unsafe extern "system" fn SyncConstraints<Impl: IContactList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactList2>, ::windows::core::GetTrustLevel, RegisterSyncManagerAsync::<Impl, OFFSET>, SetSupportsServerSearch::<Impl, OFFSET>, SyncConstraints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactList3Impl: Sized {
    fn LimitedWriteOperations(&self) -> ::windows::core::Result<ContactListLimitedWriteOperations>;
    fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ContactChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactList3 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactList3";
}
#[cfg(feature = "implement_exclusive")]
impl IContactList3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactList3Impl, const OFFSET: isize>() -> IContactList3Vtbl {
        unsafe extern "system" fn LimitedWriteOperations<Impl: IContactList3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LimitedWriteOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeTracker<Impl: IContactList3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactList3>, ::windows::core::GetTrustLevel, LimitedWriteOperations::<Impl, OFFSET>, GetChangeTracker::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListLimitedWriteOperationsImpl: Sized {
    fn TryCreateOrUpdateContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDeleteContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactListLimitedWriteOperations";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListLimitedWriteOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListLimitedWriteOperationsImpl, const OFFSET: isize>() -> IContactListLimitedWriteOperationsVtbl {
        unsafe extern "system" fn TryCreateOrUpdateContactAsync<Impl: IContactListLimitedWriteOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateOrUpdateContactAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDeleteContactAsync<Impl: IContactListLimitedWriteOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDeleteContactAsync(&*(&contactid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListLimitedWriteOperations>, ::windows::core::GetTrustLevel, TryCreateOrUpdateContactAsync::<Impl, OFFSET>, TryDeleteContactAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncConstraintsImpl: Sized {
    fn CanSyncDescriptions(&self) -> ::windows::core::Result<bool>;
    fn SetCanSyncDescriptions(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxHomePhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxHomePhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxMobilePhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxMobilePhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWorkPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWorkPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxPagerPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxPagerPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxBusinessFaxPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxBusinessFaxPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxHomeFaxPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxHomeFaxPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxCompanyPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxCompanyPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxAssistantPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxAssistantPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxRadioPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxRadioPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxPersonalEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxPersonalEmailAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWorkEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWorkEmailAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherEmailAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxHomeAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxHomeAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWorkAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWorkAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxBirthdayDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxBirthdayDates(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxAnniversaryDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxAnniversaryDates(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherDates(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxSpouseRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxSpouseRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxPartnerRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxPartnerRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxSiblingRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxSiblingRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxParentRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxParentRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxChildRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxChildRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxJobInfo(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxJobInfo(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWebsites(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWebsites(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListSyncConstraints {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactListSyncConstraints";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListSyncConstraintsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>() -> IContactListSyncConstraintsVtbl {
        unsafe extern "system" fn CanSyncDescriptions<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSyncDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanSyncDescriptions<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanSyncDescriptions(value).into()
        }
        unsafe extern "system" fn MaxHomePhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxHomePhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHomePhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxHomePhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxMobilePhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxMobilePhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxMobilePhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxMobilePhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxWorkPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxWorkPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxWorkPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxWorkPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxOtherPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxOtherPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxOtherPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxOtherPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxPagerPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPagerPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPagerPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPagerPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxBusinessFaxPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBusinessFaxPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBusinessFaxPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBusinessFaxPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxHomeFaxPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxHomeFaxPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHomeFaxPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxHomeFaxPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxCompanyPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxCompanyPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCompanyPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxCompanyPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxAssistantPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAssistantPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxAssistantPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxAssistantPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxRadioPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxRadioPhoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRadioPhoneNumbers<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxRadioPhoneNumbers(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxPersonalEmailAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPersonalEmailAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPersonalEmailAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPersonalEmailAddresses(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxWorkEmailAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxWorkEmailAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxWorkEmailAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxWorkEmailAddresses(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxOtherEmailAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxOtherEmailAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxOtherEmailAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxOtherEmailAddresses(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxHomeAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxHomeAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHomeAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxHomeAddresses(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxWorkAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxWorkAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxWorkAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxWorkAddresses(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxOtherAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxOtherAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxOtherAddresses<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxOtherAddresses(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxBirthdayDates<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBirthdayDates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBirthdayDates<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBirthdayDates(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxAnniversaryDates<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAnniversaryDates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxAnniversaryDates<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxAnniversaryDates(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxOtherDates<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxOtherDates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxOtherDates<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxOtherDates(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxOtherRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxOtherRelationships() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxOtherRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxOtherRelationships(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSpouseRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSpouseRelationships() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSpouseRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSpouseRelationships(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxPartnerRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPartnerRelationships() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPartnerRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPartnerRelationships(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSiblingRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSiblingRelationships() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSiblingRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSiblingRelationships(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxParentRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxParentRelationships() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxParentRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxParentRelationships(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxChildRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxChildRelationships() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxChildRelationships<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxChildRelationships(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxJobInfo<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxJobInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxJobInfo<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxJobInfo(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxWebsites<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxWebsites() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxWebsites<Impl: IContactListSyncConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxWebsites(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContactListSyncConstraints>,
            ::windows::core::GetTrustLevel,
            CanSyncDescriptions::<Impl, OFFSET>,
            SetCanSyncDescriptions::<Impl, OFFSET>,
            MaxHomePhoneNumbers::<Impl, OFFSET>,
            SetMaxHomePhoneNumbers::<Impl, OFFSET>,
            MaxMobilePhoneNumbers::<Impl, OFFSET>,
            SetMaxMobilePhoneNumbers::<Impl, OFFSET>,
            MaxWorkPhoneNumbers::<Impl, OFFSET>,
            SetMaxWorkPhoneNumbers::<Impl, OFFSET>,
            MaxOtherPhoneNumbers::<Impl, OFFSET>,
            SetMaxOtherPhoneNumbers::<Impl, OFFSET>,
            MaxPagerPhoneNumbers::<Impl, OFFSET>,
            SetMaxPagerPhoneNumbers::<Impl, OFFSET>,
            MaxBusinessFaxPhoneNumbers::<Impl, OFFSET>,
            SetMaxBusinessFaxPhoneNumbers::<Impl, OFFSET>,
            MaxHomeFaxPhoneNumbers::<Impl, OFFSET>,
            SetMaxHomeFaxPhoneNumbers::<Impl, OFFSET>,
            MaxCompanyPhoneNumbers::<Impl, OFFSET>,
            SetMaxCompanyPhoneNumbers::<Impl, OFFSET>,
            MaxAssistantPhoneNumbers::<Impl, OFFSET>,
            SetMaxAssistantPhoneNumbers::<Impl, OFFSET>,
            MaxRadioPhoneNumbers::<Impl, OFFSET>,
            SetMaxRadioPhoneNumbers::<Impl, OFFSET>,
            MaxPersonalEmailAddresses::<Impl, OFFSET>,
            SetMaxPersonalEmailAddresses::<Impl, OFFSET>,
            MaxWorkEmailAddresses::<Impl, OFFSET>,
            SetMaxWorkEmailAddresses::<Impl, OFFSET>,
            MaxOtherEmailAddresses::<Impl, OFFSET>,
            SetMaxOtherEmailAddresses::<Impl, OFFSET>,
            MaxHomeAddresses::<Impl, OFFSET>,
            SetMaxHomeAddresses::<Impl, OFFSET>,
            MaxWorkAddresses::<Impl, OFFSET>,
            SetMaxWorkAddresses::<Impl, OFFSET>,
            MaxOtherAddresses::<Impl, OFFSET>,
            SetMaxOtherAddresses::<Impl, OFFSET>,
            MaxBirthdayDates::<Impl, OFFSET>,
            SetMaxBirthdayDates::<Impl, OFFSET>,
            MaxAnniversaryDates::<Impl, OFFSET>,
            SetMaxAnniversaryDates::<Impl, OFFSET>,
            MaxOtherDates::<Impl, OFFSET>,
            SetMaxOtherDates::<Impl, OFFSET>,
            MaxOtherRelationships::<Impl, OFFSET>,
            SetMaxOtherRelationships::<Impl, OFFSET>,
            MaxSpouseRelationships::<Impl, OFFSET>,
            SetMaxSpouseRelationships::<Impl, OFFSET>,
            MaxPartnerRelationships::<Impl, OFFSET>,
            SetMaxPartnerRelationships::<Impl, OFFSET>,
            MaxSiblingRelationships::<Impl, OFFSET>,
            SetMaxSiblingRelationships::<Impl, OFFSET>,
            MaxParentRelationships::<Impl, OFFSET>,
            SetMaxParentRelationships::<Impl, OFFSET>,
            MaxChildRelationships::<Impl, OFFSET>,
            SetMaxChildRelationships::<Impl, OFFSET>,
            MaxJobInfo::<Impl, OFFSET>,
            SetMaxJobInfo::<Impl, OFFSET>,
            MaxWebsites::<Impl, OFFSET>,
            SetMaxWebsites::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ContactListSyncStatus>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactListSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactListSyncManager";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListSyncManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListSyncManagerImpl, const OFFSET: isize>() -> IContactListSyncManagerVtbl {
        unsafe extern "system" fn Status<Impl: IContactListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactListSyncStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LastSuccessfulSyncTime<Impl: IContactListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LastAttemptedSyncTime<Impl: IContactListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SyncAsync<Impl: IContactListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SyncStatusChanged<Impl: IContactListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ContactListSyncManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ContactListSyncManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSyncStatusChanged<Impl: IContactListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSyncStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListSyncManager>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, LastSuccessfulSyncTime::<Impl, OFFSET>, LastAttemptedSyncTime::<Impl, OFFSET>, SyncAsync::<Impl, OFFSET>, SyncStatusChanged::<Impl, OFFSET>, RemoveSyncStatusChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManager2Impl: Sized {
    fn SetStatus(&self, value: ContactListSyncStatus) -> ::windows::core::Result<()>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetLastAttemptedSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactListSyncManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactListSyncManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactListSyncManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactListSyncManager2Impl, const OFFSET: isize>() -> IContactListSyncManager2Vtbl {
        unsafe extern "system" fn SetStatus<Impl: IContactListSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactListSyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn SetLastSuccessfulSyncTime<Impl: IContactListSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastSuccessfulSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetLastAttemptedSyncTime<Impl: IContactListSyncManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastAttemptedSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactListSyncManager2>, ::windows::core::GetTrustLevel, SetStatus::<Impl, OFFSET>, SetLastSuccessfulSyncTime::<Impl, OFFSET>, SetLastAttemptedSyncTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactLocationFieldImpl: Sized + IContactFieldImpl {
    fn UnstructuredAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactLocationField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactLocationField";
}
#[cfg(feature = "implement_exclusive")]
impl IContactLocationFieldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactLocationFieldImpl, const OFFSET: isize>() -> IContactLocationFieldVtbl {
        unsafe extern "system" fn UnstructuredAddress<Impl: IContactLocationFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnstructuredAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Street<Impl: IContactLocationFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Street() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Impl: IContactLocationFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Region<Impl: IContactLocationFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Region() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Country<Impl: IContactLocationFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Country() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Impl: IContactLocationFieldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactLocationField>, ::windows::core::GetTrustLevel, UnstructuredAddress::<Impl, OFFSET>, Street::<Impl, OFFSET>, City::<Impl, OFFSET>, Region::<Impl, OFFSET>, Country::<Impl, OFFSET>, PostalCode::<Impl, OFFSET>)
    }
}
pub trait IContactLocationFieldFactoryImpl: Sized {
    fn CreateLocation_Default(&self, unstructuredaddress: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField>;
    fn CreateLocation_Category(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactLocationField>;
    fn CreateLocation_All(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory, street: &::windows::core::HSTRING, city: &::windows::core::HSTRING, region: &::windows::core::HSTRING, country: &::windows::core::HSTRING, postalcode: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField>;
}
impl ::windows::core::RuntimeName for IContactLocationFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactLocationFieldFactory";
}
impl IContactLocationFieldFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactLocationFieldFactoryImpl, const OFFSET: isize>() -> IContactLocationFieldFactoryVtbl {
        unsafe extern "system" fn CreateLocation_Default<Impl: IContactLocationFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLocation_Default(&*(&unstructuredaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLocation_Category<Impl: IContactLocationFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLocation_Category(&*(&unstructuredaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLocation_All<Impl: IContactLocationFieldFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, street: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, city: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, region: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, country: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, postalcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLocation_All(
                &*(&unstructuredaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                category,
                &*(&street as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&city as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&region as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&country as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&postalcode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactLocationFieldFactory>, ::windows::core::GetTrustLevel, CreateLocation_Default::<Impl, OFFSET>, CreateLocation_Category::<Impl, OFFSET>, CreateLocation_All::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerForUserImpl: Sized {
    fn ConvertContactToVCardAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertContactToVCardAsyncWithMaxBytes(&self, contact: &::core::option::Option<Contact>, maxbytes: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertVCardToContactAsync(&self, vcard: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn RequestStoreAsync(&self, accesstype: ContactStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
    fn RequestAnnotationStoreAsync(&self, accesstype: ContactAnnotationStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>>;
    fn SystemDisplayNameOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemDisplayNameOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
    fn SystemSortOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemSortOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactManagerForUser";
}
#[cfg(feature = "implement_exclusive")]
impl IContactManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerForUserImpl, const OFFSET: isize>() -> IContactManagerForUserVtbl {
        unsafe extern "system" fn ConvertContactToVCardAsync<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertContactToVCardAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertContactToVCardAsyncWithMaxBytes<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, maxbytes: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertContactToVCardAsyncWithMaxBytes(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), maxbytes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVCardToContactAsync<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcard: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertVCardToContactAsync(&*(&vcard as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStoreAsync<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: ContactStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAnnotationStoreAsync<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: ContactAnnotationStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAnnotationStoreAsync(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemDisplayNameOrder<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemDisplayNameOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemDisplayNameOrder<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemDisplayNameOrder(value).into()
        }
        unsafe extern "system" fn SystemSortOrder<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemSortOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemSortOrder<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemSortOrder(value).into()
        }
        unsafe extern "system" fn User<Impl: IContactManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContactManagerForUser>,
            ::windows::core::GetTrustLevel,
            ConvertContactToVCardAsync::<Impl, OFFSET>,
            ConvertContactToVCardAsyncWithMaxBytes::<Impl, OFFSET>,
            ConvertVCardToContactAsync::<Impl, OFFSET>,
            RequestStoreAsync::<Impl, OFFSET>,
            RequestAnnotationStoreAsync::<Impl, OFFSET>,
            SystemDisplayNameOrder::<Impl, OFFSET>,
            SetSystemDisplayNameOrder::<Impl, OFFSET>,
            SystemSortOrder::<Impl, OFFSET>,
            SetSystemSortOrder::<Impl, OFFSET>,
            User::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerForUser2Impl: Sized {
    fn ShowFullContactCard(&self, contact: &::core::option::Option<Contact>, fullcontactcardoptions: &::core::option::Option<FullContactCardOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactManagerForUser2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactManagerForUser2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactManagerForUser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerForUser2Impl, const OFFSET: isize>() -> IContactManagerForUser2Vtbl {
        unsafe extern "system" fn ShowFullContactCard<Impl: IContactManagerForUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, fullcontactcardoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowFullContactCard(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), &*(&fullcontactcardoptions as *const <FullContactCardOptions as ::windows::core::Abi>::Abi as *const <FullContactCardOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactManagerForUser2>, ::windows::core::GetTrustLevel, ShowFullContactCard::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStaticsImpl: Sized {
    fn ShowContactCard(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowContactCardWithPlacement(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn ShowDelayLoadedContactCard(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<ContactCardDelayedDataLoader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContactManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerStaticsImpl, const OFFSET: isize>() -> IContactManagerStaticsVtbl {
        unsafe extern "system" fn ShowContactCard<Impl: IContactManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContactCard(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowContactCardWithPlacement<Impl: IContactManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContactCardWithPlacement(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement).into()
        }
        unsafe extern "system" fn ShowDelayLoadedContactCard<Impl: IContactManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowDelayLoadedContactCard(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactManagerStatics>, ::windows::core::GetTrustLevel, ShowContactCard::<Impl, OFFSET>, ShowContactCardWithPlacement::<Impl, OFFSET>, ShowDelayLoadedContactCard::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics2Impl: Sized + IContactManagerStaticsImpl {
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerStatics2Impl, const OFFSET: isize>() -> IContactManagerStatics2Vtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IContactManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactManagerStatics2>, ::windows::core::GetTrustLevel, RequestStoreAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics3Impl: Sized + IContactManagerStaticsImpl + IContactManagerStatics2Impl {
    fn ConvertContactToVCardAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertContactToVCardAsyncWithMaxBytes(&self, contact: &::core::option::Option<Contact>, maxbytes: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertVCardToContactAsync(&self, vcard: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn RequestStoreAsyncWithAccessType(&self, accesstype: ContactStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
    fn RequestAnnotationStoreAsync(&self, accesstype: ContactAnnotationStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>>;
    fn IsShowContactCardSupported(&self) -> ::windows::core::Result<bool>;
    fn ShowContactCardWithOptions(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: &::core::option::Option<ContactCardOptions>) -> ::windows::core::Result<()>;
    fn IsShowDelayLoadedContactCardSupported(&self) -> ::windows::core::Result<bool>;
    fn ShowDelayLoadedContactCardWithOptions(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: &::core::option::Option<ContactCardOptions>) -> ::windows::core::Result<ContactCardDelayedDataLoader>;
    fn ShowFullContactCard(&self, contact: &::core::option::Option<Contact>, fullcontactcardoptions: &::core::option::Option<FullContactCardOptions>) -> ::windows::core::Result<()>;
    fn SystemDisplayNameOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemDisplayNameOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
    fn SystemSortOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemSortOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IContactManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerStatics3Impl, const OFFSET: isize>() -> IContactManagerStatics3Vtbl {
        unsafe extern "system" fn ConvertContactToVCardAsync<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertContactToVCardAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertContactToVCardAsyncWithMaxBytes<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, maxbytes: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertContactToVCardAsyncWithMaxBytes(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), maxbytes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVCardToContactAsync<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcard: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertVCardToContactAsync(&*(&vcard as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStoreAsyncWithAccessType<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: ContactStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsyncWithAccessType(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAnnotationStoreAsync<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: ContactAnnotationStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAnnotationStoreAsync(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShowContactCardSupported<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShowContactCardSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowContactCardWithOptions<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ShowContactCardWithOptions(
                    &*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType),
                    &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                    preferredplacement,
                    &*(&contactcardoptions as *const <ContactCardOptions as ::windows::core::Abi>::Abi as *const <ContactCardOptions as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn IsShowDelayLoadedContactCardSupported<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShowDelayLoadedContactCardSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowDelayLoadedContactCardWithOptions<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowDelayLoadedContactCardWithOptions(
                &*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType),
                &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                preferredplacement,
                &*(&contactcardoptions as *const <ContactCardOptions as ::windows::core::Abi>::Abi as *const <ContactCardOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowFullContactCard<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, fullcontactcardoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowFullContactCard(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), &*(&fullcontactcardoptions as *const <FullContactCardOptions as ::windows::core::Abi>::Abi as *const <FullContactCardOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemDisplayNameOrder<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemDisplayNameOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemDisplayNameOrder<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemDisplayNameOrder(value).into()
        }
        unsafe extern "system" fn SystemSortOrder<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemSortOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemSortOrder<Impl: IContactManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemSortOrder(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContactManagerStatics3>,
            ::windows::core::GetTrustLevel,
            ConvertContactToVCardAsync::<Impl, OFFSET>,
            ConvertContactToVCardAsyncWithMaxBytes::<Impl, OFFSET>,
            ConvertVCardToContactAsync::<Impl, OFFSET>,
            RequestStoreAsyncWithAccessType::<Impl, OFFSET>,
            RequestAnnotationStoreAsync::<Impl, OFFSET>,
            IsShowContactCardSupported::<Impl, OFFSET>,
            ShowContactCardWithOptions::<Impl, OFFSET>,
            IsShowDelayLoadedContactCardSupported::<Impl, OFFSET>,
            ShowDelayLoadedContactCardWithOptions::<Impl, OFFSET>,
            ShowFullContactCard::<Impl, OFFSET>,
            SystemDisplayNameOrder::<Impl, OFFSET>,
            SetSystemDisplayNameOrder::<Impl, OFFSET>,
            SystemSortOrder::<Impl, OFFSET>,
            SetSystemSortOrder::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics4Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<ContactManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactManagerStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IContactManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerStatics4Impl, const OFFSET: isize>() -> IContactManagerStatics4Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IContactManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactManagerStatics4>, ::windows::core::GetTrustLevel, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics5Impl: Sized {
    fn IsShowFullContactCardSupportedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn IncludeMiddleNameInSystemDisplayAndSort(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeMiddleNameInSystemDisplayAndSort(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactManagerStatics5 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactManagerStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IContactManagerStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactManagerStatics5Impl, const OFFSET: isize>() -> IContactManagerStatics5Vtbl {
        unsafe extern "system" fn IsShowFullContactCardSupportedAsync<Impl: IContactManagerStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShowFullContactCardSupportedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeMiddleNameInSystemDisplayAndSort<Impl: IContactManagerStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeMiddleNameInSystemDisplayAndSort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeMiddleNameInSystemDisplayAndSort<Impl: IContactManagerStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeMiddleNameInSystemDisplayAndSort(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactManagerStatics5>, ::windows::core::GetTrustLevel, IsShowFullContactCardSupportedAsync::<Impl, OFFSET>, IncludeMiddleNameInSystemDisplayAndSort::<Impl, OFFSET>, SetIncludeMiddleNameInSystemDisplayAndSort::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactMatchReasonImpl: Sized {
    fn Field(&self) -> ::windows::core::Result<ContactMatchReasonKind>;
    fn Segments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactMatchReason {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactMatchReason";
}
#[cfg(feature = "implement_exclusive")]
impl IContactMatchReasonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactMatchReasonImpl, const OFFSET: isize>() -> IContactMatchReasonVtbl {
        unsafe extern "system" fn Field<Impl: IContactMatchReasonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactMatchReasonKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Field() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Segments<Impl: IContactMatchReasonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Segments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IContactMatchReasonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactMatchReason>, ::windows::core::GetTrustLevel, Field::<Impl, OFFSET>, Segments::<Impl, OFFSET>, Text::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactNameImpl: Sized {
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFirstName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLastName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MiddleName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMiddleName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn YomiGivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetYomiGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn YomiFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetYomiFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificNameSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificNameSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactName {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactName";
}
#[cfg(feature = "implement_exclusive")]
impl IContactNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactNameImpl, const OFFSET: isize>() -> IContactNameVtbl {
        unsafe extern "system" fn FirstName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirstName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFirstName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LastName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MiddleName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MiddleName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMiddleName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMiddleName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn YomiGivenName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YomiGivenName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYomiGivenName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYomiGivenName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn YomiFamilyName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YomiFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYomiFamilyName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYomiFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HonorificNameSuffix<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HonorificNameSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificNameSuffix<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHonorificNameSuffix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HonorificNamePrefix<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HonorificNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificNamePrefix<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHonorificNamePrefix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn YomiDisplayName<Impl: IContactNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YomiDisplayName() {
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
            ::windows::core::GetRuntimeClassName::<IContactName>,
            ::windows::core::GetTrustLevel,
            FirstName::<Impl, OFFSET>,
            SetFirstName::<Impl, OFFSET>,
            LastName::<Impl, OFFSET>,
            SetLastName::<Impl, OFFSET>,
            MiddleName::<Impl, OFFSET>,
            SetMiddleName::<Impl, OFFSET>,
            YomiGivenName::<Impl, OFFSET>,
            SetYomiGivenName::<Impl, OFFSET>,
            YomiFamilyName::<Impl, OFFSET>,
            SetYomiFamilyName::<Impl, OFFSET>,
            HonorificNameSuffix::<Impl, OFFSET>,
            SetHonorificNameSuffix::<Impl, OFFSET>,
            HonorificNamePrefix::<Impl, OFFSET>,
            SetHonorificNamePrefix::<Impl, OFFSET>,
            DisplayName::<Impl, OFFSET>,
            YomiDisplayName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPanelImpl: Sized {
    fn ClosePanel(&self) -> ::windows::core::Result<()>;
    fn HeaderColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::UI::Color>>;
    fn SetHeaderColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::UI::Color>>) -> ::windows::core::Result<()>;
    fn LaunchFullAppRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelLaunchFullAppRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLaunchFullAppRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelClosingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPanel {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPanel";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPanelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPanelImpl, const OFFSET: isize>() -> IContactPanelVtbl {
        unsafe extern "system" fn ClosePanel<Impl: IContactPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClosePanel().into()
        }
        unsafe extern "system" fn HeaderColor<Impl: IContactPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderColor<Impl: IContactPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeaderColor(&*(&value as *const <super::super::Foundation::IReference<super::super::UI::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::UI::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LaunchFullAppRequested<Impl: IContactPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFullAppRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelLaunchFullAppRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelLaunchFullAppRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLaunchFullAppRequested<Impl: IContactPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLaunchFullAppRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closing<Impl: IContactPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closing(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelClosingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelClosingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosing<Impl: IContactPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContactPanel>,
            ::windows::core::GetTrustLevel,
            ClosePanel::<Impl, OFFSET>,
            HeaderColor::<Impl, OFFSET>,
            SetHeaderColor::<Impl, OFFSET>,
            LaunchFullAppRequested::<Impl, OFFSET>,
            RemoveLaunchFullAppRequested::<Impl, OFFSET>,
            Closing::<Impl, OFFSET>,
            RemoveClosing::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPanelClosingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPanelClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPanelClosingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPanelClosingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPanelClosingEventArgsImpl, const OFFSET: isize>() -> IContactPanelClosingEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IContactPanelClosingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPanelClosingEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPanelLaunchFullAppRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPanelLaunchFullAppRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPanelLaunchFullAppRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPanelLaunchFullAppRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPanelLaunchFullAppRequestedEventArgsImpl, const OFFSET: isize>() -> IContactPanelLaunchFullAppRequestedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IContactPanelLaunchFullAppRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IContactPanelLaunchFullAppRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPanelLaunchFullAppRequestedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPhoneImpl: Sized {
    fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactPhoneKind>;
    fn SetKind(&self, value: ContactPhoneKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPhone {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPhone";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPhoneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPhoneImpl, const OFFSET: isize>() -> IContactPhoneVtbl {
        unsafe extern "system" fn Number<Impl: IContactPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Number() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumber<Impl: IContactPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Kind<Impl: IContactPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactPhoneKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKind<Impl: IContactPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactPhoneKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        unsafe extern "system" fn Description<Impl: IContactPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IContactPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPhone>, ::windows::core::GetTrustLevel, Number::<Impl, OFFSET>, SetNumber::<Impl, OFFSET>, Kind::<Impl, OFFSET>, SetKind::<Impl, OFFSET>, Description::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPickerImpl: Sized {
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectionMode(&self) -> ::windows::core::Result<ContactSelectionMode>;
    fn SetSelectionMode(&self, value: ContactSelectionMode) -> ::windows::core::Result<()>;
    fn DesiredFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactInformation>>;
    fn PickMultipleContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactInformation>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPicker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPicker";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPickerImpl, const OFFSET: isize>() -> IContactPickerVtbl {
        unsafe extern "system" fn CommitButtonText<Impl: IContactPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitButtonText<Impl: IContactPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionMode<Impl: IContactPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionMode<Impl: IContactPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionMode(value).into()
        }
        unsafe extern "system" fn DesiredFields<Impl: IContactPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredFields() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleContactAsync<Impl: IContactPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleContactAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickMultipleContactsAsync<Impl: IContactPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickMultipleContactsAsync() {
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
            ::windows::core::GetRuntimeClassName::<IContactPicker>,
            ::windows::core::GetTrustLevel,
            CommitButtonText::<Impl, OFFSET>,
            SetCommitButtonText::<Impl, OFFSET>,
            SelectionMode::<Impl, OFFSET>,
            SetSelectionMode::<Impl, OFFSET>,
            DesiredFields::<Impl, OFFSET>,
            PickSingleContactAsync::<Impl, OFFSET>,
            PickMultipleContactsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPicker2Impl: Sized {
    fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactFieldType>>;
    fn PickContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn PickContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<Contact>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPicker2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPicker2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPicker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPicker2Impl, const OFFSET: isize>() -> IContactPicker2Vtbl {
        unsafe extern "system" fn DesiredFieldsWithContactFieldType<Impl: IContactPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredFieldsWithContactFieldType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickContactAsync<Impl: IContactPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickContactAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickContactsAsync<Impl: IContactPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickContactsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPicker2>, ::windows::core::GetTrustLevel, DesiredFieldsWithContactFieldType::<Impl, OFFSET>, PickContactAsync::<Impl, OFFSET>, PickContactsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPicker3 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPicker3";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPicker3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPicker3Impl, const OFFSET: isize>() -> IContactPicker3Vtbl {
        unsafe extern "system" fn User<Impl: IContactPicker3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPicker3>, ::windows::core::GetTrustLevel, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<ContactPicker>;
    fn IsSupportedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPickerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactPickerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPickerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPickerStaticsImpl, const OFFSET: isize>() -> IContactPickerStaticsVtbl {
        unsafe extern "system" fn CreateForUser<Impl: IContactPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedAsync<Impl: IContactPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactPickerStatics>, ::windows::core::GetTrustLevel, CreateForUser::<Impl, OFFSET>, IsSupportedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryOptionsImpl: Sized {
    fn TextSearch(&self) -> ::windows::core::Result<ContactQueryTextSearch>;
    fn ContactListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IncludeContactsFromHiddenLists(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeContactsFromHiddenLists(&self, value: bool) -> ::windows::core::Result<()>;
    fn DesiredFields(&self) -> ::windows::core::Result<ContactQueryDesiredFields>;
    fn SetDesiredFields(&self, value: ContactQueryDesiredFields) -> ::windows::core::Result<()>;
    fn DesiredOperations(&self) -> ::windows::core::Result<ContactAnnotationOperations>;
    fn SetDesiredOperations(&self, value: ContactAnnotationOperations) -> ::windows::core::Result<()>;
    fn AnnotationListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactQueryOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IContactQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactQueryOptionsImpl, const OFFSET: isize>() -> IContactQueryOptionsVtbl {
        unsafe extern "system" fn TextSearch<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContactListIds<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactListIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeContactsFromHiddenLists<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeContactsFromHiddenLists() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeContactsFromHiddenLists<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeContactsFromHiddenLists(value).into()
        }
        unsafe extern "system" fn DesiredFields<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactQueryDesiredFields) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredFields() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredFields<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactQueryDesiredFields) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredFields(value).into()
        }
        unsafe extern "system" fn DesiredOperations<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactAnnotationOperations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredOperations<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactAnnotationOperations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredOperations(value).into()
        }
        unsafe extern "system" fn AnnotationListIds<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationListIds() {
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
            ::windows::core::GetRuntimeClassName::<IContactQueryOptions>,
            ::windows::core::GetTrustLevel,
            TextSearch::<Impl, OFFSET>,
            ContactListIds::<Impl, OFFSET>,
            IncludeContactsFromHiddenLists::<Impl, OFFSET>,
            SetIncludeContactsFromHiddenLists::<Impl, OFFSET>,
            DesiredFields::<Impl, OFFSET>,
            SetDesiredFields::<Impl, OFFSET>,
            DesiredOperations::<Impl, OFFSET>,
            SetDesiredOperations::<Impl, OFFSET>,
            AnnotationListIds::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryOptionsFactoryImpl: Sized {
    fn CreateWithText(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<ContactQueryOptions>;
    fn CreateWithTextAndFields(&self, text: &::windows::core::HSTRING, fields: ContactQuerySearchFields) -> ::windows::core::Result<ContactQueryOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactQueryOptionsFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactQueryOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IContactQueryOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactQueryOptionsFactoryImpl, const OFFSET: isize>() -> IContactQueryOptionsFactoryVtbl {
        unsafe extern "system" fn CreateWithText<Impl: IContactQueryOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithTextAndFields<Impl: IContactQueryOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ContactQuerySearchFields, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactQueryOptionsFactory>, ::windows::core::GetTrustLevel, CreateWithText::<Impl, OFFSET>, CreateWithTextAndFields::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryTextSearchImpl: Sized {
    fn Fields(&self) -> ::windows::core::Result<ContactQuerySearchFields>;
    fn SetFields(&self, value: ContactQuerySearchFields) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SearchScope(&self) -> ::windows::core::Result<ContactQuerySearchScope>;
    fn SetSearchScope(&self, value: ContactQuerySearchScope) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactQueryTextSearch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactQueryTextSearch";
}
#[cfg(feature = "implement_exclusive")]
impl IContactQueryTextSearchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactQueryTextSearchImpl, const OFFSET: isize>() -> IContactQueryTextSearchVtbl {
        unsafe extern "system" fn Fields<Impl: IContactQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactQuerySearchFields) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFields<Impl: IContactQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactQuerySearchFields) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFields(value).into()
        }
        unsafe extern "system" fn Text<Impl: IContactQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetText<Impl: IContactQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SearchScope<Impl: IContactQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactQuerySearchScope) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSearchScope<Impl: IContactQueryTextSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactQuerySearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchScope(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactQueryTextSearch>, ::windows::core::GetTrustLevel, Fields::<Impl, OFFSET>, SetFields::<Impl, OFFSET>, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, SearchScope::<Impl, OFFSET>, SetSearchScope::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactBatch>>;
    fn GetMatchingPropertiesWithMatchReason(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactMatchReason>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactReader";
}
#[cfg(feature = "implement_exclusive")]
impl IContactReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactReaderImpl, const OFFSET: isize>() -> IContactReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IContactReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMatchingPropertiesWithMatchReason<Impl: IContactReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingPropertiesWithMatchReason(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactReader>, ::windows::core::GetTrustLevel, ReadBatchAsync::<Impl, OFFSET>, GetMatchingPropertiesWithMatchReason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactSignificantOtherImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactSignificantOther {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactSignificantOther";
}
#[cfg(feature = "implement_exclusive")]
impl IContactSignificantOtherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactSignificantOtherImpl, const OFFSET: isize>() -> IContactSignificantOtherVtbl {
        unsafe extern "system" fn Name<Impl: IContactSignificantOtherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IContactSignificantOtherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IContactSignificantOtherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IContactSignificantOtherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactSignificantOther>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>, Description::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactSignificantOther2Impl: Sized + IContactSignificantOtherImpl {
    fn Relationship(&self) -> ::windows::core::Result<ContactRelationship>;
    fn SetRelationship(&self, value: ContactRelationship) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactSignificantOther2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactSignificantOther2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactSignificantOther2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactSignificantOther2Impl, const OFFSET: isize>() -> IContactSignificantOther2Vtbl {
        unsafe extern "system" fn Relationship<Impl: IContactSignificantOther2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactRelationship) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Relationship() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelationship<Impl: IContactSignificantOther2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactRelationship) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelationship(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactSignificantOther2>, ::windows::core::GetTrustLevel, Relationship::<Impl, OFFSET>, SetRelationship::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreImpl: Sized {
    fn FindContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>;
    fn FindContactsWithSearchTextAsync(&self, searchtext: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>;
    fn GetContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactStore";
}
#[cfg(feature = "implement_exclusive")]
impl IContactStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStoreImpl, const OFFSET: isize>() -> IContactStoreVtbl {
        unsafe extern "system" fn FindContactsAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContactsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindContactsWithSearchTextAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchtext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContactsWithSearchTextAsync(&*(&searchtext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactAsync(&*(&contactid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactStore>, ::windows::core::GetTrustLevel, FindContactsAsync::<Impl, OFFSET>, FindContactsWithSearchTextAsync::<Impl, OFFSET>, GetContactAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStore2Impl: Sized + IContactStoreImpl {
    fn ChangeTracker(&self) -> ::windows::core::Result<ContactChangeTracker>;
    fn ContactChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactStore, ContactChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContactChanged(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AggregateContactManager(&self) -> ::windows::core::Result<AggregateContactManager>;
    fn FindContactListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactList>>>;
    fn GetContactListAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>>;
    fn CreateContactListAsync(&self, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>>;
    fn GetMeContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn GetContactReader(&self) -> ::windows::core::Result<ContactReader>;
    fn GetContactReaderWithOptions(&self, options: &::core::option::Option<ContactQueryOptions>) -> ::windows::core::Result<ContactReader>;
    fn CreateContactListInAccountAsync(&self, displayname: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactStore2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactStore2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStore2Impl, const OFFSET: isize>() -> IContactStore2Vtbl {
        unsafe extern "system" fn ChangeTracker<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContactChanged<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<ContactStore, ContactChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ContactStore, ContactChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContactChanged<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContactChanged(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AggregateContactManager<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AggregateContactManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindContactListsAsync<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContactListsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactListAsync<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactlistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactListAsync(&*(&contactlistid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContactListAsync<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContactListAsync(&*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeContactAsync<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeContactAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactReader<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactReaderWithOptions<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactReaderWithOptions(&*(&options as *const <ContactQueryOptions as ::windows::core::Abi>::Abi as *const <ContactQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContactListInAccountAsync<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContactListInAccountAsync(&*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&userdataaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IContactStore2>,
            ::windows::core::GetTrustLevel,
            ChangeTracker::<Impl, OFFSET>,
            ContactChanged::<Impl, OFFSET>,
            RemoveContactChanged::<Impl, OFFSET>,
            AggregateContactManager::<Impl, OFFSET>,
            FindContactListsAsync::<Impl, OFFSET>,
            GetContactListAsync::<Impl, OFFSET>,
            CreateContactListAsync::<Impl, OFFSET>,
            GetMeContactAsync::<Impl, OFFSET>,
            GetContactReader::<Impl, OFFSET>,
            GetContactReaderWithOptions::<Impl, OFFSET>,
            CreateContactListInAccountAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStore3Impl: Sized {
    fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ContactChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactStore3 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactStore3";
}
#[cfg(feature = "implement_exclusive")]
impl IContactStore3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStore3Impl, const OFFSET: isize>() -> IContactStore3Vtbl {
        unsafe extern "system" fn GetChangeTracker<Impl: IContactStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactStore3>, ::windows::core::GetTrustLevel, GetChangeTracker::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreNotificationTriggerDetailsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactStoreNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IContactStoreNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStoreNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IContactStoreNotificationTriggerDetailsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactStoreNotificationTriggerDetails>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactWebsiteImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactWebsite {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactWebsite";
}
#[cfg(feature = "implement_exclusive")]
impl IContactWebsiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactWebsiteImpl, const OFFSET: isize>() -> IContactWebsiteVtbl {
        unsafe extern "system" fn Uri<Impl: IContactWebsiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IContactWebsiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IContactWebsiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IContactWebsiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactWebsite>, ::windows::core::GetTrustLevel, Uri::<Impl, OFFSET>, SetUri::<Impl, OFFSET>, Description::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactWebsite2Impl: Sized + IContactWebsiteImpl {
    fn RawValue(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRawValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactWebsite2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactWebsite2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactWebsite2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactWebsite2Impl, const OFFSET: isize>() -> IContactWebsite2Vtbl {
        unsafe extern "system" fn RawValue<Impl: IContactWebsite2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRawValue<Impl: IContactWebsite2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRawValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContactWebsite2>, ::windows::core::GetTrustLevel, RawValue::<Impl, OFFSET>, SetRawValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullContactCardOptionsImpl: Sized {
    fn DesiredRemainingView(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&self, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFullContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IFullContactCardOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFullContactCardOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFullContactCardOptionsImpl, const OFFSET: isize>() -> IFullContactCardOptionsVtbl {
        unsafe extern "system" fn DesiredRemainingView<Impl: IFullContactCardOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredRemainingView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRemainingView<Impl: IFullContactCardOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredRemainingView(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFullContactCardOptions>, ::windows::core::GetTrustLevel, DesiredRemainingView::<Impl, OFFSET>, SetDesiredRemainingView::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownContactFieldStaticsImpl: Sized {
    fn Email(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstantMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConvertNameToType(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ContactFieldType>;
    fn ConvertTypeToName(&self, r#type: ContactFieldType) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownContactFieldStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IKnownContactFieldStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownContactFieldStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownContactFieldStaticsImpl, const OFFSET: isize>() -> IKnownContactFieldStaticsVtbl {
        unsafe extern "system" fn Email<Impl: IKnownContactFieldStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Email() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneNumber<Impl: IKnownContactFieldStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Location<Impl: IKnownContactFieldStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InstantMessage<Impl: IKnownContactFieldStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstantMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertNameToType<Impl: IKnownContactFieldStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ContactFieldType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertNameToType(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertTypeToName<Impl: IKnownContactFieldStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ContactFieldType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertTypeToName(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownContactFieldStatics>, ::windows::core::GetTrustLevel, Email::<Impl, OFFSET>, PhoneNumber::<Impl, OFFSET>, Location::<Impl, OFFSET>, InstantMessage::<Impl, OFFSET>, ConvertNameToType::<Impl, OFFSET>, ConvertTypeToName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinnedContactIdsQueryResultImpl: Sized {
    fn ContactIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPinnedContactIdsQueryResult {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IPinnedContactIdsQueryResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPinnedContactIdsQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPinnedContactIdsQueryResultImpl, const OFFSET: isize>() -> IPinnedContactIdsQueryResultVtbl {
        unsafe extern "system" fn ContactIds<Impl: IPinnedContactIdsQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPinnedContactIdsQueryResult>, ::windows::core::GetTrustLevel, ContactIds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinnedContactManagerImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn IsPinSurfaceSupported(&self, surface: PinnedContactSurface) -> ::windows::core::Result<bool>;
    fn IsContactPinned(&self, contact: &::core::option::Option<Contact>, surface: PinnedContactSurface) -> ::windows::core::Result<bool>;
    fn RequestPinContactAsync(&self, contact: &::core::option::Option<Contact>, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinContactsAsync(&self, contacts: &::core::option::Option<super::super::Foundation::Collections::IIterable<Contact>>, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestUnpinContactAsync(&self, contact: &::core::option::Option<Contact>, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SignalContactActivity(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<()>;
    fn GetPinnedContactIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PinnedContactIdsQueryResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPinnedContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IPinnedContactManager";
}
#[cfg(feature = "implement_exclusive")]
impl IPinnedContactManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPinnedContactManagerImpl, const OFFSET: isize>() -> IPinnedContactManagerVtbl {
        unsafe extern "system" fn User<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPinSurfaceSupported<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinSurfaceSupported(surface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContactPinned<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, surface: PinnedContactSurface, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContactPinned(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), surface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinContactAsync<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, surface: PinnedContactSurface, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPinContactAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), surface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinContactsAsync<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contacts: ::windows::core::RawPtr, surface: PinnedContactSurface, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPinContactsAsync(&*(&contacts as *const <super::super::Foundation::Collections::IIterable<Contact> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<Contact> as ::windows::core::DefaultType>::DefaultType), surface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestUnpinContactAsync<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, surface: PinnedContactSurface, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUnpinContactAsync(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType), surface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalContactActivity<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SignalContactActivity(&*(&contact as *const <Contact as ::windows::core::Abi>::Abi as *const <Contact as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPinnedContactIdsAsync<Impl: IPinnedContactManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPinnedContactIdsAsync() {
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
            ::windows::core::GetRuntimeClassName::<IPinnedContactManager>,
            ::windows::core::GetTrustLevel,
            User::<Impl, OFFSET>,
            IsPinSurfaceSupported::<Impl, OFFSET>,
            IsContactPinned::<Impl, OFFSET>,
            RequestPinContactAsync::<Impl, OFFSET>,
            RequestPinContactsAsync::<Impl, OFFSET>,
            RequestUnpinContactAsync::<Impl, OFFSET>,
            SignalContactActivity::<Impl, OFFSET>,
            GetPinnedContactIdsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinnedContactManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PinnedContactManager>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<PinnedContactManager>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPinnedContactManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IPinnedContactManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPinnedContactManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPinnedContactManagerStaticsImpl, const OFFSET: isize>() -> IPinnedContactManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IPinnedContactManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IPinnedContactManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSupported<Impl: IPinnedContactManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPinnedContactManagerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>, GetForUser::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>)
    }
}
