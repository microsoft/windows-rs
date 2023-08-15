#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct IItemEnumerator(::windows_core::IUnknown);
impl IItemEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Current(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Current)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IItemEnumerator, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IItemEnumerator {
    type Vtable = IItemEnumerator_Vtbl;
}
impl ::core::clone::Clone for IItemEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IItemEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Current: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsContext(::windows_core::IUnknown);
impl ISettingsContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1>(&self, pstream: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Com::IStream>,
        P1: ::windows_core::IntoParam<ITargetInfo>,
    {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<P0, P1>(&self, pstream: P0, ptarget: P1, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Com::IStream>,
        P1: ::windows_core::IntoParam<ITargetInfo>,
    {
        (::windows_core::Interface::vtable(self).Deserialize)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), ptarget.into_param().abi(), pppresults, pcresultcount).ok()
    }
    pub unsafe fn SetUserData(&self, puserdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserData)(::windows_core::Interface::as_raw(self), puserdata).ok()
    }
    pub unsafe fn GetUserData(&self) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUserData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNamespaces(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNamespaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoredSettings<P0>(&self, pidentity: P0, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISettingsIdentity>,
    {
        (::windows_core::Interface::vtable(self).GetStoredSettings)(::windows_core::Interface::as_raw(self), pidentity.into_param().abi(), ::core::mem::transmute(ppaddedsettings), ::core::mem::transmute(ppmodifiedsettings), ::core::mem::transmute(ppdeletedsettings)).ok()
    }
    pub unsafe fn RevertSetting<P0, P1>(&self, pidentity: P0, pwzsetting: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISettingsIdentity>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RevertSetting)(::windows_core::Interface::as_raw(self), pidentity.into_param().abi(), pwzsetting.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISettingsContext, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for ISettingsContext {
    type Vtable = ISettingsContext_Vtbl;
}
impl ::core::clone::Clone for ISettingsContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISettingsContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    pub SetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStoredSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, ppaddedsettings: *mut *mut ::core::ffi::c_void, ppmodifiedsettings: *mut *mut ::core::ffi::c_void, ppdeletedsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RevertSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, pwzsetting: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsEngine(::windows_core::IUnknown);
impl ISettingsEngine {
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNamespaces)(::windows_core::Interface::as_raw(self), flags, reserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNamespace<P0>(&self, settingsid: P0, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<ISettingsNamespace>
    where
        P0: ::windows_core::IntoParam<ISettingsIdentity>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNamespace)(::windows_core::Interface::as_raw(self), settingsid.into_param().abi(), access, reserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorDescription)(::windows_core::Interface::as_raw(self), hresult, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSettingsIdentity(&self) -> ::windows_core::Result<ISettingsIdentity> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingsIdentity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoreStatus(&self, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<WcmUserStatus> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStoreStatus)(::windows_core::Interface::as_raw(self), reserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadStore(&self, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadStore)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn UnloadStore(&self, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnloadStore)(::windows_core::Interface::as_raw(self), reserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterNamespace<P0, P1, P2>(&self, settingsid: P0, stream: P1, pushsettings: P2) -> ::windows_core::Result<super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<ISettingsIdentity>,
        P1: ::windows_core::IntoParam<super::Com::IStream>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterNamespace)(::windows_core::Interface::as_raw(self), settingsid.into_param().abi(), stream.into_param().abi(), pushsettings.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterNamespace<P0, P1>(&self, settingsid: P0, removesettings: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISettingsIdentity>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).UnregisterNamespace)(::windows_core::Interface::as_raw(self), settingsid.into_param().abi(), removesettings.into_param().abi()).ok()
    }
    pub unsafe fn CreateTargetInfo(&self) -> ::windows_core::Result<ITargetInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateTargetInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTargetInfo(&self) -> ::windows_core::Result<ITargetInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetInfo<P0>(&self, target: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITargetInfo>,
    {
        (::windows_core::Interface::vtable(self).SetTargetInfo)(::windows_core::Interface::as_raw(self), target.into_param().abi()).ok()
    }
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<ISettingsContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingsContext)(::windows_core::Interface::as_raw(self), flags, reserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSettingsContext<P0>(&self, settingscontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISettingsContext>,
    {
        (::windows_core::Interface::vtable(self).SetSettingsContext)(::windows_core::Interface::as_raw(self), settingscontext.into_param().abi()).ok()
    }
    pub unsafe fn ApplySettingsContext<P0>(&self, settingscontext: P0, pppwzidentities: *mut *mut ::windows_core::PWSTR, pcidentities: *mut usize) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISettingsContext>,
    {
        (::windows_core::Interface::vtable(self).ApplySettingsContext)(::windows_core::Interface::as_raw(self), settingscontext.into_param().abi(), pppwzidentities, pcidentities).ok()
    }
    pub unsafe fn GetSettingsContext(&self) -> ::windows_core::Result<ISettingsContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingsContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISettingsEngine, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for ISettingsEngine {
    type Vtable = ISettingsEngine_Vtbl;
}
impl ::core::clone::Clone for ISettingsEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISettingsEngine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsEngine_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CreateSettingsIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStoreStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows_core::HRESULT,
    pub LoadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT,
    pub UnloadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RegisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, results: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RegisterNamespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, removesettings: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterNamespace: usize,
    pub CreateTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApplySettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void, pppwzidentities: *mut *mut ::windows_core::PWSTR, pcidentities: *mut usize) -> ::windows_core::HRESULT,
    pub GetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsIdentity(::windows_core::IUnknown);
impl ISettingsIdentity {
    pub unsafe fn GetAttribute<P0>(&self, reserved: *const ::core::ffi::c_void, name: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAttribute)(::windows_core::Interface::as_raw(self), reserved, name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAttribute<P0, P1>(&self, reserved: *const ::core::ffi::c_void, name: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAttribute)(::windows_core::Interface::as_raw(self), reserved, name.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), flags).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISettingsIdentity, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for ISettingsIdentity {
    type Vtable = ISettingsIdentity_Vtbl;
}
impl ::core::clone::Clone for ISettingsIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISettingsIdentity {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsIdentity_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsItem(::windows_core::IUnknown);
impl ISettingsItem {
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, value: *const super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSettingType(&self) -> ::windows_core::Result<WcmSettingType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataType(&self) -> ::windows_core::Result<WcmDataType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDataType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8, datasize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetValueRaw)(::windows_core::Interface::as_raw(self), data, datasize).ok()
    }
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValueRaw)(::windows_core::Interface::as_raw(self), datatype, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChild(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasChild)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Children(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Children)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChild<P0>(&self, name: P0) -> ::windows_core::Result<ISettingsItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetChild)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSettingByPath<P0>(&self, path: P0) -> ::windows_core::Result<ISettingsItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSettingByPath<P0>(&self, path: P0) -> ::windows_core::Result<ISettingsItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveSettingByPath<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut ::windows_core::BSTR, datatype: *mut WcmDataType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetListKeyInformation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(keyname), datatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateListElement(&self, keydata: *const super::Variant::VARIANT) -> ::windows_core::Result<ISettingsItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateListElement)(::windows_core::Interface::as_raw(self), keydata, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveListElement<P0>(&self, elementname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveListElement)(::windows_core::Interface::as_raw(self), elementname.into_param().abi()).ok()
    }
    pub unsafe fn Attributes(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Attributes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAttribute<P0>(&self, name: P0) -> ::windows_core::Result<super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAttribute)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRestrictionFacets(&self) -> ::windows_core::Result<WcmRestrictionFacets> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRestrictionFacets)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRestriction)(::windows_core::Interface::as_raw(self), restrictionfacet, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetKeyValue(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKeyValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISettingsItem, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for ISettingsItem {
    type Vtable = ISettingsItem_Vtbl;
}
impl ::core::clone::Clone for ISettingsItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISettingsItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsItem_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *const super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub GetSettingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows_core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows_core::HRESULT,
    pub GetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows_core::HRESULT,
    pub SetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChild: usize,
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, child: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetListKeyInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, datatype: *mut WcmDataType) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keydata: *const super::Variant::VARIANT, child: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateListElement: usize,
    pub RemoveListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAttribute: usize,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetRestrictionFacets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetRestriction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetRestriction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetKeyValue: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsNamespace(::windows_core::IUnknown);
impl ISettingsNamespace {
    pub unsafe fn GetIdentity(&self) -> ::windows_core::Result<ISettingsIdentity> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIdentity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Settings(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0>(&self, pushsettings: P0) -> ::windows_core::Result<ISettingsResult>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), pushsettings.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSettingByPath<P0>(&self, path: P0) -> ::windows_core::Result<ISettingsItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSettingByPath<P0>(&self, path: P0) -> ::windows_core::Result<ISettingsItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveSettingByPath<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAttribute<P0>(&self, name: P0) -> ::windows_core::Result<super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAttribute)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISettingsNamespace, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for ISettingsNamespace {
    type Vtable = ISettingsNamespace_Vtbl;
}
impl ::core::clone::Clone for ISettingsNamespace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISettingsNamespace {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsNamespace_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAttribute: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsResult(::windows_core::IUnknown);
impl ISettingsResult {
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContextDescription(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContextDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLine(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLine)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumn(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetColumn)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSource(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISettingsResult, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for ISettingsResult {
    type Vtable = ISettingsResult_Vtbl;
}
impl ::core::clone::Clone for ISettingsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISettingsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrout: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetContextDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ITargetInfo(::windows_core::IUnknown);
impl ITargetInfo {
    pub unsafe fn GetTargetMode(&self) -> ::windows_core::Result<WcmTargetMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetMode)(::windows_core::Interface::as_raw(self), targetmode).ok()
    }
    pub unsafe fn GetTemporaryStoreLocation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTemporaryStoreLocation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTemporaryStoreLocation<P0>(&self, temporarystorelocation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTemporaryStoreLocation)(::windows_core::Interface::as_raw(self), temporarystorelocation.into_param().abi()).ok()
    }
    pub unsafe fn GetTargetID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetID(&self, targetid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(targetid)).ok()
    }
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetProcessorArchitecture)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetProcessorArchitecture<P0>(&self, processorarchitecture: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTargetProcessorArchitecture)(::windows_core::Interface::as_raw(self), processorarchitecture.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<P0, P1>(&self, offline: P0, property: P1) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), property.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<P0, P1, P2>(&self, offline: P0, property: P1, value: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), property.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTarget<P0, P1>(&self, offline: P0, location: P1) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExpandTarget)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), location.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTargetPath<P0, P1>(&self, offline: P0, location: P1) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExpandTargetPath)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), location.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetModulePath<P0, P1>(&self, module: P0, path: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetModulePath)(::windows_core::Interface::as_raw(self), module.into_param().abi(), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadModule<P0>(&self, module: P0) -> ::windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoadModule)(::windows_core::Interface::as_raw(self), module.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWow64Context<P0>(&self, installermodule: P0, wow64context: *const u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetWow64Context)(::windows_core::Interface::as_raw(self), installermodule.into_param().abi(), wow64context).ok()
    }
    pub unsafe fn TranslateWow64<P0, P1>(&self, clientarchitecture: P0, value: P1) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TranslateWow64)(::windows_core::Interface::as_raw(self), clientarchitecture.into_param().abi(), value.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSchemaHiveLocation<P0>(&self, pwzhivedir: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSchemaHiveLocation)(::windows_core::Interface::as_raw(self), pwzhivedir.into_param().abi()).ok()
    }
    pub unsafe fn GetSchemaHiveLocation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSchemaHiveLocation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSchemaHiveMountName<P0>(&self, pwzmountname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSchemaHiveMountName)(::windows_core::Interface::as_raw(self), pwzmountname.into_param().abi()).ok()
    }
    pub unsafe fn GetSchemaHiveMountName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSchemaHiveMountName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITargetInfo, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for ITargetInfo {
    type Vtable = ITargetInfo_Vtbl;
}
impl ::core::clone::Clone for ITargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITargetInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows_core::HRESULT,
    pub SetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows_core::HRESULT,
    pub GetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows_core::PCWSTR, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProperty: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows_core::PCWSTR, expandedlocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTargetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows_core::PCWSTR, expandedlocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTargetPath: usize,
    pub SetModulePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows_core::PCWSTR, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows_core::PCWSTR, modulehandle: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadModule: usize,
    pub SetWow64Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, installermodule: ::windows_core::PCWSTR, wow64context: *const u8) -> ::windows_core::HRESULT,
    pub TranslateWow64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientarchitecture: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR, translatedvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzhivedir: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phivelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzmountname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmountname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const AllEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OfflineMode: WcmTargetMode = WcmTargetMode(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OnlineMode: WcmTargetMode = WcmTargetMode(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadOnlyAccess: WcmNamespaceAccess = WcmNamespaceAccess(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadWriteAccess: WcmNamespaceAccess = WcmNamespaceAccess(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const SettingsEngine: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UnknownStatus: WcmUserStatus = WcmUserStatus(0i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserLoaded: WcmUserStatus = WcmUserStatus(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserRegistered: WcmUserStatus = WcmUserStatus(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnloaded: WcmUserStatus = WcmUserStatus(4i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnregistered: WcmUserStatus = WcmUserStatus(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ABORTOPERATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255384i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ASSERTIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255398i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255420i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255421i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CONFLICTINGASSERTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255399i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CYCLICREFERENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255389i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_DUPLICATENAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255397i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255408i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_HANDLERNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255394i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INTERNALERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255424i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255385i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDDATATYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255416i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255401i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255393i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDKEY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255396i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255410i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPATH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255413i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255382i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDSTREAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255395i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255419i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUEFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255418i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255411i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_KEYNOTCHANGEABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255409i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255390i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MISSINGCONFIGURATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255383i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MIXTYPEASSERTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255388i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255403i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255404i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255400i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTPOSITIONED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255415i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255387i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_READONLYITEM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255414i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_RESTRICTIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255391i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255381i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255422i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255423i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STORECORRUPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255402i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255407i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_TYPENOTSPECIFIED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255417i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_UNKNOWNRESULT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145251325i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERALREADYREGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255406i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255405i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALIDATIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255392i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALUETOOBIG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255386i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_WRONGESCAPESTRING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255412i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_ARCHITECTURE: ::windows_core::PCWSTR = ::windows_core::w!("architecture");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_LANGUAGE: ::windows_core::PCWSTR = ::windows_core::w!("language");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_NAME: ::windows_core::PCWSTR = ::windows_core::w!("name");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_TOKEN: ::windows_core::PCWSTR = ::windows_core::w!("token");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_URI: ::windows_core::PCWSTR = ::windows_core::w!("uri");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION: ::windows_core::PCWSTR = ::windows_core::w!("version");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION_SCOPE: ::windows_core::PCWSTR = ::windows_core::w!("versionScope");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(2232325i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(2232321i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INTERNALERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(2232320i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows_core::HRESULT = ::windows_core::HRESULT(2232324i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_LEGACYSETTINGWARNING: ::windows_core::HRESULT = ::windows_core::HRESULT(2232322i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_NAMESPACENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(2232326i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeBoolean: WcmDataType = WcmDataType(11i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeByte: WcmDataType = WcmDataType(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeFlagArray: WcmDataType = WcmDataType(32768i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt16: WcmDataType = WcmDataType(4i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt32: WcmDataType = WcmDataType(6i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt64: WcmDataType = WcmDataType(8i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeSByte: WcmDataType = WcmDataType(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeString: WcmDataType = WcmDataType(12i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt16: WcmDataType = WcmDataType(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt32: WcmDataType = WcmDataType(5i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt64: WcmDataType = WcmDataType(7i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetEnumeration: WcmRestrictionFacets = WcmRestrictionFacets(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = WcmRestrictionFacets(4i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxLength: WcmRestrictionFacets = WcmRestrictionFacets(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = WcmRestrictionFacets(8i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeComplex: WcmSettingType = WcmSettingType(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeList: WcmSettingType = WcmSettingType(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeScalar: WcmSettingType = WcmSettingType(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmDataType(pub i32);
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
impl ::windows_core::TypeKind for WcmDataType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WcmDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmDataType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmNamespaceAccess(pub i32);
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
impl ::windows_core::TypeKind for WcmNamespaceAccess {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WcmNamespaceAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceAccess").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
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
impl ::windows_core::TypeKind for WcmNamespaceEnumerationFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WcmNamespaceEnumerationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceEnumerationFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmRestrictionFacets(pub i32);
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
impl ::windows_core::TypeKind for WcmRestrictionFacets {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WcmRestrictionFacets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmRestrictionFacets").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmSettingType(pub i32);
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
impl ::windows_core::TypeKind for WcmSettingType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WcmSettingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmSettingType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmTargetMode(pub i32);
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
impl ::windows_core::TypeKind for WcmTargetMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WcmTargetMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmTargetMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmUserStatus(pub i32);
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
impl ::windows_core::TypeKind for WcmUserStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WcmUserStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmUserStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
