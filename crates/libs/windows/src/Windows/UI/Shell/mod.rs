#[doc = "*Required features: `\"UI_Shell\"`*"]
pub struct AdaptiveCardBuilder;
impl AdaptiveCardBuilder {
    pub fn CreateAdaptiveCardFromJson(value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard> {
        Self::IAdaptiveCardBuilderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateAdaptiveCardFromJson)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<IAdaptiveCard>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAdaptiveCardBuilderStatics<R, F: FnOnce(&IAdaptiveCardBuilderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AdaptiveCardBuilder, IAdaptiveCardBuilderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AdaptiveCardBuilder {
    const NAME: &'static str = "Windows.UI.Shell.AdaptiveCardBuilder";
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct IAdaptiveCard(::windows::core::IUnknown);
impl IAdaptiveCard {
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ToJson)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAdaptiveCard> for ::windows::core::IUnknown {
    fn from(value: IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAdaptiveCard> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows::core::IUnknown {
    fn from(value: &IAdaptiveCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAdaptiveCard> for ::windows::core::IInspectable {
    fn from(value: IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAdaptiveCard> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows::core::IInspectable {
    fn from(value: &IAdaptiveCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAdaptiveCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveCard {}
impl ::core::fmt::Debug for IAdaptiveCard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveCard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveCard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72d0568c-a274-41cd-82a8-989d40b9b05e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAdaptiveCard {
    type Vtable = IAdaptiveCard_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d0568c_a274_41cd_82a8_989d40b9b05e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCard_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct IAdaptiveCardBuilderStatics(::windows::core::IUnknown);
impl IAdaptiveCardBuilderStatics {
    pub fn CreateAdaptiveCardFromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateAdaptiveCardFromJson)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<IAdaptiveCard>(result__)
        }
    }
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows::core::IUnknown {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAdaptiveCardBuilderStatics> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows::core::IUnknown {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows::core::IInspectable {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAdaptiveCardBuilderStatics> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows::core::IInspectable {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAdaptiveCardBuilderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveCardBuilderStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveCardBuilderStatics {}
impl ::core::fmt::Debug for IAdaptiveCardBuilderStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveCardBuilderStatics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveCardBuilderStatics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{766d8f08-d3fe-4347-a0bc-b9ea9a6dc28e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAdaptiveCardBuilderStatics {
    type Vtable = IAdaptiveCardBuilderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x766d8f08_d3fe_4347_a0bc_b9ea9a6dc28e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCardBuilderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateAdaptiveCardFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecurityAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecurityAppManager {
    type Vtable = ISecurityAppManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96ac500c_aed4_561d_bde8_953520343a2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityAppManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, detailsuri: *mut ::core::ffi::c_void, registerperuser: bool, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Register: usize,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4578dc09_a523_5756_a995_e4feb991fff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub WindowId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WindowId) -> ::windows::core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ShareWindowCommand) -> ::windows::core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ShareWindowCommand) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportCommandChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommandRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommandRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommandRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommandRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CommandInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommandInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommandInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommandInvoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareWindowCommandSourceStatics {
    type Vtable = IShareWindowCommandSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0eb6656_9cac_517c_b6c7_8ef715084295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManager {
    type Vtable = ITaskbarManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPinningAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsCurrentAppPinnedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsCurrentAppPinnedAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub IsAppListEntryPinnedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    IsAppListEntryPinnedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPinCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinCurrentAppAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestPinAppListEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestPinAppListEntryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManager2 {
    type Vtable = ITaskbarManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f0a06e_7b02_4911_918c_dee0bbd20ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub IsSecondaryTilePinnedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSecondaryTilePinnedAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    pub RequestPinSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, secondarytile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_StartScreen")))]
    RequestPinSecondaryTileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnpinSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnpinSecondaryTileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManagerStatics {
    type Vtable = ITaskbarManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb32ab74_de52_4fe6_b7b6_95ff9f8395df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: Self = Self(0i32);
}
impl ::core::marker::Copy for SecurityAppKind {}
impl ::core::clone::Clone for SecurityAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecurityAppKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SecurityAppKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecurityAppKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct SecurityAppManager(::windows::core::IUnknown);
impl SecurityAppManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SecurityAppManager, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Register<'a, P0>(&self, kind: SecurityAppKind, displayname: &::windows::core::HSTRING, detailsuri: P0, registerperuser: bool) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Register)(::windows::core::Interface::as_raw(this), kind, ::core::mem::transmute_copy(displayname), detailsuri.into().abi(), registerperuser, result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Unregister(&self, kind: SecurityAppKind, guidregistration: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this), kind, guidregistration).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateState<'a, P0>(&self, kind: SecurityAppKind, guidregistration: ::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateState)(::windows::core::Interface::as_raw(this), kind, guidregistration, state, substatus, detailsuri.into().abi()).ok() }
    }
}
impl ::core::clone::Clone for SecurityAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecurityAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecurityAppManager {}
impl ::core::fmt::Debug for SecurityAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.SecurityAppManager;{96ac500c-aed4-561d-bde8-953520343a2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SecurityAppManager {
    type Vtable = ISecurityAppManager_Vtbl;
    const IID: ::windows::core::GUID = <ISecurityAppManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.SecurityAppManager";
}
impl ::core::convert::From<SecurityAppManager> for ::windows::core::IUnknown {
    fn from(value: SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows::core::IUnknown {
    fn from(value: &SecurityAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SecurityAppManager> for &::windows::core::IUnknown {
    fn from(value: &SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SecurityAppManager> for ::windows::core::IInspectable {
    fn from(value: SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows::core::IInspectable {
    fn from(value: &SecurityAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SecurityAppManager> for &::windows::core::IInspectable {
    fn from(value: &SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for SecurityAppManager {}
unsafe impl ::core::marker::Sync for SecurityAppManager {}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for SecurityAppState {}
impl ::core::clone::Clone for SecurityAppState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecurityAppState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SecurityAppState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecurityAppState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: Self = Self(0i32);
    pub const NoActionNeeded: Self = Self(1i32);
    pub const ActionRecommended: Self = Self(2i32);
    pub const ActionNeeded: Self = Self(3i32);
}
impl ::core::marker::Copy for SecurityAppSubstatus {}
impl ::core::clone::Clone for SecurityAppSubstatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecurityAppSubstatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SecurityAppSubstatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecurityAppSubstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppSubstatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppSubstatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppSubstatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: Self = Self(0i32);
    pub const StartSharing: Self = Self(1i32);
    pub const StopSharing: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareWindowCommand {}
impl ::core::clone::Clone for ShareWindowCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShareWindowCommand {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ShareWindowCommand {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShareWindowCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.ShareWindowCommand;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct ShareWindowCommandEventArgs(::windows::core::IUnknown);
impl ShareWindowCommandEventArgs {
    pub fn WindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WindowId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WindowId>(result__)
        }
    }
    pub fn Command(&self) -> ::windows::core::Result<ShareWindowCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Command)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ShareWindowCommand>(result__)
        }
    }
    pub fn SetCommand(&self, value: ShareWindowCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCommand)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ShareWindowCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareWindowCommandEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareWindowCommandEventArgs {}
impl ::core::fmt::Debug for ShareWindowCommandEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommandEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommandEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandEventArgs;{4578dc09-a523-5756-a995-e4feb991fff0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IShareWindowCommandEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandEventArgs";
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandEventArgs {}
unsafe impl ::core::marker::Sync for ShareWindowCommandEventArgs {}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct ShareWindowCommandSource(::windows::core::IUnknown);
impl ShareWindowCommandSource {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCommandChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCommandChanged)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommandRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CommandRequested)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCommandRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCommandRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommandInvoked<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CommandInvoked)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCommandInvoked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCommandInvoked)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<ShareWindowCommandSource> {
        Self::IShareWindowCommandSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ShareWindowCommandSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IShareWindowCommandSourceStatics<R, F: FnOnce(&IShareWindowCommandSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ShareWindowCommandSource, IShareWindowCommandSourceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ShareWindowCommandSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareWindowCommandSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareWindowCommandSource {}
impl ::core::fmt::Debug for ShareWindowCommandSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommandSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommandSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandSource;{cb3b7ae3-6b9c-561e-bccc-61e68e0abfef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_Vtbl;
    const IID: ::windows::core::GUID = <IShareWindowCommandSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandSource";
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows::core::IUnknown {
    fn from(value: ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for &::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows::core::IInspectable {
    fn from(value: ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for &::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandSource {}
unsafe impl ::core::marker::Sync for ShareWindowCommandSource {}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct TaskbarManager(::windows::core::IUnknown);
impl TaskbarManager {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPinningAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPinningAllowed)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsCurrentAppPinnedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentAppPinnedAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn IsAppListEntryPinnedAsync<'a, P0>(&self, applistentry: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::ApplicationModel::Core::AppListEntry>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsAppListEntryPinnedAsync)(::windows::core::Interface::as_raw(this), applistentry.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPinCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestPinCurrentAppAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestPinAppListEntryAsync<'a, P0>(&self, applistentry: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::ApplicationModel::Core::AppListEntry>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestPinAppListEntryAsync)(::windows::core::Interface::as_raw(this), applistentry.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSecondaryTilePinnedAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSecondaryTilePinnedAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_StartScreen\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    pub fn RequestPinSecondaryTileAsync<'a, P0>(&self, secondarytile: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::StartScreen::SecondaryTile>>,
    {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestPinSecondaryTileAsync)(::windows::core::Interface::as_raw(this), secondarytile.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUnpinSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryUnpinSecondaryTileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<TaskbarManager> {
        Self::ITaskbarManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TaskbarManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITaskbarManagerStatics<R, F: FnOnce(&ITaskbarManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TaskbarManager, ITaskbarManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for TaskbarManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TaskbarManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TaskbarManager {}
impl ::core::fmt::Debug for TaskbarManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TaskbarManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TaskbarManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.TaskbarManager;{87490a19-1ad9-49f4-b2e8-86738dc5ac40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TaskbarManager {
    type Vtable = ITaskbarManager_Vtbl;
    const IID: ::windows::core::GUID = <ITaskbarManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.TaskbarManager";
}
impl ::core::convert::From<TaskbarManager> for ::windows::core::IUnknown {
    fn from(value: TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows::core::IUnknown {
    fn from(value: &TaskbarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&TaskbarManager> for &::windows::core::IUnknown {
    fn from(value: &TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<TaskbarManager> for ::windows::core::IInspectable {
    fn from(value: TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows::core::IInspectable {
    fn from(value: &TaskbarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&TaskbarManager> for &::windows::core::IInspectable {
    fn from(value: &TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for TaskbarManager {}
unsafe impl ::core::marker::Sync for TaskbarManager {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
