pub trait AsyncIAssociatedIdentityProviderImpl: Sized {
    fn Begin_AssociateIdentity();
    fn Finish_AssociateIdentity();
    fn Begin_DisassociateIdentity();
    fn Finish_DisassociateIdentity();
    fn Begin_ChangeCredential();
    fn Finish_ChangeCredential();
}
impl ::windows::core::RuntimeName for AsyncIAssociatedIdentityProvider {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.AsyncIAssociatedIdentityProvider";
}
impl AsyncIAssociatedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>() -> AsyncIAssociatedIdentityProviderVtbl {
        unsafe extern "system" fn Begin_AssociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_AssociateIdentity(&*(&hwndparent as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_AssociateIdentity(::core::mem::transmute_copy(&pppropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_DisassociateIdentity(&*(&hwndparent as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_DisassociateIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_ChangeCredential<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_ChangeCredential(&*(&hwndparent as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_ChangeCredential<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_ChangeCredential() {
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
            ::windows::core::GetRuntimeClassName::<AsyncIAssociatedIdentityProvider>,
            ::windows::core::GetTrustLevel,
            Begin_AssociateIdentity::<Impl, OFFSET>,
            Finish_AssociateIdentity::<Impl, OFFSET>,
            Begin_DisassociateIdentity::<Impl, OFFSET>,
            Finish_DisassociateIdentity::<Impl, OFFSET>,
            Begin_ChangeCredential::<Impl, OFFSET>,
            Finish_ChangeCredential::<Impl, OFFSET>,
        )
    }
}
pub trait AsyncIConnectedIdentityProviderImpl: Sized {
    fn Begin_ConnectIdentity();
    fn Finish_ConnectIdentity();
    fn Begin_DisconnectIdentity();
    fn Finish_DisconnectIdentity();
    fn Begin_IsConnected();
    fn Finish_IsConnected();
    fn Begin_GetUrl();
    fn Finish_GetUrl();
    fn Begin_GetAccountState();
    fn Finish_GetAccountState();
}
impl ::windows::core::RuntimeName for AsyncIConnectedIdentityProvider {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.AsyncIConnectedIdentityProvider";
}
impl AsyncIConnectedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>() -> AsyncIConnectedIdentityProviderVtbl {
        unsafe extern "system" fn Begin_ConnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_ConnectIdentity(authbuffer, authbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_ConnectIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_DisconnectIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_DisconnectIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_IsConnected<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_IsConnected<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_IsConnected(::core::mem::transmute_copy(&connected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetUrl<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_GetUrl(identifier, &*(&context as *const <super::super::super::super::System::Com::IBindCtx as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::IBindCtx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetUrl<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetUrl(::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetAccountState<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_GetAccountState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetAccountState<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetAccountState(::core::mem::transmute_copy(&pstate)) {
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
            ::windows::core::GetRuntimeClassName::<AsyncIConnectedIdentityProvider>,
            ::windows::core::GetTrustLevel,
            Begin_ConnectIdentity::<Impl, OFFSET>,
            Finish_ConnectIdentity::<Impl, OFFSET>,
            Begin_DisconnectIdentity::<Impl, OFFSET>,
            Finish_DisconnectIdentity::<Impl, OFFSET>,
            Begin_IsConnected::<Impl, OFFSET>,
            Finish_IsConnected::<Impl, OFFSET>,
            Begin_GetUrl::<Impl, OFFSET>,
            Finish_GetUrl::<Impl, OFFSET>,
            Begin_GetAccountState::<Impl, OFFSET>,
            Finish_GetAccountState::<Impl, OFFSET>,
        )
    }
}
pub trait AsyncIIdentityAdviseImpl: Sized {
    fn Begin_IdentityUpdated();
    fn Finish_IdentityUpdated();
}
impl ::windows::core::RuntimeName for AsyncIIdentityAdvise {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.AsyncIIdentityAdvise";
}
impl AsyncIIdentityAdviseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAdviseImpl, const OFFSET: isize>() -> AsyncIIdentityAdviseVtbl {
        unsafe extern "system" fn Begin_IdentityUpdated<Impl: AsyncIIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_IdentityUpdated(dwidentityupdateevents, &*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Impl: AsyncIIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_IdentityUpdated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIIdentityAdvise>, ::windows::core::GetTrustLevel, Begin_IdentityUpdated::<Impl, OFFSET>, Finish_IdentityUpdated::<Impl, OFFSET>)
    }
}
pub trait AsyncIIdentityAuthenticationImpl: Sized {
    fn Begin_SetIdentityCredential();
    fn Finish_SetIdentityCredential();
    fn Begin_ValidateIdentityCredential();
    fn Finish_ValidateIdentityCredential();
}
impl ::windows::core::RuntimeName for AsyncIIdentityAuthentication {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.AsyncIIdentityAuthentication";
}
impl AsyncIIdentityAuthenticationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>() -> AsyncIIdentityAuthenticationVtbl {
        unsafe extern "system" fn Begin_SetIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_SetIdentityCredential(credbuffer, credbufferlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_SetIdentityCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_ValidateIdentityCredential(credbuffer, credbufferlength, &*(&ppidentityproperties as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_ValidateIdentityCredential(&*(&ppidentityproperties as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIIdentityAuthentication>, ::windows::core::GetTrustLevel, Begin_SetIdentityCredential::<Impl, OFFSET>, Finish_SetIdentityCredential::<Impl, OFFSET>, Begin_ValidateIdentityCredential::<Impl, OFFSET>, Finish_ValidateIdentityCredential::<Impl, OFFSET>)
    }
}
pub trait AsyncIIdentityProviderImpl: Sized {
    fn Begin_GetIdentityEnum();
    fn Finish_GetIdentityEnum();
    fn Begin_Create();
    fn Finish_Create();
    fn Begin_Import();
    fn Finish_Import();
    fn Begin_Delete();
    fn Finish_Delete();
    fn Begin_FindByUniqueID();
    fn Finish_FindByUniqueID();
    fn Begin_GetProviderPropertyStore();
    fn Finish_GetProviderPropertyStore();
    fn Begin_Advise();
    fn Finish_Advise();
    fn Begin_UnAdvise();
    fn Finish_UnAdvise();
}
impl ::windows::core::RuntimeName for AsyncIIdentityProvider {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.AsyncIIdentityProvider";
}
impl AsyncIIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>() -> AsyncIIdentityProviderVtbl {
        unsafe extern "system" fn Begin_GetIdentityEnum<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_GetIdentityEnum(
                eidentitytype,
                &*(&pfilterkey as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilterpropvarvalue as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetIdentityEnum(::core::mem::transmute_copy(&ppidentityenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Create<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Create(&*(&lpszusername as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pkeywordstoadd as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Create<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Create(::core::mem::transmute_copy(&pppropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Import<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Import(&*(&ppropertystore as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Import<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Import() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Delete<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Delete(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pkeywordstodelete as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Delete<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_FindByUniqueID(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_FindByUniqueID(::core::mem::transmute_copy(&pppropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_GetProviderPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetProviderPropertyStore(::core::mem::transmute_copy(&pppropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Advise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Advise(&*(&pidentityadvise as *const <IIdentityAdvise as ::windows::core::Abi>::Abi as *const <IIdentityAdvise as ::windows::core::DefaultType>::DefaultType), dwidentityupdateevents) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Advise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Advise(::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_UnAdvise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_UnAdvise(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_UnAdvise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_UnAdvise() {
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
            ::windows::core::GetRuntimeClassName::<AsyncIIdentityProvider>,
            ::windows::core::GetTrustLevel,
            Begin_GetIdentityEnum::<Impl, OFFSET>,
            Finish_GetIdentityEnum::<Impl, OFFSET>,
            Begin_Create::<Impl, OFFSET>,
            Finish_Create::<Impl, OFFSET>,
            Begin_Import::<Impl, OFFSET>,
            Finish_Import::<Impl, OFFSET>,
            Begin_Delete::<Impl, OFFSET>,
            Finish_Delete::<Impl, OFFSET>,
            Begin_FindByUniqueID::<Impl, OFFSET>,
            Finish_FindByUniqueID::<Impl, OFFSET>,
            Begin_GetProviderPropertyStore::<Impl, OFFSET>,
            Finish_GetProviderPropertyStore::<Impl, OFFSET>,
            Begin_Advise::<Impl, OFFSET>,
            Finish_Advise::<Impl, OFFSET>,
            Begin_UnAdvise::<Impl, OFFSET>,
            Finish_UnAdvise::<Impl, OFFSET>,
        )
    }
}
pub trait AsyncIIdentityStoreImpl: Sized {
    fn Begin_GetCount();
    fn Finish_GetCount();
    fn Begin_GetAt();
    fn Finish_GetAt();
    fn Begin_AddToCache();
    fn Finish_AddToCache();
    fn Begin_ConvertToSid();
    fn Finish_ConvertToSid();
    fn Begin_EnumerateIdentities();
    fn Finish_EnumerateIdentities();
    fn Begin_Reset();
    fn Finish_Reset();
}
impl ::windows::core::RuntimeName for AsyncIIdentityStore {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.AsyncIIdentityStore";
}
impl AsyncIIdentityStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>() -> AsyncIIdentityStoreVtbl {
        unsafe extern "system" fn Begin_GetCount<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetCount<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetCount(::core::mem::transmute_copy(&pdwproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetAt<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_GetAt(dwprovider, &*(&pprovguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_GetAt<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetAt(&*(&pprovguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppidentityprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_AddToCache<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_AddToCache(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_AddToCache<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_AddToCache() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_ConvertToSid<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_ConvertToSid(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbsid, psid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_ConvertToSid<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_ConvertToSid(psid, ::core::mem::transmute_copy(&pcbrequiredsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_EnumerateIdentities(
                eidentitytype,
                &*(&pfilterkey as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilterpropvarvalue as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_EnumerateIdentities(::core::mem::transmute_copy(&ppidentityenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Reset<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Reset<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Reset() {
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
            ::windows::core::GetRuntimeClassName::<AsyncIIdentityStore>,
            ::windows::core::GetTrustLevel,
            Begin_GetCount::<Impl, OFFSET>,
            Finish_GetCount::<Impl, OFFSET>,
            Begin_GetAt::<Impl, OFFSET>,
            Finish_GetAt::<Impl, OFFSET>,
            Begin_AddToCache::<Impl, OFFSET>,
            Finish_AddToCache::<Impl, OFFSET>,
            Begin_ConvertToSid::<Impl, OFFSET>,
            Finish_ConvertToSid::<Impl, OFFSET>,
            Begin_EnumerateIdentities::<Impl, OFFSET>,
            Finish_EnumerateIdentities::<Impl, OFFSET>,
            Begin_Reset::<Impl, OFFSET>,
            Finish_Reset::<Impl, OFFSET>,
        )
    }
}
pub trait AsyncIIdentityStoreExImpl: Sized {
    fn Begin_CreateConnectedIdentity();
    fn Finish_CreateConnectedIdentity();
    fn Begin_DeleteConnectedIdentity();
    fn Finish_DeleteConnectedIdentity();
}
impl ::windows::core::RuntimeName for AsyncIIdentityStoreEx {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.AsyncIIdentityStoreEx";
}
impl AsyncIIdentityStoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>() -> AsyncIIdentityStoreExVtbl {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_CreateConnectedIdentity(
                &*(&localname as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&connectedname as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_CreateConnectedIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_DeleteConnectedIdentity(&*(&connectedname as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_DeleteConnectedIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIIdentityStoreEx>, ::windows::core::GetTrustLevel, Begin_CreateConnectedIdentity::<Impl, OFFSET>, Finish_CreateConnectedIdentity::<Impl, OFFSET>, Begin_DeleteConnectedIdentity::<Impl, OFFSET>, Finish_DeleteConnectedIdentity::<Impl, OFFSET>)
    }
}
pub trait IAssociatedIdentityProviderImpl: Sized {
    fn AssociateIdentity();
    fn DisassociateIdentity();
    fn ChangeCredential();
}
impl ::windows::core::RuntimeName for IAssociatedIdentityProvider {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.IAssociatedIdentityProvider";
}
impl IAssociatedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>() -> IAssociatedIdentityProviderVtbl {
        unsafe extern "system" fn AssociateIdentity<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateIdentity(&*(&hwndparent as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisassociateIdentity<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisassociateIdentity(&*(&hwndparent as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCredential<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeCredential(&*(&hwndparent as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAssociatedIdentityProvider>, ::windows::core::GetTrustLevel, AssociateIdentity::<Impl, OFFSET>, DisassociateIdentity::<Impl, OFFSET>, ChangeCredential::<Impl, OFFSET>)
    }
}
pub trait IConnectedIdentityProviderImpl: Sized {
    fn ConnectIdentity();
    fn DisconnectIdentity();
    fn IsConnected();
    fn GetUrl();
    fn GetAccountState();
}
impl ::windows::core::RuntimeName for IConnectedIdentityProvider {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.IConnectedIdentityProvider";
}
impl IConnectedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>() -> IConnectedIdentityProviderVtbl {
        unsafe extern "system" fn ConnectIdentity<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectIdentity(authbuffer, authbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectIdentity<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected(::core::mem::transmute_copy(&connected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUrl<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUrl(identifier, &*(&context as *const <super::super::super::super::System::Com::IBindCtx as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::IBindCtx as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccountState<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccountState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectedIdentityProvider>, ::windows::core::GetTrustLevel, ConnectIdentity::<Impl, OFFSET>, DisconnectIdentity::<Impl, OFFSET>, IsConnected::<Impl, OFFSET>, GetUrl::<Impl, OFFSET>, GetAccountState::<Impl, OFFSET>)
    }
}
pub trait IIdentityAdviseImpl: Sized {
    fn IdentityUpdated();
}
impl ::windows::core::RuntimeName for IIdentityAdvise {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.IIdentityAdvise";
}
impl IIdentityAdviseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAdviseImpl, const OFFSET: isize>() -> IIdentityAdviseVtbl {
        unsafe extern "system" fn IdentityUpdated<Impl: IIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdentityUpdated(dwidentityupdateevents, &*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIdentityAdvise>, ::windows::core::GetTrustLevel, IdentityUpdated::<Impl, OFFSET>)
    }
}
pub trait IIdentityAuthenticationImpl: Sized {
    fn SetIdentityCredential();
    fn ValidateIdentityCredential();
}
impl ::windows::core::RuntimeName for IIdentityAuthentication {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.IIdentityAuthentication";
}
impl IIdentityAuthenticationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAuthenticationImpl, const OFFSET: isize>() -> IIdentityAuthenticationVtbl {
        unsafe extern "system" fn SetIdentityCredential<Impl: IIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIdentityCredential(credbuffer, credbufferlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateIdentityCredential<Impl: IIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateIdentityCredential(credbuffer, credbufferlength, &*(&ppidentityproperties as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIdentityAuthentication>, ::windows::core::GetTrustLevel, SetIdentityCredential::<Impl, OFFSET>, ValidateIdentityCredential::<Impl, OFFSET>)
    }
}
pub trait IIdentityProviderImpl: Sized {
    fn GetIdentityEnum();
    fn Create();
    fn Import();
    fn Delete();
    fn FindByUniqueID();
    fn GetProviderPropertyStore();
    fn Advise();
    fn UnAdvise();
}
impl ::windows::core::RuntimeName for IIdentityProvider {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.IIdentityProvider";
}
impl IIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProviderImpl, const OFFSET: isize>() -> IIdentityProviderVtbl {
        unsafe extern "system" fn GetIdentityEnum<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentityEnum(
                eidentitytype,
                &*(&pfilterkey as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilterpropvarvalue as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppidentityenum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&lpszusername as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pppropertystore),
                &*(&pkeywordstoadd as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Import<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Import(&*(&ppropertystore as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pkeywordstodelete as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindByUniqueID<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindByUniqueID(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderPropertyStore<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderPropertyStore(::core::mem::transmute_copy(&pppropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&pidentityadvise as *const <IIdentityAdvise as ::windows::core::Abi>::Abi as *const <IIdentityAdvise as ::windows::core::DefaultType>::DefaultType), dwidentityupdateevents, ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnAdvise(dwcookie) {
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
            ::windows::core::GetRuntimeClassName::<IIdentityProvider>,
            ::windows::core::GetTrustLevel,
            GetIdentityEnum::<Impl, OFFSET>,
            Create::<Impl, OFFSET>,
            Import::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            FindByUniqueID::<Impl, OFFSET>,
            GetProviderPropertyStore::<Impl, OFFSET>,
            Advise::<Impl, OFFSET>,
            UnAdvise::<Impl, OFFSET>,
        )
    }
}
pub trait IIdentityStoreImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn AddToCache();
    fn ConvertToSid();
    fn EnumerateIdentities();
    fn Reset();
}
impl ::windows::core::RuntimeName for IIdentityStore {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.IIdentityStore";
}
impl IIdentityStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreImpl, const OFFSET: isize>() -> IIdentityStoreVtbl {
        unsafe extern "system" fn GetCount<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pdwproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(dwprovider, &*(&pprovguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppidentityprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToCache<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddToCache(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertToSid<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertToSid(&*(&lpszuniqueid as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbsid, psid, ::core::mem::transmute_copy(&pcbrequiredsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateIdentities<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateIdentities(
                eidentitytype,
                &*(&pfilterkey as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pfilterpropvarvalue as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppidentityenum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIdentityStore>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, AddToCache::<Impl, OFFSET>, ConvertToSid::<Impl, OFFSET>, EnumerateIdentities::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IIdentityStoreExImpl: Sized {
    fn CreateConnectedIdentity();
    fn DeleteConnectedIdentity();
}
impl ::windows::core::RuntimeName for IIdentityStoreEx {
    const NAME: &'static str = "Windows.Win32.Security.Authentication.Identity.Provider.IIdentityStoreEx";
}
impl IIdentityStoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreExImpl, const OFFSET: isize>() -> IIdentityStoreExVtbl {
        unsafe extern "system" fn CreateConnectedIdentity<Impl: IIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConnectedIdentity(
                &*(&localname as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&connectedname as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Impl: IIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteConnectedIdentity(&*(&connectedname as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&providerguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIdentityStoreEx>, ::windows::core::GetTrustLevel, CreateConnectedIdentity::<Impl, OFFSET>, DeleteConnectedIdentity::<Impl, OFFSET>)
    }
}
