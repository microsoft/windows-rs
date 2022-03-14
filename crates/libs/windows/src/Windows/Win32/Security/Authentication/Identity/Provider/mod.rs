#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACCOUNT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const NOT_CONNECTED: ACCOUNT_STATE = ACCOUNT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECTING: ACCOUNT_STATE = ACCOUNT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECT_COMPLETED: ACCOUNT_STATE = ACCOUNT_STATE(2i32);
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
unsafe impl ::windows::core::Abi for ACCOUNT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACCOUNT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCOUNT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIAssociatedIdentityProvider(::windows::core::IUnknown);
impl AsyncIAssociatedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_AssociateIdentity<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_AssociateIdentity)(::core::mem::transmute_copy(self), hwndparent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_AssociateIdentity(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_AssociateIdentity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_DisassociateIdentity<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_DisassociateIdentity)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_DisassociateIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_DisassociateIdentity)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_ChangeCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_ChangeCredential)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_ChangeCredential(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_ChangeCredential)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIAssociatedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIAssociatedIdentityProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAssociatedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIAssociatedIdentityProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for AsyncIAssociatedIdentityProvider {
    type Vtable = AsyncIAssociatedIdentityProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2834d6ed_297e_4e72_8a51_961e86f05152);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAssociatedIdentityProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_AssociateIdentity: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_DisassociateIdentity: usize,
    pub Finish_DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_ChangeCredential: usize,
    pub Finish_ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIConnectedIdentityProvider(::windows::core::IUnknown);
