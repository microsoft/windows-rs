#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIAssociatedIdentityProviderImpl: Sized {
    fn Begin_AssociateIdentity();
    fn Finish_AssociateIdentity();
    fn Begin_DisassociateIdentity();
    fn Finish_DisassociateIdentity();
    fn Begin_ChangeCredential();
    fn Finish_ChangeCredential();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIAssociatedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAssociatedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIAssociatedIdentityProviderVtbl {
        unsafe extern "system" fn Begin_AssociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_ChangeCredential<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_ChangeCredential<Impl: AsyncIAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl AsyncIConnectedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIConnectedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIConnectedIdentityProviderVtbl {
        unsafe extern "system" fn Begin_ConnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_IsConnected<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_IsConnected<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_GetUrl<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_GetUrl<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_GetAccountState<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_GetAccountState<Impl: AsyncIConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Begin_IdentityUpdated();
    fn Finish_IdentityUpdated();
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIIdentityAdviseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAdviseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityAdviseVtbl {
        unsafe extern "system" fn Begin_IdentityUpdated<Impl: AsyncIIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Impl: AsyncIIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Begin_SetIdentityCredential();
    fn Finish_SetIdentityCredential();
    fn Begin_ValidateIdentityCredential();
    fn Finish_ValidateIdentityCredential();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIIdentityAuthenticationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityAuthenticationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityAuthenticationVtbl {
        unsafe extern "system" fn Begin_SetIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Impl: AsyncIIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityProviderVtbl {
        unsafe extern "system" fn Begin_GetIdentityEnum<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Create<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Create<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Import<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Import<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Delete<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Delete<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Advise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Advise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_UnAdvise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_UnAdvise<Impl: AsyncIIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityStoreVtbl {
        unsafe extern "system" fn Begin_GetCount<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_GetCount<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_GetAt<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_GetAt<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_AddToCache<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_AddToCache<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_ConvertToSid<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_ConvertToSid<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Reset<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Reset<Impl: AsyncIIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Begin_CreateConnectedIdentity();
    fn Finish_CreateConnectedIdentity();
    fn Begin_DeleteConnectedIdentity();
    fn Finish_DeleteConnectedIdentity();
}
#[cfg(feature = "Win32_Foundation")]
impl AsyncIIdentityStoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIIdentityStoreExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIIdentityStoreExVtbl {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Impl: AsyncIIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn AssociateIdentity();
    fn DisassociateIdentity();
    fn ChangeCredential();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IAssociatedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssociatedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssociatedIdentityProviderVtbl {
        unsafe extern "system" fn AssociateIdentity<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisassociateIdentity<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeCredential<Impl: IAssociatedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ConnectIdentity();
    fn DisconnectIdentity();
    fn IsConnected();
    fn GetUrl();
    fn GetAccountState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IConnectedIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedIdentityProviderVtbl {
        unsafe extern "system" fn ConnectIdentity<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectIdentity<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnected<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUrl<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccountState<Impl: IConnectedIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn IdentityUpdated();
}
#[cfg(feature = "Win32_Foundation")]
impl IIdentityAdviseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAdviseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityAdviseVtbl {
        unsafe extern "system" fn IdentityUpdated<Impl: IIdentityAdviseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IdentityUpdated: IdentityUpdated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdentityAdvise as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IIdentityAuthenticationImpl: Sized {
    fn SetIdentityCredential();
    fn ValidateIdentityCredential();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IIdentityAuthenticationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityAuthenticationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityAuthenticationVtbl {
        unsafe extern "system" fn SetIdentityCredential<Impl: IIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateIdentityCredential<Impl: IIdentityAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetIdentityEnum();
    fn Create();
    fn Import();
    fn Delete();
    fn FindByUniqueID();
    fn GetProviderPropertyStore();
    fn Advise();
    fn UnAdvise();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityProviderVtbl {
        unsafe extern "system" fn GetIdentityEnum<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Import<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindByUniqueID<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProviderPropertyStore<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Advise<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnAdvise<Impl: IIdentityProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCount();
    fn GetAt();
    fn AddToCache();
    fn ConvertToSid();
    fn EnumerateIdentities();
    fn Reset();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityStoreVtbl {
        unsafe extern "system" fn GetCount<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToCache<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertToSid<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateIdentities<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IIdentityStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn CreateConnectedIdentity();
    fn DeleteConnectedIdentity();
}
#[cfg(feature = "Win32_Foundation")]
impl IIdentityStoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdentityStoreExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdentityStoreExVtbl {
        unsafe extern "system" fn CreateConnectedIdentity<Impl: IIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Impl: IIdentityStoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
