#[doc = "*Required features: `\"Phone_PersonalInformation\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IContactInformation_Impl: Sized {
    fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn FamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetFamilyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GivenName(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetGivenName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn HonorificPrefix(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetHonorificPrefix(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn HonorificSuffix(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetHonorificSuffix(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GetDisplayPictureAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SetDisplayPictureAsync(&self, stream: ::core::option::Option<&super::super::Storage::Streams::IInputStream>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn DisplayPicture(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>>;
    fn ToVcardAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ToVcardWithOptionsAsync(&self, format: VCardFormat) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IContactInformation {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IContactInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>() -> IContactInformation_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn FamilyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFamilyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFamilyName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GivenName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GivenName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGivenName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGivenName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn HonorificPrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HonorificPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificPrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHonorificPrefix(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn HonorificSuffix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HonorificSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificSuffix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHonorificSuffix(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetDisplayPictureAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayPictureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetDisplayPictureAsync(::windows_core::from_raw_borrowed(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayPicture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToVcardAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardWithOptionsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: VCardFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToVcardWithOptionsAsync(format) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IContactInformation, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContactInformation as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IContactInformation2_Impl: Sized {
    fn DisplayPictureDate(&self) -> ::windows_core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureDate(&self, returnvalue: &super::super::Foundation::DateTime) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IContactInformation2 {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation2";
}
#[cfg(feature = "Foundation")]
impl IContactInformation2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation2_Impl, const OFFSET: isize>() -> IContactInformation2_Vtbl {
        unsafe extern "system" fn DisplayPictureDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayPictureDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnvalue: super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayPictureDate(::core::mem::transmute(&returnvalue)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IContactInformation2, OFFSET>(),
            DisplayPictureDate: DisplayPictureDate::<Identity, Impl, OFFSET>,
            SetDisplayPictureDate: SetDisplayPictureDate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContactInformation2 as ::windows_core::ComInterface>::IID
    }
}
