#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct IItemEnumerator(::windows::core::IUnknown);
impl IItemEnumerator {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Current(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Current)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MoveNext)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IItemEnumerator> for ::windows::core::IUnknown {
    fn from(value: IItemEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IItemEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IItemEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IItemEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IItemEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IItemEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemEnumerator {}
impl ::core::fmt::Debug for IItemEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IItemEnumerator {
    type Vtable = IItemEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemEnumerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Current: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsContext(::windows::core::IUnknown);
impl ISettingsContext {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IStream>, Param1: ::windows::core::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IStream>, Param1: ::windows::core::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deserialize)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ptarget.into_param().abi(), ::core::mem::transmute(pppresults), ::core::mem::transmute(pcresultcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetUserData(&self, puserdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUserData)(::core::mem::transmute_copy(self), ::core::mem::transmute(puserdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetUserData(&self) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut ::core::ffi::c_void>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetNamespaces(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNamespaces)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetStoredSettings<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>>(&self, pidentity: Param0, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStoredSettings)(::core::mem::transmute_copy(self), pidentity.into_param().abi(), ::core::mem::transmute(ppaddedsettings), ::core::mem::transmute(ppmodifiedsettings), ::core::mem::transmute(ppdeletedsettings)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn RevertSetting<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pidentity: Param0, pwzsetting: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevertSetting)(::core::mem::transmute_copy(self), pidentity.into_param().abi(), pwzsetting.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISettingsContext> for ::windows::core::IUnknown {
    fn from(value: ISettingsContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsContext> for ::windows::core::IUnknown {
    fn from(value: &ISettingsContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsContext {}
impl ::core::fmt::Debug for ISettingsContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISettingsContext {
    type Vtable = ISettingsContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsContext_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr, pppresults: *mut *mut ::windows::core::RawPtr, pcresultcount: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    pub SetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStoredSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: ::windows::core::RawPtr, ppaddedsettings: *mut ::windows::core::RawPtr, ppmodifiedsettings: *mut ::windows::core::RawPtr, ppdeletedsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RevertSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: ::windows::core::RawPtr, pwzsetting: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsEngine(::windows::core::IUnknown);
impl ISettingsEngine {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNamespaces)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetNamespace<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>>(&self, settingsid: Param0, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsNamespace> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNamespace)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), ::core::mem::transmute(access), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsNamespace>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn CreateSettingsIdentity(&self) -> ::windows::core::Result<ISettingsIdentity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSettingsIdentity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetStoreStatus(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<WcmUserStatus> {
        let mut result__: WcmUserStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStoreStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<WcmUserStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn LoadStore(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn UnloadStore(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnloadStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterNamespace<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::core::IntoParam<'a, super::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, stream: Param1, pushsettings: Param2) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterNamespace)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), stream.into_param().abi(), pushsettings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterNamespace<'a, Param0: ::windows::core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, removesettings: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterNamespace)(::core::mem::transmute_copy(self), settingsid.into_param().abi(), removesettings.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn CreateTargetInfo(&self) -> ::windows::core::Result<ITargetInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTargetInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITargetInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetTargetInfo(&self) -> ::windows::core::Result<ITargetInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTargetInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITargetInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetTargetInfo<'a, Param0: ::windows::core::IntoParam<'a, ITargetInfo>>(&self, target: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetInfo)(::core::mem::transmute_copy(self), target.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSettingsContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsContext>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetSettingsContext<'a, Param0: ::windows::core::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSettingsContext)(::core::mem::transmute_copy(self), settingscontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn ApplySettingsContext<'a, Param0: ::windows::core::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0, pppwzidentities: *mut *mut ::windows::core::PWSTR, pcidentities: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplySettingsContext)(::core::mem::transmute_copy(self), settingscontext.into_param().abi(), ::core::mem::transmute(pppwzidentities), ::core::mem::transmute(pcidentities)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetSettingsContext(&self) -> ::windows::core::Result<ISettingsContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSettingsContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsContext>(result__)
    }
}
impl ::core::convert::From<ISettingsEngine> for ::windows::core::IUnknown {
    fn from(value: ISettingsEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsEngine> for ::windows::core::IUnknown {
    fn from(value: &ISettingsEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsEngine {}
impl ::core::fmt::Debug for ISettingsEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISettingsEngine {
    type Vtable = ISettingsEngine_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsEngine_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetErrorDescription: usize,
    pub CreateSettingsIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStoreStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows::core::HRESULT,
    pub LoadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub UnloadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, pushsettings: super::super::Foundation::BOOL, results: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterNamespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, removesettings: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterNamespace: usize,
    pub CreateTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ApplySettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: ::windows::core::RawPtr, pppwzidentities: *mut *mut ::windows::core::PWSTR, pcidentities: *mut usize) -> ::windows::core::HRESULT,
    pub GetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsIdentity(::windows::core::IUnknown);
impl ISettingsIdentity {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1, value: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<ISettingsIdentity> for ::windows::core::IUnknown {
    fn from(value: ISettingsIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsIdentity> for ::windows::core::IUnknown {
    fn from(value: &ISettingsIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsIdentity {}
impl ::core::fmt::Debug for ISettingsIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISettingsIdentity {
    type Vtable = ISettingsIdentity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsIdentity_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttribute: usize,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsItem(::windows::core::IUnknown);
impl ISettingsItem {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, value: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetSettingType(&self) -> ::windows::core::Result<WcmSettingType> {
        let mut result__: WcmSettingType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSettingType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WcmSettingType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetDataType(&self) -> ::windows::core::Result<WcmDataType> {
        let mut result__: WcmDataType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDataType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WcmDataType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValueRaw)(::core::mem::transmute_copy(self), ::core::mem::transmute(data), ::core::mem::transmute(datasize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValueRaw)(::core::mem::transmute_copy(self), ::core::mem::transmute(datatype), ::core::mem::transmute(::windows::core::as_ptr_or_null(data)), data.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChild(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasChild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn Children(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Children)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetChild<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChild)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSettingByPath)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSettingByPath)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveSettingByPath)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetListKeyInformation)(::core::mem::transmute_copy(self), ::core::mem::transmute(keyname), ::core::mem::transmute(datatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateListElement(&self, keydata: *const super::Com::VARIANT) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateListElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(keydata), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn RemoveListElement<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, elementname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveListElement)(::core::mem::transmute_copy(self), elementname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Attributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttribute)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetRestrictionFacets(&self) -> ::windows::core::Result<WcmRestrictionFacets> {
        let mut result__: WcmRestrictionFacets = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRestrictionFacets)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WcmRestrictionFacets>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRestriction)(::core::mem::transmute_copy(self), ::core::mem::transmute(restrictionfacet), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetKeyValue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetKeyValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ISettingsItem> for ::windows::core::IUnknown {
    fn from(value: ISettingsItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsItem> for ::windows::core::IUnknown {
    fn from(value: &ISettingsItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsItem {}
impl ::core::fmt::Debug for ISettingsItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISettingsItem {
    type Vtable = ISettingsItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsItem_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub GetSettingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows::core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows::core::HRESULT,
    pub GetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::HRESULT,
    pub SetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChild: usize,
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetListKeyInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetListKeyInformation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keydata: *const super::Com::VARIANT, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateListElement: usize,
    pub RemoveListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    pub GetRestrictionFacets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRestriction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRestriction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetKeyValue: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsNamespace(::windows::core::IUnknown);
impl ISettingsNamespace {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetIdentity(&self) -> ::windows::core::Result<ISettingsIdentity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIdentity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn Settings(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Settings)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pushsettings: Param0) -> ::windows::core::Result<ISettingsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), pushsettings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSettingByPath)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, path: Param0) -> ::windows::core::Result<ISettingsItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSettingByPath)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISettingsItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveSettingByPath)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, name: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttribute)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ISettingsNamespace> for ::windows::core::IUnknown {
    fn from(value: ISettingsNamespace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsNamespace> for ::windows::core::IUnknown {
    fn from(value: &ISettingsNamespace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsNamespace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsNamespace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsNamespace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsNamespace {}
impl ::core::fmt::Debug for ISettingsNamespace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsNamespace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISettingsNamespace {
    type Vtable = ISettingsNamespace_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsNamespace_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsResult(::windows::core::IUnknown);
impl ISettingsResult {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContextDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetContextDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetLine(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLine)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetColumn(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetColumn)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ISettingsResult> for ::windows::core::IUnknown {
    fn from(value: ISettingsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsResult> for ::windows::core::IUnknown {
    fn from(value: &ISettingsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsResult {}
impl ::core::fmt::Debug for ISettingsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISettingsResult {
    type Vtable = ISettingsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsResult_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrout: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContextDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContextDescription: usize,
    pub GetLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows::core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSource: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ITargetInfo(::windows::core::IUnknown);
impl ITargetInfo {
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetTargetMode(&self) -> ::windows::core::Result<WcmTargetMode> {
        let mut result__: WcmTargetMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTargetMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WcmTargetMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(targetmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemporaryStoreLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTemporaryStoreLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetTemporaryStoreLocation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, temporarystorelocation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTemporaryStoreLocation)(::core::mem::transmute_copy(self), temporarystorelocation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTargetID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetTargetID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, targetid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetID)(::core::mem::transmute_copy(self), targetid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTargetProcessorArchitecture)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetTargetProcessorArchitecture<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, processorarchitecture: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetProcessorArchitecture)(::core::mem::transmute_copy(self), processorarchitecture.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, offline: Param0, property: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), offline.into_param().abi(), property.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, offline: Param0, property: Param1, value: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), offline.into_param().abi(), property.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, offline: Param0, location: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExpandTarget)(::core::mem::transmute_copy(self), offline.into_param().abi(), location.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTargetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, offline: Param0, location: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExpandTargetPath)(::core::mem::transmute_copy(self), offline.into_param().abi(), location.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetModulePath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, module: Param0, path: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetModulePath)(::core::mem::transmute_copy(self), module.into_param().abi(), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadModule<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, module: Param0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE> {
        let mut result__: super::super::Foundation::HINSTANCE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoadModule)(::core::mem::transmute_copy(self), module.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HINSTANCE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetWow64Context<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, installermodule: Param0, wow64context: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWow64Context)(::core::mem::transmute_copy(self), installermodule.into_param().abi(), ::core::mem::transmute(wow64context)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateWow64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, clientarchitecture: Param0, value: Param1) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TranslateWow64)(::core::mem::transmute_copy(self), clientarchitecture.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetSchemaHiveLocation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwzhivedir: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSchemaHiveLocation)(::core::mem::transmute_copy(self), pwzhivedir.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemaHiveLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSchemaHiveLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
    pub unsafe fn SetSchemaHiveMountName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwzmountname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSchemaHiveMountName)(::core::mem::transmute_copy(self), pwzmountname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemaHiveMountName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSchemaHiveMountName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITargetInfo> for ::windows::core::IUnknown {
    fn from(value: ITargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITargetInfo> for ::windows::core::IUnknown {
    fn from(value: &ITargetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetInfo {}
impl ::core::fmt::Debug for ITargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetInfo {
    type Vtable = ITargetInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows::core::HRESULT,
    pub SetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTemporaryStoreLocation: usize,
    pub SetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetID: usize,
    pub SetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetProcessorArchitecture: usize,
    pub SetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows::core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProperty: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows::core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTargetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows::core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTargetPath: usize,
    pub SetModulePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows::core::PCWSTR, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows::core::PCWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadModule: usize,
    pub SetWow64Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, installermodule: ::windows::core::PCWSTR, wow64context: *const u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateWow64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientarchitecture: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR, translatedvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateWow64: usize,
    pub SetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzhivedir: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phivelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSchemaHiveLocation: usize,
    pub SetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzmountname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmountname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSchemaHiveMountName: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const SettingsEngine: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ABORTOPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255384i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ASSERTIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255398i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255420i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255421i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CONFLICTINGASSERTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255399i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CYCLICREFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255389i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_DUPLICATENAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255397i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255408i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_HANDLERNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255394i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INTERNALERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255424i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255385i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDDATATYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255416i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255401i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255393i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDKEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255396i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255410i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255413i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255382i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDSTREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255395i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255419i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255418i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255411i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_KEYNOTCHANGEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255409i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255390i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MISSINGCONFIGURATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255383i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MIXTYPEASSERTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255388i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255403i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255404i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255400i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTPOSITIONED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255415i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255387i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_READONLYITEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255414i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_RESTRICTIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255391i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255381i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255422i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255423i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STORECORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255402i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255407i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_TYPENOTSPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255417i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_UNKNOWNRESULT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145251325i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERALREADYREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255406i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255405i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALIDATIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255392i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALUETOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255386i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_WRONGESCAPESTRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255412i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_ARCHITECTURE: &'static str = "architecture";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_LANGUAGE: &'static str = "language";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_NAME: &'static str = "name";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_TOKEN: &'static str = "token";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_URI: &'static str = "uri";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION: &'static str = "version";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION_SCOPE: &'static str = "versionScope";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(2232325i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(2232321i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INTERNALERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(2232320i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(2232324i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_LEGACYSETTINGWARNING: ::windows::core::HRESULT = ::windows::core::HRESULT(2232322i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_NAMESPACENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(2232326i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmDataType(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeByte: WcmDataType = WcmDataType(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeSByte: WcmDataType = WcmDataType(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt16: WcmDataType = WcmDataType(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt16: WcmDataType = WcmDataType(4i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt32: WcmDataType = WcmDataType(5i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt32: WcmDataType = WcmDataType(6i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt64: WcmDataType = WcmDataType(7i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt64: WcmDataType = WcmDataType(8i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeBoolean: WcmDataType = WcmDataType(11i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeString: WcmDataType = WcmDataType(12i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeFlagArray: WcmDataType = WcmDataType(32768i32);
impl ::core::marker::Copy for WcmDataType {}
impl ::core::clone::Clone for WcmDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmDataType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmNamespaceAccess(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadOnlyAccess: WcmNamespaceAccess = WcmNamespaceAccess(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadWriteAccess: WcmNamespaceAccess = WcmNamespaceAccess(2i32);
impl ::core::marker::Copy for WcmNamespaceAccess {}
impl ::core::clone::Clone for WcmNamespaceAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmNamespaceAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmNamespaceAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmNamespaceAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceAccess").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const AllEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(3i32);
impl ::core::marker::Copy for WcmNamespaceEnumerationFlags {}
impl ::core::clone::Clone for WcmNamespaceEnumerationFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmNamespaceEnumerationFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmNamespaceEnumerationFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmNamespaceEnumerationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceEnumerationFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmRestrictionFacets(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxLength: WcmRestrictionFacets = WcmRestrictionFacets(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetEnumeration: WcmRestrictionFacets = WcmRestrictionFacets(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = WcmRestrictionFacets(4i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = WcmRestrictionFacets(8i32);
impl ::core::marker::Copy for WcmRestrictionFacets {}
impl ::core::clone::Clone for WcmRestrictionFacets {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmRestrictionFacets {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmRestrictionFacets {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmRestrictionFacets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmRestrictionFacets").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmSettingType(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeScalar: WcmSettingType = WcmSettingType(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeComplex: WcmSettingType = WcmSettingType(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeList: WcmSettingType = WcmSettingType(3i32);
impl ::core::marker::Copy for WcmSettingType {}
impl ::core::clone::Clone for WcmSettingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmSettingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmSettingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmSettingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmSettingType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmTargetMode(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OfflineMode: WcmTargetMode = WcmTargetMode(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OnlineMode: WcmTargetMode = WcmTargetMode(2i32);
impl ::core::marker::Copy for WcmTargetMode {}
impl ::core::clone::Clone for WcmTargetMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmTargetMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmTargetMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmTargetMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmTargetMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmUserStatus(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UnknownStatus: WcmUserStatus = WcmUserStatus(0i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserRegistered: WcmUserStatus = WcmUserStatus(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnregistered: WcmUserStatus = WcmUserStatus(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserLoaded: WcmUserStatus = WcmUserStatus(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnloaded: WcmUserStatus = WcmUserStatus(4i32);
impl ::core::marker::Copy for WcmUserStatus {}
impl ::core::clone::Clone for WcmUserStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmUserStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmUserStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmUserStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmUserStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
