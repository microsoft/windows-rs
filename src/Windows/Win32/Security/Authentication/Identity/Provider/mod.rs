#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACCOUNT_STATE(pub i32);
pub const NOT_CONNECTED: ACCOUNT_STATE = ACCOUNT_STATE(0i32);
pub const CONNECTING: ACCOUNT_STATE = ACCOUNT_STATE(1i32);
pub const CONNECT_COMPLETED: ACCOUNT_STATE = ACCOUNT_STATE(2i32);
impl ::std::convert::From<i32> for ACCOUNT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACCOUNT_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIAssociatedIdentityProvider(::windows::runtime::IUnknown);
impl AsyncIAssociatedIdentityProvider {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_AssociateIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndparent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Finish_AssociateIdentity(&self) -> ::windows::runtime::Result<super::super::super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_DisassociateIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_DisassociateIdentity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_ChangeCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_ChangeCredential(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIAssociatedIdentityProvider {
    type Vtable = AsyncIAssociatedIdentityProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(674551533, 10622, 20082, [138, 81, 150, 30, 134, 240, 81, 82]);
}
impl ::std::convert::From<AsyncIAssociatedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIAssociatedIdentityProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIAssociatedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIAssociatedIdentityProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAssociatedIdentityProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIConnectedIdentityProvider(::windows::runtime::IUnknown);
impl AsyncIConnectedIdentityProvider {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(authbuffer), ::std::mem::transmute(authbuffersize)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_ConnectIdentity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_DisconnectIdentity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_DisconnectIdentity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Finish_IsConnected(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_Com`*"]
    pub unsafe fn Begin_GetUrl<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::System::Com::IBindCtx>>(&self, identifier: IDENTITY_URL, context: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(identifier), context.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Finish_GetUrl(&self, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(postdata), ::std::mem::transmute(url)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_GetAccountState(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_GetAccountState(&self) -> ::windows::runtime::Result<ACCOUNT_STATE> {
        let mut result__: <ACCOUNT_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ACCOUNT_STATE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIConnectedIdentityProvider {
    type Vtable = AsyncIConnectedIdentityProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2632274241, 48361, 19989, [130, 77, 67, 215, 159, 81, 47, 147]);
}
impl ::std::convert::From<AsyncIConnectedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIConnectedIdentityProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIConnectedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIConnectedIdentityProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIConnectedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIConnectedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIConnectedIdentityProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, authbuffer: *const u8, authbuffersize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: IDENTITY_URL, context: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, postdata: *mut ::std::mem::ManuallyDrop<super::super::super::super::System::Com::VARIANT>, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACCOUNT_STATE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIIdentityAdvise(::windows::runtime::IUnknown);
impl AsyncIIdentityAdvise {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_IdentityUpdated<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, dwidentityupdateevents: u32, lpszuniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwidentityupdateevents), lpszuniqueid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_IdentityUpdated(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIIdentityAdvise {
    type Vtable = AsyncIIdentityAdvise_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(984926426, 53304, 18480, [141, 217, 50, 83, 197, 90, 18, 127]);
}
impl ::std::convert::From<AsyncIIdentityAdvise> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIIdentityAdvise) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIIdentityAdvise> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIIdentityAdvise) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIIdentityAdvise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIIdentityAdvise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAdvise_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwidentityupdateevents: u32, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIIdentityAuthentication(::windows::runtime::IUnknown);
impl AsyncIIdentityAuthentication {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(credbuffer), ::std::mem::transmute(credbufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_SetIdentityCredential(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Begin_ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::std::option::Option<super::super::super::super::System::PropertiesSystem::IPropertyStore>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(credbuffer), ::std::mem::transmute(credbufferlength), ::std::mem::transmute(ppidentityproperties)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: *mut ::std::option::Option<super::super::super::super::System::PropertiesSystem::IPropertyStore>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppidentityproperties)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIIdentityAuthentication {
    type Vtable = AsyncIIdentityAuthentication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4188207384, 65226, 20124, [150, 51, 97, 203, 241, 62, 211, 77]);
}
impl ::std::convert::From<AsyncIIdentityAuthentication> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIIdentityAuthentication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIIdentityAuthentication> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIIdentityAuthentication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIIdentityAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIIdentityAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAuthentication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credbuffer: *const u8, credbufferlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppidentityproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIIdentityProvider(::windows::runtime::IUnknown);
impl AsyncIIdentityProvider {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(eidentitytype), ::std::mem::transmute(pfilterkey), ::std::mem::transmute(pfilterpropvarvalue)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_Com`*"]
    pub unsafe fn Finish_GetIdentityEnum(&self) -> ::windows::runtime::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::super::super::System::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Begin_Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszusername: Param0, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), lpszusername.into_param().abi(), ::std::mem::transmute(pkeywordstoadd)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Finish_Create(&self) -> ::windows::runtime::Result<super::super::super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Begin_Import<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::PropertiesSystem::IPropertyStore>>(&self, ppropertystore: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ppropertystore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_Import(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Begin_Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::std::mem::transmute(pkeywordstodelete)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_FindByUniqueID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Finish_FindByUniqueID(&self) -> ::windows::runtime::Result<super::super::super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_GetProviderPropertyStore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Finish_GetProviderPropertyStore(&self) -> ::windows::runtime::Result<super::super::super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_Advise<'a, Param0: ::windows::runtime::IntoParam<'a, IIdentityAdvise>>(&self, pidentityadvise: Param0, dwidentityupdateevents: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pidentityadvise.into_param().abi(), ::std::mem::transmute(dwidentityupdateevents)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_Advise(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_UnAdvise(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_UnAdvise(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIIdentityProvider {
    type Vtable = AsyncIIdentityProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3338443009, 50227, 17990, [143, 72, 78, 70, 135, 170, 226, 160]);
}
impl ::std::convert::From<AsyncIIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIIdentityProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIIdentityProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppidentityenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszusername: super::super::super::super::Foundation::PWSTR, pkeywordstoadd: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropertystore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidentityadvise: ::windows::runtime::RawPtr, dwidentityupdateevents: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIIdentityStore(::windows::runtime::IUnknown);
impl AsyncIIdentityStore {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_GetCount(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_GetAt(&self, dwprovider: u32, pprovguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprovider), ::std::mem::transmute(pprovguid)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_GetAt(&self, pprovguid: *mut ::windows::runtime::GUID, ppidentityprovider: *mut ::std::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprovguid), ::std::mem::transmute(ppidentityprovider)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_AddToCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::std::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_AddToCache(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_ConvertToSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::runtime::GUID, cbsid: u16, psid: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::std::mem::transmute(providerguid), ::std::mem::transmute(cbsid), ::std::mem::transmute(psid)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_ConvertToSid(&self, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(psid), ::std::mem::transmute(pcbrequiredsid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(eidentitytype), ::std::mem::transmute(pfilterkey), ::std::mem::transmute(pfilterpropvarvalue)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_Com`*"]
    pub unsafe fn Finish_EnumerateIdentities(&self) -> ::windows::runtime::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::super::super::System::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Begin_Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIIdentityStore {
    type Vtable = AsyncIIdentityStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009367062, 18654, 18546, [170, 100, 110, 98, 6, 83, 90, 81]);
}
impl ::std::convert::From<AsyncIIdentityStore> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIIdentityStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIIdentityStore> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIIdentityStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIIdentityStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIIdentityStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwproviders: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprovider: u32, pprovguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprovguid: *mut ::windows::runtime::GUID, ppidentityprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID, cbsid: u16, psid: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppidentityenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIIdentityStoreEx(::windows::runtime::IUnknown);
impl AsyncIIdentityStoreEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_CreateConnectedIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, localname: Param0, connectedname: Param1, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), localname.into_param().abi(), connectedname.into_param().abi(), ::std::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_CreateConnectedIdentity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn Begin_DeleteConnectedIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, connectedname: Param0, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), connectedname.into_param().abi(), ::std::mem::transmute(providerguid)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Finish_DeleteConnectedIdentity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIIdentityStoreEx {
    type Vtable = AsyncIIdentityStoreEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4238585754, 35335, 20142, [134, 50, 236, 61, 230, 88, 163, 106]);
}
impl ::std::convert::From<AsyncIIdentityStoreEx> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIIdentityStoreEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIIdentityStoreEx> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIIdentityStoreEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIIdentityStoreEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIIdentityStoreEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStoreEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
pub const CIdentityProfileHandler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3975528262, 58294, 17562, [181, 107, 67, 245, 143, 134, 120, 20]);
pub const CoClassIdentityStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(819237446, 53783, 18015, [176, 11, 172, 157, 221, 101, 46, 183]);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAssociatedIdentityProvider(::windows::runtime::IUnknown);
impl IAssociatedIdentityProvider {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn AssociateIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::runtime::Result<super::super::super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), &mut result__).from_abi::<super::super::super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn DisassociateIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn ChangeCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, hwndparent: Param0, lpszuniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), lpszuniqueid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAssociatedIdentityProvider {
    type Vtable = IAssociatedIdentityProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(720398003, 19643, 19642, [167, 152, 32, 75, 106, 246, 140, 192]);
}
impl ::std::convert::From<IAssociatedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: IAssociatedIdentityProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAssociatedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IAssociatedIdentityProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAssociatedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssociatedIdentityProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IConnectedIdentityProvider(::windows::runtime::IUnknown);
impl IConnectedIdentityProvider {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(authbuffer), ::std::mem::transmute(authbuffersize)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn DisconnectIdentity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetUrl<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::System::Com::IBindCtx>>(&self, identifier: IDENTITY_URL, context: Param1, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(identifier), context.into_param().abi(), ::std::mem::transmute(postdata), ::std::mem::transmute(url)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn GetAccountState(&self) -> ::windows::runtime::Result<ACCOUNT_STATE> {
        let mut result__: <ACCOUNT_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ACCOUNT_STATE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConnectedIdentityProvider {
    type Vtable = IConnectedIdentityProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3074521940, 57484, 17051, [150, 200, 103, 141, 19, 105, 236, 177]);
}
impl ::std::convert::From<IConnectedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: IConnectedIdentityProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConnectedIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IConnectedIdentityProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConnectedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConnectedIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedIdentityProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, authbuffer: *const u8, authbuffersize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: IDENTITY_URL, context: ::windows::runtime::RawPtr, postdata: *mut ::std::mem::ManuallyDrop<super::super::super::super::System::Com::VARIANT>, url: *mut super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACCOUNT_STATE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IDENTITY_TYPE(pub i32);
pub const IDENTITIES_ALL: IDENTITY_TYPE = IDENTITY_TYPE(0i32);
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
impl ::std::convert::From<i32> for IDENTITY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IDENTITY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IDENTITY_URL(pub i32);
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = IDENTITY_URL(0i32);
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = IDENTITY_URL(1i32);
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = IDENTITY_URL(2i32);
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = IDENTITY_URL(3i32);
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = IDENTITY_URL(4i32);
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = IDENTITY_URL(5i32);
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = IDENTITY_URL(6i32);
impl ::std::convert::From<i32> for IDENTITY_URL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IDENTITY_URL {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IIdentityAdvise(::windows::runtime::IUnknown);
impl IIdentityAdvise {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn IdentityUpdated<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwidentityupdateevents), lpszuniqueid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IIdentityAdvise {
    type Vtable = IIdentityAdvise_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1318596589, 53579, 17420, [184, 214, 187, 56, 100, 83, 211, 134]);
}
impl ::std::convert::From<IIdentityAdvise> for ::windows::runtime::IUnknown {
    fn from(value: IIdentityAdvise) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIdentityAdvise> for ::windows::runtime::IUnknown {
    fn from(value: &IIdentityAdvise) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIdentityAdvise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IIdentityAdvise {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAdvise_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IIdentityAuthentication(::windows::runtime::IUnknown);
impl IIdentityAuthentication {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(credbuffer), ::std::mem::transmute(credbufferlength)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::std::option::Option<super::super::super::super::System::PropertiesSystem::IPropertyStore>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(credbuffer), ::std::mem::transmute(credbufferlength), ::std::mem::transmute(ppidentityproperties)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IIdentityAuthentication {
    type Vtable = IIdentityAuthentication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1585377876, 38815, 17333, [183, 78, 6, 228, 235, 125, 240, 249]);
}
impl ::std::convert::From<IIdentityAuthentication> for ::windows::runtime::IUnknown {
    fn from(value: IIdentityAuthentication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIdentityAuthentication> for ::windows::runtime::IUnknown {
    fn from(value: &IIdentityAuthentication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIdentityAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IIdentityAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAuthentication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credbuffer: *const u8, credbufferlength: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IIdentityProvider(::windows::runtime::IUnknown);
impl IIdentityProvider {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::super::super::System::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(eidentitytype), ::std::mem::transmute(pfilterkey), ::std::mem::transmute(pfilterpropvarvalue), &mut result__).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszusername: Param0, pppropertystore: *mut ::std::option::Option<super::super::super::super::System::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), lpszusername.into_param().abi(), ::std::mem::transmute(pppropertystore), ::std::mem::transmute(pkeywordstoadd)).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Import<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::System::PropertiesSystem::IPropertyStore>>(&self, ppropertystore: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ppropertystore.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::std::mem::transmute(pkeywordstodelete)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn FindByUniqueID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0) -> ::windows::runtime::Result<super::super::super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetProviderPropertyStore(&self) -> ::windows::runtime::Result<super::super::super::super::System::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::super::super::System::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::System::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, IIdentityAdvise>>(&self, pidentityadvise: Param0, dwidentityupdateevents: IdentityUpdateEvent) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pidentityadvise.into_param().abi(), ::std::mem::transmute(dwidentityupdateevents), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn UnAdvise(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IIdentityProvider {
    type Vtable = IIdentityProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(219913740, 59578, 20309, [168, 27, 188, 233, 52, 185, 72, 245]);
}
impl ::std::convert::From<IIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: IIdentityProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIdentityProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IIdentityProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IIdentityProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppidentityenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszusername: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::runtime::RawPtr, pkeywordstoadd: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropertystore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pkeywordstodelete: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR, pppropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertystore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidentityadvise: ::windows::runtime::RawPtr, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IIdentityStore(::windows::runtime::IUnknown);
impl IIdentityStore {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn GetAt(&self, dwprovider: u32, pprovguid: *mut ::windows::runtime::GUID, ppidentityprovider: *mut ::std::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprovider), ::std::mem::transmute(pprovguid), ::std::mem::transmute(ppidentityprovider)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn AddToCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::std::mem::transmute(providerguid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn ConvertToSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, lpszuniqueid: Param0, providerguid: *const ::windows::runtime::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), lpszuniqueid.into_param().abi(), ::std::mem::transmute(providerguid), ::std::mem::transmute(cbsid), ::std::mem::transmute(psid), ::std::mem::transmute(pcbrequiredsid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::super::super::System::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(eidentitytype), ::std::mem::transmute(pfilterkey), ::std::mem::transmute(pfilterpropvarvalue), &mut result__).from_abi::<super::super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IIdentityStore {
    type Vtable = IIdentityStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3747114917, 28469, 17649, [178, 9, 179, 142, 22, 151, 114, 235]);
}
impl ::std::convert::From<IIdentityStore> for ::windows::runtime::IUnknown {
    fn from(value: IIdentityStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIdentityStore> for ::windows::runtime::IUnknown {
    fn from(value: &IIdentityStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIdentityStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IIdentityStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwproviders: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprovider: u32, pprovguid: *mut ::windows::runtime::GUID, ppidentityprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszuniqueid: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::System::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const ::std::mem::ManuallyDrop<super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppidentityenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IIdentityStoreEx(::windows::runtime::IUnknown);
impl IIdentityStoreEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn CreateConnectedIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, localname: Param0, connectedname: Param1, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), localname.into_param().abi(), connectedname.into_param().abi(), ::std::mem::transmute(providerguid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`, `Win32_Foundation`*"]
    pub unsafe fn DeleteConnectedIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, connectedname: Param0, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), connectedname.into_param().abi(), ::std::mem::transmute(providerguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IIdentityStoreEx {
    type Vtable = IIdentityStoreEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4193905560, 36735, 20024, [149, 119, 105, 128, 17, 76, 227, 43]);
}
impl ::std::convert::From<IIdentityStoreEx> for ::windows::runtime::IUnknown {
    fn from(value: IIdentityStoreEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIdentityStoreEx> for ::windows::runtime::IUnknown {
    fn from(value: &IIdentityStoreEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIdentityStoreEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IIdentityStoreEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStoreEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localname: super::super::super::super::Foundation::PWSTR, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectedname: super::super::super::super::Foundation::PWSTR, providerguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authentication_Identity_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IdentityUpdateEvent(pub u32);
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(1u32);
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(2u32);
pub const IDENTITY_CREATED: IdentityUpdateEvent = IdentityUpdateEvent(4u32);
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = IdentityUpdateEvent(8u32);
pub const IDENTITY_DELETED: IdentityUpdateEvent = IdentityUpdateEvent(16u32);
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = IdentityUpdateEvent(32u32);
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(64u32);
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(128u32);
impl ::std::convert::From<u32> for IdentityUpdateEvent {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IdentityUpdateEvent {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IdentityUpdateEvent {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IdentityUpdateEvent {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IdentityUpdateEvent {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IdentityUpdateEvent {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IdentityUpdateEvent {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const OID_OAssociatedIdentityProviderObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2563089373, 56168, 20250, [141, 43, 144, 121, 205, 254, 175, 97]);
