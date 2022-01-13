#[cfg(feature = "implement_exclusive")]
pub trait IContactAddressImpl: Sized {
    fn Country(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCountry(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Locality(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocality(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Region(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PostalCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPostalCode(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StreetAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreetAddress(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactAddress {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactAddress";
}
#[cfg(feature = "implement_exclusive")]
impl IContactAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactAddressVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactAddress, BASE_OFFSET>(),
            Country: Country::<Impl, IMPL_OFFSET>,
            SetCountry: SetCountry::<Impl, IMPL_OFFSET>,
            Locality: Locality::<Impl, IMPL_OFFSET>,
            SetLocality: SetLocality::<Impl, IMPL_OFFSET>,
            Region: Region::<Impl, IMPL_OFFSET>,
            SetRegion: SetRegion::<Impl, IMPL_OFFSET>,
            PostalCode: PostalCode::<Impl, IMPL_OFFSET>,
            SetPostalCode: SetPostalCode::<Impl, IMPL_OFFSET>,
            StreetAddress: StreetAddress::<Impl, IMPL_OFFSET>,
            SetStreetAddress: SetStreetAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeRecordImpl: Sized {
    fn ChangeType(&mut self) -> ::windows::core::Result<ContactChangeType>;
    fn RevisionNumber(&mut self) -> ::windows::core::Result<u64>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactChangeRecord {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactChangeRecord";
}
#[cfg(feature = "implement_exclusive")]
impl IContactChangeRecordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactChangeRecordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactChangeRecordVtbl {
        unsafe extern "system" fn ChangeType<Impl: IContactChangeRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactChangeType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RevisionNumber<Impl: IContactChangeRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevisionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IContactChangeRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteId<Impl: IContactChangeRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactChangeRecord, BASE_OFFSET>(),
            ChangeType: ChangeType::<Impl, IMPL_OFFSET>,
            RevisionNumber: RevisionNumber::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactChangeRecord as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IContactInformationImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFamilyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GivenName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGivenName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificPrefix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificPrefix(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificSuffix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificSuffix(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDisplayPictureAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SetDisplayPictureAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisplayPicture(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetPropertiesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn ToVcardAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ToVcardWithOptionsAsync(&mut self, format: VCardFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IContactInformation {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IContactInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactInformationVtbl {
        unsafe extern "system" fn DisplayName<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FamilyName<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFamilyName<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GivenName<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GivenName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGivenName<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGivenName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HonorificPrefix<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HonorificPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificPrefix<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHonorificPrefix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HonorificSuffix<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HonorificSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificSuffix<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHonorificSuffix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDisplayPictureAsync<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayPictureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureAsync<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisplayPictureAsync(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayPicture<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesAsync<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardAsync<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToVcardAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardWithOptionsAsync<Impl: IContactInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: VCardFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToVcardWithOptionsAsync(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactInformation, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            FamilyName: FamilyName::<Impl, IMPL_OFFSET>,
            SetFamilyName: SetFamilyName::<Impl, IMPL_OFFSET>,
            GivenName: GivenName::<Impl, IMPL_OFFSET>,
            SetGivenName: SetGivenName::<Impl, IMPL_OFFSET>,
            HonorificPrefix: HonorificPrefix::<Impl, IMPL_OFFSET>,
            SetHonorificPrefix: SetHonorificPrefix::<Impl, IMPL_OFFSET>,
            HonorificSuffix: HonorificSuffix::<Impl, IMPL_OFFSET>,
            SetHonorificSuffix: SetHonorificSuffix::<Impl, IMPL_OFFSET>,
            GetDisplayPictureAsync: GetDisplayPictureAsync::<Impl, IMPL_OFFSET>,
            SetDisplayPictureAsync: SetDisplayPictureAsync::<Impl, IMPL_OFFSET>,
            DisplayPicture: DisplayPicture::<Impl, IMPL_OFFSET>,
            GetPropertiesAsync: GetPropertiesAsync::<Impl, IMPL_OFFSET>,
            ToVcardAsync: ToVcardAsync::<Impl, IMPL_OFFSET>,
            ToVcardWithOptionsAsync: ToVcardWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IContactInformation2Impl: Sized {
    fn DisplayPictureDate(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureDate(&mut self, returnvalue: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IContactInformation2 {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation2";
}
#[cfg(feature = "Foundation")]
impl IContactInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactInformation2Vtbl {
        unsafe extern "system" fn DisplayPictureDate<Impl: IContactInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPictureDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureDate<Impl: IContactInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnvalue: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayPictureDate(&*(&returnvalue as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactInformation2, BASE_OFFSET>(),
            DisplayPictureDate: DisplayPictureDate::<Impl, IMPL_OFFSET>,
            SetDisplayPictureDate: SetDisplayPictureDate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IContactInformationStaticsImpl: Sized {
    fn ParseVcardAsync(&mut self, vcard: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactInformation>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactInformationStatics {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IContactInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactInformationStaticsVtbl {
        unsafe extern "system" fn ParseVcardAsync<Impl: IContactInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcard: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseVcardAsync(&*(&vcard as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactInformationStatics, BASE_OFFSET>(),
            ParseVcardAsync: ParseVcardAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContactQueryOptionsImpl: Sized {
    fn DesiredFields(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OrderBy(&mut self) -> ::windows::core::Result<ContactQueryResultOrdering>;
    fn SetOrderBy(&mut self, value: ContactQueryResultOrdering) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactQueryOptions {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactQueryOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContactQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactQueryOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactQueryOptionsVtbl {
        unsafe extern "system" fn DesiredFields<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OrderBy<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactQueryResultOrdering) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrderBy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrderBy<Impl: IContactQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ContactQueryResultOrdering) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrderBy(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactQueryOptions, BASE_OFFSET>(),
            DesiredFields: DesiredFields::<Impl, IMPL_OFFSET>,
            OrderBy: OrderBy::<Impl, IMPL_OFFSET>,
            SetOrderBy: SetOrderBy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactQueryOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactQueryResultImpl: Sized {
    fn GetContactCountAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetContactsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoredContact>>>;
    fn GetContactsAsyncInRange(&mut self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoredContact>>>;
    fn GetCurrentQueryOptions(&mut self) -> ::windows::core::Result<ContactQueryOptions>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactQueryResult {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactQueryResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactQueryResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactQueryResultVtbl {
        unsafe extern "system" fn GetContactCountAsync<Impl: IContactQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactsAsync<Impl: IContactQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContactsAsyncInRange<Impl: IContactQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContactsAsyncInRange(startindex, maxnumberofitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Impl: IContactQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentQueryOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactQueryResult, BASE_OFFSET>(),
            GetContactCountAsync: GetContactCountAsync::<Impl, IMPL_OFFSET>,
            GetContactsAsync: GetContactsAsync::<Impl, IMPL_OFFSET>,
            GetContactsAsyncInRange: GetContactsAsyncInRange::<Impl, IMPL_OFFSET>,
            GetCurrentQueryOptions: GetCurrentQueryOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactQueryResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContactStoreImpl: Sized {
    fn FindContactByRemoteIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>>;
    fn FindContactByIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>>;
    fn DeleteContactAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateContactQueryDefault(&mut self) -> ::windows::core::Result<ContactQueryResult>;
    fn CreateContactQueryWithOptions(&mut self, options: &::core::option::Option<ContactQueryOptions>) -> ::windows::core::Result<ContactQueryResult>;
    fn DeleteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RevisionNumber(&mut self) -> ::windows::core::Result<u64>;
    fn GetChangesAsync(&mut self, baserevisionnumber: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChangeRecord>>>;
    fn LoadExtendedPropertiesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn SaveExtendedPropertiesAsync(&mut self, data: &::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactStore {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactStore";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContactStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactStoreVtbl {
        unsafe extern "system" fn FindContactByRemoteIdAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContactByRemoteIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindContactByIdAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContactByIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteContactAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteContactAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContactQueryDefault<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContactQueryDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContactQueryWithOptions<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContactQueryWithOptions(&*(&options as *const <ContactQueryOptions as ::windows::core::Abi>::Abi as *const <ContactQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RevisionNumber<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevisionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangesAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baserevisionnumber: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangesAsync(baserevisionnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadExtendedPropertiesAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadExtendedPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveExtendedPropertiesAsync<Impl: IContactStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveExtendedPropertiesAsync(&*(&data as *const <super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactStore, BASE_OFFSET>(),
            FindContactByRemoteIdAsync: FindContactByRemoteIdAsync::<Impl, IMPL_OFFSET>,
            FindContactByIdAsync: FindContactByIdAsync::<Impl, IMPL_OFFSET>,
            DeleteContactAsync: DeleteContactAsync::<Impl, IMPL_OFFSET>,
            CreateContactQueryDefault: CreateContactQueryDefault::<Impl, IMPL_OFFSET>,
            CreateContactQueryWithOptions: CreateContactQueryWithOptions::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            RevisionNumber: RevisionNumber::<Impl, IMPL_OFFSET>,
            GetChangesAsync: GetChangesAsync::<Impl, IMPL_OFFSET>,
            LoadExtendedPropertiesAsync: LoadExtendedPropertiesAsync::<Impl, IMPL_OFFSET>,
            SaveExtendedPropertiesAsync: SaveExtendedPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactStore2Impl: Sized {
    fn CreateMeContactAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactStore2 {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactStore2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStore2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactStore2Vtbl {
        unsafe extern "system" fn CreateMeContactAsync<Impl: IContactStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMeContactAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactStore2, BASE_OFFSET>(),
            CreateMeContactAsync: CreateMeContactAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactStoreStaticsImpl: Sized {
    fn CreateOrOpenAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
    fn CreateOrOpenWithOptionsAsync(&mut self, access: ContactStoreSystemAccessMode, sharing: ContactStoreApplicationAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactStoreStatics {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactStoreStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContactStoreStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStoreStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactStoreStaticsVtbl {
        unsafe extern "system" fn CreateOrOpenAsync<Impl: IContactStoreStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrOpenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrOpenWithOptionsAsync<Impl: IContactStoreStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: ContactStoreSystemAccessMode, sharing: ContactStoreApplicationAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrOpenWithOptionsAsync(access, sharing) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactStoreStatics, BASE_OFFSET>(),
            CreateOrOpenAsync: CreateOrOpenAsync::<Impl, IMPL_OFFSET>,
            CreateOrOpenWithOptionsAsync: CreateOrOpenWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactStoreStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownContactPropertiesStaticsImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GivenName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HonorificPrefix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HonorificSuffix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdditionalName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OtherAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Email(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkTelephone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JobTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Birthdate(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Anniversary(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Telephone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileTelephone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Url(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Notes(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkFax(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Children(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignificantOther(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CompanyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CompanyTelephone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HomeFax(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlternateTelephone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Manager(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nickname(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OfficeLocation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkEmail(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiGivenName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiCompanyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OtherEmail(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlternateMobileTelephone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlternateWorkTelephone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownContactPropertiesStatics {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IKnownContactPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownContactPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownContactPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownContactPropertiesStaticsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FamilyName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GivenName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GivenName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HonorificPrefix<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HonorificPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HonorificSuffix<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HonorificSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OtherAddress<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Email<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WorkAddress<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkTelephone<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkTelephone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobTitle<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Birthdate<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Birthdate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Anniversary<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Anniversary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Telephone<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Telephone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MobileTelephone<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MobileTelephone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Url<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WorkFax<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkFax() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignificantOther<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignificantOther() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompanyName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CompanyTelephone<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompanyTelephone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeFax<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeFax() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateTelephone<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateTelephone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manager<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Nickname<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OfficeLocation<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfficeLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkEmail<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkEmail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YomiGivenName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn YomiFamilyName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn YomiCompanyName<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YomiCompanyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherEmail<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherEmail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateMobileTelephone<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateMobileTelephone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateWorkTelephone<Impl: IKnownContactPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateWorkTelephone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownContactPropertiesStatics, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            FamilyName: FamilyName::<Impl, IMPL_OFFSET>,
            GivenName: GivenName::<Impl, IMPL_OFFSET>,
            HonorificPrefix: HonorificPrefix::<Impl, IMPL_OFFSET>,
            HonorificSuffix: HonorificSuffix::<Impl, IMPL_OFFSET>,
            AdditionalName: AdditionalName::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            OtherAddress: OtherAddress::<Impl, IMPL_OFFSET>,
            Email: Email::<Impl, IMPL_OFFSET>,
            WorkAddress: WorkAddress::<Impl, IMPL_OFFSET>,
            WorkTelephone: WorkTelephone::<Impl, IMPL_OFFSET>,
            JobTitle: JobTitle::<Impl, IMPL_OFFSET>,
            Birthdate: Birthdate::<Impl, IMPL_OFFSET>,
            Anniversary: Anniversary::<Impl, IMPL_OFFSET>,
            Telephone: Telephone::<Impl, IMPL_OFFSET>,
            MobileTelephone: MobileTelephone::<Impl, IMPL_OFFSET>,
            Url: Url::<Impl, IMPL_OFFSET>,
            Notes: Notes::<Impl, IMPL_OFFSET>,
            WorkFax: WorkFax::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            SignificantOther: SignificantOther::<Impl, IMPL_OFFSET>,
            CompanyName: CompanyName::<Impl, IMPL_OFFSET>,
            CompanyTelephone: CompanyTelephone::<Impl, IMPL_OFFSET>,
            HomeFax: HomeFax::<Impl, IMPL_OFFSET>,
            AlternateTelephone: AlternateTelephone::<Impl, IMPL_OFFSET>,
            Manager: Manager::<Impl, IMPL_OFFSET>,
            Nickname: Nickname::<Impl, IMPL_OFFSET>,
            OfficeLocation: OfficeLocation::<Impl, IMPL_OFFSET>,
            WorkEmail: WorkEmail::<Impl, IMPL_OFFSET>,
            YomiGivenName: YomiGivenName::<Impl, IMPL_OFFSET>,
            YomiFamilyName: YomiFamilyName::<Impl, IMPL_OFFSET>,
            YomiCompanyName: YomiCompanyName::<Impl, IMPL_OFFSET>,
            OtherEmail: OtherEmail::<Impl, IMPL_OFFSET>,
            AlternateMobileTelephone: AlternateMobileTelephone::<Impl, IMPL_OFFSET>,
            AlternateWorkTelephone: AlternateWorkTelephone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownContactPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStoredContactImpl: Sized + IContactInformationImpl {
    fn Store(&mut self) -> ::windows::core::Result<ContactStore>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetExtendedPropertiesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReplaceExistingContactAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoredContact {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IStoredContact";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStoredContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoredContactImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoredContactVtbl {
        unsafe extern "system" fn Store<Impl: IStoredContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Store() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IStoredContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteId<Impl: IStoredContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteId<Impl: IStoredContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetExtendedPropertiesAsync<Impl: IStoredContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtendedPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IStoredContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReplaceExistingContactAsync<Impl: IStoredContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceExistingContactAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoredContact, BASE_OFFSET>(),
            Store: Store::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
            GetExtendedPropertiesAsync: GetExtendedPropertiesAsync::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            ReplaceExistingContactAsync: ReplaceExistingContactAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoredContact as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoredContactFactoryImpl: Sized {
    fn CreateStoredContact(&mut self, store: &::core::option::Option<ContactStore>) -> ::windows::core::Result<StoredContact>;
    fn CreateStoredContactFromInformation(&mut self, store: &::core::option::Option<ContactStore>, contact: &::core::option::Option<ContactInformation>) -> ::windows::core::Result<StoredContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoredContactFactory {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IStoredContactFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStoredContactFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoredContactFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoredContactFactoryVtbl {
        unsafe extern "system" fn CreateStoredContact<Impl: IStoredContactFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, store: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStoredContact(&*(&store as *const <ContactStore as ::windows::core::Abi>::Abi as *const <ContactStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoredContactFromInformation<Impl: IStoredContactFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, store: ::windows::core::RawPtr, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStoredContactFromInformation(&*(&store as *const <ContactStore as ::windows::core::Abi>::Abi as *const <ContactStore as ::windows::core::DefaultType>::DefaultType), &*(&contact as *const <ContactInformation as ::windows::core::Abi>::Abi as *const <ContactInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoredContactFactory, BASE_OFFSET>(),
            CreateStoredContact: CreateStoredContact::<Impl, IMPL_OFFSET>,
            CreateStoredContactFromInformation: CreateStoredContactFromInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoredContactFactory as ::windows::core::Interface>::IID
    }
}
