#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIAssociatedIdentityProvider(::windows_core::IUnknown);
impl AsyncIAssociatedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_AssociateIdentity<P0>(&self, hwndparent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).Begin_AssociateIdentity)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_AssociateIdentity(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_AssociateIdentity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_DisassociateIdentity<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_DisassociateIdentity)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    pub unsafe fn Finish_DisassociateIdentity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_DisassociateIdentity)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_ChangeCredential<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_ChangeCredential)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    pub unsafe fn Finish_ChangeCredential(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_ChangeCredential)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(AsyncIAssociatedIdentityProvider, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIAssociatedIdentityProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAssociatedIdentityProvider {}
impl ::core::fmt::Debug for AsyncIAssociatedIdentityProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAssociatedIdentityProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIAssociatedIdentityProvider {
    type Vtable = AsyncIAssociatedIdentityProvider_Vtbl;
}
impl ::core::clone::Clone for AsyncIAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for AsyncIAssociatedIdentityProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2834d6ed_297e_4e72_8a51_961e86f05152);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAssociatedIdentityProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_AssociateIdentity: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_DisassociateIdentity: usize,
    pub Finish_DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_ChangeCredential: usize,
    pub Finish_ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIConnectedIdentityProvider(::windows_core::IUnknown);
impl AsyncIConnectedIdentityProvider {
    pub unsafe fn Begin_ConnectIdentity(&self, authbuffer: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_ConnectIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(authbuffer.as_ptr()), authbuffer.len() as _).ok()
    }
    pub unsafe fn Finish_ConnectIdentity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_ConnectIdentity)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_DisconnectIdentity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_DisconnectIdentity)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_DisconnectIdentity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_DisconnectIdentity)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_IsConnected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_IsConnected)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_IsConnected(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_IsConnected)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Begin_GetUrl<P0>(&self, identifier: IDENTITY_URL, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::Com::IBindCtx>,
    {
        (::windows_core::Interface::vtable(self).Begin_GetUrl)(::windows_core::Interface::as_raw(self), identifier, context.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Finish_GetUrl(&self, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_GetUrl)(::windows_core::Interface::as_raw(self), postdata, url).ok()
    }
    pub unsafe fn Begin_GetAccountState(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_GetAccountState)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_GetAccountState(&self) -> ::windows_core::Result<ACCOUNT_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_GetAccountState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(AsyncIConnectedIdentityProvider, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIConnectedIdentityProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIConnectedIdentityProvider {}
impl ::core::fmt::Debug for AsyncIConnectedIdentityProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIConnectedIdentityProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIConnectedIdentityProvider {
    type Vtable = AsyncIConnectedIdentityProvider_Vtbl;
}
impl ::core::clone::Clone for AsyncIConnectedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for AsyncIConnectedIdentityProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ce55141_bce9_4e15_824d_43d79f512f93);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIConnectedIdentityProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Begin_ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::HRESULT,
    pub Finish_ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin_DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish_DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Finish_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Finish_IsConnected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin_GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin_GetUrl: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Finish_GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Finish_GetUrl: usize,
    pub Begin_GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish_GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityAdvise(::windows_core::IUnknown);
impl AsyncIIdentityAdvise {
    pub unsafe fn Begin_IdentityUpdated<P0>(&self, dwidentityupdateevents: u32, lpszuniqueid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_IdentityUpdated)(::windows_core::Interface::as_raw(self), dwidentityupdateevents, lpszuniqueid.into_param().abi()).ok()
    }
    pub unsafe fn Finish_IdentityUpdated(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_IdentityUpdated)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(AsyncIIdentityAdvise, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIIdentityAdvise {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIIdentityAdvise {}
impl ::core::fmt::Debug for AsyncIIdentityAdvise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIIdentityAdvise").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIIdentityAdvise {
    type Vtable = AsyncIIdentityAdvise_Vtbl;
}
impl ::core::clone::Clone for AsyncIIdentityAdvise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for AsyncIIdentityAdvise {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ab4c8da_d038_4830_8dd9_3253c55a127f);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAdvise_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Begin_IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Finish_IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityAuthentication(::windows_core::IUnknown);
impl AsyncIIdentityAuthentication {
    pub unsafe fn Begin_SetIdentityCredential(&self, credbuffer: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_SetIdentityCredential)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(credbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), credbuffer.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn Finish_SetIdentityCredential(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_SetIdentityCredential)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: ::core::option::Option<*mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_ValidateIdentityCredential)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(credbuffer.as_ptr()), credbuffer.len() as _, ::core::mem::transmute(ppidentityproperties.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: ::core::option::Option<*mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_ValidateIdentityCredential)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppidentityproperties.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(AsyncIIdentityAuthentication, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIIdentityAuthentication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIIdentityAuthentication {}
impl ::core::fmt::Debug for AsyncIIdentityAuthentication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIIdentityAuthentication").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIIdentityAuthentication {
    type Vtable = AsyncIIdentityAuthentication_Vtbl;
}
impl ::core::clone::Clone for AsyncIIdentityAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for AsyncIIdentityAuthentication {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9a2f918_feca_4e9c_9633_61cbf13ed34d);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAuthentication_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Begin_SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::HRESULT,
    pub Finish_SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_ValidateIdentityCredential: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_ValidateIdentityCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityProvider(::windows_core::IUnknown);
impl AsyncIIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_GetIdentityEnum)(::windows_core::Interface::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_GetIdentityEnum(&self) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_GetIdentityEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Begin_Create<P0>(&self, lpszusername: P0, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_Create)(::windows_core::Interface::as_raw(self), lpszusername.into_param().abi(), pkeywordstoadd).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_Create(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_Create)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_Import<P0>(&self, ppropertystore: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (::windows_core::Interface::vtable(self).Begin_Import)(::windows_core::Interface::as_raw(self), ppropertystore.into_param().abi()).ok()
    }
    pub unsafe fn Finish_Import(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Import)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Begin_Delete<P0>(&self, lpszuniqueid: P0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_Delete)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi(), pkeywordstodelete).ok()
    }
    pub unsafe fn Finish_Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_FindByUniqueID<P0>(&self, lpszuniqueid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_FindByUniqueID)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_FindByUniqueID(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_FindByUniqueID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Begin_GetProviderPropertyStore(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_GetProviderPropertyStore)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_GetProviderPropertyStore(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_GetProviderPropertyStore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Begin_Advise<P0>(&self, pidentityadvise: P0, dwidentityupdateevents: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IIdentityAdvise>,
    {
        (::windows_core::Interface::vtable(self).Begin_Advise)(::windows_core::Interface::as_raw(self), pidentityadvise.into_param().abi(), dwidentityupdateevents).ok()
    }
    pub unsafe fn Finish_Advise(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_Advise)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Begin_UnAdvise(&self, dwcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_UnAdvise)(::windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn Finish_UnAdvise(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_UnAdvise)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(AsyncIIdentityProvider, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIIdentityProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIIdentityProvider {}
impl ::core::fmt::Debug for AsyncIIdentityProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIIdentityProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIIdentityProvider {
    type Vtable = AsyncIIdentityProvider_Vtbl;
}
impl ::core::clone::Clone for AsyncIIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for AsyncIIdentityProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6fc9901_c433_4646_8f48_4e4687aae2a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_GetIdentityEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Begin_Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: ::windows_core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Begin_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_Import: usize,
    pub Finish_Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Begin_Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Begin_Delete: usize,
    pub Finish_Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin_FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_FindByUniqueID: usize,
    pub Begin_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_GetProviderPropertyStore: usize,
    pub Begin_Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32) -> ::windows_core::HRESULT,
    pub Finish_Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub Begin_UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT,
    pub Finish_UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityStore(::windows_core::IUnknown);
impl AsyncIIdentityStore {
    pub unsafe fn Begin_GetCount(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_GetCount)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Begin_GetAt(&self, dwprovider: u32, pprovguid: ::core::option::Option<*mut ::windows_core::GUID>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_GetAt)(::windows_core::Interface::as_raw(self), dwprovider, ::core::mem::transmute(pprovguid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Finish_GetAt(&self, pprovguid: ::core::option::Option<*mut ::windows_core::GUID>, ppidentityprovider: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pprovguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppidentityprovider)).ok()
    }
    pub unsafe fn Begin_AddToCache<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_AddToCache)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi(), providerguid).ok()
    }
    pub unsafe fn Finish_AddToCache(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_AddToCache)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_ConvertToSid<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: ::core::option::Option<*mut u8>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_ConvertToSid)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi(), providerguid, cbsid, ::core::mem::transmute(psid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Finish_ConvertToSid(&self, psid: ::core::option::Option<*mut u8>, pcbrequiredsid: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_ConvertToSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psid.unwrap_or(::std::ptr::null_mut())), pcbrequiredsid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_EnumerateIdentities)(::windows_core::Interface::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_EnumerateIdentities(&self) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Finish_EnumerateIdentities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Begin_Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(AsyncIIdentityStore, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIIdentityStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIIdentityStore {}
impl ::core::fmt::Debug for AsyncIIdentityStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIIdentityStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIIdentityStore {
    type Vtable = AsyncIIdentityStore_Vtbl;
}
impl ::core::clone::Clone for AsyncIIdentityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for AsyncIIdentityStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeefa1616_48de_4872_aa64_6e6206535a51);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStore_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Begin_GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish_GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows_core::HRESULT,
    pub Begin_GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Finish_GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin_AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Finish_AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin_ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8) -> ::windows_core::HRESULT,
    pub Finish_ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_EnumerateIdentities: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_EnumerateIdentities: usize,
    pub Begin_Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish_Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityStoreEx(::windows_core::IUnknown);
impl AsyncIIdentityStoreEx {
    pub unsafe fn Begin_CreateConnectedIdentity<P0, P1>(&self, localname: P0, connectedname: P1, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_CreateConnectedIdentity)(::windows_core::Interface::as_raw(self), localname.into_param().abi(), connectedname.into_param().abi(), providerguid).ok()
    }
    pub unsafe fn Finish_CreateConnectedIdentity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_CreateConnectedIdentity)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_DeleteConnectedIdentity<P0>(&self, connectedname: P0, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Begin_DeleteConnectedIdentity)(::windows_core::Interface::as_raw(self), connectedname.into_param().abi(), providerguid).ok()
    }
    pub unsafe fn Finish_DeleteConnectedIdentity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_DeleteConnectedIdentity)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(AsyncIIdentityStoreEx, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIIdentityStoreEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIIdentityStoreEx {}
impl ::core::fmt::Debug for AsyncIIdentityStoreEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIIdentityStoreEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIIdentityStoreEx {
    type Vtable = AsyncIIdentityStoreEx_Vtbl;
}
impl ::core::clone::Clone for AsyncIIdentityStoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for AsyncIIdentityStoreEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfca3af9a_8a07_4eae_8632_ec3de658a36a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStoreEx_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Begin_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localname: ::windows_core::PCWSTR, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Finish_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Finish_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IAssociatedIdentityProvider(::windows_core::IUnknown);
impl IAssociatedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn AssociateIdentity<P0>(&self, hwndparent: P0) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssociateIdentity)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisassociateIdentity<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DisassociateIdentity)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeCredential<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ChangeCredential)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IAssociatedIdentityProvider, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IAssociatedIdentityProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssociatedIdentityProvider {}
impl ::core::fmt::Debug for IAssociatedIdentityProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssociatedIdentityProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAssociatedIdentityProvider {
    type Vtable = IAssociatedIdentityProvider_Vtbl;
}
impl ::core::clone::Clone for IAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAssociatedIdentityProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2af066b3_4cbb_4cba_a798_204b6af68cc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssociatedIdentityProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisassociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IConnectedIdentityProvider(::windows_core::IUnknown);
impl IConnectedIdentityProvider {
    pub unsafe fn ConnectIdentity(&self, authbuffer: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConnectIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(authbuffer.as_ptr()), authbuffer.len() as _).ok()
    }
    pub unsafe fn DisconnectIdentity(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectIdentity)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsConnected)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetUrl<P0>(&self, identifier: IDENTITY_URL, context: P0, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::Com::IBindCtx>,
    {
        (::windows_core::Interface::vtable(self).GetUrl)(::windows_core::Interface::as_raw(self), identifier, context.into_param().abi(), postdata, url).ok()
    }
    pub unsafe fn GetAccountState(&self) -> ::windows_core::Result<ACCOUNT_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAccountState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IConnectedIdentityProvider, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IConnectedIdentityProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectedIdentityProvider {}
impl ::core::fmt::Debug for IConnectedIdentityProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectedIdentityProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IConnectedIdentityProvider {
    type Vtable = IConnectedIdentityProvider_Vtbl;
}
impl ::core::clone::Clone for IConnectedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IConnectedIdentityProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7417b54_e08c_429b_96c8_678d1369ecb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedIdentityProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows_core::HRESULT,
    pub DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetUrl: usize,
    pub GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityAdvise(::windows_core::IUnknown);
impl IIdentityAdvise {
    pub unsafe fn IdentityUpdated<P0>(&self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).IdentityUpdated)(::windows_core::Interface::as_raw(self), dwidentityupdateevents.0 as _, lpszuniqueid.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IIdentityAdvise, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IIdentityAdvise {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIdentityAdvise {}
impl ::core::fmt::Debug for IIdentityAdvise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdentityAdvise").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIdentityAdvise {
    type Vtable = IIdentityAdvise_Vtbl;
}
impl ::core::clone::Clone for IIdentityAdvise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIdentityAdvise {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e982fed_d14b_440c_b8d6_bb386453d386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAdvise_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityAuthentication(::windows_core::IUnknown);
impl IIdentityAuthentication {
    pub unsafe fn SetIdentityCredential(&self, credbuffer: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIdentityCredential)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(credbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), credbuffer.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: ::core::option::Option<*mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ValidateIdentityCredential)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(credbuffer.as_ptr()), credbuffer.len() as _, ::core::mem::transmute(ppidentityproperties.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IIdentityAuthentication, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IIdentityAuthentication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIdentityAuthentication {}
impl ::core::fmt::Debug for IIdentityAuthentication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdentityAuthentication").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIdentityAuthentication {
    type Vtable = IIdentityAuthentication_Vtbl;
}
impl ::core::clone::Clone for IIdentityAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIdentityAuthentication {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e7ef254_979f_43b5_b74e_06e4eb7df0f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAuthentication_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ValidateIdentityCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityProvider(::windows_core::IUnknown);
impl IIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIdentityEnum)(::windows_core::Interface::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Create<P0>(&self, lpszusername: P0, pppropertystore: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), lpszusername.into_param().abi(), ::core::mem::transmute(pppropertystore), pkeywordstoadd).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Import<P0>(&self, ppropertystore: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (::windows_core::Interface::vtable(self).Import)(::windows_core::Interface::as_raw(self), ppropertystore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Delete<P0>(&self, lpszuniqueid: P0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi(), pkeywordstodelete).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn FindByUniqueID<P0>(&self, lpszuniqueid: P0) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindByUniqueID)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProviderPropertyStore(&self) -> ::windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProviderPropertyStore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Advise<P0>(&self, pidentityadvise: P0, dwidentityupdateevents: IdentityUpdateEvent) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<IIdentityAdvise>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), pidentityadvise.into_param().abi(), dwidentityupdateevents.0 as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn UnAdvise(&self, dwcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnAdvise)(::windows_core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IIdentityProvider, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IIdentityProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIdentityProvider {}
impl ::core::fmt::Debug for IIdentityProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdentityProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIdentityProvider {
    type Vtable = IIdentityProvider_Vtbl;
}
impl ::core::clone::Clone for IIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIdentityProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d1b9e0c_e8ba_4f55_a81b_bce934b948f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: ::windows_core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Import: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Delete: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    FindByUniqueID: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProviderPropertyStore: usize,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityStore(::windows_core::IUnknown);
impl IIdentityStore {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, dwprovider: u32, pprovguid: ::core::option::Option<*mut ::windows_core::GUID>, ppidentityprovider: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), dwprovider, ::core::mem::transmute(pprovguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppidentityprovider)).ok()
    }
    pub unsafe fn AddToCache<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddToCache)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi(), providerguid).ok()
    }
    pub unsafe fn ConvertToSid<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows_core::GUID, psid: ::core::option::Option<&mut [u8]>, pcbrequiredsid: *mut u16) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ConvertToSid)(::windows_core::Interface::as_raw(self), lpszuniqueid.into_param().abi(), providerguid, psid.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(psid.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcbrequiredsid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateIdentities)(::windows_core::Interface::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IIdentityStore, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IIdentityStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIdentityStore {}
impl ::core::fmt::Debug for IIdentityStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdentityStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIdentityStore {
    type Vtable = IIdentityStore_Vtbl;
}
impl ::core::clone::Clone for IIdentityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIdentityStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf586fa5_6f35_44f1_b209_b38e169772eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStore_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows_core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    EnumerateIdentities: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityStoreEx(::windows_core::IUnknown);
impl IIdentityStoreEx {
    pub unsafe fn CreateConnectedIdentity<P0, P1>(&self, localname: P0, connectedname: P1, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateConnectedIdentity)(::windows_core::Interface::as_raw(self), localname.into_param().abi(), connectedname.into_param().abi(), providerguid).ok()
    }
    pub unsafe fn DeleteConnectedIdentity<P0>(&self, connectedname: P0, providerguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteConnectedIdentity)(::windows_core::Interface::as_raw(self), connectedname.into_param().abi(), providerguid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IIdentityStoreEx, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IIdentityStoreEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIdentityStoreEx {}
impl ::core::fmt::Debug for IIdentityStoreEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdentityStoreEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIdentityStoreEx {
    type Vtable = IIdentityStoreEx_Vtbl;
}
impl ::core::clone::Clone for IIdentityStoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIdentityStoreEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9f9eb98_8f7f_4e38_9577_6980114ce32b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStoreEx_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localname: ::windows_core::PCWSTR, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectedname: ::windows_core::PCWSTR, providerguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CIdentityProfileHandler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecf5bf46_e3b6_449a_b56b_43f58f867814);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECTING: ACCOUNT_STATE = ACCOUNT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECT_COMPLETED: ACCOUNT_STATE = ACCOUNT_STATE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CoClassIdentityStore: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30d49246_d217_465f_b00b_ac9ddd652eb7);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ALL: IDENTITY_TYPE = IDENTITY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(64i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CREATED: IdentityUpdateEvent = IdentityUpdateEvent(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DELETED: IdentityUpdateEvent = IdentityUpdateEvent(16i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(128i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = IdentityUpdateEvent(8i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_ASSOCIATED: ::windows_core::PCWSTR = ::windows_core::w!("associated");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_CONNECTED: ::windows_core::PCWSTR = ::windows_core::w!("connected");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_HOMEGROUP: ::windows_core::PCWSTR = ::windows_core::w!("homegroup");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_LOCAL: ::windows_core::PCWSTR = ::windows_core::w!("local");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = IdentityUpdateEvent(32i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = IDENTITY_URL(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = IDENTITY_URL(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = IDENTITY_URL(6i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = IDENTITY_URL(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = IDENTITY_URL(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = IDENTITY_URL(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = IDENTITY_URL(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const NOT_CONNECTED: ACCOUNT_STATE = ACCOUNT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const OID_OAssociatedIdentityProviderObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98c5a3dd_db68_4f1a_8d2b_9079cdfeaf61);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_COMPLETE_ACCOUNT: ::windows_core::PCWSTR = ::windows_core::w!("CompleteAccount");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_MODERN_SETTINGS_ADD_USER: ::windows_core::PCWSTR = ::windows_core::w!("ModernSettingsAddUser");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_NTH_USER_FIRST_AUTH: ::windows_core::PCWSTR = ::windows_core::w!("NthUserFirstAuth");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_EXPERIENCE: ::windows_core::PCWSTR = ::windows_core::w!("OutOfBoxExperience");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_UPGRADE_EXPERIENCE: ::windows_core::PCWSTR = ::windows_core::w!("OutOfBoxUpgradeExperience");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_PROPERTY_STORE: ::windows_core::PCWSTR = ::windows_core::w!("PropertyStore");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_USER_NAME: ::windows_core::PCWSTR = ::windows_core::w!("Username");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACCOUNT_STATE(pub i32);
impl ::core::marker::Copy for ACCOUNT_STATE {}
impl ::core::clone::Clone for ACCOUNT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACCOUNT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ACCOUNT_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ACCOUNT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCOUNT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IDENTITY_TYPE(pub i32);
impl ::core::marker::Copy for IDENTITY_TYPE {}
impl ::core::clone::Clone for IDENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IDENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IDENTITY_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IDENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDENTITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IDENTITY_URL(pub i32);
impl ::core::marker::Copy for IDENTITY_URL {}
impl ::core::clone::Clone for IDENTITY_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IDENTITY_URL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IDENTITY_URL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IDENTITY_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDENTITY_URL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IdentityUpdateEvent(pub i32);
impl ::core::marker::Copy for IdentityUpdateEvent {}
impl ::core::clone::Clone for IdentityUpdateEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IdentityUpdateEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IdentityUpdateEvent {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IdentityUpdateEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IdentityUpdateEvent").field(&self.0).finish()
    }
}
impl IdentityUpdateEvent {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IdentityUpdateEvent {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IdentityUpdateEvent {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IdentityUpdateEvent {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IdentityUpdateEvent {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IdentityUpdateEvent {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
