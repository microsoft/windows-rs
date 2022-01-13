#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIAssociatedIdentityProviderImpl: Sized {
    fn Begin_AssociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Finish_AssociateIdentity(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_DisassociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_DisassociateIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_ChangeCredential(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_ChangeCredential(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIAssociatedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIAssociatedIdentityProviderVtbl {
        unsafe extern "system" fn Begin_AssociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_AssociateIdentity(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_AssociateIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_DisassociateIdentity(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_DisassociateIdentity().into()
        }
        unsafe extern "system" fn Begin_ChangeCredential<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_ChangeCredential(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_ChangeCredential<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_ChangeCredential().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_AssociateIdentity: Begin_AssociateIdentity::<Impl, IMPL_OFFSET>,
            Finish_AssociateIdentity: Finish_AssociateIdentity::<Impl, IMPL_OFFSET>,
            Begin_DisassociateIdentity: Begin_DisassociateIdentity::<Impl, IMPL_OFFSET>,
            Finish_DisassociateIdentity: Finish_DisassociateIdentity::<Impl, IMPL_OFFSET>,
            Begin_ChangeCredential: Begin_ChangeCredential::<Impl, IMPL_OFFSET>,
            Finish_ChangeCredential: Finish_ChangeCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIAssociatedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait AsyncIConnectedIdentityProviderImpl: Sized {
    fn Begin_ConnectIdentity(&mut self, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::Result<()>;
    fn Finish_ConnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_DisconnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Finish_DisconnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_IsConnected(&mut self) -> ::windows::core::Result<()>;
    fn Finish_IsConnected(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL>;
    fn Begin_GetUrl(&mut self, identifier: IDENTITY_URL, context: ::core::option::Option<super::super::super::super::System::Com::IBindCtx>) -> ::windows::core::Result<()>;
    fn Finish_GetUrl(&mut self, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Begin_GetAccountState(&mut self) -> ::windows::core::Result<()>;
    fn Finish_GetAccountState(&mut self) -> ::windows::core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl AsyncIConnectedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIConnectedIdentityProviderVtbl {
        unsafe extern "system" fn Begin_ConnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_ConnectIdentity(::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_ConnectIdentity().into()
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_DisconnectIdentity().into()
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_DisconnectIdentity().into()
        }
        unsafe extern "system" fn Begin_IsConnected<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_IsConnected().into()
        }
        unsafe extern "system" fn Finish_IsConnected<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *connected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetUrl<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetUrl(::core::mem::transmute_copy(&identifier), ::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Finish_GetUrl<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_GetUrl(::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn Begin_GetAccountState<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetAccountState().into()
        }
        unsafe extern "system" fn Finish_GetAccountState<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetAccountState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_ConnectIdentity: Begin_ConnectIdentity::<Impl, IMPL_OFFSET>,
            Finish_ConnectIdentity: Finish_ConnectIdentity::<Impl, IMPL_OFFSET>,
            Begin_DisconnectIdentity: Begin_DisconnectIdentity::<Impl, IMPL_OFFSET>,
            Finish_DisconnectIdentity: Finish_DisconnectIdentity::<Impl, IMPL_OFFSET>,
            Begin_IsConnected: Begin_IsConnected::<Impl, IMPL_OFFSET>,
            Finish_IsConnected: Finish_IsConnected::<Impl, IMPL_OFFSET>,
            Begin_GetUrl: Begin_GetUrl::<Impl, IMPL_OFFSET>,
            Finish_GetUrl: Finish_GetUrl::<Impl, IMPL_OFFSET>,
            Begin_GetAccountState: Begin_GetAccountState::<Impl, IMPL_OFFSET>,
            Finish_GetAccountState: Finish_GetAccountState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIConnectedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIIdentityAdviseImpl: Sized {
    fn Begin_IdentityUpdated(&mut self, dwidentityupdateevents: u32, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_IdentityUpdated(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIIdentityAdviseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAdviseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityAdviseVtbl {
        unsafe extern "system" fn Begin_IdentityUpdated<Impl: AsyncIIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_IdentityUpdated(::core::mem::transmute_copy(&dwidentityupdateevents), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Impl: AsyncIIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_IdentityUpdated().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_IdentityUpdated: Begin_IdentityUpdated::<Impl, IMPL_OFFSET>,
            Finish_IdentityUpdated: Finish_IdentityUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityAdvise as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIIdentityAuthenticationImpl: Sized {
    fn Begin_SetIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::Result<()>;
    fn Finish_SetIdentityCredential(&mut self) -> ::windows::core::Result<()>;
    fn Begin_ValidateIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Finish_ValidateIdentityCredential(&mut self, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIIdentityAuthenticationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthenticationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityAuthenticationVtbl {
        unsafe extern "system" fn Begin_SetIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_SetIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_SetIdentityCredential().into()
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_ValidateIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_ValidateIdentityCredential(::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_SetIdentityCredential: Begin_SetIdentityCredential::<Impl, IMPL_OFFSET>,
            Finish_SetIdentityCredential: Finish_SetIdentityCredential::<Impl, IMPL_OFFSET>,
            Begin_ValidateIdentityCredential: Begin_ValidateIdentityCredential::<Impl, IMPL_OFFSET>,
            Finish_ValidateIdentityCredential: Finish_ValidateIdentityCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityAuthentication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityProviderImpl: Sized {
    fn Begin_GetIdentityEnum(&mut self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Finish_GetIdentityEnum(&mut self) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Create(&mut self, lpszusername: super::super::super::super::Foundation::PWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Finish_Create(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Import(&mut self, ppropertystore: ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Finish_Import(&mut self) -> ::windows::core::Result<()>;
    fn Begin_Delete(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Finish_Delete(&mut self) -> ::windows::core::Result<()>;
    fn Begin_FindByUniqueID(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_FindByUniqueID(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_GetProviderPropertyStore(&mut self) -> ::windows::core::Result<()>;
    fn Finish_GetProviderPropertyStore(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Advise(&mut self, pidentityadvise: ::core::option::Option<IIdentityAdvise>, dwidentityupdateevents: u32) -> ::windows::core::Result<()>;
    fn Finish_Advise(&mut self) -> ::windows::core::Result<u32>;
    fn Begin_UnAdvise(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn Finish_UnAdvise(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityProviderVtbl {
        unsafe extern "system" fn Begin_GetIdentityEnum<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetIdentityEnum(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetIdentityEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Create<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_Create(::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Finish_Create<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Create() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Import<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_Import(::core::mem::transmute(&ppropertystore)).into()
        }
        unsafe extern "system" fn Finish_Import<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_Import().into()
        }
        unsafe extern "system" fn Begin_Delete<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_Delete(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn Finish_Delete<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_Delete().into()
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_FindByUniqueID(::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_FindByUniqueID() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetProviderPropertyStore().into()
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetProviderPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Advise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_Advise(::core::mem::transmute(&pidentityadvise), ::core::mem::transmute_copy(&dwidentityupdateevents)).into()
        }
        unsafe extern "system" fn Finish_Advise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Advise() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_UnAdvise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_UnAdvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn Finish_UnAdvise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_UnAdvise().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_GetIdentityEnum: Begin_GetIdentityEnum::<Impl, IMPL_OFFSET>,
            Finish_GetIdentityEnum: Finish_GetIdentityEnum::<Impl, IMPL_OFFSET>,
            Begin_Create: Begin_Create::<Impl, IMPL_OFFSET>,
            Finish_Create: Finish_Create::<Impl, IMPL_OFFSET>,
            Begin_Import: Begin_Import::<Impl, IMPL_OFFSET>,
            Finish_Import: Finish_Import::<Impl, IMPL_OFFSET>,
            Begin_Delete: Begin_Delete::<Impl, IMPL_OFFSET>,
            Finish_Delete: Finish_Delete::<Impl, IMPL_OFFSET>,
            Begin_FindByUniqueID: Begin_FindByUniqueID::<Impl, IMPL_OFFSET>,
            Finish_FindByUniqueID: Finish_FindByUniqueID::<Impl, IMPL_OFFSET>,
            Begin_GetProviderPropertyStore: Begin_GetProviderPropertyStore::<Impl, IMPL_OFFSET>,
            Finish_GetProviderPropertyStore: Finish_GetProviderPropertyStore::<Impl, IMPL_OFFSET>,
            Begin_Advise: Begin_Advise::<Impl, IMPL_OFFSET>,
            Finish_Advise: Finish_Advise::<Impl, IMPL_OFFSET>,
            Begin_UnAdvise: Begin_UnAdvise::<Impl, IMPL_OFFSET>,
            Finish_UnAdvise: Finish_UnAdvise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityStoreImpl: Sized {
    fn Begin_GetCount(&mut self) -> ::windows::core::Result<()>;
    fn Finish_GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn Begin_GetAt(&mut self, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Finish_GetAt(&mut self, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Begin_AddToCache(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Finish_AddToCache(&mut self) -> ::windows::core::Result<()>;
    fn Begin_ConvertToSid(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::Result<()>;
    fn Finish_ConvertToSid(&mut self, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::Result<()>;
    fn Begin_EnumerateIdentities(&mut self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Finish_EnumerateIdentities(&mut self) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Reset(&mut self) -> ::windows::core::Result<()>;
    fn Finish_Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityStoreVtbl {
        unsafe extern "system" fn Begin_GetCount<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetCount().into()
        }
        unsafe extern "system" fn Finish_GetCount<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetAt<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_GetAt(::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid)).into()
        }
        unsafe extern "system" fn Finish_GetAt<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_GetAt(::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn Begin_AddToCache<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_AddToCache(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_AddToCache<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_AddToCache().into()
        }
        unsafe extern "system" fn Begin_ConvertToSid<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_ConvertToSid(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn Finish_ConvertToSid<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_ConvertToSid(::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_EnumerateIdentities(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_EnumerateIdentities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Reset<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_Reset().into()
        }
        unsafe extern "system" fn Finish_Reset<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_GetCount: Begin_GetCount::<Impl, IMPL_OFFSET>,
            Finish_GetCount: Finish_GetCount::<Impl, IMPL_OFFSET>,
            Begin_GetAt: Begin_GetAt::<Impl, IMPL_OFFSET>,
            Finish_GetAt: Finish_GetAt::<Impl, IMPL_OFFSET>,
            Begin_AddToCache: Begin_AddToCache::<Impl, IMPL_OFFSET>,
            Finish_AddToCache: Finish_AddToCache::<Impl, IMPL_OFFSET>,
            Begin_ConvertToSid: Begin_ConvertToSid::<Impl, IMPL_OFFSET>,
            Finish_ConvertToSid: Finish_ConvertToSid::<Impl, IMPL_OFFSET>,
            Begin_EnumerateIdentities: Begin_EnumerateIdentities::<Impl, IMPL_OFFSET>,
            Finish_EnumerateIdentities: Finish_EnumerateIdentities::<Impl, IMPL_OFFSET>,
            Begin_Reset: Begin_Reset::<Impl, IMPL_OFFSET>,
            Finish_Reset: Finish_Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIIdentityStoreExImpl: Sized {
    fn Begin_CreateConnectedIdentity(&mut self, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Finish_CreateConnectedIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_DeleteConnectedIdentity(&mut self, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Finish_DeleteConnectedIdentity(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIIdentityStoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityStoreExVtbl {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_CreateConnectedIdentity(::core::mem::transmute_copy(&localname), ::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_CreateConnectedIdentity().into()
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_DeleteConnectedIdentity(::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_DeleteConnectedIdentity().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin_CreateConnectedIdentity: Begin_CreateConnectedIdentity::<Impl, IMPL_OFFSET>,
            Finish_CreateConnectedIdentity: Finish_CreateConnectedIdentity::<Impl, IMPL_OFFSET>,
            Begin_DeleteConnectedIdentity: Begin_DeleteConnectedIdentity::<Impl, IMPL_OFFSET>,
            Finish_DeleteConnectedIdentity: Finish_DeleteConnectedIdentity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityStoreEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAssociatedIdentityProviderImpl: Sized {
    fn AssociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn DisassociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ChangeCredential(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IAssociatedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssociatedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssociatedIdentityProviderVtbl {
        unsafe extern "system" fn AssociateIdentity<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateIdentity(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisassociateIdentity<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisassociateIdentity(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn ChangeCredential<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeCredential(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AssociateIdentity: AssociateIdentity::<Impl, IMPL_OFFSET>,
            DisassociateIdentity: DisassociateIdentity::<Impl, IMPL_OFFSET>,
            ChangeCredential: ChangeCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssociatedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IConnectedIdentityProviderImpl: Sized {
    fn ConnectIdentity(&mut self, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::Result<()>;
    fn DisconnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn IsConnected(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL>;
    fn GetUrl(&mut self, identifier: IDENTITY_URL, context: ::core::option::Option<super::super::super::super::System::Com::IBindCtx>, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAccountState(&mut self) -> ::windows::core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IConnectedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedIdentityProviderVtbl {
        unsafe extern "system" fn ConnectIdentity<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectIdentity(::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn DisconnectIdentity<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectIdentity().into()
        }
        unsafe extern "system" fn IsConnected<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *connected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUrl<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUrl(::core::mem::transmute_copy(&identifier), ::core::mem::transmute(&context), ::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn GetAccountState<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccountState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConnectIdentity: ConnectIdentity::<Impl, IMPL_OFFSET>,
            DisconnectIdentity: DisconnectIdentity::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            GetUrl: GetUrl::<Impl, IMPL_OFFSET>,
            GetAccountState: GetAccountState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IIdentityAdviseImpl: Sized {
    fn IdentityUpdated(&mut self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IIdentityAdviseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAdviseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityAdviseVtbl {
        unsafe extern "system" fn IdentityUpdated<Impl: IIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IdentityUpdated(::core::mem::transmute_copy(&dwidentityupdateevents), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IdentityUpdated: IdentityUpdated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityAdvise as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IIdentityAuthenticationImpl: Sized {
    fn SetIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::Result<()>;
    fn ValidateIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IIdentityAuthenticationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAuthenticationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityAuthenticationVtbl {
        unsafe extern "system" fn SetIdentityCredential<Impl: IIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn ValidateIdentityCredential<Impl: IIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetIdentityCredential: SetIdentityCredential::<Impl, IMPL_OFFSET>,
            ValidateIdentityCredential: ValidateIdentityCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityAuthentication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityProviderImpl: Sized {
    fn GetIdentityEnum(&mut self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Create(&mut self, lpszusername: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Import(&mut self, ppropertystore: ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Delete(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn FindByUniqueID(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetProviderPropertyStore(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Advise(&mut self, pidentityadvise: ::core::option::Option<IIdentityAdvise>, dwidentityupdateevents: IdentityUpdateEvent) -> ::windows::core::Result<u32>;
    fn UnAdvise(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityProviderVtbl {
        unsafe extern "system" fn GetIdentityEnum<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentityEnum(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&pppropertystore), ::core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Import<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Import(::core::mem::transmute(&ppropertystore)).into()
        }
        unsafe extern "system" fn Delete<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn FindByUniqueID<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindByUniqueID(::core::mem::transmute_copy(&lpszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderPropertyStore<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(::core::mem::transmute(&pidentityadvise), ::core::mem::transmute_copy(&dwidentityupdateevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnAdvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIdentityEnum: GetIdentityEnum::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Import: Import::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            FindByUniqueID: FindByUniqueID::<Impl, IMPL_OFFSET>,
            GetProviderPropertyStore: GetProviderPropertyStore::<Impl, IMPL_OFFSET>,
            Advise: Advise::<Impl, IMPL_OFFSET>,
            UnAdvise: UnAdvise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityStoreImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn AddToCache(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ConvertToSid(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::Result<()>;
    fn EnumerateIdentities(&mut self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityStoreVtbl {
        unsafe extern "system" fn GetCount<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn AddToCache<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToCache(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn ConvertToSid<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertToSid(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn EnumerateIdentities<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateIdentities(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            AddToCache: AddToCache::<Impl, IMPL_OFFSET>,
            ConvertToSid: ConvertToSid::<Impl, IMPL_OFFSET>,
            EnumerateIdentities: EnumerateIdentities::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IIdentityStoreExImpl: Sized {
    fn CreateConnectedIdentity(&mut self, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DeleteConnectedIdentity(&mut self, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IIdentityStoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityStoreExVtbl {
        unsafe extern "system" fn CreateConnectedIdentity<Impl: IIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateConnectedIdentity(::core::mem::transmute_copy(&localname), ::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Impl: IIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteConnectedIdentity(::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateConnectedIdentity: CreateConnectedIdentity::<Impl, IMPL_OFFSET>,
            DeleteConnectedIdentity: DeleteConnectedIdentity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityStoreEx as ::windows::core::Interface>::IID
    }
}
