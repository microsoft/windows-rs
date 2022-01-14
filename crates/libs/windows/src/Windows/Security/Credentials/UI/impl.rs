#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICredentialPickerOptions_Impl: Sized {
    fn SetCaption(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Caption(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetErrorCode(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAuthenticationProtocol(&mut self, value: AuthenticationProtocol) -> ::windows::core::Result<()>;
    fn AuthenticationProtocol(&mut self) -> ::windows::core::Result<AuthenticationProtocol>;
    fn SetCustomAuthenticationProtocol(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CustomAuthenticationProtocol(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreviousCredential(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn PreviousCredential(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetAlwaysDisplayDialog(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysDisplayDialog(&mut self) -> ::windows::core::Result<bool>;
    fn SetCallerSavesCredential(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CallerSavesCredential(&mut self) -> ::windows::core::Result<bool>;
    fn SetCredentialSaveOption(&mut self, value: CredentialSaveOption) -> ::windows::core::Result<()>;
    fn CredentialSaveOption(&mut self) -> ::windows::core::Result<CredentialSaveOption>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICredentialPickerOptions {
    const NAME: &'static str = "Windows.Security.Credentials.UI.ICredentialPickerOptions";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICredentialPickerOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialPickerOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICredentialPickerOptions_Vtbl {
        unsafe extern "system" fn SetCaption<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaption(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Caption<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMessage<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Message<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetErrorCode<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorCode(value).into()
        }
        unsafe extern "system" fn ErrorCode<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetName<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAuthenticationProtocol<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AuthenticationProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProtocol(value).into()
        }
        unsafe extern "system" fn AuthenticationProtocol<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AuthenticationProtocol) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCustomAuthenticationProtocol<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomAuthenticationProtocol(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CustomAuthenticationProtocol<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreviousCredential<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreviousCredential(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviousCredential<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlwaysDisplayDialog<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysDisplayDialog(value).into()
        }
        unsafe extern "system" fn AlwaysDisplayDialog<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCallerSavesCredential<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallerSavesCredential(value).into()
        }
        unsafe extern "system" fn CallerSavesCredential<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCredentialSaveOption<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CredentialSaveOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentialSaveOption(value).into()
        }
        unsafe extern "system" fn CredentialSaveOption<Impl: ICredentialPickerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICredentialPickerOptions, BASE_OFFSET>(),
            SetCaption: SetCaption::<Impl, IMPL_OFFSET>,
            Caption: Caption::<Impl, IMPL_OFFSET>,
            SetMessage: SetMessage::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            SetErrorCode: SetErrorCode::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetAuthenticationProtocol: SetAuthenticationProtocol::<Impl, IMPL_OFFSET>,
            AuthenticationProtocol: AuthenticationProtocol::<Impl, IMPL_OFFSET>,
            SetCustomAuthenticationProtocol: SetCustomAuthenticationProtocol::<Impl, IMPL_OFFSET>,
            CustomAuthenticationProtocol: CustomAuthenticationProtocol::<Impl, IMPL_OFFSET>,
            SetPreviousCredential: SetPreviousCredential::<Impl, IMPL_OFFSET>,
            PreviousCredential: PreviousCredential::<Impl, IMPL_OFFSET>,
            SetAlwaysDisplayDialog: SetAlwaysDisplayDialog::<Impl, IMPL_OFFSET>,
            AlwaysDisplayDialog: AlwaysDisplayDialog::<Impl, IMPL_OFFSET>,
            SetCallerSavesCredential: SetCallerSavesCredential::<Impl, IMPL_OFFSET>,
            CallerSavesCredential: CallerSavesCredential::<Impl, IMPL_OFFSET>,
            SetCredentialSaveOption: SetCredentialSaveOption::<Impl, IMPL_OFFSET>,
            CredentialSaveOption: CredentialSaveOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICredentialPickerOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICredentialPickerResults_Impl: Sized {
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
    fn CredentialSaveOption(&mut self) -> ::windows::core::Result<CredentialSaveOption>;
    fn CredentialSaved(&mut self) -> ::windows::core::Result<bool>;
    fn Credential(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CredentialDomainName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CredentialUserName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CredentialPassword(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICredentialPickerResults {
    const NAME: &'static str = "Windows.Security.Credentials.UI.ICredentialPickerResults";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICredentialPickerResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialPickerResults_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICredentialPickerResults_Vtbl {
        unsafe extern "system" fn ErrorCode<Impl: ICredentialPickerResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CredentialSaveOption<Impl: ICredentialPickerResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CredentialSaved<Impl: ICredentialPickerResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Credential<Impl: ICredentialPickerResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CredentialDomainName<Impl: ICredentialPickerResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CredentialUserName<Impl: ICredentialPickerResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CredentialPassword<Impl: ICredentialPickerResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICredentialPickerResults, BASE_OFFSET>(),
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            CredentialSaveOption: CredentialSaveOption::<Impl, IMPL_OFFSET>,
            CredentialSaved: CredentialSaved::<Impl, IMPL_OFFSET>,
            Credential: Credential::<Impl, IMPL_OFFSET>,
            CredentialDomainName: CredentialDomainName::<Impl, IMPL_OFFSET>,
            CredentialUserName: CredentialUserName::<Impl, IMPL_OFFSET>,
            CredentialPassword: CredentialPassword::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICredentialPickerResults as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICredentialPickerStatics_Impl: Sized {
    fn PickWithOptionsAsync(&mut self, options: &::core::option::Option<CredentialPickerOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
    fn PickWithMessageAsync(&mut self, targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
    fn PickWithCaptionAsync(&mut self, targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING, caption: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICredentialPickerStatics {
    const NAME: &'static str = "Windows.Security.Credentials.UI.ICredentialPickerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICredentialPickerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialPickerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICredentialPickerStatics_Vtbl {
        unsafe extern "system" fn PickWithOptionsAsync<Impl: ICredentialPickerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickWithMessageAsync<Impl: ICredentialPickerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickWithCaptionAsync<Impl: ICredentialPickerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, caption: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICredentialPickerStatics, BASE_OFFSET>(),
            PickWithOptionsAsync: PickWithOptionsAsync::<Impl, IMPL_OFFSET>,
            PickWithMessageAsync: PickWithMessageAsync::<Impl, IMPL_OFFSET>,
            PickWithCaptionAsync: PickWithCaptionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICredentialPickerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserConsentVerifierStatics_Impl: Sized {
    fn CheckAvailabilityAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerifierAvailability>>;
    fn RequestVerificationAsync(&mut self, message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerificationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserConsentVerifierStatics {
    const NAME: &'static str = "Windows.Security.Credentials.UI.IUserConsentVerifierStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserConsentVerifierStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserConsentVerifierStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserConsentVerifierStatics_Vtbl {
        unsafe extern "system" fn CheckAvailabilityAsync<Impl: IUserConsentVerifierStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestVerificationAsync<Impl: IUserConsentVerifierStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserConsentVerifierStatics, BASE_OFFSET>(),
            CheckAvailabilityAsync: CheckAvailabilityAsync::<Impl, IMPL_OFFSET>,
            RequestVerificationAsync: RequestVerificationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserConsentVerifierStatics as ::windows::core::Interface>::IID
    }
}