impl AsyncIConnectedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_ConnectIdentity(&self, authbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_ConnectIdentity)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(authbuffer)), authbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_ConnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_ConnectIdentity)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_DisconnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_DisconnectIdentity)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_DisconnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_DisconnectIdentity)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_IsConnected)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_IsConnected(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Begin_GetUrl<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::super::System::Com::IBindCtx>>(&self, identifier: IDENTITY_URL, context: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetUrl)(::core::mem::transmute_copy(self), ::core::mem::transmute(identifier), context.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Finish_GetUrl(&self, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_GetUrl)(::core::mem::transmute_copy(self), ::core::mem::transmute(postdata), ::core::mem::transmute(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_GetAccountState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetAccountState)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_GetAccountState(&self) -> ::windows::core::Result<ACCOUNT_STATE> {
        let mut result__: ACCOUNT_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_GetAccountState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ACCOUNT_STATE>(result__)
    }
}
impl ::core::convert::From<AsyncIConnectedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIConnectedIdentityProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIConnectedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIConnectedIdentityProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIConnectedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIConnectedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIConnectedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for AsyncIConnectedIdentityProvider {
    type Vtable = AsyncIConnectedIdentityProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ce55141_bce9_4e15_824d_43d79f512f93);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIConnectedIdentityProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT,
    pub Finish_ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Finish_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Finish_IsConnected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin_GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin_GetUrl: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Finish_GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Finish_GetUrl: usize,
    pub Begin_GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityAdvise(::windows::core::IUnknown);
impl AsyncIIdentityAdvise {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_IdentityUpdated<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwidentityupdateevents: u32, lpszuniqueid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_IdentityUpdated)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwidentityupdateevents), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_IdentityUpdated(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_IdentityUpdated)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIIdentityAdvise> for ::windows::core::IUnknown {
    fn from(value: AsyncIIdentityAdvise) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIIdentityAdvise> for ::windows::core::IUnknown {
    fn from(value: &AsyncIIdentityAdvise) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIIdentityAdvise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIIdentityAdvise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIIdentityAdvise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for AsyncIIdentityAdvise {
    type Vtable = AsyncIIdentityAdvise_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ab4c8da_d038_4830_8dd9_3253c55a127f);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAdvise_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Finish_IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityAuthentication(::windows::core::IUnknown);
impl AsyncIIdentityAuthentication {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_SetIdentityCredential(&self, credbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_SetIdentityCredential)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(credbuffer)), credbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_SetIdentityCredential(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_SetIdentityCredential)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_ValidateIdentityCredential)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(credbuffer)), credbuffer.len() as _, ::core::mem::transmute(ppidentityproperties)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_ValidateIdentityCredential)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppidentityproperties)).ok()
    }
}
impl ::core::convert::From<AsyncIIdentityAuthentication> for ::windows::core::IUnknown {
    fn from(value: AsyncIIdentityAuthentication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIIdentityAuthentication> for ::windows::core::IUnknown {
    fn from(value: &AsyncIIdentityAuthentication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIIdentityAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIIdentityAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIIdentityAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for AsyncIIdentityAuthentication {
    type Vtable = AsyncIIdentityAuthentication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9a2f918_feca_4e9c_9633_61cbf13ed34d);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAuthentication_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT,
    pub Finish_SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_ValidateIdentityCredential: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_ValidateIdentityCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityProvider(::windows::core::IUnknown);
impl AsyncIIdentityProvider {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetIdentityEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(eidentitytype), ::core::mem::transmute(pfilterkey), ::core::mem::transmute(pfilterpropvarvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_GetIdentityEnum(&self) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_GetIdentityEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszusername: Param0, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Create)(::core::mem::transmute_copy(self), lpszusername.into_param().abi(), ::core::mem::transmute(pkeywordstoadd)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_Create(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_Create)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_Import<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, ppropertystore: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Import)(::core::mem::transmute_copy(self), ppropertystore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_Import(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Import)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_Delete<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Delete)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::core::mem::transmute(pkeywordstodelete)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_FindByUniqueID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_FindByUniqueID)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_FindByUniqueID(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_FindByUniqueID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_GetProviderPropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetProviderPropertyStore)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_GetProviderPropertyStore(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_GetProviderPropertyStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_Advise<'a, Param0: ::windows::core::IntoParam<'a, IIdentityAdvise>>(&self, pidentityadvise: Param0, dwidentityupdateevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Advise)(::core::mem::transmute_copy(self), pidentityadvise.into_param().abi(), ::core::mem::transmute(dwidentityupdateevents)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_Advise(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_Advise)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_UnAdvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_UnAdvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_UnAdvise(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_UnAdvise)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIIdentityProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIIdentityProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for AsyncIIdentityProvider {
    type Vtable = AsyncIIdentityProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6fc9901_c433_4646_8f48_4e4687aae2a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_GetIdentityEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: ::windows::core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_Import: usize,
    pub Finish_Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_Delete: usize,
    pub Finish_Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_FindByUniqueID: usize,
    pub Begin_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_GetProviderPropertyStore: usize,
    pub Begin_Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: u32) -> ::windows::core::HRESULT,
    pub Finish_Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub Finish_UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityStore(::windows::core::IUnknown);
impl AsyncIIdentityStore {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_GetCount(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetCount)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_GetAt(&self, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprovider), ::core::mem::transmute(pprovguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_GetAt(&self, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprovguid), ::core::mem::transmute(ppidentityprovider)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_AddToCache<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_AddToCache)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::core::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_AddToCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_AddToCache)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_ConvertToSid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_ConvertToSid)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::core::mem::transmute(providerguid), ::core::mem::transmute(cbsid), ::core::mem::transmute(psid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_ConvertToSid(&self, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_ConvertToSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(psid), ::core::mem::transmute(pcbrequiredsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_EnumerateIdentities)(::core::mem::transmute_copy(self), ::core::mem::transmute(eidentitytype), ::core::mem::transmute(pfilterkey), ::core::mem::transmute(pfilterpropvarvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_EnumerateIdentities(&self) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Finish_EnumerateIdentities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Reset)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIIdentityStore> for ::windows::core::IUnknown {
    fn from(value: AsyncIIdentityStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIIdentityStore> for ::windows::core::IUnknown {
    fn from(value: &AsyncIIdentityStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIIdentityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIIdentityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIIdentityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for AsyncIIdentityStore {
    type Vtable = AsyncIIdentityStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefa1616_48de_4872_aa64_6e6206535a51);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::HRESULT,
    pub Finish_ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_EnumerateIdentities: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_EnumerateIdentities: usize,
    pub Begin_Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityStoreEx(::windows::core::IUnknown);
impl AsyncIIdentityStoreEx {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_CreateConnectedIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, localname: Param0, connectedname: Param1, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_CreateConnectedIdentity)(::core::mem::transmute_copy(self), localname.into_param().abi(), connectedname.into_param().abi(), ::core::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_CreateConnectedIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_CreateConnectedIdentity)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Begin_DeleteConnectedIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, connectedname: Param0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_DeleteConnectedIdentity)(::core::mem::transmute_copy(self), connectedname.into_param().abi(), ::core::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Finish_DeleteConnectedIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_DeleteConnectedIdentity)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIIdentityStoreEx> for ::windows::core::IUnknown {
    fn from(value: AsyncIIdentityStoreEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIIdentityStoreEx> for ::windows::core::IUnknown {
    fn from(value: &AsyncIIdentityStoreEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIIdentityStoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIIdentityStoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIIdentityStoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for AsyncIIdentityStoreEx {
    type Vtable = AsyncIIdentityStoreEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfca3af9a_8a07_4eae_8632_ec3de658a36a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStoreEx_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localname: ::windows::core::PCWSTR, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
pub const CIdentityProfileHandler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecf5bf46_e3b6_449a_b56b_43f58f867814);
pub const CoClassIdentityStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d49246_d217_465f_b00b_ac9ddd652eb7);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IAssociatedIdentityProvider(::windows::core::IUnknown);
impl IAssociatedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn AssociateIdentity<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AssociateIdentity)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisassociateIdentity<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisassociateIdentity)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangeCredential)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAssociatedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: IAssociatedIdentityProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAssociatedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: &IAssociatedIdentityProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IAssociatedIdentityProvider {
    type Vtable = IAssociatedIdentityProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2af066b3_4cbb_4cba_a798_204b6af68cc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssociatedIdentityProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisassociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IConnectedIdentityProvider(::windows::core::IUnknown);
impl IConnectedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn ConnectIdentity(&self, authbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectIdentity)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(authbuffer)), authbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn DisconnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectIdentity)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetUrl<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::super::System::Com::IBindCtx>>(&self, identifier: IDENTITY_URL, context: Param1, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUrl)(::core::mem::transmute_copy(self), ::core::mem::transmute(identifier), context.into_param().abi(), ::core::mem::transmute(postdata), ::core::mem::transmute(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn GetAccountState(&self) -> ::windows::core::Result<ACCOUNT_STATE> {
        let mut result__: ACCOUNT_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccountState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ACCOUNT_STATE>(result__)
    }
}
impl ::core::convert::From<IConnectedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: IConnectedIdentityProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectedIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: &IConnectedIdentityProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConnectedIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IConnectedIdentityProvider {
    type Vtable = IConnectedIdentityProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7417b54_e08c_429b_96c8_678d1369ecb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedIdentityProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT,
    pub DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: ::windows::core::RawPtr, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetUrl: usize,
    pub GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_ASSOCIATED: &'static str = "associated";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_CONNECTED: &'static str = "connected";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_HOMEGROUP: &'static str = "homegroup";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_LOCAL: &'static str = "local";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IDENTITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ALL: IDENTITY_TYPE = IDENTITY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
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
unsafe impl ::windows::core::Abi for IDENTITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IDENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDENTITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IDENTITY_URL(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = IDENTITY_URL(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = IDENTITY_URL(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = IDENTITY_URL(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = IDENTITY_URL(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = IDENTITY_URL(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = IDENTITY_URL(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = IDENTITY_URL(6i32);
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
unsafe impl ::windows::core::Abi for IDENTITY_URL {
    type Abi = Self;
}
impl ::core::fmt::Debug for IDENTITY_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDENTITY_URL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityAdvise(::windows::core::IUnknown);
impl IIdentityAdvise {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn IdentityUpdated<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IdentityUpdated)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwidentityupdateevents), lpszuniqueid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IIdentityAdvise> for ::windows::core::IUnknown {
    fn from(value: IIdentityAdvise) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIdentityAdvise> for ::windows::core::IUnknown {
    fn from(value: &IIdentityAdvise) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIdentityAdvise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIdentityAdvise {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIdentityAdvise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IIdentityAdvise {
    type Vtable = IIdentityAdvise_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e982fed_d14b_440c_b8d6_bb386453d386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAdvise_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityAuthentication(::windows::core::IUnknown);
impl IIdentityAuthentication {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn SetIdentityCredential(&self, credbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIdentityCredential)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(credbuffer)), credbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ValidateIdentityCredential)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(credbuffer)), credbuffer.len() as _, ::core::mem::transmute(ppidentityproperties)).ok()
    }
}
impl ::core::convert::From<IIdentityAuthentication> for ::windows::core::IUnknown {
    fn from(value: IIdentityAuthentication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIdentityAuthentication> for ::windows::core::IUnknown {
    fn from(value: &IIdentityAuthentication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIdentityAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIdentityAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIdentityAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IIdentityAuthentication {
    type Vtable = IIdentityAuthentication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e7ef254_979f_43b5_b74e_06e4eb7df0f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAuthentication_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ValidateIdentityCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityProvider(::windows::core::IUnknown);
impl IIdentityProvider {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIdentityEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(eidentitytype), ::core::mem::transmute(pfilterkey), ::core::mem::transmute(pfilterpropvarvalue), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszusername: Param0, pppropertystore: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Create)(::core::mem::transmute_copy(self), lpszusername.into_param().abi(), ::core::mem::transmute(pppropertystore), ::core::mem::transmute(pkeywordstoadd)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Import<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, ppropertystore: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Import)(::core::mem::transmute_copy(self), ppropertystore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::core::mem::transmute(pkeywordstodelete)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn FindByUniqueID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindByUniqueID)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProviderPropertyStore(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderPropertyStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, IIdentityAdvise>>(&self, pidentityadvise: Param0, dwidentityupdateevents: IdentityUpdateEvent) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Advise)(::core::mem::transmute_copy(self), pidentityadvise.into_param().abi(), ::core::mem::transmute(dwidentityupdateevents), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn UnAdvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnAdvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
impl ::core::convert::From<IIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: IIdentityProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIdentityProvider> for ::windows::core::IUnknown {
    fn from(value: &IIdentityProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIdentityProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IIdentityProvider {
    type Vtable = IIdentityProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d1b9e0c_e8ba_4f55_a81b_bce934b948f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: ::windows::core::PCWSTR, pppropertystore: *mut ::windows::core::RawPtr, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Import: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Delete: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    FindByUniqueID: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProviderPropertyStore: usize,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentityadvise: ::windows::core::RawPtr, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityStore(::windows::core::IUnknown);
impl IIdentityStore {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn GetAt(&self, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprovider), ::core::mem::transmute(pprovguid), ::core::mem::transmute(ppidentityprovider)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn AddToCache<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddToCache)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::core::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn ConvertToSid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::core::GUID, psid: &mut [u8], pcbrequiredsid: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConvertToSid)(::core::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::core::mem::transmute(providerguid), psid.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psid)), ::core::mem::transmute(pcbrequiredsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumerateIdentities)(::core::mem::transmute_copy(self), ::core::mem::transmute(eidentitytype), ::core::mem::transmute(pfilterkey), ::core::mem::transmute(pfilterpropvarvalue), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IIdentityStore> for ::windows::core::IUnknown {
    fn from(value: IIdentityStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIdentityStore> for ::windows::core::IUnknown {
    fn from(value: &IIdentityStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIdentityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIdentityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIdentityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IIdentityStore {
    type Vtable = IIdentityStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf586fa5_6f35_44f1_b209_b38e169772eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    EnumerateIdentities: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityStoreEx(::windows::core::IUnknown);
impl IIdentityStoreEx {
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn CreateConnectedIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, localname: Param0, connectedname: Param1, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateConnectedIdentity)(::core::mem::transmute_copy(self), localname.into_param().abi(), connectedname.into_param().abi(), ::core::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
    pub unsafe fn DeleteConnectedIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, connectedname: Param0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteConnectedIdentity)(::core::mem::transmute_copy(self), connectedname.into_param().abi(), ::core::mem::transmute(providerguid)).ok()
    }
}
impl ::core::convert::From<IIdentityStoreEx> for ::windows::core::IUnknown {
    fn from(value: IIdentityStoreEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIdentityStoreEx> for ::windows::core::IUnknown {
    fn from(value: &IIdentityStoreEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIdentityStoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIdentityStoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIdentityStoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IIdentityStoreEx {
    type Vtable = IIdentityStoreEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f9eb98_8f7f_4e38_9577_6980114ce32b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStoreEx_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localname: ::windows::core::PCWSTR, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IdentityUpdateEvent(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CREATED: IdentityUpdateEvent = IdentityUpdateEvent(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = IdentityUpdateEvent(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DELETED: IdentityUpdateEvent = IdentityUpdateEvent(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = IdentityUpdateEvent(32u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(128u32);
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
unsafe impl ::windows::core::Abi for IdentityUpdateEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for IdentityUpdateEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IdentityUpdateEvent").field(&self.0).finish()
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
pub const OID_OAssociatedIdentityProviderObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98c5a3dd_db68_4f1a_8d2b_9079cdfeaf61);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_COMPLETE_ACCOUNT: &'static str = "CompleteAccount";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_MODERN_SETTINGS_ADD_USER: &'static str = "ModernSettingsAddUser";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_NTH_USER_FIRST_AUTH: &'static str = "NthUserFirstAuth";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_EXPERIENCE: &'static str = "OutOfBoxExperience";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_UPGRADE_EXPERIENCE: &'static str = "OutOfBoxUpgradeExperience";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_PROPERTY_STORE: &'static str = "PropertyStore";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_USER_NAME: &'static str = "Username";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
