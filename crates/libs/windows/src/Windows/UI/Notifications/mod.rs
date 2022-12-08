#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct IAdaptiveNotificationContent(::windows::core::IUnknown);
impl IAdaptiveNotificationContent {
    pub fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hints)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IAdaptiveNotificationContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IAdaptiveNotificationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveNotificationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveNotificationContent {}
impl ::core::fmt::Debug for IAdaptiveNotificationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveNotificationContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveNotificationContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{eb0dbe66-7448-448d-9db8-d78acd2abba9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IAdaptiveNotificationContent {
    type Vtable = IAdaptiveNotificationContent_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdaptiveNotificationContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb0dbe66_7448_448d_9db8_d78acd2abba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationContent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveNotificationContentKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveNotificationText(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdaptiveNotificationText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46d4a3be_609a_4326_a40b_bfde872034a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationText_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBadgeNotification {
    type Vtable = IBadgeNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IBadgeNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x075cb4ca_d08a_4e2f_9233_7e289c1f7722);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBadgeNotificationFactory {
    type Vtable = IBadgeNotificationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBadgeNotificationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedf255ce_0618_4d59_948a_5a61040c52f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateBadgeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateBadgeNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdateManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for IBadgeUpdateManagerForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x996b21bc_0386_44e5_ba8d_0c1077a62e92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBadgeUpdateManagerStatics {
    type Vtable = IBadgeUpdateManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBadgeUpdateManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33400faa_6dd5_4105_aebc_9b50fca492da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: BadgeTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBadgeUpdateManagerStatics2 {
    type Vtable = IBadgeUpdateManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBadgeUpdateManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x979a35ce_f940_48bf_94e8_ca244d400b41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdater(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBadgeUpdater {
    type Vtable = IBadgeUpdater_Vtbl;
}
unsafe impl ::windows::core::Interface for IBadgeUpdater {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5fa1fd4_7562_4f6c_bfa3_1b6ed2e57f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdater_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, badgecontent: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, badgecontent: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdateAtTime: usize,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationHintsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownAdaptiveNotificationHintsStatics {
    type Vtable = IKnownAdaptiveNotificationHintsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownAdaptiveNotificationHintsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06206598_d496_497d_8692_4f7d7c2770df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationHintsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Style: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Wrap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TextStacking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Align: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationTextStylesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownAdaptiveNotificationTextStylesStatics {
    type Vtable = IKnownAdaptiveNotificationTextStylesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownAdaptiveNotificationTextStylesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202192d7_8996_45aa_8ba1_d461d72c2a1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationTextStylesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Base: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subheader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TitleNumeral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubheaderNumeral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HeaderNumeral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CaptionSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BodySubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BaseSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubtitleSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TitleSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubheaderSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubheaderNumeralSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HeaderSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HeaderNumeralSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownNotificationBindingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownNotificationBindingsStatics {
    type Vtable = IKnownNotificationBindingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownNotificationBindingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79427bae_a8b7_4d58_89ea_76a7b7bccded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownNotificationBindingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ToastGeneric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotification {
    type Vtable = INotification_Vtbl;
}
unsafe impl ::windows::core::Interface for INotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108037fe_eb76_4f82_97bc_da07530a2e20);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    pub Visual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationBinding(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotificationBinding {
    type Vtable = INotificationBinding_Vtbl;
}
unsafe impl ::windows::core::Interface for INotificationBinding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29e4b85_0370_4ad3_b4ea_da9e35e7eabf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationBinding_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextElements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotificationData {
    type Vtable = INotificationData_Vtbl;
}
unsafe impl ::windows::core::Interface for INotificationData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ffd2312_9d6a_4aaf_b6ac_ff17f0c1f280);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
    pub SequenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetSequenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationDataFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotificationDataFactory {
    type Vtable = INotificationDataFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for INotificationDataFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23c1e33a_1c10_46fb_8040_dec384621cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationDataFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValuesAndSequenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalues: *mut ::core::ffi::c_void, sequencenumber: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValuesAndSequenceNumber: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalues: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValues: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationVisual(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotificationVisual {
    type Vtable = INotificationVisual_Vtbl;
}
unsafe impl ::windows::core::Interface for INotificationVisual {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68835b8e_aa56_4e11_86d3_5f9a6957bc5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationVisual_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Bindings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bindings: usize,
    pub GetBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, templatename: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledTileNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledTileNotification {
    type Vtable = IScheduledTileNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledTileNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abca6d5_99dc_4c78_a11c_c9e7f86d7ef7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub DeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledTileNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledTileNotificationFactory {
    type Vtable = IScheduledTileNotificationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledTileNotificationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3383138a_98c0_4c3b_bbd6_4a633c7cfc29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub CreateScheduledTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, deliverytime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))]
    CreateScheduledTileNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledToastNotification {
    type Vtable = IScheduledToastNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledToastNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f577f8_0de7_48cd_9740_9b370490c838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub DeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub SnoozeInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeInterval: usize,
    pub MaximumSnoozeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledToastNotification2 {
    type Vtable = IScheduledToastNotification2_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledToastNotification2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa66ea09c_31b4_43b0_b5dd_7a40e85363b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledToastNotification3 {
    type Vtable = IScheduledToastNotification3_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledToastNotification3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98429e8b_bd32_4a3b_9d15_22aea49462a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledToastNotification4 {
    type Vtable = IScheduledToastNotification4_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledToastNotification4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d4761fd_bdef_4e4a_96be_0101369b58d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledToastNotificationFactory {
    type Vtable = IScheduledToastNotificationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledToastNotificationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7bed191_0bb9_4189_8394_31761b476fd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub CreateScheduledToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, deliverytime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))]
    CreateScheduledToastNotification: usize,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub CreateScheduledToastNotificationRecurring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, deliverytime: super::super::Foundation::DateTime, snoozeinterval: super::super::Foundation::TimeSpan, maximumsnoozecount: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))]
    CreateScheduledToastNotificationRecurring: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotificationShowingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IScheduledToastNotificationShowingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6173f6b4_412a_5e2c_a6ed_a0209aef9a09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationShowingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ScheduledToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShownTileNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IShownTileNotification {
    type Vtable = IShownTileNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IShownTileNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342d8988_5af2_481a_a6a3_f2fdc78de88e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShownTileNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileFlyoutNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a53b261_c70c_42be_b2f3_f42aa97d34e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileFlyoutNotificationFactory {
    type Vtable = ITileFlyoutNotificationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileFlyoutNotificationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef556ff5_5226_4f2b_b278_88a35dfe569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileFlyoutNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileFlyoutNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutUpdateManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileFlyoutUpdateManagerStatics {
    type Vtable = ITileFlyoutUpdateManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileFlyoutUpdateManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04363b0b_1ac0_4b99_88e7_ada83e953d48);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdateManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateTileFlyoutUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTileFlyoutUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTileFlyoutUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TileFlyoutTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutUpdater(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileFlyoutUpdater {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d40c76a_c465_4052_a740_5c2654c1a089);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdater_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileflyoutcontent: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileflyoutcontent: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdateAtTime: usize,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileNotification {
    type Vtable = ITileNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebaec8fa_50ec_4c18_b4d0_3af02e5540ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileNotificationFactory {
    type Vtable = ITileNotificationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileNotificationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6abdd6e_4928_46c8_bdbf_81a047dea0d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdateManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileUpdateManagerForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55141348_2ee2_4e2d_9cc1_216a20decc9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdateManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileUpdateManagerStatics {
    type Vtable = ITileUpdateManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileUpdateManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda159e5d_3ea9_4986_8d84_b09d5e12276d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TileTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdateManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileUpdateManagerStatics2 {
    type Vtable = ITileUpdateManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileUpdateManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x731c1ddc_8e14_4b7c_a34b_9d22de76c84d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdater(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileUpdater {
    type Vtable = ITileUpdater_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileUpdater {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0942a48b_1d91_44ec_9243_c1e821c29a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableNotificationQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledTileNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledTileNotifications: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontent: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontent: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdateAtTime: usize,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontents: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatch: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatchAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontents: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatchAtTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdater2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileUpdater2 {
    type Vtable = ITileUpdater2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileUpdater2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2266e12_15ee_43ed_83f5_65b352bb1a84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EnableNotificationQueueForSquare150x150: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT,
    pub EnableNotificationQueueForWide310x150: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT,
    pub EnableNotificationQueueForSquare310x310: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bf92f3_c197_436f_8265_0625824f8dac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastActivatedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastActivatedEventArgs2 {
    type Vtable = IToastActivatedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastActivatedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab7da512_cc61_568e_81be_304ac31038fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastCollection {
    type Vtable = IToastCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a8bc3b0_e0be_4858_bc2a_89dfe0b32863);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LaunchArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLaunchArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Icon: usize,
    #[cfg(feature = "Foundation")]
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastCollectionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastCollectionFactory {
    type Vtable = IToastCollectionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastCollectionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x164dd3d7_73c4_44f7_b4ff_fb6d4bf1f4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, launchargs: *mut ::core::ffi::c_void, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastCollectionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastCollectionManager {
    type Vtable = IToastCollectionManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastCollectionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a1821fe_179d_49bc_b79d_a527920d3665);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SaveToastCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveToastCollectionAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllToastCollectionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllToastCollectionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetToastCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetToastCollectionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveToastCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveToastCollectionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAllToastCollectionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAllToastCollectionsAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastDismissedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastDismissedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f89d935_d9cb_4538_a0f0_ffe7659938f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastDismissedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ToastDismissalReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastFailedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35176862_cfd4_44f8_ad64_f500fd896c3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastFailedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotification {
    type Vtable = IToastNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997e2675_059e_4e60_8b06_1760917c8b80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub Dismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotification2 {
    type Vtable = IToastNotification2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotification2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dfb9fd1_143a_490e_90bf_b9fba7132de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotification3 {
    type Vtable = IToastNotification3_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotification3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31e8aed8_8141_4f99_bc0a_c4ed21297d77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotification4 {
    type Vtable = IToastNotification4_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotification4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15154935_28ea_4727_88e9_c58680e2d118);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ToastNotificationPriority) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ToastNotificationPriority) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotification6 {
    type Vtable = IToastNotification6_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotification6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43ebfe53_89ae_5c1e_a279_3aecfe9b6f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExpiresOnReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetExpiresOnReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationActionTriggerDetail(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationActionTriggerDetail {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9445135a_38f3_42f6_96aa_7955b0f03da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerDetail_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Argument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationFactory {
    type Vtable = IToastNotificationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04124b20_82c6_4229_b109_fd9ed4662b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateToastNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationHistory {
    type Vtable = IToastNotificationHistory_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationHistory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5caddc63_01d3_4c97_986f_0533483fee14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoveGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveGroupWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveGroupedTagWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: *mut ::core::ffi::c_void, group: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveGroupedTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: *mut ::core::ffi::c_void, group: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationHistory2 {
    type Vtable = IToastNotificationHistory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationHistory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bc3d253_2f31_4092_9129_8ad5abf067da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistory: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistoryWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistoryWithId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationHistoryChangedTriggerDetail {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb037ffa_0068_412c_9c83_267c37f65670);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ToastHistoryChangedType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationHistoryChangedTriggerDetail2 {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationHistoryChangedTriggerDetail2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b36e982_c871_49fb_babb_25bdbc4cc45b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CollectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationManagerForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79ab57f6_43fe_487b_8a7f_99567200ae94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateToastNotifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub History: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerForUser2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationManagerForUser2 {
    type Vtable = IToastNotificationManagerForUser2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationManagerForUser2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x679c64b7_81ab_42c2_8819_c958767753f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetToastNotifierForToastCollectionIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetToastNotifierForToastCollectionIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetHistoryForToastCollectionIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHistoryForToastCollectionIdAsync: usize,
    pub GetToastCollectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetToastCollectionManagerWithAppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerForUser3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationManagerForUser3 {
    type Vtable = IToastNotificationManagerForUser3_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationManagerForUser3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3efcb176_6cc1_56dc_973b_251f7aacb1c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ToastNotificationMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotificationModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotificationModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotificationModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotificationModeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationManagerStatics {
    type Vtable = IToastNotificationManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ac103f_d235_4598_bbef_98fe4d1a3ad4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateToastNotifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ToastTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationManagerStatics2 {
    type Vtable = IToastNotificationManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ab93c52_0e48_4750_ba9d_1a4113981847);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub History: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationManagerStatics4 {
    type Vtable = IToastNotificationManagerStatics4_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f993fd3_e516_45fb_8130_398e93fa52c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub ConfigureNotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationManagerStatics5 {
    type Vtable = IToastNotificationManagerStatics5_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationManagerStatics5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6f5f569_d40d_407c_8989_88cab42cfd14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotifier {
    type Vtable = IToastNotifier_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75927b93_03f3_41ec_91d3_6e5bac1b38e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtoast: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtoast: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledToastNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledToastNotifications: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotifier2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotifier2 {
    type Vtable = IToastNotifier2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotifier2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x354389c6_7c01_4bd5_9c20_604340cd2b74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UpdateWithTagAndGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, tag: *mut ::core::ffi::c_void, group: *mut ::core::ffi::c_void, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT,
    pub UpdateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, tag: *mut ::core::ffi::c_void, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotifier3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotifier3 {
    type Vtable = IToastNotifier3_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotifier3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae75a04a_3b0c_51ad_b7e8_b08ab6052549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ScheduledToastNotificationShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScheduledToastNotificationShowing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScheduledToastNotificationShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScheduledToastNotificationShowing: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserNotification {
    type Vtable = IUserNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadf7e52f_4e53_42d5_9c33_eb5ea515b23e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Notification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreationTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserNotificationChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6bd6839_79cf_4b25_82c0_0ce1eef81f8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserNotificationChangedKind) -> ::windows::core::HRESULT,
    pub UserNotificationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct AdaptiveNotificationText(::windows::core::IUnknown);
impl AdaptiveNotificationText {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AdaptiveNotificationText, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind> {
        let this = &::windows::core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hints)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveNotificationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveNotificationText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveNotificationText {}
impl ::core::fmt::Debug for AdaptiveNotificationText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveNotificationText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveNotificationText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.AdaptiveNotificationText;{46d4a3be-609a-4326-a40b-bfde872034a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_Vtbl;
}
unsafe impl ::windows::core::Interface for AdaptiveNotificationText {
    const IID: ::windows::core::GUID = <IAdaptiveNotificationText as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveNotificationText {
    const NAME: &'static str = "Windows.UI.Notifications.AdaptiveNotificationText";
}
::windows::core::interface_hierarchy!(AdaptiveNotificationText, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows::core::Error;
    fn try_from(value: AdaptiveNotificationText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &AdaptiveNotificationText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&AdaptiveNotificationText> for ::windows::core::InParam<IAdaptiveNotificationContent> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AdaptiveNotificationText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AdaptiveNotificationText {}
unsafe impl ::core::marker::Sync for AdaptiveNotificationText {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct BadgeNotification(::windows::core::IUnknown);
impl BadgeNotification {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateBadgeNotification(content: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<BadgeNotification> {
        Self::IBadgeNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBadgeNotification)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(content), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBadgeNotificationFactory<R, F: FnOnce(&IBadgeNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BadgeNotification, IBadgeNotificationFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BadgeNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BadgeNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeNotification {}
impl ::core::fmt::Debug for BadgeNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BadgeNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeNotification;{075cb4ca-d08a-4e2f-9233-7e289c1f7722})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BadgeNotification {
    type Vtable = IBadgeNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for BadgeNotification {
    const IID: ::windows::core::GUID = <IBadgeNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BadgeNotification {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeNotification";
}
::windows::core::interface_hierarchy!(BadgeNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BadgeNotification {}
unsafe impl ::core::marker::Sync for BadgeNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
pub struct BadgeUpdateManager;
impl BadgeUpdateManager {
    pub fn CreateBadgeUpdaterForApplication() -> ::windows::core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBadgeUpdaterForApplication)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateBadgeUpdaterForApplicationWithId(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBadgeUpdaterForApplicationWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateBadgeUpdaterForSecondaryTile(tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBadgeUpdaterForSecondaryTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: BadgeTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTemplateContent)(::windows::core::Vtable::as_raw(this), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<BadgeUpdateManagerForUser> {
        Self::IBadgeUpdateManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBadgeUpdateManagerStatics<R, F: FnOnce(&IBadgeUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBadgeUpdateManagerStatics2<R, F: FnOnce(&IBadgeUpdateManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for BadgeUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManager";
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct BadgeUpdateManagerForUser(::windows::core::IUnknown);
impl BadgeUpdateManagerForUser {
    pub fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBadgeUpdaterForApplication)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBadgeUpdaterForApplicationWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBadgeUpdaterForSecondaryTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BadgeUpdateManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BadgeUpdateManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeUpdateManagerForUser {}
impl ::core::fmt::Debug for BadgeUpdateManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeUpdateManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BadgeUpdateManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeUpdateManagerForUser;{996b21bc-0386-44e5-ba8d-0c1077a62e92})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for BadgeUpdateManagerForUser {
    const IID: ::windows::core::GUID = <IBadgeUpdateManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BadgeUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManagerForUser";
}
::windows::core::interface_hierarchy!(BadgeUpdateManagerForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BadgeUpdateManagerForUser {}
unsafe impl ::core::marker::Sync for BadgeUpdateManagerForUser {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct BadgeUpdater(::windows::core::IUnknown);
impl BadgeUpdater {
    pub fn Update(&self, notification: &BadgeNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Update)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(notification)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate(&self, badgecontent: &super::super::Foundation::Uri, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(badgecontent), requestedinterval).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime(&self, badgecontent: &super::super::Foundation::Uri, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdateAtTime)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(badgecontent), starttime, requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StopPeriodicUpdate)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for BadgeUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BadgeUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeUpdater {}
impl ::core::fmt::Debug for BadgeUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BadgeUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeUpdater;{b5fa1fd4-7562-4f6c-bfa3-1b6ed2e57f2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BadgeUpdater {
    type Vtable = IBadgeUpdater_Vtbl;
}
unsafe impl ::windows::core::Interface for BadgeUpdater {
    const IID: ::windows::core::GUID = <IBadgeUpdater as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BadgeUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdater";
}
::windows::core::interface_hierarchy!(BadgeUpdater, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BadgeUpdater {}
unsafe impl ::core::marker::Sync for BadgeUpdater {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
pub struct KnownAdaptiveNotificationHints;
impl KnownAdaptiveNotificationHints {
    pub fn Style() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Style)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Wrap() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Wrap)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MaxLines() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxLines)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MinLines() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinLines)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TextStacking() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextStacking)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Align() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Align)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownAdaptiveNotificationHintsStatics<R, F: FnOnce(&IKnownAdaptiveNotificationHintsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownAdaptiveNotificationHints, IKnownAdaptiveNotificationHintsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownAdaptiveNotificationHints {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationHints";
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
pub struct KnownAdaptiveNotificationTextStyles;
impl KnownAdaptiveNotificationTextStyles {
    pub fn Caption() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Caption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Body() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Body)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Base() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Base)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Subtitle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subtitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Title() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Subheader() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subheader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Header() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Header)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TitleNumeral() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TitleNumeral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SubheaderNumeral() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubheaderNumeral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HeaderNumeral() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderNumeral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CaptionSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CaptionSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn BodySubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BodySubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn BaseSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SubtitleSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubtitleSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TitleSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TitleSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SubheaderSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubheaderSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SubheaderNumeralSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubheaderNumeralSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HeaderSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HeaderNumeralSubtle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderNumeralSubtle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownAdaptiveNotificationTextStylesStatics<R, F: FnOnce(&IKnownAdaptiveNotificationTextStylesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownAdaptiveNotificationTextStyles, IKnownAdaptiveNotificationTextStylesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownAdaptiveNotificationTextStyles {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles";
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
pub struct KnownNotificationBindings;
impl KnownNotificationBindings {
    pub fn ToastGeneric() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownNotificationBindingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToastGeneric)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownNotificationBindingsStatics<R, F: FnOnce(&IKnownNotificationBindingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownNotificationBindings, IKnownNotificationBindingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownNotificationBindings {
    const NAME: &'static str = "Windows.UI.Notifications.KnownNotificationBindings";
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct Notification(::windows::core::IUnknown);
impl Notification {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Notification, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Visual(&self) -> ::windows::core::Result<NotificationVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Visual)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetVisual(&self, value: &NotificationVisual) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVisual)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for Notification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Notification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Notification {}
impl ::core::fmt::Debug for Notification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Notification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Notification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.Notification;{108037fe-eb76-4f82-97bc-da07530a2e20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Notification {
    type Vtable = INotification_Vtbl;
}
unsafe impl ::windows::core::Interface for Notification {
    const IID: ::windows::core::GUID = <INotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Notification {
    const NAME: &'static str = "Windows.UI.Notifications.Notification";
}
::windows::core::interface_hierarchy!(Notification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Notification {}
unsafe impl ::core::marker::Sync for Notification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct NotificationBinding(::windows::core::IUnknown);
impl NotificationBinding {
    pub fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Template)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTemplate(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTemplate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hints)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTextElements)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for NotificationBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotificationBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationBinding {}
impl ::core::fmt::Debug for NotificationBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationBinding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationBinding;{f29e4b85-0370-4ad3-b4ea-da9e35e7eabf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NotificationBinding {
    type Vtable = INotificationBinding_Vtbl;
}
unsafe impl ::windows::core::Interface for NotificationBinding {
    const IID: ::windows::core::GUID = <INotificationBinding as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotificationBinding {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationBinding";
}
::windows::core::interface_hierarchy!(NotificationBinding, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NotificationBinding {}
unsafe impl ::core::marker::Sync for NotificationBinding {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct NotificationData(::windows::core::IUnknown);
impl NotificationData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NotificationData, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Values)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SequenceNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SequenceNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSequenceNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSequenceNumber)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValuesAndSequenceNumber<P0, E0>(initialvalues: P0, sequencenumber: u32) -> ::windows::core::Result<NotificationData>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNotificationDataWithValuesAndSequenceNumber)(::windows::core::Vtable::as_raw(this), initialvalues.try_into().map_err(|e| e.into())?.abi(), sequencenumber, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValues<P0, E0>(initialvalues: P0) -> ::windows::core::Result<NotificationData>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNotificationDataWithValues)(::windows::core::Vtable::as_raw(this), initialvalues.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INotificationDataFactory<R, F: FnOnce(&INotificationDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NotificationData, INotificationDataFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for NotificationData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotificationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationData {}
impl ::core::fmt::Debug for NotificationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationData;{9ffd2312-9d6a-4aaf-b6ac-ff17f0c1f280})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NotificationData {
    type Vtable = INotificationData_Vtbl;
}
unsafe impl ::windows::core::Interface for NotificationData {
    const IID: ::windows::core::GUID = <INotificationData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotificationData {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationData";
}
::windows::core::interface_hierarchy!(NotificationData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NotificationData {}
unsafe impl ::core::marker::Sync for NotificationData {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct NotificationVisual(::windows::core::IUnknown);
impl NotificationVisual {
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bindings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<NotificationBinding>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bindings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBinding(&self, templatename: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationBinding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBinding)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(templatename), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for NotificationVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotificationVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationVisual {}
impl ::core::fmt::Debug for NotificationVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationVisual {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationVisual;{68835b8e-aa56-4e11-86d3-5f9a6957bc5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NotificationVisual {
    type Vtable = INotificationVisual_Vtbl;
}
unsafe impl ::windows::core::Interface for NotificationVisual {
    const IID: ::windows::core::GUID = <INotificationVisual as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotificationVisual {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationVisual";
}
::windows::core::interface_hierarchy!(NotificationVisual, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NotificationVisual {}
unsafe impl ::core::marker::Sync for NotificationVisual {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ScheduledTileNotification(::windows::core::IUnknown);
impl ScheduledTileNotification {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeliveryTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTag)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledTileNotification(content: &super::super::Data::Xml::Dom::XmlDocument, deliverytime: super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledTileNotification> {
        Self::IScheduledTileNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateScheduledTileNotification)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(content), deliverytime, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IScheduledTileNotificationFactory<R, F: FnOnce(&IScheduledTileNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ScheduledTileNotification, IScheduledTileNotificationFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ScheduledTileNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScheduledTileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledTileNotification {}
impl ::core::fmt::Debug for ScheduledTileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledTileNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScheduledTileNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledTileNotification;{0abca6d5-99dc-4c78-a11c-c9e7f86d7ef7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ScheduledTileNotification {
    type Vtable = IScheduledTileNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for ScheduledTileNotification {
    const IID: ::windows::core::GUID = <IScheduledTileNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScheduledTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledTileNotification";
}
::windows::core::interface_hierarchy!(ScheduledTileNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ScheduledTileNotification {}
unsafe impl ::core::marker::Sync for ScheduledTileNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ScheduledToastNotification(::windows::core::IUnknown);
impl ScheduledToastNotification {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeliveryTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnoozeInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SnoozeInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaximumSnoozeCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaximumSnoozeCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTag)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetGroup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Group)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSuppressPopup)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuppressPopup)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotificationMirroring)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNotificationMirroring)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRemoteId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledToastNotification(content: &super::super::Data::Xml::Dom::XmlDocument, deliverytime: super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateScheduledToastNotification)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(content), deliverytime, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub fn CreateScheduledToastNotificationRecurring(content: &super::super::Data::Xml::Dom::XmlDocument, deliverytime: super::super::Foundation::DateTime, snoozeinterval: super::super::Foundation::TimeSpan, maximumsnoozecount: u32) -> ::windows::core::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateScheduledToastNotificationRecurring)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(content), deliverytime, snoozeinterval, maximumsnoozecount, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IScheduledToastNotificationFactory<R, F: FnOnce(&IScheduledToastNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ScheduledToastNotification, IScheduledToastNotificationFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ScheduledToastNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScheduledToastNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledToastNotification {}
impl ::core::fmt::Debug for ScheduledToastNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledToastNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScheduledToastNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledToastNotification;{79f577f8-0de7-48cd-9740-9b370490c838})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ScheduledToastNotification {
    type Vtable = IScheduledToastNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for ScheduledToastNotification {
    const IID: ::windows::core::GUID = <IScheduledToastNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotification";
}
::windows::core::interface_hierarchy!(ScheduledToastNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ScheduledToastNotification {}
unsafe impl ::core::marker::Sync for ScheduledToastNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ScheduledToastNotificationShowingEventArgs(::windows::core::IUnknown);
impl ScheduledToastNotificationShowingEventArgs {
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCancel)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScheduledToastNotification(&self) -> ::windows::core::Result<ScheduledToastNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScheduledToastNotification)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ScheduledToastNotificationShowingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScheduledToastNotificationShowingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledToastNotificationShowingEventArgs {}
impl ::core::fmt::Debug for ScheduledToastNotificationShowingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledToastNotificationShowingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScheduledToastNotificationShowingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs;{6173f6b4-412a-5e2c-a6ed-a0209aef9a09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ScheduledToastNotificationShowingEventArgs {
    const IID: ::windows::core::GUID = <IScheduledToastNotificationShowingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScheduledToastNotificationShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs";
}
::windows::core::interface_hierarchy!(ScheduledToastNotificationShowingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ScheduledToastNotificationShowingEventArgs {}
unsafe impl ::core::marker::Sync for ScheduledToastNotificationShowingEventArgs {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ShownTileNotification(::windows::core::IUnknown);
impl ShownTileNotification {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Arguments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ShownTileNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShownTileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShownTileNotification {}
impl ::core::fmt::Debug for ShownTileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShownTileNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShownTileNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ShownTileNotification;{342d8988-5af2-481a-a6a3-f2fdc78de88e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ShownTileNotification {
    type Vtable = IShownTileNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for ShownTileNotification {
    const IID: ::windows::core::GUID = <IShownTileNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ShownTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ShownTileNotification";
}
::windows::core::interface_hierarchy!(ShownTileNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShownTileNotification {}
unsafe impl ::core::marker::Sync for ShownTileNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct TileFlyoutNotification(::windows::core::IUnknown);
impl TileFlyoutNotification {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileFlyoutNotification(content: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<TileFlyoutNotification> {
        Self::ITileFlyoutNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileFlyoutNotification)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(content), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITileFlyoutNotificationFactory<R, F: FnOnce(&ITileFlyoutNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TileFlyoutNotification, ITileFlyoutNotificationFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for TileFlyoutNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileFlyoutNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileFlyoutNotification {}
impl ::core::fmt::Debug for TileFlyoutNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileFlyoutNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileFlyoutNotification;{9a53b261-c70c-42be-b2f3-f42aa97d34e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for TileFlyoutNotification {
    const IID: ::windows::core::GUID = <ITileFlyoutNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TileFlyoutNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutNotification";
}
::windows::core::interface_hierarchy!(TileFlyoutNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TileFlyoutNotification {}
unsafe impl ::core::marker::Sync for TileFlyoutNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
pub struct TileFlyoutUpdateManager;
impl TileFlyoutUpdateManager {
    pub fn CreateTileFlyoutUpdaterForApplication() -> ::windows::core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileFlyoutUpdaterForApplication)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForApplicationWithId(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileFlyoutUpdaterForApplicationWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForSecondaryTile(tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileFlyoutUpdaterForSecondaryTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileFlyoutTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTemplateContent)(::windows::core::Vtable::as_raw(this), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITileFlyoutUpdateManagerStatics<R, F: FnOnce(&ITileFlyoutUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TileFlyoutUpdateManager, ITileFlyoutUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for TileFlyoutUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdateManager";
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct TileFlyoutUpdater(::windows::core::IUnknown);
impl TileFlyoutUpdater {
    pub fn Update(&self, notification: &TileFlyoutNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Update)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(notification)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate(&self, tileflyoutcontent: &super::super::Foundation::Uri, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileflyoutcontent), requestedinterval).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime(&self, tileflyoutcontent: &super::super::Foundation::Uri, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdateAtTime)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileflyoutcontent), starttime, requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StopPeriodicUpdate)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Setting(&self) -> ::windows::core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Setting)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for TileFlyoutUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileFlyoutUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileFlyoutUpdater {}
impl ::core::fmt::Debug for TileFlyoutUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileFlyoutUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileFlyoutUpdater;{8d40c76a-c465-4052-a740-5c2654c1a089})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_Vtbl;
}
unsafe impl ::windows::core::Interface for TileFlyoutUpdater {
    const IID: ::windows::core::GUID = <ITileFlyoutUpdater as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TileFlyoutUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdater";
}
::windows::core::interface_hierarchy!(TileFlyoutUpdater, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct TileNotification(::windows::core::IUnknown);
impl TileNotification {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTag)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileNotification(content: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<TileNotification> {
        Self::ITileNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileNotification)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(content), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITileNotificationFactory<R, F: FnOnce(&ITileNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TileNotification, ITileNotificationFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for TileNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileNotification {}
impl ::core::fmt::Debug for TileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileNotification;{ebaec8fa-50ec-4c18-b4d0-3af02e5540ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TileNotification {
    type Vtable = ITileNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for TileNotification {
    const IID: ::windows::core::GUID = <ITileNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileNotification";
}
::windows::core::interface_hierarchy!(TileNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TileNotification {}
unsafe impl ::core::marker::Sync for TileNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
pub struct TileUpdateManager;
impl TileUpdateManager {
    pub fn CreateTileUpdaterForApplication() -> ::windows::core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileUpdaterForApplication)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTileUpdaterForApplicationWithId(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileUpdaterForApplicationWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTileUpdaterForSecondaryTile(tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileUpdaterForSecondaryTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTemplateContent)(::windows::core::Vtable::as_raw(this), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<TileUpdateManagerForUser> {
        Self::ITileUpdateManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITileUpdateManagerStatics<R, F: FnOnce(&ITileUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITileUpdateManagerStatics2<R, F: FnOnce(&ITileUpdateManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for TileUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManager";
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct TileUpdateManagerForUser(::windows::core::IUnknown);
impl TileUpdateManagerForUser {
    pub fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileUpdaterForApplication)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileUpdaterForApplicationWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateTileUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTileUpdaterForSecondaryTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for TileUpdateManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileUpdateManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileUpdateManagerForUser {}
impl ::core::fmt::Debug for TileUpdateManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileUpdateManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileUpdateManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileUpdateManagerForUser;{55141348-2ee2-4e2d-9cc1-216a20decc9f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for TileUpdateManagerForUser {
    const IID: ::windows::core::GUID = <ITileUpdateManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TileUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManagerForUser";
}
::windows::core::interface_hierarchy!(TileUpdateManagerForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TileUpdateManagerForUser {}
unsafe impl ::core::marker::Sync for TileUpdateManagerForUser {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct TileUpdater(::windows::core::IUnknown);
impl TileUpdater {
    pub fn Update(&self, notification: &TileNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Update)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(notification)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn EnableNotificationQueue(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).EnableNotificationQueue)(::windows::core::Vtable::as_raw(this), enable).ok() }
    }
    pub fn Setting(&self) -> ::windows::core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Setting)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AddToSchedule(&self, scheduledtile: &ScheduledTileNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddToSchedule)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheduledtile)).ok() }
    }
    pub fn RemoveFromSchedule(&self, scheduledtile: &ScheduledTileNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFromSchedule)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheduledtile)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledTileNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetScheduledTileNotifications)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdate(&self, tilecontent: &super::super::Foundation::Uri, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tilecontent), requestedinterval).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPeriodicUpdateAtTime(&self, tilecontent: &super::super::Foundation::Uri, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdateAtTime)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tilecontent), starttime, requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StopPeriodicUpdate)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartPeriodicUpdateBatch<P0, E0>(&self, tilecontents: P0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdateBatch)(::windows::core::Vtable::as_raw(this), tilecontents.try_into().map_err(|e| e.into())?.abi(), requestedinterval).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartPeriodicUpdateBatchAtTime<P0, E0>(&self, tilecontents: P0, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartPeriodicUpdateBatchAtTime)(::windows::core::Vtable::as_raw(this), tilecontents.try_into().map_err(|e| e.into())?.abi(), starttime, requestedinterval).ok() }
    }
    pub fn EnableNotificationQueueForSquare150x150(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).EnableNotificationQueueForSquare150x150)(::windows::core::Vtable::as_raw(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForWide310x150(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).EnableNotificationQueueForWide310x150)(::windows::core::Vtable::as_raw(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForSquare310x310(&self, enable: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).EnableNotificationQueueForSquare310x310)(::windows::core::Vtable::as_raw(this), enable).ok() }
    }
}
impl ::core::clone::Clone for TileUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileUpdater {}
impl ::core::fmt::Debug for TileUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileUpdater;{0942a48b-1d91-44ec-9243-c1e821c29a20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TileUpdater {
    type Vtable = ITileUpdater_Vtbl;
}
unsafe impl ::windows::core::Interface for TileUpdater {
    const IID: ::windows::core::GUID = <ITileUpdater as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TileUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdater";
}
::windows::core::interface_hierarchy!(TileUpdater, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TileUpdater {}
unsafe impl ::core::marker::Sync for TileUpdater {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastActivatedEventArgs(::windows::core::IUnknown);
impl ToastActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Arguments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IToastActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ToastActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastActivatedEventArgs {}
impl ::core::fmt::Debug for ToastActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastActivatedEventArgs;{e3bf92f3-c197-436f-8265-0625824f8dac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastActivatedEventArgs {
    const IID: ::windows::core::GUID = <IToastActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastActivatedEventArgs";
}
::windows::core::interface_hierarchy!(ToastActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastCollection(::windows::core::IUnknown);
impl ToastCollection {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LaunchArgs(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchArgs)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLaunchArgs(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLaunchArgs)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Icon)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIcon(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIcon)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstance(collectionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, launchargs: &::windows::core::HSTRING, iconuri: &super::super::Foundation::Uri) -> ::windows::core::Result<ToastCollection> {
        Self::IToastCollectionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(collectionid), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(launchargs), ::core::mem::transmute_copy(iconuri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastCollectionFactory<R, F: FnOnce(&IToastCollectionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ToastCollection, IToastCollectionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ToastCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastCollection {}
impl ::core::fmt::Debug for ToastCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastCollection;{0a8bc3b0-e0be-4858-bc2a-89dfe0b32863})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastCollection {
    type Vtable = IToastCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastCollection {
    const IID: ::windows::core::GUID = <IToastCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastCollection {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollection";
}
::windows::core::interface_hierarchy!(ToastCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ToastCollection {}
unsafe impl ::core::marker::Sync for ToastCollection {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastCollectionManager(::windows::core::IUnknown);
impl ToastCollectionManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveToastCollectionAsync(&self, collection: &ToastCollection) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveToastCollectionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(collection), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ToastCollection>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllToastCollectionsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetToastCollectionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(collectionid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveToastCollectionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(collectionid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveAllToastCollectionsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ToastCollectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastCollectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastCollectionManager {}
impl ::core::fmt::Debug for ToastCollectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastCollectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastCollectionManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastCollectionManager;{2a1821fe-179d-49bc-b79d-a527920d3665})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastCollectionManager {
    type Vtable = IToastCollectionManager_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastCollectionManager {
    const IID: ::windows::core::GUID = <IToastCollectionManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastCollectionManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollectionManager";
}
::windows::core::interface_hierarchy!(ToastCollectionManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ToastCollectionManager {}
unsafe impl ::core::marker::Sync for ToastCollectionManager {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastDismissedEventArgs(::windows::core::IUnknown);
impl ToastDismissedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<ToastDismissalReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ToastDismissedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastDismissedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastDismissedEventArgs {}
impl ::core::fmt::Debug for ToastDismissedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastDismissedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastDismissedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastDismissedEventArgs;{3f89d935-d9cb-4538-a0f0-ffe7659938f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastDismissedEventArgs {
    const IID: ::windows::core::GUID = <IToastDismissedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastDismissedEventArgs";
}
::windows::core::interface_hierarchy!(ToastDismissedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ToastDismissedEventArgs {}
unsafe impl ::core::marker::Sync for ToastDismissedEventArgs {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastFailedEventArgs(::windows::core::IUnknown);
impl ToastFailedEventArgs {
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ToastFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastFailedEventArgs {}
impl ::core::fmt::Debug for ToastFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastFailedEventArgs;{35176862-cfd4-44f8-ad64-f500fd896c3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastFailedEventArgs {
    const IID: ::windows::core::GUID = <IToastFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastFailedEventArgs";
}
::windows::core::interface_hierarchy!(ToastFailedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ToastFailedEventArgs {}
unsafe impl ::core::marker::Sync for ToastFailedEventArgs {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastNotification(::windows::core::IUnknown);
impl ToastNotification {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Dismissed(&self, handler: &super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dismissed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDismissed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Activated(&self, handler: &super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Activated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveActivated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Failed(&self, handler: &super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Failed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFailed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTag)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetGroup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Group)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSuppressPopup)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuppressPopup)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotificationMirroring)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNotificationMirroring)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRemoteId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Data(&self) -> ::windows::core::Result<NotificationData> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &NotificationData) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Priority(&self) -> ::windows::core::Result<ToastNotificationPriority> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Priority)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPriority(&self, value: ToastNotificationPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPriority)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ExpiresOnReboot(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpiresOnReboot)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotification6>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpiresOnReboot)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateToastNotification(content: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<ToastNotification> {
        Self::IToastNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateToastNotification)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(content), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationFactory<R, F: FnOnce(&IToastNotificationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ToastNotification, IToastNotificationFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ToastNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotification {}
impl ::core::fmt::Debug for ToastNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotification;{997e2675-059e-4e60-8b06-1760917c8b80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastNotification {
    type Vtable = IToastNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastNotification {
    const IID: ::windows::core::GUID = <IToastNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotification";
}
::windows::core::interface_hierarchy!(ToastNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ToastNotification {}
unsafe impl ::core::marker::Sync for ToastNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastNotificationActionTriggerDetail(::windows::core::IUnknown);
impl ToastNotificationActionTriggerDetail {
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Argument)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationActionTriggerDetail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActionTriggerDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActionTriggerDetail {}
impl ::core::fmt::Debug for ToastNotificationActionTriggerDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationActionTriggerDetail").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationActionTriggerDetail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationActionTriggerDetail;{9445135a-38f3-42f6-96aa-7955b0f03da2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastNotificationActionTriggerDetail {
    const IID: ::windows::core::GUID = <IToastNotificationActionTriggerDetail as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationActionTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationActionTriggerDetail";
}
::windows::core::interface_hierarchy!(ToastNotificationActionTriggerDetail, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastNotificationHistory(::windows::core::IUnknown);
impl ToastNotificationHistory {
    pub fn RemoveGroup(&self, group: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveGroup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(group)).ok() }
    }
    pub fn RemoveGroupWithId(&self, group: &::windows::core::HSTRING, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveGroupWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(group), ::core::mem::transmute_copy(applicationid)).ok() }
    }
    pub fn RemoveGroupedTagWithId(&self, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveGroupedTagWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tag), ::core::mem::transmute_copy(group), ::core::mem::transmute_copy(applicationid)).ok() }
    }
    pub fn RemoveGroupedTag(&self, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveGroupedTag)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tag), ::core::mem::transmute_copy(group)).ok() }
    }
    pub fn Remove(&self, tag: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tag)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn ClearWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ClearWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistory(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHistory)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistoryWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHistoryWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationHistory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistory {}
impl ::core::fmt::Debug for ToastNotificationHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationHistory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationHistory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationHistory;{5caddc63-01d3-4c97-986f-0533483fee14})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastNotificationHistory {
    type Vtable = IToastNotificationHistory_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastNotificationHistory {
    const IID: ::windows::core::GUID = <IToastNotificationHistory as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationHistory {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistory";
}
::windows::core::interface_hierarchy!(ToastNotificationHistory, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTriggerDetail(::windows::core::IUnknown);
impl ToastNotificationHistoryChangedTriggerDetail {
    pub fn ChangeType(&self) -> ::windows::core::Result<ToastHistoryChangedType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CollectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotificationHistoryChangedTriggerDetail2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CollectionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationHistoryChangedTriggerDetail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistoryChangedTriggerDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistoryChangedTriggerDetail {}
impl ::core::fmt::Debug for ToastNotificationHistoryChangedTriggerDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationHistoryChangedTriggerDetail").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationHistoryChangedTriggerDetail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail;{db037ffa-0068-412c-9c83-267c37f65670})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastNotificationHistoryChangedTriggerDetail {
    const IID: ::windows::core::GUID = <IToastNotificationHistoryChangedTriggerDetail as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationHistoryChangedTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail";
}
::windows::core::interface_hierarchy!(ToastNotificationHistoryChangedTriggerDetail, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Notifications\"`*"]
pub struct ToastNotificationManager;
impl ToastNotificationManager {
    pub fn CreateToastNotifier() -> ::windows::core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateToastNotifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateToastNotifierWithId(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateToastNotifierWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: ToastTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTemplateContent)(::windows::core::Vtable::as_raw(this), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn History() -> ::windows::core::Result<ToastNotificationHistory> {
        Self::IToastNotificationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).History)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ConfigureNotificationMirroring(value: NotificationMirroring) -> ::windows::core::Result<()> {
        Self::IToastNotificationManagerStatics4(|this| unsafe { (::windows::core::Vtable::vtable(this).ConfigureNotificationMirroring)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    pub fn GetDefault() -> ::windows::core::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics<R, F: FnOnce(&IToastNotificationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics2<R, F: FnOnce(&IToastNotificationManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics4<R, F: FnOnce(&IToastNotificationManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics5<R, F: FnOnce(&IToastNotificationManagerStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics5> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ToastNotificationManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManager";
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastNotificationManagerForUser(::windows::core::IUnknown);
impl ToastNotificationManagerForUser {
    pub fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateToastNotifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateToastNotifierWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateToastNotifierWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn History(&self) -> ::windows::core::Result<ToastNotificationHistory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).History)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetToastNotifierForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotifier>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetToastNotifierForToastCollectionIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(collectionid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetHistoryForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHistoryForToastCollectionIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(collectionid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetToastCollectionManager(&self) -> ::windows::core::Result<ToastCollectionManager> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetToastCollectionManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetToastCollectionManagerWithAppId(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastCollectionManager> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetToastCollectionManagerWithAppId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NotificationMode(&self) -> ::windows::core::Result<ToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotificationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotificationModeChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ToastNotificationManagerForUser, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotificationModeChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotificationModeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotificationManagerForUser3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNotificationModeChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for ToastNotificationManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationManagerForUser {}
impl ::core::fmt::Debug for ToastNotificationManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationManagerForUser;{79ab57f6-43fe-487b-8a7f-99567200ae94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastNotificationManagerForUser {
    const IID: ::windows::core::GUID = <IToastNotificationManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManagerForUser";
}
::windows::core::interface_hierarchy!(ToastNotificationManagerForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ToastNotificationManagerForUser {}
unsafe impl ::core::marker::Sync for ToastNotificationManagerForUser {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastNotifier(::windows::core::IUnknown);
impl ToastNotifier {
    pub fn Show(&self, notification: &ToastNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Show)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(notification)).ok() }
    }
    pub fn Hide(&self, notification: &ToastNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Hide)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(notification)).ok() }
    }
    pub fn Setting(&self) -> ::windows::core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Setting)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AddToSchedule(&self, scheduledtoast: &ScheduledToastNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddToSchedule)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheduledtoast)).ok() }
    }
    pub fn RemoveFromSchedule(&self, scheduledtoast: &ScheduledToastNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFromSchedule)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheduledtoast)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledToastNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetScheduledToastNotifications)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UpdateWithTagAndGroup(&self, data: &NotificationData, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationUpdateResult> {
        let this = &::windows::core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateWithTagAndGroup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), ::core::mem::transmute_copy(tag), ::core::mem::transmute_copy(group), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UpdateWithTag(&self, data: &NotificationData, tag: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationUpdateResult> {
        let this = &::windows::core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateWithTag)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), ::core::mem::transmute_copy(tag), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScheduledToastNotificationShowing(&self, handler: &super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScheduledToastNotificationShowing)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScheduledToastNotificationShowing(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveScheduledToastNotificationShowing)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for ToastNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotifier {}
impl ::core::fmt::Debug for ToastNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotifier;{75927b93-03f3-41ec-91d3-6e5bac1b38e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ToastNotifier {
    type Vtable = IToastNotifier_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastNotifier {
    const IID: ::windows::core::GUID = <IToastNotifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotifier";
}
::windows::core::interface_hierarchy!(ToastNotifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ToastNotifier {}
unsafe impl ::core::marker::Sync for ToastNotifier {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct UserNotification(::windows::core::IUnknown);
impl UserNotification {
    pub fn Notification(&self) -> ::windows::core::Result<Notification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Notification)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows::core::Result<super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotification {}
impl ::core::fmt::Debug for UserNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.UserNotification;{adf7e52f-4e53-42d5-9c33-eb5ea515b23e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserNotification {
    type Vtable = IUserNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for UserNotification {
    const IID: ::windows::core::GUID = <IUserNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserNotification {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotification";
}
::windows::core::interface_hierarchy!(UserNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserNotification {}
unsafe impl ::core::marker::Sync for UserNotification {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct UserNotificationChangedEventArgs(::windows::core::IUnknown);
impl UserNotificationChangedEventArgs {
    pub fn ChangeKind(&self) -> ::windows::core::Result<UserNotificationChangedKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UserNotificationId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserNotificationId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserNotificationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserNotificationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotificationChangedEventArgs {}
impl ::core::fmt::Debug for UserNotificationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotificationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.UserNotificationChangedEventArgs;{b6bd6839-79cf-4b25-82c0-0ce1eef81f8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserNotificationChangedEventArgs {
    const IID: ::windows::core::GUID = <IUserNotificationChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserNotificationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotificationChangedEventArgs";
}
::windows::core::interface_hierarchy!(UserNotificationChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserNotificationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserNotificationChangedEventArgs {}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: Self = Self(0i32);
}
impl ::core::marker::Copy for AdaptiveNotificationContentKind {}
impl ::core::clone::Clone for AdaptiveNotificationContentKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveNotificationContentKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdaptiveNotificationContentKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveNotificationContentKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveNotificationContentKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveNotificationContentKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.AdaptiveNotificationContentKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: Self = Self(0i32);
    pub const BadgeNumber: Self = Self(1i32);
}
impl ::core::marker::Copy for BadgeTemplateType {}
impl ::core::clone::Clone for BadgeTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BadgeTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BadgeTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for BadgeTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BadgeTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.BadgeTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: Self = Self(0u32);
    pub const Toast: Self = Self(1u32);
}
impl ::core::marker::Copy for NotificationKinds {}
impl ::core::clone::Clone for NotificationKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NotificationKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NotificationKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NotificationKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NotificationKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NotificationKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NotificationKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationKinds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationKinds;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for NotificationMirroring {}
impl ::core::clone::Clone for NotificationMirroring {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationMirroring {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NotificationMirroring {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationMirroring {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationMirroring").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationMirroring {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationMirroring;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
}
impl ::core::marker::Copy for NotificationSetting {}
impl ::core::clone::Clone for NotificationSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationSetting {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NotificationSetting {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationSetting;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const NotificationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for NotificationUpdateResult {}
impl ::core::clone::Clone for NotificationUpdateResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationUpdateResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NotificationUpdateResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationUpdateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotificationUpdateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationUpdateResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const SixHours: Self = Self(2i32);
    pub const TwelveHours: Self = Self(3i32);
    pub const Daily: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicUpdateRecurrence {}
impl ::core::clone::Clone for PeriodicUpdateRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeriodicUpdateRecurrence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PeriodicUpdateRecurrence {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeriodicUpdateRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicUpdateRecurrence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeriodicUpdateRecurrence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.PeriodicUpdateRecurrence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: Self = Self(0i32);
}
impl ::core::marker::Copy for TileFlyoutTemplateType {}
impl ::core::clone::Clone for TileFlyoutTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TileFlyoutTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TileFlyoutTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for TileFlyoutTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileFlyoutTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileFlyoutTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: Self = Self(0i32);
    pub const TileSquareBlock: Self = Self(1i32);
    pub const TileSquareText01: Self = Self(2i32);
    pub const TileSquareText02: Self = Self(3i32);
    pub const TileSquareText03: Self = Self(4i32);
    pub const TileSquareText04: Self = Self(5i32);
    pub const TileSquarePeekImageAndText01: Self = Self(6i32);
    pub const TileSquarePeekImageAndText02: Self = Self(7i32);
    pub const TileSquarePeekImageAndText03: Self = Self(8i32);
    pub const TileSquarePeekImageAndText04: Self = Self(9i32);
    pub const TileWideImage: Self = Self(10i32);
    pub const TileWideImageCollection: Self = Self(11i32);
    pub const TileWideImageAndText01: Self = Self(12i32);
    pub const TileWideImageAndText02: Self = Self(13i32);
    pub const TileWideBlockAndText01: Self = Self(14i32);
    pub const TileWideBlockAndText02: Self = Self(15i32);
    pub const TileWidePeekImageCollection01: Self = Self(16i32);
    pub const TileWidePeekImageCollection02: Self = Self(17i32);
    pub const TileWidePeekImageCollection03: Self = Self(18i32);
    pub const TileWidePeekImageCollection04: Self = Self(19i32);
    pub const TileWidePeekImageCollection05: Self = Self(20i32);
    pub const TileWidePeekImageCollection06: Self = Self(21i32);
    pub const TileWidePeekImageAndText01: Self = Self(22i32);
    pub const TileWidePeekImageAndText02: Self = Self(23i32);
    pub const TileWidePeekImage01: Self = Self(24i32);
    pub const TileWidePeekImage02: Self = Self(25i32);
    pub const TileWidePeekImage03: Self = Self(26i32);
    pub const TileWidePeekImage04: Self = Self(27i32);
    pub const TileWidePeekImage05: Self = Self(28i32);
    pub const TileWidePeekImage06: Self = Self(29i32);
    pub const TileWideSmallImageAndText01: Self = Self(30i32);
    pub const TileWideSmallImageAndText02: Self = Self(31i32);
    pub const TileWideSmallImageAndText03: Self = Self(32i32);
    pub const TileWideSmallImageAndText04: Self = Self(33i32);
    pub const TileWideSmallImageAndText05: Self = Self(34i32);
    pub const TileWideText01: Self = Self(35i32);
    pub const TileWideText02: Self = Self(36i32);
    pub const TileWideText03: Self = Self(37i32);
    pub const TileWideText04: Self = Self(38i32);
    pub const TileWideText05: Self = Self(39i32);
    pub const TileWideText06: Self = Self(40i32);
    pub const TileWideText07: Self = Self(41i32);
    pub const TileWideText08: Self = Self(42i32);
    pub const TileWideText09: Self = Self(43i32);
    pub const TileWideText10: Self = Self(44i32);
    pub const TileWideText11: Self = Self(45i32);
    pub const TileSquare150x150Image: Self = Self(0i32);
    pub const TileSquare150x150Block: Self = Self(1i32);
    pub const TileSquare150x150Text01: Self = Self(2i32);
    pub const TileSquare150x150Text02: Self = Self(3i32);
    pub const TileSquare150x150Text03: Self = Self(4i32);
    pub const TileSquare150x150Text04: Self = Self(5i32);
    pub const TileSquare150x150PeekImageAndText01: Self = Self(6i32);
    pub const TileSquare150x150PeekImageAndText02: Self = Self(7i32);
    pub const TileSquare150x150PeekImageAndText03: Self = Self(8i32);
    pub const TileSquare150x150PeekImageAndText04: Self = Self(9i32);
    pub const TileWide310x150Image: Self = Self(10i32);
    pub const TileWide310x150ImageCollection: Self = Self(11i32);
    pub const TileWide310x150ImageAndText01: Self = Self(12i32);
    pub const TileWide310x150ImageAndText02: Self = Self(13i32);
    pub const TileWide310x150BlockAndText01: Self = Self(14i32);
    pub const TileWide310x150BlockAndText02: Self = Self(15i32);
    pub const TileWide310x150PeekImageCollection01: Self = Self(16i32);
    pub const TileWide310x150PeekImageCollection02: Self = Self(17i32);
    pub const TileWide310x150PeekImageCollection03: Self = Self(18i32);
    pub const TileWide310x150PeekImageCollection04: Self = Self(19i32);
    pub const TileWide310x150PeekImageCollection05: Self = Self(20i32);
    pub const TileWide310x150PeekImageCollection06: Self = Self(21i32);
    pub const TileWide310x150PeekImageAndText01: Self = Self(22i32);
    pub const TileWide310x150PeekImageAndText02: Self = Self(23i32);
    pub const TileWide310x150PeekImage01: Self = Self(24i32);
    pub const TileWide310x150PeekImage02: Self = Self(25i32);
    pub const TileWide310x150PeekImage03: Self = Self(26i32);
    pub const TileWide310x150PeekImage04: Self = Self(27i32);
    pub const TileWide310x150PeekImage05: Self = Self(28i32);
    pub const TileWide310x150PeekImage06: Self = Self(29i32);
    pub const TileWide310x150SmallImageAndText01: Self = Self(30i32);
    pub const TileWide310x150SmallImageAndText02: Self = Self(31i32);
    pub const TileWide310x150SmallImageAndText03: Self = Self(32i32);
    pub const TileWide310x150SmallImageAndText04: Self = Self(33i32);
    pub const TileWide310x150SmallImageAndText05: Self = Self(34i32);
    pub const TileWide310x150Text01: Self = Self(35i32);
    pub const TileWide310x150Text02: Self = Self(36i32);
    pub const TileWide310x150Text03: Self = Self(37i32);
    pub const TileWide310x150Text04: Self = Self(38i32);
    pub const TileWide310x150Text05: Self = Self(39i32);
    pub const TileWide310x150Text06: Self = Self(40i32);
    pub const TileWide310x150Text07: Self = Self(41i32);
    pub const TileWide310x150Text08: Self = Self(42i32);
    pub const TileWide310x150Text09: Self = Self(43i32);
    pub const TileWide310x150Text10: Self = Self(44i32);
    pub const TileWide310x150Text11: Self = Self(45i32);
    pub const TileSquare310x310BlockAndText01: Self = Self(46i32);
    pub const TileSquare310x310BlockAndText02: Self = Self(47i32);
    pub const TileSquare310x310Image: Self = Self(48i32);
    pub const TileSquare310x310ImageAndText01: Self = Self(49i32);
    pub const TileSquare310x310ImageAndText02: Self = Self(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: Self = Self(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: Self = Self(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: Self = Self(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: Self = Self(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: Self = Self(55i32);
    pub const TileSquare310x310ImageCollection: Self = Self(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: Self = Self(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: Self = Self(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: Self = Self(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: Self = Self(60i32);
    pub const TileSquare310x310Text01: Self = Self(61i32);
    pub const TileSquare310x310Text02: Self = Self(62i32);
    pub const TileSquare310x310Text03: Self = Self(63i32);
    pub const TileSquare310x310Text04: Self = Self(64i32);
    pub const TileSquare310x310Text05: Self = Self(65i32);
    pub const TileSquare310x310Text06: Self = Self(66i32);
    pub const TileSquare310x310Text07: Self = Self(67i32);
    pub const TileSquare310x310Text08: Self = Self(68i32);
    pub const TileSquare310x310TextList01: Self = Self(69i32);
    pub const TileSquare310x310TextList02: Self = Self(70i32);
    pub const TileSquare310x310TextList03: Self = Self(71i32);
    pub const TileSquare310x310SmallImageAndText01: Self = Self(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: Self = Self(73i32);
    pub const TileSquare310x310Text09: Self = Self(74i32);
    pub const TileSquare71x71IconWithBadge: Self = Self(75i32);
    pub const TileSquare150x150IconWithBadge: Self = Self(76i32);
    pub const TileWide310x150IconWithBadgeAndText: Self = Self(77i32);
    pub const TileSquare71x71Image: Self = Self(78i32);
    pub const TileTall150x310Image: Self = Self(79i32);
}
impl ::core::marker::Copy for TileTemplateType {}
impl ::core::clone::Clone for TileTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TileTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TileTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for TileTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: Self = Self(0i32);
    pub const ApplicationHidden: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
}
impl ::core::marker::Copy for ToastDismissalReason {}
impl ::core::clone::Clone for ToastDismissalReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastDismissalReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ToastDismissalReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastDismissalReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastDismissalReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastDismissalReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastDismissalReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Expired: Self = Self(2i32);
    pub const Added: Self = Self(3i32);
}
impl ::core::marker::Copy for ToastHistoryChangedType {}
impl ::core::clone::Clone for ToastHistoryChangedType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastHistoryChangedType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ToastHistoryChangedType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastHistoryChangedType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastHistoryChangedType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastHistoryChangedType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastHistoryChangedType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ToastNotificationMode(pub i32);
impl ToastNotificationMode {
    pub const Unrestricted: Self = Self(0i32);
    pub const PriorityOnly: Self = Self(1i32);
    pub const AlarmsOnly: Self = Self(2i32);
}
impl ::core::marker::Copy for ToastNotificationMode {}
impl ::core::clone::Clone for ToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastNotificationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ToastNotificationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastNotificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastNotificationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for ToastNotificationPriority {}
impl ::core::clone::Clone for ToastNotificationPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastNotificationPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ToastNotificationPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastNotificationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastNotificationPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: Self = Self(0i32);
    pub const ToastImageAndText02: Self = Self(1i32);
    pub const ToastImageAndText03: Self = Self(2i32);
    pub const ToastImageAndText04: Self = Self(3i32);
    pub const ToastText01: Self = Self(4i32);
    pub const ToastText02: Self = Self(5i32);
    pub const ToastText03: Self = Self(6i32);
    pub const ToastText04: Self = Self(7i32);
}
impl ::core::marker::Copy for ToastTemplateType {}
impl ::core::clone::Clone for ToastTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ToastTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastTemplateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
}
impl ::core::marker::Copy for UserNotificationChangedKind {}
impl ::core::clone::Clone for UserNotificationChangedKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserNotificationChangedKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserNotificationChangedKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserNotificationChangedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationChangedKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotificationChangedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.UserNotificationChangedKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
