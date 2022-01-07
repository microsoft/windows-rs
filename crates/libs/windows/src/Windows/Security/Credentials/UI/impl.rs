#[cfg(feature = "implement_exclusive")]
pub trait ICredentialPickerOptionsImpl: Sized {
    fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetErrorCode(&self, value: u32) -> ::windows::core::Result<()>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAuthenticationProtocol(&self, value: AuthenticationProtocol) -> ::windows::core::Result<()>;
    fn AuthenticationProtocol(&self) -> ::windows::core::Result<AuthenticationProtocol>;
    fn SetCustomAuthenticationProtocol(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CustomAuthenticationProtocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreviousCredential(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn PreviousCredential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetAlwaysDisplayDialog(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysDisplayDialog(&self) -> ::windows::core::Result<bool>;
    fn SetCallerSavesCredential(&self, value: bool) -> ::windows::core::Result<()>;
    fn CallerSavesCredential(&self) -> ::windows::core::Result<bool>;
    fn SetCredentialSaveOption(&self, value: CredentialSaveOption) -> ::windows::core::Result<()>;
    fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICredentialPickerOptions {
    const NAME: &'static str = "Windows.Security.Credentials.UI.ICredentialPickerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ICredentialPickerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>() -> ICredentialPickerOptionsVtbl {
        unsafe extern "system" fn SetCaption<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaption(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Caption<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Message<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetErrorCode<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorCode(value).into()
        }
        unsafe extern "system" fn ErrorCode<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetName<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProtocol<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AuthenticationProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProtocol(value).into()
        }
        unsafe extern "system" fn AuthenticationProtocol<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AuthenticationProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomAuthenticationProtocol<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomAuthenticationProtocol(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CustomAuthenticationProtocol<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomAuthenticationProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviousCredential<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreviousCredential(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviousCredential<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysDisplayDialog<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysDisplayDialog(value).into()
        }
        unsafe extern "system" fn AlwaysDisplayDialog<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysDisplayDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallerSavesCredential<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallerSavesCredential(value).into()
        }
        unsafe extern "system" fn CallerSavesCredential<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerSavesCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialSaveOption<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CredentialSaveOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentialSaveOption(value).into()
        }
        unsafe extern "system" fn CredentialSaveOption<Impl: ICredentialPickerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialSaveOption() {
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
            ::windows::core::GetRuntimeClassName::<ICredentialPickerOptions>,
            ::windows::core::GetTrustLevel,
            SetCaption::<Impl, OFFSET>,
            Caption::<Impl, OFFSET>,
            SetMessage::<Impl, OFFSET>,
            Message::<Impl, OFFSET>,
            SetErrorCode::<Impl, OFFSET>,
            ErrorCode::<Impl, OFFSET>,
            SetTargetName::<Impl, OFFSET>,
            TargetName::<Impl, OFFSET>,
            SetAuthenticationProtocol::<Impl, OFFSET>,
            AuthenticationProtocol::<Impl, OFFSET>,
            SetCustomAuthenticationProtocol::<Impl, OFFSET>,
            CustomAuthenticationProtocol::<Impl, OFFSET>,
            SetPreviousCredential::<Impl, OFFSET>,
            PreviousCredential::<Impl, OFFSET>,
            SetAlwaysDisplayDialog::<Impl, OFFSET>,
            AlwaysDisplayDialog::<Impl, OFFSET>,
            SetCallerSavesCredential::<Impl, OFFSET>,
            CallerSavesCredential::<Impl, OFFSET>,
            SetCredentialSaveOption::<Impl, OFFSET>,
            CredentialSaveOption::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialPickerResultsImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
    fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption>;
    fn CredentialSaved(&self) -> ::windows::core::Result<bool>;
    fn Credential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CredentialDomainName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CredentialUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CredentialPassword(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICredentialPickerResults {
    const NAME: &'static str = "Windows.Security.Credentials.UI.ICredentialPickerResults";
}
#[cfg(feature = "implement_exclusive")]
impl ICredentialPickerResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialPickerResultsImpl, const OFFSET: isize>() -> ICredentialPickerResultsVtbl {
        unsafe extern "system" fn ErrorCode<Impl: ICredentialPickerResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialSaveOption<Impl: ICredentialPickerResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialSaveOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialSaved<Impl: ICredentialPickerResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialSaved() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Credential<Impl: ICredentialPickerResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Credential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialDomainName<Impl: ICredentialPickerResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialDomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialUserName<Impl: ICredentialPickerResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialUserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialPassword<Impl: ICredentialPickerResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialPassword() {
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
            ::windows::core::GetRuntimeClassName::<ICredentialPickerResults>,
            ::windows::core::GetTrustLevel,
            ErrorCode::<Impl, OFFSET>,
            CredentialSaveOption::<Impl, OFFSET>,
            CredentialSaved::<Impl, OFFSET>,
            Credential::<Impl, OFFSET>,
            CredentialDomainName::<Impl, OFFSET>,
            CredentialUserName::<Impl, OFFSET>,
            CredentialPassword::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialPickerStaticsImpl: Sized {
    fn PickWithOptionsAsync(&self, options: &::core::option::Option<CredentialPickerOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
    fn PickWithMessageAsync(&self, targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
    fn PickWithCaptionAsync(&self, targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING, caption: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICredentialPickerStatics {
    const NAME: &'static str = "Windows.Security.Credentials.UI.ICredentialPickerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICredentialPickerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialPickerStaticsImpl, const OFFSET: isize>() -> ICredentialPickerStaticsVtbl {
        unsafe extern "system" fn PickWithOptionsAsync<Impl: ICredentialPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickWithOptionsAsync(&*(&options as *const <CredentialPickerOptions as ::windows::core::Abi>::Abi as *const <CredentialPickerOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickWithMessageAsync<Impl: ICredentialPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickWithMessageAsync(&*(&targetname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickWithCaptionAsync<Impl: ICredentialPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, caption: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickWithCaptionAsync(
                &*(&targetname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&caption as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICredentialPickerStatics>, ::windows::core::GetTrustLevel, PickWithOptionsAsync::<Impl, OFFSET>, PickWithMessageAsync::<Impl, OFFSET>, PickWithCaptionAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserConsentVerifierStaticsImpl: Sized {
    fn CheckAvailabilityAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerifierAvailability>>;
    fn RequestVerificationAsync(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerificationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserConsentVerifierStatics {
    const NAME: &'static str = "Windows.Security.Credentials.UI.IUserConsentVerifierStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserConsentVerifierStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserConsentVerifierStaticsImpl, const OFFSET: isize>() -> IUserConsentVerifierStaticsVtbl {
        unsafe extern "system" fn CheckAvailabilityAsync<Impl: IUserConsentVerifierStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckAvailabilityAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestVerificationAsync<Impl: IUserConsentVerifierStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestVerificationAsync(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserConsentVerifierStatics>, ::windows::core::GetTrustLevel, CheckAvailabilityAsync::<Impl, OFFSET>, RequestVerificationAsync::<Impl, OFFSET>)
    }
}
