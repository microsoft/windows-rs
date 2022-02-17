#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IContactInformation_Impl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificPrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificPrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDisplayPictureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SetDisplayPictureAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn ToVcardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ToVcardWithOptionsAsync(&self, format: VCardFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IContactInformation {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IContactInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>() -> IContactInformation_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn FamilyName<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFamilyName<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFamilyName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GivenName<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GivenName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGivenName<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGivenName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn HonorificPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HonorificPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHonorificPrefix(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn HonorificSuffix<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HonorificSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificSuffix<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHonorificSuffix(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetDisplayPictureAsync<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDisplayPictureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureAsync<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetDisplayPictureAsync(::core::mem::transmute(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayPicture<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesAsync<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardAsync<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ToVcardAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardWithOptionsAsync<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: VCardFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactInformation, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            FamilyName: FamilyName::<Identity, Impl, OFFSET>,
            SetFamilyName: SetFamilyName::<Identity, Impl, OFFSET>,
            GivenName: GivenName::<Identity, Impl, OFFSET>,
            SetGivenName: SetGivenName::<Identity, Impl, OFFSET>,
            HonorificPrefix: HonorificPrefix::<Identity, Impl, OFFSET>,
            SetHonorificPrefix: SetHonorificPrefix::<Identity, Impl, OFFSET>,
            HonorificSuffix: HonorificSuffix::<Identity, Impl, OFFSET>,
            SetHonorificSuffix: SetHonorificSuffix::<Identity, Impl, OFFSET>,
            GetDisplayPictureAsync: GetDisplayPictureAsync::<Identity, Impl, OFFSET>,
            SetDisplayPictureAsync: SetDisplayPictureAsync::<Identity, Impl, OFFSET>,
            DisplayPicture: DisplayPicture::<Identity, Impl, OFFSET>,
            GetPropertiesAsync: GetPropertiesAsync::<Identity, Impl, OFFSET>,
            ToVcardAsync: ToVcardAsync::<Identity, Impl, OFFSET>,
            ToVcardWithOptionsAsync: ToVcardWithOptionsAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IContactInformation2_Impl: Sized {
    fn DisplayPictureDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureDate(&self, returnvalue: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IContactInformation2 {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation2";
}
#[cfg(feature = "Foundation")]
impl IContactInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation2_Impl, const OFFSET: isize>() -> IContactInformation2_Vtbl {
        unsafe extern "system" fn DisplayPictureDate<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayPictureDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureDate<Identity: ::windows::core::IUnknownImpl, Impl: IContactInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnvalue: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayPictureDate(::core::mem::transmute(&returnvalue)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactInformation2, OFFSET>(),
            DisplayPictureDate: DisplayPictureDate::<Identity, Impl, OFFSET>,
            SetDisplayPictureDate: SetDisplayPictureDate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactInformation2 as ::windows::core::Interface>::IID
    }
}
