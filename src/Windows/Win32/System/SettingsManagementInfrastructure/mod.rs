#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IItemEnumerator(pub ::windows::runtime::IUnknown);
impl IItemEnumerator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Current(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IItemEnumerator {
    type Vtable = IItemEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<IItemEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IItemEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IItemEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IItemEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IItemEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IItemEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsContext(pub ::windows::runtime::IUnknown);
impl ISettingsContext {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_System_Com`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_System_Com`*"]
    pub unsafe fn Deserialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ptarget.into_param().abi(), ::core::mem::transmute(pppresults), ::core::mem::transmute(pcresultcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetUserData(&self, puserdata: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(puserdata)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetUserData(&self, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(puserdata)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetNamespaces(&self) -> ::windows::runtime::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetStoredSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ISettingsIdentity>>(&self, pidentity: Param0, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pidentity.into_param().abi(), ::core::mem::transmute(ppaddedsettings), ::core::mem::transmute(ppmodifiedsettings), ::core::mem::transmute(ppdeletedsettings)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RevertSetting<'a, Param0: ::windows::runtime::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pidentity: Param0, pwzsetting: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pidentity.into_param().abi(), pwzsetting.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISettingsContext {
    type Vtable = ISettingsContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsContext> for ::windows::runtime::IUnknown {
    fn from(value: ISettingsContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsContext> for ::windows::runtime::IUnknown {
    fn from(value: &ISettingsContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISettingsContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISettingsContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr, pppresults: *mut *mut ::windows::runtime::RawPtr, pcresultcount: *mut usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puserdata: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnamespaceids: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidentity: ::windows::runtime::RawPtr, ppaddedsettings: *mut ::windows::runtime::RawPtr, ppmodifiedsettings: *mut ::windows::runtime::RawPtr, ppdeletedsettings: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidentity: ::windows::runtime::RawPtr, pwzsetting: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsEngine(pub ::windows::runtime::IUnknown);
impl ISettingsEngine {
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ISettingsIdentity>>(&self, settingsid: Param0, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<ISettingsNamespace> {
        let mut result__: <ISettingsNamespace as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), ::core::mem::transmute(access), ::core::mem::transmute(reserved), &mut result__).from_abi::<ISettingsNamespace>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn CreateSettingsIdentity(&self) -> ::windows::runtime::Result<ISettingsIdentity> {
        let mut result__: <ISettingsIdentity as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISettingsIdentity>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetStoreStatus(&self, reserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<WcmUserStatus> {
        let mut result__: <WcmUserStatus as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), &mut result__).from_abi::<WcmUserStatus>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn LoadStore(&self, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn UnloadStore(&self, reserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RegisterNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, stream: Param1, pushsettings: Param2) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), stream.into_param().abi(), pushsettings.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn UnregisterNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, removesettings: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), removesettings.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn CreateTargetInfo(&self) -> ::windows::runtime::Result<ITargetInfo> {
        let mut result__: <ITargetInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITargetInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetTargetInfo(&self) -> ::windows::runtime::Result<ITargetInfo> {
        let mut result__: <ITargetInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITargetInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetTargetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ITargetInfo>>(&self, target: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), target.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<ISettingsContext> {
        let mut result__: <ISettingsContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), &mut result__).from_abi::<ISettingsContext>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetSettingsContext<'a, Param0: ::windows::runtime::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), settingscontext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn ApplySettingsContext<'a, Param0: ::windows::runtime::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0, pppwzidentities: *mut *mut super::super::Foundation::PWSTR, pcidentities: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), settingscontext.into_param().abi(), ::core::mem::transmute(pppwzidentities), ::core::mem::transmute(pcidentities)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetSettingsContext(&self) -> ::windows::runtime::Result<ISettingsContext> {
        let mut result__: <ISettingsContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISettingsContext>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISettingsEngine {
    type Vtable = ISettingsEngine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsEngine> for ::windows::runtime::IUnknown {
    fn from(value: ISettingsEngine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsEngine> for ::windows::runtime::IUnknown {
    fn from(value: &ISettingsEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISettingsEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISettingsEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingsid: ::windows::runtime::RawPtr, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hresult: i32, message: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingsid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingsid: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, pushsettings: super::super::Foundation::BOOL, results: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingsid: ::windows::runtime::RawPtr, removesettings: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingscontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingscontext: ::windows::runtime::RawPtr, pppwzidentities: *mut *mut super::super::Foundation::PWSTR, pcidentities: *mut usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingscontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsIdentity(pub ::windows::runtime::IUnknown);
impl ISettingsIdentity {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetFlags(&self, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISettingsIdentity {
    type Vtable = ISettingsIdentity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsIdentity> for ::windows::runtime::IUnknown {
    fn from(value: ISettingsIdentity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsIdentity> for ::windows::runtime::IUnknown {
    fn from(value: &ISettingsIdentity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISettingsIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISettingsIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsIdentity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsItem(pub ::windows::runtime::IUnknown);
impl ISettingsItem {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetValue(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetValue(&self, value: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetSettingType(&self) -> ::windows::runtime::Result<WcmSettingType> {
        let mut result__: <WcmSettingType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmSettingType>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetDataType(&self) -> ::windows::runtime::Result<WcmDataType> {
        let mut result__: <WcmDataType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmDataType>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8, datasize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(data), ::core::mem::transmute(datasize)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: *const u8, datasize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(datatype), ::core::mem::transmute(data), ::core::mem::transmute(datasize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn HasChild(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Children(&self) -> ::windows::runtime::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetChild<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::runtime::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::runtime::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(keyname), ::core::mem::transmute(datatype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CreateListElement(&self, keydata: *const super::Com::VARIANT) -> ::windows::runtime::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(keydata), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RemoveListElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, elementname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), elementname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Attributes(&self) -> ::windows::runtime::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetRestrictionFacets(&self) -> ::windows::runtime::Result<WcmRestrictionFacets> {
        let mut result__: <WcmRestrictionFacets as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmRestrictionFacets>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(restrictionfacet), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetKeyValue(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISettingsItem {
    type Vtable = ISettingsItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsItem> for ::windows::runtime::IUnknown {
    fn from(value: ISettingsItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsItem> for ::windows::runtime::IUnknown {
    fn from(value: &ISettingsItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISettingsItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISettingsItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut WcmSettingType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut WcmDataType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: *mut *mut u8, datasize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: i32, data: *const u8, datasize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, children: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, child: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keyname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, datatype: *mut WcmDataType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keydata: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, child: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, elementname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, restrictionfacet: WcmRestrictionFacets, facetdata: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsNamespace(pub ::windows::runtime::IUnknown);
impl ISettingsNamespace {
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetIdentity(&self) -> ::windows::runtime::Result<ISettingsIdentity> {
        let mut result__: <ISettingsIdentity as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISettingsIdentity>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Settings(&self) -> ::windows::runtime::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pushsettings: Param0) -> ::windows::runtime::Result<ISettingsResult> {
        let mut result__: <ISettingsResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pushsettings.into_param().abi(), &mut result__).from_abi::<ISettingsResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::runtime::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::runtime::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISettingsNamespace {
    type Vtable = ISettingsNamespace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsNamespace> for ::windows::runtime::IUnknown {
    fn from(value: ISettingsNamespace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsNamespace> for ::windows::runtime::IUnknown {
    fn from(value: &ISettingsNamespace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISettingsNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISettingsNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsNamespace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingsid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settings: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pushsettings: super::super::Foundation::BOOL, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsResult(pub ::windows::runtime::IUnknown);
impl ISettingsResult {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetContextDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetLine(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetColumn(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSource(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISettingsResult {
    type Vtable = ISettingsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsResult> for ::windows::runtime::IUnknown {
    fn from(value: ISettingsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsResult> for ::windows::runtime::IUnknown {
    fn from(value: &ISettingsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISettingsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISettingsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrout: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwline: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcolumn: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITargetInfo(pub ::windows::runtime::IUnknown);
impl ITargetInfo {
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetTargetMode(&self) -> ::windows::runtime::Result<WcmTargetMode> {
        let mut result__: <WcmTargetMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmTargetMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(targetmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetTemporaryStoreLocation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetTemporaryStoreLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, temporarystorelocation: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), temporarystorelocation.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetTargetID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetTargetID<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, targetid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), targetid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetTargetProcessorArchitecture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, processorarchitecture: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), processorarchitecture.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, property: Param1) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), offline.into_param().abi(), property.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, property: Param1, value: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), offline.into_param().abi(), property.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn ExpandTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, location: Param1) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), offline.into_param().abi(), location.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn ExpandTargetPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, location: Param1) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), offline.into_param().abi(), location.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetModulePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, module: Param0, path: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), module.into_param().abi(), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn LoadModule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, module: Param0) -> ::windows::runtime::Result<super::super::Foundation::HINSTANCE> {
        let mut result__: <super::super::Foundation::HINSTANCE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), module.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HINSTANCE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetWow64Context<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, installermodule: Param0, wow64context: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), installermodule.into_param().abi(), ::core::mem::transmute(wow64context)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn TranslateWow64<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, clientarchitecture: Param0, value: Param1) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), clientarchitecture.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetSchemaHiveLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwzhivedir: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pwzhivedir.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemaHiveLocation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetSchemaHiveMountName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwzmountname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwzmountname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemaHiveMountName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITargetInfo {
    type Vtable = ITargetInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ITargetInfo> for ::windows::runtime::IUnknown {
    fn from(value: ITargetInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITargetInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ITargetInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITargetInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITargetInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetmode: *mut WcmTargetMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetmode: WcmTargetMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, temporarystorelocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, temporarystorelocation: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, processorarchitecture: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, processorarchitecture: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, module: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, module: super::super::Foundation::PWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, installermodule: super::super::Foundation::PWSTR, wow64context: *const u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clientarchitecture: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, translatedvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwzhivedir: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phivelocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwzmountname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmountname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const SettingsEngine: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ABORTOPERATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255384i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ASSERTIONFAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255398i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255420i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255421i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_CONFLICTINGASSERTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255399i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_CYCLICREFERENCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255389i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_DUPLICATENAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255397i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255408i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_HANDLERNOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255394i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INTERNALERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255424i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255385i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDDATATYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255416i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255401i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255393i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDKEY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255396i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255410i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDPATH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255413i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255382i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDSTREAM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255395i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVALUE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255419i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVALUEFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255418i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255411i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_KEYNOTCHANGEABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255409i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255390i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MISSINGCONFIGURATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255383i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MIXTYPEASSERTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255388i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255403i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NAMESPACENOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255404i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255400i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTPOSITIONED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255415i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255387i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_READONLYITEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255414i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_RESTRICTIONFAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255391i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255381i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STATENODENOTALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255422i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STATENODENOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255423i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STORECORRUPTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255402i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255407i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_TYPENOTSPECIFIED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255417i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_UNKNOWNRESULT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145251325i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_USERALREADYREGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255406i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_USERNOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255405i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_VALIDATIONFAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255392i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_VALUETOOBIG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255386i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_WRONGESCAPESTRING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145255412i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2232325i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2232321i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_INTERNALERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2232320i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2232324i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_LEGACYSETTINGWARNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2232322i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_NAMESPACENOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2232326i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WcmDataType(pub i32);
pub const dataTypeByte: WcmDataType = WcmDataType(1i32);
pub const dataTypeSByte: WcmDataType = WcmDataType(2i32);
pub const dataTypeUInt16: WcmDataType = WcmDataType(3i32);
pub const dataTypeInt16: WcmDataType = WcmDataType(4i32);
pub const dataTypeUInt32: WcmDataType = WcmDataType(5i32);
pub const dataTypeInt32: WcmDataType = WcmDataType(6i32);
pub const dataTypeUInt64: WcmDataType = WcmDataType(7i32);
pub const dataTypeInt64: WcmDataType = WcmDataType(8i32);
pub const dataTypeBoolean: WcmDataType = WcmDataType(11i32);
pub const dataTypeString: WcmDataType = WcmDataType(12i32);
pub const dataTypeFlagArray: WcmDataType = WcmDataType(32768i32);
impl ::core::convert::From<i32> for WcmDataType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WcmDataType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WcmNamespaceAccess(pub i32);
pub const ReadOnlyAccess: WcmNamespaceAccess = WcmNamespaceAccess(1i32);
pub const ReadWriteAccess: WcmNamespaceAccess = WcmNamespaceAccess(2i32);
impl ::core::convert::From<i32> for WcmNamespaceAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WcmNamespaceAccess {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(1i32);
pub const UserEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(2i32);
pub const AllEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(3i32);
impl ::core::convert::From<i32> for WcmNamespaceEnumerationFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WcmNamespaceEnumerationFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WcmRestrictionFacets(pub i32);
pub const restrictionFacetMaxLength: WcmRestrictionFacets = WcmRestrictionFacets(1i32);
pub const restrictionFacetEnumeration: WcmRestrictionFacets = WcmRestrictionFacets(2i32);
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = WcmRestrictionFacets(4i32);
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = WcmRestrictionFacets(8i32);
impl ::core::convert::From<i32> for WcmRestrictionFacets {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WcmRestrictionFacets {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WcmSettingType(pub i32);
pub const settingTypeScalar: WcmSettingType = WcmSettingType(1i32);
pub const settingTypeComplex: WcmSettingType = WcmSettingType(2i32);
pub const settingTypeList: WcmSettingType = WcmSettingType(3i32);
impl ::core::convert::From<i32> for WcmSettingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WcmSettingType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WcmTargetMode(pub i32);
pub const OfflineMode: WcmTargetMode = WcmTargetMode(1i32);
pub const OnlineMode: WcmTargetMode = WcmTargetMode(2i32);
impl ::core::convert::From<i32> for WcmTargetMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WcmTargetMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WcmUserStatus(pub i32);
pub const UnknownStatus: WcmUserStatus = WcmUserStatus(0i32);
pub const UserRegistered: WcmUserStatus = WcmUserStatus(1i32);
pub const UserUnregistered: WcmUserStatus = WcmUserStatus(2i32);
pub const UserLoaded: WcmUserStatus = WcmUserStatus(3i32);
pub const UserUnloaded: WcmUserStatus = WcmUserStatus(4i32);
impl ::core::convert::From<i32> for WcmUserStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WcmUserStatus {
    type Abi = Self;
}
