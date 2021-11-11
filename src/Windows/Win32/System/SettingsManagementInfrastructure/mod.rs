#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IItemEnumerator(pub ::windows::core::IUnknown);
impl IItemEnumerator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Current(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IItemEnumerator {
    type Vtable = IItemEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<IItemEnumerator> for ::windows::core::IUnknown {
    fn from(value: IItemEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IItemEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IItemEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IItemEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IItemEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsContext(pub ::windows::core::IUnknown);
impl ISettingsContext {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_System_Com`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IStream>, Param1: ::windows::core::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_System_Com`*"]
    pub unsafe fn Deserialize<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IStream>, Param1: ::windows::core::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ptarget.into_param().abi(), ::core::mem::transmute(pppresults), ::core::mem::transmute(pcresultcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetUserData(&self, puserdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(puserdata)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetUserData(&self, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(puserdata)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetNamespaces(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetStoredSettings<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>>(&self, pidentity: Param0, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pidentity.into_param().abi(), ::core::mem::transmute(ppaddedsettings), ::core::mem::transmute(ppmodifiedsettings), ::core::mem::transmute(ppdeletedsettings)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RevertSetting<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pidentity: Param0, pwzsetting: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pidentity.into_param().abi(), pwzsetting.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ISettingsContext {
    type Vtable = ISettingsContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsContext> for ::windows::core::IUnknown {
    fn from(value: ISettingsContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsContext> for ::windows::core::IUnknown {
    fn from(value: &ISettingsContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr, pppresults: *mut *mut ::windows::core::RawPtr, pcresultcount: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puserdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppnamespaceids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pidentity: ::windows::core::RawPtr, ppaddedsettings: *mut ::windows::core::RawPtr, ppmodifiedsettings: *mut ::windows::core::RawPtr, ppdeletedsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pidentity: ::windows::core::RawPtr, pwzsetting: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsEngine(pub ::windows::core::IUnknown);
impl ISettingsEngine {
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetNamespace<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>>(&self, settingsid: Param0, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsNamespace> {
        let mut result__: <ISettingsNamespace as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), ::core::mem::transmute(access), ::core::mem::transmute(reserved), &mut result__).from_abi::<ISettingsNamespace>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn CreateSettingsIdentity(&self) -> ::windows::core::Result<ISettingsIdentity> {
        let mut result__: <ISettingsIdentity as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISettingsIdentity>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetStoreStatus(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<WcmUserStatus> {
        let mut result__: <WcmUserStatus as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), &mut result__).from_abi::<WcmUserStatus>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn LoadStore(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn UnloadStore(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RegisterNamespace<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::core::IntoParam<'a, super::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, stream: Param1, pushsettings: Param2) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), stream.into_param().abi(), pushsettings.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn UnregisterNamespace<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, removesettings: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), removesettings.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn CreateTargetInfo(&self) -> ::windows::core::Result<ITargetInfo> {
        let mut result__: <ITargetInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITargetInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetTargetInfo(&self) -> ::windows::core::Result<ITargetInfo> {
        let mut result__: <ITargetInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITargetInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetTargetInfo<'a, Param0: ::windows::core::IntoParam<'a, ITargetInfo>>(&self, target: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), target.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsContext> {
        let mut result__: <ISettingsContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), &mut result__).from_abi::<ISettingsContext>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetSettingsContext<'a, Param0: ::windows::core::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), settingscontext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn ApplySettingsContext<'a, Param0: ::windows::core::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0, pppwzidentities: *mut *mut super::super::Foundation::PWSTR, pcidentities: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), settingscontext.into_param().abi(), ::core::mem::transmute(pppwzidentities), ::core::mem::transmute(pcidentities)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetSettingsContext(&self) -> ::windows::core::Result<ISettingsContext> {
        let mut result__: <ISettingsContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISettingsContext>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISettingsEngine {
    type Vtable = ISettingsEngine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsEngine> for ::windows::core::IUnknown {
    fn from(value: ISettingsEngine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsEngine> for ::windows::core::IUnknown {
    fn from(value: &ISettingsEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingsid: ::windows::core::RawPtr, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: i32, message: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingsid: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, pushsettings: super::super::Foundation::BOOL, results: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingsid: ::windows::core::RawPtr, removesettings: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingscontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingscontext: ::windows::core::RawPtr, pppwzidentities: *mut *mut super::super::Foundation::PWSTR, pcidentities: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsIdentity(pub ::windows::core::IUnknown);
impl ISettingsIdentity {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1, value: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISettingsIdentity {
    type Vtable = ISettingsIdentity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsIdentity> for ::windows::core::IUnknown {
    fn from(value: ISettingsIdentity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsIdentity> for ::windows::core::IUnknown {
    fn from(value: &ISettingsIdentity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsIdentity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsItem(pub ::windows::core::IUnknown);
impl ISettingsItem {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetValue(&self, value: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetSettingType(&self) -> ::windows::core::Result<WcmSettingType> {
        let mut result__: <WcmSettingType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmSettingType>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetDataType(&self) -> ::windows::core::Result<WcmDataType> {
        let mut result__: <WcmDataType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmDataType>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(data), ::core::mem::transmute(datasize)).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(datatype), ::core::mem::transmute(data), ::core::mem::transmute(datasize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn HasChild(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Children(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetChild<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(keyname), ::core::mem::transmute(datatype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CreateListElement(&self, keydata: *const super::Com::VARIANT) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(keydata), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RemoveListElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, elementname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), elementname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetRestrictionFacets(&self) -> ::windows::core::Result<WcmRestrictionFacets> {
        let mut result__: <WcmRestrictionFacets as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmRestrictionFacets>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(restrictionfacet), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetKeyValue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISettingsItem {
    type Vtable = ISettingsItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsItem> for ::windows::core::IUnknown {
    fn from(value: ISettingsItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsItem> for ::windows::core::IUnknown {
    fn from(value: &ISettingsItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut WcmSettingType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut WcmDataType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::Foundation::PWSTR, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, keyname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, datatype: *mut WcmDataType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, keydata: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, elementname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, restrictionfacet: WcmRestrictionFacets, facetdata: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsNamespace(pub ::windows::core::IUnknown);
impl ISettingsNamespace {
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetIdentity(&self) -> ::windows::core::Result<ISettingsIdentity> {
        let mut result__: <ISettingsIdentity as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISettingsIdentity>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn Settings(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pushsettings: Param0) -> ::windows::core::Result<ISettingsResult> {
        let mut result__: <ISettingsResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pushsettings.into_param().abi(), &mut result__).from_abi::<ISettingsResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: <ISettingsItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<ISettingsItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISettingsNamespace {
    type Vtable = ISettingsNamespace_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsNamespace> for ::windows::core::IUnknown {
    fn from(value: ISettingsNamespace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsNamespace> for ::windows::core::IUnknown {
    fn from(value: &ISettingsNamespace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsNamespace_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pushsettings: super::super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISettingsResult(pub ::windows::core::IUnknown);
impl ISettingsResult {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: <::windows::core::HRESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetContextDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetLine(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetColumn(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISettingsResult {
    type Vtable = ISettingsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ISettingsResult> for ::windows::core::IUnknown {
    fn from(value: ISettingsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISettingsResult> for ::windows::core::IUnknown {
    fn from(value: &ISettingsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hrout: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwline: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcolumn: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITargetInfo(pub ::windows::core::IUnknown);
impl ITargetInfo {
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetTargetMode(&self) -> ::windows::core::Result<WcmTargetMode> {
        let mut result__: <WcmTargetMode as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WcmTargetMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(targetmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetTemporaryStoreLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetTemporaryStoreLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, temporarystorelocation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), temporarystorelocation.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetTargetID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn SetTargetID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, targetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), targetid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetTargetProcessorArchitecture<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, processorarchitecture: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), processorarchitecture.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, property: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), offline.into_param().abi(), property.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, property: Param1, value: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), offline.into_param().abi(), property.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: <IItemEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn ExpandTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, location: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), offline.into_param().abi(), location.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn ExpandTargetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, offline: Param0, location: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), offline.into_param().abi(), location.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetModulePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, module: Param0, path: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), module.into_param().abi(), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn LoadModule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, module: Param0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE> {
        let mut result__: <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), module.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HINSTANCE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetWow64Context<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, installermodule: Param0, wow64context: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), installermodule.into_param().abi(), ::core::mem::transmute(wow64context)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn TranslateWow64<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, clientarchitecture: Param0, value: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), clientarchitecture.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetSchemaHiveLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwzhivedir: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pwzhivedir.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemaHiveLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn SetSchemaHiveMountName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwzmountname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwzmountname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemaHiveMountName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITargetInfo {
    type Vtable = ITargetInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
}
impl ::core::convert::From<ITargetInfo> for ::windows::core::IUnknown {
    fn from(value: ITargetInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITargetInfo> for ::windows::core::IUnknown {
    fn from(value: &ITargetInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetmode: *mut WcmTargetMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetmode: WcmTargetMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, temporarystorelocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, temporarystorelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, processorarchitecture: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, processorarchitecture: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, module: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, module: super::super::Foundation::PWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, installermodule: super::super::Foundation::PWSTR, wow64context: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clientarchitecture: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, translatedvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzhivedir: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phivelocation: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzmountname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmountname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const SettingsEngine: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ABORTOPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255384i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ASSERTIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255398i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255420i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255421i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_CONFLICTINGASSERTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255399i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_CYCLICREFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255389i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_DUPLICATENAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255397i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255408i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_HANDLERNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255394i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INTERNALERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255424i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255385i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDDATATYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255416i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255401i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255393i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDKEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255396i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255410i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDPATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255413i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255382i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDSTREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255395i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255419i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVALUEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255418i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255411i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_KEYNOTCHANGEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255409i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255390i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MISSINGCONFIGURATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255383i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MIXTYPEASSERTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255388i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255403i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NAMESPACENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255404i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255400i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTPOSITIONED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255415i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255387i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_READONLYITEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255414i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_RESTRICTIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255391i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255381i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STATENODENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255422i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STATENODENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255423i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STORECORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255402i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255407i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_TYPENOTSPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255417i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_UNKNOWNRESULT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145251325i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_USERALREADYREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255406i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_USERNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255405i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_VALIDATIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255392i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_VALUETOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255386i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_WRONGESCAPESTRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255412i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(2232325i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(2232321i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_INTERNALERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(2232320i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(2232324i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_LEGACYSETTINGWARNING: ::windows::core::HRESULT = ::windows::core::HRESULT(2232322i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_NAMESPACENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(2232326i32 as _);
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
unsafe impl ::windows::core::Abi for WcmDataType {
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
unsafe impl ::windows::core::Abi for WcmNamespaceAccess {
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
unsafe impl ::windows::core::Abi for WcmNamespaceEnumerationFlags {
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
unsafe impl ::windows::core::Abi for WcmRestrictionFacets {
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
unsafe impl ::windows::core::Abi for WcmSettingType {
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
unsafe impl ::windows::core::Abi for WcmTargetMode {
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
unsafe impl ::windows::core::Abi for WcmUserStatus {
    type Abi = Self;
}
