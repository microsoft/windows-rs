#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct IAdaptiveCard(::windows::core::IUnknown);
impl IAdaptiveCard {
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToJson)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAdaptiveCard, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IAdaptiveCard {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{72d0568c-a274-41cd-82a8-989d40b9b05e}");
}
unsafe impl ::windows::core::Interface for IAdaptiveCard {
    type Vtable = IAdaptiveCard_Vtbl;
}
impl ::core::clone::Clone for IAdaptiveCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAdaptiveCard {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d0568c_a274_41cd_82a8_989d40b9b05e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCard_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct IAdaptiveCardBuilderStatics(::windows::core::IUnknown);
impl IAdaptiveCardBuilderStatics {
    pub fn CreateAdaptiveCardFromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IAdaptiveCard>();
            (::windows::core::Interface::vtable(this).CreateAdaptiveCardFromJson)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAdaptiveCardBuilderStatics, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IAdaptiveCardBuilderStatics {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{766d8f08-d3fe-4347-a0bc-b9ea9a6dc28e}");
}
unsafe impl ::windows::core::Interface for IAdaptiveCardBuilderStatics {
    type Vtable = IAdaptiveCardBuilderStatics_Vtbl;
}
impl ::core::clone::Clone for IAdaptiveCardBuilderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAdaptiveCardBuilderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x766d8f08_d3fe_4347_a0bc_b9ea9a6dc28e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCardBuilderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateAdaptiveCardFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusSession {
    type Vtable = IFocusSession_Vtbl;
}
impl ::core::clone::Clone for IFocusSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFocusSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x069fbab8_0e84_5f2f_8614_9b6544326277);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusSessionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusSessionManager {
    type Vtable = IFocusSessionManager_Vtbl;
}
impl ::core::clone::Clone for IFocusSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFocusSessionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ffbaa9_d8be_5dbf_bac6_49364842e37e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusSessionManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFocusActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryStartFocusSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryStartFocusSession2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endtime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStartFocusSession2: usize,
    pub DeactivateFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsFocusActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsFocusActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsFocusActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsFocusActiveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusSessionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusSessionManagerStatics {
    type Vtable = IFocusSessionManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IFocusSessionManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFocusSessionManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x834df764_cb9a_5d0a_aa9f_73df4f249395);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusSessionManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecurityAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecurityAppManager {
    type Vtable = ISecurityAppManager_Vtbl;
}
impl ::core::clone::Clone for ISecurityAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISecurityAppManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96ac500c_aed4_561d_bde8_953520343a2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityAppManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, displayname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, detailsuri: *mut ::core::ffi::c_void, registerperuser: bool, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IShareWindowCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareWindowCommandEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4578dc09_a523_5756_a995_e4feb991fff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WindowId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WindowId) -> ::windows::core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ShareWindowCommand) -> ::windows::core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ShareWindowCommand) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_Vtbl;
}
impl ::core::clone::Clone for IShareWindowCommandSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareWindowCommandSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IShareWindowCommandSourceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareWindowCommandSourceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0eb6656_9cac_517c_b6c7_8ef715084295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManager {
    type Vtable = ITaskbarManager_Vtbl;
}
impl ::core::clone::Clone for ITaskbarManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITaskbarManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for ITaskbarManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITaskbarManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f0a06e_7b02_4911_918c_dee0bbd20ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IsSecondaryTilePinnedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSecondaryTilePinnedAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    pub RequestPinSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, secondarytile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_StartScreen")))]
    RequestPinSecondaryTileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnpinSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnpinSecondaryTileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManagerStatics {
    type Vtable = ITaskbarManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ITaskbarManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITaskbarManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb32ab74_de52_4fe6_b7b6_95ff9f8395df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
pub struct AdaptiveCardBuilder;
impl AdaptiveCardBuilder {
    pub fn CreateAdaptiveCardFromJson(value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard> {
        Self::IAdaptiveCardBuilderStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<IAdaptiveCard>();
            (::windows::core::Interface::vtable(this).CreateAdaptiveCardFromJson)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAdaptiveCardBuilderStatics<R, F: FnOnce(&IAdaptiveCardBuilderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AdaptiveCardBuilder, IAdaptiveCardBuilderStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AdaptiveCardBuilder {
    const NAME: &'static str = "Windows.UI.Shell.AdaptiveCardBuilder";
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct FocusSession(::windows::core::IUnknown);
impl FocusSession {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn End(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).End)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for FocusSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusSession {}
impl ::core::fmt::Debug for FocusSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FocusSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.FocusSession;{069fbab8-0e84-5f2f-8614-9b6544326277})");
}
impl ::core::clone::Clone for FocusSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FocusSession {
    type Vtable = IFocusSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FocusSession {
    const IID: ::windows::core::GUID = <IFocusSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FocusSession {
    const NAME: &'static str = "Windows.UI.Shell.FocusSession";
}
::windows::imp::interface_hierarchy!(FocusSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FocusSession {}
unsafe impl ::core::marker::Sync for FocusSession {}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct FocusSessionManager(::windows::core::IUnknown);
impl FocusSessionManager {
    pub fn IsFocusActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsFocusActive)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetSession(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<FocusSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FocusSession>();
            (::windows::core::Interface::vtable(this).GetSession)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    pub fn TryStartFocusSession(&self) -> ::windows::core::Result<FocusSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FocusSession>();
            (::windows::core::Interface::vtable(this).TryStartFocusSession)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryStartFocusSession2(&self, endtime: super::super::Foundation::DateTime) -> ::windows::core::Result<FocusSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FocusSession>();
            (::windows::core::Interface::vtable(this).TryStartFocusSession2)(::windows::core::Interface::as_raw(this), endtime, &mut result__).from_abi(result__)
        }
    }
    pub fn DeactivateFocus(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DeactivateFocus)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsFocusActiveChanged(&self, handler: &super::super::Foundation::TypedEventHandler<FocusSessionManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).IsFocusActiveChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsFocusActiveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsFocusActiveChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<FocusSessionManager> {
        Self::IFocusSessionManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<FocusSessionManager>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IFocusSessionManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFocusSessionManagerStatics<R, F: FnOnce(&IFocusSessionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<FocusSessionManager, IFocusSessionManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for FocusSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusSessionManager {}
impl ::core::fmt::Debug for FocusSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusSessionManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FocusSessionManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.FocusSessionManager;{e7ffbaa9-d8be-5dbf-bac6-49364842e37e})");
}
impl ::core::clone::Clone for FocusSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FocusSessionManager {
    type Vtable = IFocusSessionManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FocusSessionManager {
    const IID: ::windows::core::GUID = <IFocusSessionManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FocusSessionManager {
    const NAME: &'static str = "Windows.UI.Shell.FocusSessionManager";
}
::windows::imp::interface_hierarchy!(FocusSessionManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FocusSessionManager {}
unsafe impl ::core::marker::Sync for FocusSessionManager {}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct SecurityAppManager(::windows::core::IUnknown);
impl SecurityAppManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SecurityAppManager, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Register(&self, kind: SecurityAppKind, displayname: &::windows::core::HSTRING, detailsuri: &super::super::Foundation::Uri, registerperuser: bool) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Register)(::windows::core::Interface::as_raw(this), kind, ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(detailsuri), registerperuser, &mut result__).from_abi(result__)
        }
    }
    pub fn Unregister(&self, kind: SecurityAppKind, guidregistration: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this), kind, guidregistration).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateState(&self, kind: SecurityAppKind, guidregistration: ::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateState)(::windows::core::Interface::as_raw(this), kind, guidregistration, state, substatus, ::core::mem::transmute_copy(detailsuri)).ok() }
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
impl ::windows::core::RuntimeType for SecurityAppManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.SecurityAppManager;{96ac500c-aed4-561d-bde8-953520343a2d})");
}
impl ::core::clone::Clone for SecurityAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SecurityAppManager {
    type Vtable = ISecurityAppManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SecurityAppManager {
    const IID: ::windows::core::GUID = <ISecurityAppManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.SecurityAppManager";
}
::windows::imp::interface_hierarchy!(SecurityAppManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SecurityAppManager {}
unsafe impl ::core::marker::Sync for SecurityAppManager {}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct ShareWindowCommandEventArgs(::windows::core::IUnknown);
impl ShareWindowCommandEventArgs {
    pub fn WindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::WindowId>();
            (::windows::core::Interface::vtable(this).WindowId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Command(&self) -> ::windows::core::Result<ShareWindowCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ShareWindowCommand>();
            (::windows::core::Interface::vtable(this).Command)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCommand(&self, value: ShareWindowCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCommand)(::windows::core::Interface::as_raw(this), value).ok() }
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
impl ::windows::core::RuntimeType for ShareWindowCommandEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandEventArgs;{4578dc09-a523-5756-a995-e4feb991fff0})");
}
impl ::core::clone::Clone for ShareWindowCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareWindowCommandEventArgs {
    const IID: ::windows::core::GUID = <IShareWindowCommandEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandEventArgs";
}
::windows::imp::interface_hierarchy!(ShareWindowCommandEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
    pub fn CommandRequested(&self, handler: &super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CommandRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
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
    pub fn CommandInvoked(&self, handler: &super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CommandInvoked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
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
            let mut result__ = ::windows::core::zeroed::<ShareWindowCommandSource>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IShareWindowCommandSourceStatics<R, F: FnOnce(&IShareWindowCommandSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ShareWindowCommandSource, IShareWindowCommandSourceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for ShareWindowCommandSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandSource;{cb3b7ae3-6b9c-561e-bccc-61e68e0abfef})");
}
impl ::core::clone::Clone for ShareWindowCommandSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareWindowCommandSource {
    const IID: ::windows::core::GUID = <IShareWindowCommandSource as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandSource";
}
::windows::imp::interface_hierarchy!(ShareWindowCommandSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShareWindowCommandSource {}
unsafe impl ::core::marker::Sync for ShareWindowCommandSource {}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct TaskbarManager(::windows::core::IUnknown);
impl TaskbarManager {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPinningAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPinningAllowed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsCurrentAppPinnedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).IsCurrentAppPinnedAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn IsAppListEntryPinnedAsync(&self, applistentry: &super::super::ApplicationModel::Core::AppListEntry) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).IsAppListEntryPinnedAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applistentry), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPinCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).RequestPinCurrentAppAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestPinAppListEntryAsync(&self, applistentry: &super::super::ApplicationModel::Core::AppListEntry) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).RequestPinAppListEntryAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applistentry), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSecondaryTilePinnedAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).IsSecondaryTilePinnedAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_StartScreen\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    pub fn RequestPinSecondaryTileAsync(&self, secondarytile: &super::StartScreen::SecondaryTile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).RequestPinSecondaryTileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(secondarytile), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUnpinSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryUnpinSecondaryTileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<TaskbarManager> {
        Self::ITaskbarManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TaskbarManager>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITaskbarManagerStatics<R, F: FnOnce(&ITaskbarManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TaskbarManager, ITaskbarManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TaskbarManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.TaskbarManager;{87490a19-1ad9-49f4-b2e8-86738dc5ac40})");
}
impl ::core::clone::Clone for TaskbarManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TaskbarManager {
    type Vtable = ITaskbarManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TaskbarManager {
    const IID: ::windows::core::GUID = <ITaskbarManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.TaskbarManager";
}
::windows::imp::interface_hierarchy!(TaskbarManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TaskbarManager {}
unsafe impl ::core::marker::Sync for TaskbarManager {}
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
impl ::windows::core::TypeKind for SecurityAppKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SecurityAppKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SecurityAppKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppKind;i4)");
}
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
impl ::windows::core::TypeKind for SecurityAppState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SecurityAppState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SecurityAppState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppState;i4)");
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
impl ::windows::core::TypeKind for SecurityAppSubstatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SecurityAppSubstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppSubstatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SecurityAppSubstatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppSubstatus;i4)");
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
impl ::windows::core::TypeKind for ShareWindowCommand {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ShareWindowCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommand").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareWindowCommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.ShareWindowCommand;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
