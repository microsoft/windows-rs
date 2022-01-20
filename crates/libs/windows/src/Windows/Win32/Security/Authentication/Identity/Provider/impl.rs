#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIAssociatedIdentityProvider_Impl: Sized {
    fn Begin_AssociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Finish_AssociateIdentity(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_DisassociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_DisassociateIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_ChangeCredential(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_ChangeCredential(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>() -> AsyncIAssociatedIdentityProvider_Vtbl {
        unsafe extern "system" fn Begin_AssociateIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_AssociateIdentity(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_AssociateIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_DisassociateIdentity(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_DisassociateIdentity().into()
        }
        unsafe extern "system" fn Begin_ChangeCredential<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_ChangeCredential(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_ChangeCredential<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_ChangeCredential().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_AssociateIdentity: Begin_AssociateIdentity::<Identity, Impl, OFFSET>,
            Finish_AssociateIdentity: Finish_AssociateIdentity::<Identity, Impl, OFFSET>,
            Begin_DisassociateIdentity: Begin_DisassociateIdentity::<Identity, Impl, OFFSET>,
            Finish_DisassociateIdentity: Finish_DisassociateIdentity::<Identity, Impl, OFFSET>,
            Begin_ChangeCredential: Begin_ChangeCredential::<Identity, Impl, OFFSET>,
            Finish_ChangeCredential: Finish_ChangeCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIAssociatedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait AsyncIConnectedIdentityProvider_Impl: Sized {
    fn Begin_ConnectIdentity(&mut self, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::Result<()>;
    fn Finish_ConnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_DisconnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Finish_DisconnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_IsConnected(&mut self) -> ::windows::core::Result<()>;
    fn Finish_IsConnected(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL>;
    fn Begin_GetUrl(&mut self, identifier: IDENTITY_URL, context: &::core::option::Option<super::super::super::super::System::Com::IBindCtx>) -> ::windows::core::Result<()>;
    fn Finish_GetUrl(&mut self, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Begin_GetAccountState(&mut self) -> ::windows::core::Result<()>;
    fn Finish_GetAccountState(&mut self) -> ::windows::core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl AsyncIConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>() -> AsyncIConnectedIdentityProvider_Vtbl {
        unsafe extern "system" fn Begin_ConnectIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_ConnectIdentity(::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_ConnectIdentity().into()
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_DisconnectIdentity().into()
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_DisconnectIdentity().into()
        }
        unsafe extern "system" fn Begin_IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_IsConnected().into()
        }
        unsafe extern "system" fn Finish_IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *connected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetUrl<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetUrl(::core::mem::transmute_copy(&identifier), ::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Finish_GetUrl<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_GetUrl(::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn Begin_GetAccountState<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetAccountState().into()
        }
        unsafe extern "system" fn Finish_GetAccountState<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_GetAccountState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_ConnectIdentity: Begin_ConnectIdentity::<Identity, Impl, OFFSET>,
            Finish_ConnectIdentity: Finish_ConnectIdentity::<Identity, Impl, OFFSET>,
            Begin_DisconnectIdentity: Begin_DisconnectIdentity::<Identity, Impl, OFFSET>,
            Finish_DisconnectIdentity: Finish_DisconnectIdentity::<Identity, Impl, OFFSET>,
            Begin_IsConnected: Begin_IsConnected::<Identity, Impl, OFFSET>,
            Finish_IsConnected: Finish_IsConnected::<Identity, Impl, OFFSET>,
            Begin_GetUrl: Begin_GetUrl::<Identity, Impl, OFFSET>,
            Finish_GetUrl: Finish_GetUrl::<Identity, Impl, OFFSET>,
            Begin_GetAccountState: Begin_GetAccountState::<Identity, Impl, OFFSET>,
            Finish_GetAccountState: Finish_GetAccountState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIConnectedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIIdentityAdvise_Impl: Sized {
    fn Begin_IdentityUpdated(&mut self, dwidentityupdateevents: u32, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_IdentityUpdated(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIIdentityAdvise_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: isize>() -> AsyncIIdentityAdvise_Vtbl {
        unsafe extern "system" fn Begin_IdentityUpdated<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_IdentityUpdated(::core::mem::transmute_copy(&dwidentityupdateevents), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAdvise_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_IdentityUpdated().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_IdentityUpdated: Begin_IdentityUpdated::<Identity, Impl, OFFSET>,
            Finish_IdentityUpdated: Finish_IdentityUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityAdvise as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIIdentityAuthentication_Impl: Sized {
    fn Begin_SetIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::Result<()>;
    fn Finish_SetIdentityCredential(&mut self) -> ::windows::core::Result<()>;
    fn Begin_ValidateIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Finish_ValidateIdentityCredential(&mut self, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIIdentityAuthentication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>() -> AsyncIIdentityAuthentication_Vtbl {
        unsafe extern "system" fn Begin_SetIdentityCredential<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_SetIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_SetIdentityCredential().into()
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_ValidateIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_ValidateIdentityCredential(::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_SetIdentityCredential: Begin_SetIdentityCredential::<Identity, Impl, OFFSET>,
            Finish_SetIdentityCredential: Finish_SetIdentityCredential::<Identity, Impl, OFFSET>,
            Begin_ValidateIdentityCredential: Begin_ValidateIdentityCredential::<Identity, Impl, OFFSET>,
            Finish_ValidateIdentityCredential: Finish_ValidateIdentityCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityAuthentication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityProvider_Impl: Sized {
    fn Begin_GetIdentityEnum(&mut self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Finish_GetIdentityEnum(&mut self) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Create(&mut self, lpszusername: super::super::super::super::Foundation::PWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Finish_Create(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Import(&mut self, ppropertystore: &::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Finish_Import(&mut self) -> ::windows::core::Result<()>;
    fn Begin_Delete(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Finish_Delete(&mut self) -> ::windows::core::Result<()>;
    fn Begin_FindByUniqueID(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Finish_FindByUniqueID(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_GetProviderPropertyStore(&mut self) -> ::windows::core::Result<()>;
    fn Finish_GetProviderPropertyStore(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Advise(&mut self, pidentityadvise: &::core::option::Option<IIdentityAdvise>, dwidentityupdateevents: u32) -> ::windows::core::Result<()>;
    fn Finish_Advise(&mut self) -> ::windows::core::Result<u32>;
    fn Begin_UnAdvise(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn Finish_UnAdvise(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>() -> AsyncIIdentityProvider_Vtbl {
        unsafe extern "system" fn Begin_GetIdentityEnum<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetIdentityEnum(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_GetIdentityEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Create<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_Create(::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Finish_Create<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_Create() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Import<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_Import(::core::mem::transmute(&ppropertystore)).into()
        }
        unsafe extern "system" fn Finish_Import<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_Import().into()
        }
        unsafe extern "system" fn Begin_Delete<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_Delete(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn Finish_Delete<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_Delete().into()
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_FindByUniqueID(::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_FindByUniqueID() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetProviderPropertyStore().into()
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_GetProviderPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Advise<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_Advise(::core::mem::transmute(&pidentityadvise), ::core::mem::transmute_copy(&dwidentityupdateevents)).into()
        }
        unsafe extern "system" fn Finish_Advise<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_Advise() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_UnAdvise<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_UnAdvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn Finish_UnAdvise<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_UnAdvise().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_GetIdentityEnum: Begin_GetIdentityEnum::<Identity, Impl, OFFSET>,
            Finish_GetIdentityEnum: Finish_GetIdentityEnum::<Identity, Impl, OFFSET>,
            Begin_Create: Begin_Create::<Identity, Impl, OFFSET>,
            Finish_Create: Finish_Create::<Identity, Impl, OFFSET>,
            Begin_Import: Begin_Import::<Identity, Impl, OFFSET>,
            Finish_Import: Finish_Import::<Identity, Impl, OFFSET>,
            Begin_Delete: Begin_Delete::<Identity, Impl, OFFSET>,
            Finish_Delete: Finish_Delete::<Identity, Impl, OFFSET>,
            Begin_FindByUniqueID: Begin_FindByUniqueID::<Identity, Impl, OFFSET>,
            Finish_FindByUniqueID: Finish_FindByUniqueID::<Identity, Impl, OFFSET>,
            Begin_GetProviderPropertyStore: Begin_GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Finish_GetProviderPropertyStore: Finish_GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Begin_Advise: Begin_Advise::<Identity, Impl, OFFSET>,
            Finish_Advise: Finish_Advise::<Identity, Impl, OFFSET>,
            Begin_UnAdvise: Begin_UnAdvise::<Identity, Impl, OFFSET>,
            Finish_UnAdvise: Finish_UnAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityStore_Impl: Sized {
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
impl AsyncIIdentityStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>() -> AsyncIIdentityStore_Vtbl {
        unsafe extern "system" fn Begin_GetCount<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetCount().into()
        }
        unsafe extern "system" fn Finish_GetCount<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetAt<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_GetAt(::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid)).into()
        }
        unsafe extern "system" fn Finish_GetAt<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_GetAt(::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn Begin_AddToCache<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_AddToCache(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_AddToCache<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_AddToCache().into()
        }
        unsafe extern "system" fn Begin_ConvertToSid<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_ConvertToSid(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn Finish_ConvertToSid<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_ConvertToSid(::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_EnumerateIdentities(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Finish_EnumerateIdentities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Reset<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_Reset().into()
        }
        unsafe extern "system" fn Finish_Reset<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_GetCount: Begin_GetCount::<Identity, Impl, OFFSET>,
            Finish_GetCount: Finish_GetCount::<Identity, Impl, OFFSET>,
            Begin_GetAt: Begin_GetAt::<Identity, Impl, OFFSET>,
            Finish_GetAt: Finish_GetAt::<Identity, Impl, OFFSET>,
            Begin_AddToCache: Begin_AddToCache::<Identity, Impl, OFFSET>,
            Finish_AddToCache: Finish_AddToCache::<Identity, Impl, OFFSET>,
            Begin_ConvertToSid: Begin_ConvertToSid::<Identity, Impl, OFFSET>,
            Finish_ConvertToSid: Finish_ConvertToSid::<Identity, Impl, OFFSET>,
            Begin_EnumerateIdentities: Begin_EnumerateIdentities::<Identity, Impl, OFFSET>,
            Finish_EnumerateIdentities: Finish_EnumerateIdentities::<Identity, Impl, OFFSET>,
            Begin_Reset: Begin_Reset::<Identity, Impl, OFFSET>,
            Finish_Reset: Finish_Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIIdentityStoreEx_Impl: Sized {
    fn Begin_CreateConnectedIdentity(&mut self, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Finish_CreateConnectedIdentity(&mut self) -> ::windows::core::Result<()>;
    fn Begin_DeleteConnectedIdentity(&mut self, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Finish_DeleteConnectedIdentity(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIIdentityStoreEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>() -> AsyncIIdentityStoreEx_Vtbl {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_CreateConnectedIdentity(::core::mem::transmute_copy(&localname), ::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_CreateConnectedIdentity().into()
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin_DeleteConnectedIdentity(::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish_DeleteConnectedIdentity().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_CreateConnectedIdentity: Begin_CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            Finish_CreateConnectedIdentity: Finish_CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            Begin_DeleteConnectedIdentity: Begin_DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
            Finish_DeleteConnectedIdentity: Finish_DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIIdentityStoreEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAssociatedIdentityProvider_Impl: Sized {
    fn AssociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn DisassociateIdentity(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ChangeCredential(&mut self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>() -> IAssociatedIdentityProvider_Vtbl {
        unsafe extern "system" fn AssociateIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AssociateIdentity(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisassociateIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisassociateIdentity(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn ChangeCredential<Identity: ::windows::core::IUnknownImpl, Impl: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeCredential(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssociateIdentity: AssociateIdentity::<Identity, Impl, OFFSET>,
            DisassociateIdentity: DisassociateIdentity::<Identity, Impl, OFFSET>,
            ChangeCredential: ChangeCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssociatedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IConnectedIdentityProvider_Impl: Sized {
    fn ConnectIdentity(&mut self, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::Result<()>;
    fn DisconnectIdentity(&mut self) -> ::windows::core::Result<()>;
    fn IsConnected(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL>;
    fn GetUrl(&mut self, identifier: IDENTITY_URL, context: &::core::option::Option<super::super::super::super::System::Com::IBindCtx>, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAccountState(&mut self) -> ::windows::core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>() -> IConnectedIdentityProvider_Vtbl {
        unsafe extern "system" fn ConnectIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectIdentity(::core::mem::transmute_copy(&authbuffer), ::core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn DisconnectIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectIdentity().into()
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *connected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUrl<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUrl(::core::mem::transmute_copy(&identifier), ::core::mem::transmute(&context), ::core::mem::transmute_copy(&postdata), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn GetAccountState<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAccountState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectIdentity: ConnectIdentity::<Identity, Impl, OFFSET>,
            DisconnectIdentity: DisconnectIdentity::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetUrl: GetUrl::<Identity, Impl, OFFSET>,
            GetAccountState: GetAccountState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IIdentityAdvise_Impl: Sized {
    fn IdentityUpdated(&mut self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IIdentityAdvise_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAdvise_Impl, const OFFSET: isize>() -> IIdentityAdvise_Vtbl {
        unsafe extern "system" fn IdentityUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAdvise_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IdentityUpdated(::core::mem::transmute_copy(&dwidentityupdateevents), ::core::mem::transmute_copy(&lpszuniqueid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IdentityUpdated: IdentityUpdated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityAdvise as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IIdentityAuthentication_Impl: Sized {
    fn SetIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::Result<()>;
    fn ValidateIdentityCredential(&mut self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IIdentityAuthentication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAuthentication_Impl, const OFFSET: isize>() -> IIdentityAuthentication_Vtbl {
        unsafe extern "system" fn SetIdentityCredential<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn ValidateIdentityCredential<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateIdentityCredential(::core::mem::transmute_copy(&credbuffer), ::core::mem::transmute_copy(&credbufferlength), ::core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetIdentityCredential: SetIdentityCredential::<Identity, Impl, OFFSET>,
            ValidateIdentityCredential: ValidateIdentityCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityAuthentication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityProvider_Impl: Sized {
    fn GetIdentityEnum(&mut self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Create(&mut self, lpszusername: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Import(&mut self, ppropertystore: &::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Delete(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn FindByUniqueID(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetProviderPropertyStore(&mut self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Advise(&mut self, pidentityadvise: &::core::option::Option<IIdentityAdvise>, dwidentityupdateevents: IdentityUpdateEvent) -> ::windows::core::Result<u32>;
    fn UnAdvise(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>() -> IIdentityProvider_Vtbl {
        unsafe extern "system" fn GetIdentityEnum<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIdentityEnum(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&pppropertystore), ::core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Import(::core::mem::transmute(&ppropertystore)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn FindByUniqueID<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindByUniqueID(::core::mem::transmute_copy(&lpszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderPropertyStore<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Advise(::core::mem::transmute(&pidentityadvise), ::core::mem::transmute_copy(&dwidentityupdateevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnAdvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIdentityEnum: GetIdentityEnum::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            FindByUniqueID: FindByUniqueID::<Identity, Impl, OFFSET>,
            GetProviderPropertyStore: GetProviderPropertyStore::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            UnAdvise: UnAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityStore_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn AddToCache(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ConvertToSid(&mut self, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::Result<()>;
    fn EnumerateIdentities(&mut self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStore_Impl, const OFFSET: isize>() -> IIdentityStore_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwprovider), ::core::mem::transmute_copy(&pprovguid), ::core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn AddToCache<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToCache(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn ConvertToSid<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertToSid(::core::mem::transmute_copy(&lpszuniqueid), ::core::mem::transmute_copy(&providerguid), ::core::mem::transmute_copy(&cbsid), ::core::mem::transmute_copy(&psid), ::core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn EnumerateIdentities<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateIdentities(::core::mem::transmute_copy(&eidentitytype), ::core::mem::transmute_copy(&pfilterkey), ::core::mem::transmute_copy(&pfilterpropvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidentityenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            AddToCache: AddToCache::<Identity, Impl, OFFSET>,
            ConvertToSid: ConvertToSid::<Identity, Impl, OFFSET>,
            EnumerateIdentities: EnumerateIdentities::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IIdentityStoreEx_Impl: Sized {
    fn CreateConnectedIdentity(&mut self, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DeleteConnectedIdentity(&mut self, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IIdentityStoreEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreEx_Impl, const OFFSET: isize>() -> IIdentityStoreEx_Vtbl {
        unsafe extern "system" fn CreateConnectedIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateConnectedIdentity(::core::mem::transmute_copy(&localname), ::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteConnectedIdentity(::core::mem::transmute_copy(&connectedname), ::core::mem::transmute_copy(&providerguid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateConnectedIdentity: CreateConnectedIdentity::<Identity, Impl, OFFSET>,
            DeleteConnectedIdentity: DeleteConnectedIdentity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityStoreEx as ::windows::core::Interface>::IID
    }
}
