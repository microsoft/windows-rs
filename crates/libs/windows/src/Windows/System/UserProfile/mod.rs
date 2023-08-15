#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_Vtbl;
}
impl ::core::clone::Clone for IAdvertisingManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvertisingManagerForUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x928bf3d0_cf7c_4ab0_a7dc_6dc5bcd44252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvertisingManagerStatics {
    type Vtable = IAdvertisingManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAdvertisingManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvertisingManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadd3468c_a273_48cb_b346_3544522d5581);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvertisingManagerStatics2 {
    type Vtable = IAdvertisingManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IAdvertisingManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvertisingManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd0947af_1a6d_46b0_95bc_f3f9d6beb9fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAssignedAccessSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_Vtbl;
}
impl ::core::clone::Clone for IAssignedAccessSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAssignedAccessSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bc57f1c_e971_5757_b8e0_512f8b8c46d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSingleAppKioskMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAssignedAccessSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAssignedAccessSettingsStatics {
    type Vtable = IAssignedAccessSettingsStatics_Vtbl;
}
impl ::core::clone::Clone for IAssignedAccessSettingsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAssignedAccessSettingsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34a81d0d_8a29_5ef3_a7be_618e6ac3bd01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticsSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_Vtbl;
}
impl ::core::clone::Clone for IDiagnosticsSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDiagnosticsSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5e9eccd_2711_44e0_973c_491d78048d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanUseDiagnosticsToTailorExperiences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticsSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticsSettingsStatics {
    type Vtable = IDiagnosticsSettingsStatics_Vtbl;
}
impl ::core::clone::Clone for IDiagnosticsSettingsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDiagnosticsSettingsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72d2e80f_5390_4793_990b_3ccc7d6ac9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFirstSignInSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFirstSignInSettings {
    type Vtable = IFirstSignInSettings_Vtbl;
}
impl ::core::clone::Clone for IFirstSignInSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFirstSignInSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e945153_3a5e_452e_a601_f5baad2a4870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFirstSignInSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFirstSignInSettingsStatics {
    type Vtable = IFirstSignInSettingsStatics_Vtbl;
}
impl ::core::clone::Clone for IFirstSignInSettingsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFirstSignInSettingsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ce18f0f_1c41_4ea0_b7a2_6f0c1c7e8438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_Vtbl;
}
impl ::core::clone::Clone for IGlobalizationPreferencesForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGlobalizationPreferencesForUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x150f0795_4f6e_40ba_a010_e27d81bda7f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesForUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Calendars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Calendars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clocks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Currencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Currencies: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesStatics {
    type Vtable = IGlobalizationPreferencesStatics_Vtbl;
}
impl ::core::clone::Clone for IGlobalizationPreferencesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGlobalizationPreferencesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01bf4326_ed37_4e96_b0e9_c1340d1ea158);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Calendars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Calendars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clocks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Currencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Currencies: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesStatics2 {
    type Vtable = IGlobalizationPreferencesStatics2_Vtbl;
}
impl ::core::clone::Clone for IGlobalizationPreferencesStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGlobalizationPreferencesStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcce85f1_4300_4cd0_9cac_1a8e7b7e18f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TrySetHomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySetLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetags: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySetLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesStatics3 {
    type Vtable = IGlobalizationPreferencesStatics3_Vtbl;
}
impl ::core::clone::Clone for IGlobalizationPreferencesStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGlobalizationPreferencesStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e059733_35f5_40d8_b9e8_aef3ef856fce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenImageFeedStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILockScreenImageFeedStatics {
    type Vtable = ILockScreenImageFeedStatics_Vtbl;
}
impl ::core::clone::Clone for ILockScreenImageFeedStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILockScreenImageFeedStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c0d73f6_03a9_41a6_9b01_495251ff51d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenImageFeedStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSetImageFeedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syndicationfeeduri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetImageFeedAsync: usize,
    pub TryRemoveImageFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILockScreenStatics {
    type Vtable = ILockScreenStatics_Vtbl;
}
impl ::core::clone::Clone for ILockScreenStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILockScreenStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ee9d3ad_b607_40ae_b426_7631d9821269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OriginalImageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalImageFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetImageStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SetImageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SetImageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetImageStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetImageStreamAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IUserInformationStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IUserInformationStatics {
    type Vtable = IUserInformationStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IUserInformationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IUserInformationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77f3a910_48fa_489c_934e_2ae85ba8f772);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IUserInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AccountPictureChangeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AccountPictureChangeEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub NameAccessAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NameAccessAllowed: usize,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub GetAccountPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: AccountPictureKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    GetAccountPicture: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub SetAccountPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    SetAccountPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub SetAccountPicturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smallimage: *mut ::core::ffi::c_void, largeimage: *mut ::core::ffi::c_void, video: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    SetAccountPicturesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub SetAccountPictureFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    SetAccountPictureFromStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub SetAccountPicturesFromStreamsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smallimage: *mut ::core::ffi::c_void, largeimage: *mut ::core::ffi::c_void, video: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    SetAccountPicturesFromStreamsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AccountPictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AccountPictureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAccountPictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAccountPictureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDisplayNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDisplayNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetFirstNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetFirstNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLastNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLastNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetPrincipalNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetPrincipalNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetSessionInitiationProtocolUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetSessionInitiationProtocolUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDomainNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDomainNameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_Vtbl;
}
impl ::core::clone::Clone for IUserProfilePersonalizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserProfilePersonalizationSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ceddab4_7998_46d5_8dd3_184f1c5f9ab9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TrySetLockScreenImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TrySetLockScreenImageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TrySetWallpaperImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TrySetWallpaperImageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserProfilePersonalizationSettingsStatics {
    type Vtable = IUserProfilePersonalizationSettingsStatics_Vtbl;
}
impl ::core::clone::Clone for IUserProfilePersonalizationSettingsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserProfilePersonalizationSettingsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91acb841_5037_454b_9883_bb772d08dd16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
pub struct AdvertisingManager;
impl AdvertisingManager {
    pub fn AdvertisingId() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAdvertisingManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisingId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<AdvertisingManagerForUser>
    where
        P0: ::windows_core::IntoParam<super::User>,
    {
        Self::IAdvertisingManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAdvertisingManagerStatics<R, F: FnOnce(&IAdvertisingManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAdvertisingManagerStatics2<R, F: FnOnce(&IAdvertisingManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AdvertisingManager {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManager";
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct AdvertisingManagerForUser(::windows_core::IUnknown);
impl AdvertisingManagerForUser {
    pub fn AdvertisingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisingId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AdvertisingManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvertisingManagerForUser {}
impl ::core::fmt::Debug for AdvertisingManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvertisingManagerForUser").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AdvertisingManagerForUser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AdvertisingManagerForUser;{928bf3d0-cf7c-4ab0-a7dc-6dc5bcd44252})");
}
impl ::core::clone::Clone for AdvertisingManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AdvertisingManagerForUser {
    const IID: ::windows_core::GUID = <IAdvertisingManagerForUser as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AdvertisingManagerForUser {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManagerForUser";
}
::windows_core::imp::interface_hierarchy!(AdvertisingManagerForUser, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AdvertisingManagerForUser {}
unsafe impl ::core::marker::Sync for AdvertisingManagerForUser {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct AssignedAccessSettings(::windows_core::IUnknown);
impl AssignedAccessSettings {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSingleAppKioskMode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSingleAppKioskMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<AssignedAccessSettings>
    where
        P0: ::windows_core::IntoParam<super::User>,
    {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAssignedAccessSettingsStatics<R, F: FnOnce(&IAssignedAccessSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AssignedAccessSettings, IAssignedAccessSettingsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AssignedAccessSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AssignedAccessSettings {}
impl ::core::fmt::Debug for AssignedAccessSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AssignedAccessSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AssignedAccessSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AssignedAccessSettings;{1bc57f1c-e971-5757-b8e0-512f8b8c46d2})");
}
impl ::core::clone::Clone for AssignedAccessSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AssignedAccessSettings {
    const IID: ::windows_core::GUID = <IAssignedAccessSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AssignedAccessSettings {
    const NAME: &'static str = "Windows.System.UserProfile.AssignedAccessSettings";
}
::windows_core::imp::interface_hierarchy!(AssignedAccessSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AssignedAccessSettings {}
unsafe impl ::core::marker::Sync for AssignedAccessSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct DiagnosticsSettings(::windows_core::IUnknown);
impl DiagnosticsSettings {
    pub fn CanUseDiagnosticsToTailorExperiences(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanUseDiagnosticsToTailorExperiences)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<DiagnosticsSettings>
    where
        P0: ::windows_core::IntoParam<super::User>,
    {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDiagnosticsSettingsStatics<R, F: FnOnce(&IDiagnosticsSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DiagnosticsSettings, IDiagnosticsSettingsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DiagnosticsSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiagnosticsSettings {}
impl ::core::fmt::Debug for DiagnosticsSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticsSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DiagnosticsSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.DiagnosticsSettings;{e5e9eccd-2711-44e0-973c-491d78048d24})");
}
impl ::core::clone::Clone for DiagnosticsSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DiagnosticsSettings {
    const IID: ::windows_core::GUID = <IDiagnosticsSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DiagnosticsSettings {
    const NAME: &'static str = "Windows.System.UserProfile.DiagnosticsSettings";
}
::windows_core::imp::interface_hierarchy!(DiagnosticsSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DiagnosticsSettings {}
unsafe impl ::core::marker::Sync for DiagnosticsSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct FirstSignInSettings(::windows_core::IUnknown);
impl FirstSignInSettings {
    pub fn GetDefault() -> ::windows_core::Result<FirstSignInSettings> {
        Self::IFirstSignInSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>, second: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc(hidden)]
    pub fn IFirstSignInSettingsStatics<R, F: FnOnce(&IFirstSignInSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FirstSignInSettings, IFirstSignInSettingsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for FirstSignInSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FirstSignInSettings {}
impl ::core::fmt::Debug for FirstSignInSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FirstSignInSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FirstSignInSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.FirstSignInSettings;{3e945153-3a5e-452e-a601-f5baad2a4870})");
}
impl ::core::clone::Clone for FirstSignInSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FirstSignInSettings {
    type Vtable = IFirstSignInSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FirstSignInSettings {
    const IID: ::windows_core::GUID = <IFirstSignInSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FirstSignInSettings {
    const NAME: &'static str = "Windows.System.UserProfile.FirstSignInSettings";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FirstSignInSettings {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FirstSignInSettings {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(FirstSignInSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for FirstSignInSettings {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> for FirstSignInSettings {}
unsafe impl ::core::marker::Send for FirstSignInSettings {}
unsafe impl ::core::marker::Sync for FirstSignInSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
pub struct GlobalizationPreferences;
impl GlobalizationPreferences {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Calendars() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Calendars)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clocks() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clocks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Currencies() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Currencies)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HomeGeographicRegion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HomeGeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn WeekStartsOn() -> ::windows_core::Result<super::super::Globalization::DayOfWeek> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WeekStartsOn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TrySetHomeGeographicRegion(region: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetHomeGeographicRegion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(region), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrySetLanguages<P0>(languagetags: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetLanguages)(::windows_core::Interface::as_raw(this), languagetags.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<GlobalizationPreferencesForUser>
    where
        P0: ::windows_core::IntoParam<super::User>,
    {
        Self::IGlobalizationPreferencesStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGlobalizationPreferencesStatics<R, F: FnOnce(&IGlobalizationPreferencesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGlobalizationPreferencesStatics2<R, F: FnOnce(&IGlobalizationPreferencesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGlobalizationPreferencesStatics3<R, F: FnOnce(&IGlobalizationPreferencesStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for GlobalizationPreferences {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferences";
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct GlobalizationPreferencesForUser(::windows_core::IUnknown);
impl GlobalizationPreferencesForUser {
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Calendars(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Calendars)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clocks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clocks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Currencies(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Currencies)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HomeGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HomeGeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn WeekStartsOn(&self) -> ::windows_core::Result<super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WeekStartsOn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GlobalizationPreferencesForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalizationPreferencesForUser {}
impl ::core::fmt::Debug for GlobalizationPreferencesForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalizationPreferencesForUser").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GlobalizationPreferencesForUser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.GlobalizationPreferencesForUser;{150f0795-4f6e-40ba-a010-e27d81bda7f5})");
}
impl ::core::clone::Clone for GlobalizationPreferencesForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GlobalizationPreferencesForUser {
    const IID: ::windows_core::GUID = <IGlobalizationPreferencesForUser as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferencesForUser";
}
::windows_core::imp::interface_hierarchy!(GlobalizationPreferencesForUser, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GlobalizationPreferencesForUser {}
unsafe impl ::core::marker::Sync for GlobalizationPreferencesForUser {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
pub struct LockScreen;
impl LockScreen {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetImageFeedAsync<P0>(syndicationfeeduri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SetImageFeedResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestSetImageFeedAsync)(::windows_core::Interface::as_raw(this), syndicationfeeduri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn TryRemoveImageFeed() -> ::windows_core::Result<bool> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRemoveImageFeed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OriginalImageFile() -> ::windows_core::Result<super::super::Foundation::Uri> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OriginalImageFile)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetImageStream() -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetImageStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SetImageFileAsync<P0>(value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetImageFileAsync)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetImageStreamAsync<P0>(value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetImageStreamAsync)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILockScreenImageFeedStatics<R, F: FnOnce(&ILockScreenImageFeedStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LockScreen, ILockScreenImageFeedStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILockScreenStatics<R, F: FnOnce(&ILockScreenStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LockScreen, ILockScreenStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for LockScreen {
    const NAME: &'static str = "Windows.System.UserProfile.LockScreen";
}
#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct UserInformation;
#[cfg(feature = "deprecated")]
impl UserInformation {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AccountPictureChangeEnabled() -> ::windows_core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountPictureChangeEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn NameAccessAllowed() -> ::windows_core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NameAccessAllowed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn GetAccountPicture(kind: AccountPictureKind) -> ::windows_core::Result<super::super::Storage::IStorageFile> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccountPicture)(::windows_core::Interface::as_raw(this), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn SetAccountPictureAsync<P0>(image: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPictureAsync)(::windows_core::Interface::as_raw(this), image.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn SetAccountPicturesAsync<P0, P1, P2>(smallimage: P0, largeimage: P1, video: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P2: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPicturesAsync)(::windows_core::Interface::as_raw(this), smallimage.try_into_param()?.abi(), largeimage.try_into_param()?.abi(), video.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetAccountPictureFromStreamAsync<P0>(image: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPictureFromStreamAsync)(::windows_core::Interface::as_raw(this), image.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetAccountPicturesFromStreamsAsync<P0, P1, P2>(smallimage: P0, largeimage: P1, video: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P2: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPicturesFromStreamsAsync)(::windows_core::Interface::as_raw(this), smallimage.try_into_param()?.abi(), largeimage.try_into_param()?.abi(), video.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AccountPictureChanged<P0>(changehandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountPictureChanged)(::windows_core::Interface::as_raw(this), changehandler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAccountPictureChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IUserInformationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAccountPictureChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDisplayNameAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDisplayNameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetFirstNameAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFirstNameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetLastNameAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLastNameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetPrincipalNameAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPrincipalNameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetSessionInitiationProtocolUriAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSessionInitiationProtocolUriAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDomainNameAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDomainNameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IUserInformationStatics<R, F: FnOnce(&IUserInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UserInformation, IUserInformationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for UserInformation {
    const NAME: &'static str = "Windows.System.UserProfile.UserInformation";
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct UserProfilePersonalizationSettings(::windows_core::IUnknown);
impl UserProfilePersonalizationSettings {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TrySetLockScreenImageAsync<P0>(&self, imagefile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetLockScreenImageAsync)(::windows_core::Interface::as_raw(this), imagefile.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TrySetWallpaperImageAsync<P0>(&self, imagefile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetWallpaperImageAsync)(::windows_core::Interface::as_raw(this), imagefile.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<UserProfilePersonalizationSettings> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserProfilePersonalizationSettingsStatics<R, F: FnOnce(&IUserProfilePersonalizationSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UserProfilePersonalizationSettings, IUserProfilePersonalizationSettingsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for UserProfilePersonalizationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserProfilePersonalizationSettings {}
impl ::core::fmt::Debug for UserProfilePersonalizationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserProfilePersonalizationSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserProfilePersonalizationSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.UserProfilePersonalizationSettings;{8ceddab4-7998-46d5-8dd3-184f1c5f9ab9})");
}
impl ::core::clone::Clone for UserProfilePersonalizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserProfilePersonalizationSettings {
    const IID: ::windows_core::GUID = <IUserProfilePersonalizationSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.UserProfilePersonalizationSettings";
}
::windows_core::imp::interface_hierarchy!(UserProfilePersonalizationSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserProfilePersonalizationSettings {}
unsafe impl ::core::marker::Sync for UserProfilePersonalizationSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AccountPictureKind(pub i32);
#[cfg(feature = "deprecated")]
impl AccountPictureKind {
    pub const SmallImage: Self = Self(0i32);
    pub const LargeImage: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AccountPictureKind {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AccountPictureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for AccountPictureKind {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for AccountPictureKind {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AccountPictureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccountPictureKind").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for AccountPictureKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.AccountPictureKind;i4)");
}
#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SetAccountPictureResult(pub i32);
#[cfg(feature = "deprecated")]
impl SetAccountPictureResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const LargeOrDynamicError: Self = Self(2i32);
    pub const VideoFrameSizeError: Self = Self(3i32);
    pub const FileSizeError: Self = Self(4i32);
    pub const Failure: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SetAccountPictureResult {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SetAccountPictureResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SetAccountPictureResult {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for SetAccountPictureResult {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SetAccountPictureResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetAccountPictureResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SetAccountPictureResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetAccountPictureResult;i4)");
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const UserCanceled: Self = Self(2i32);
}
impl ::core::marker::Copy for SetImageFeedResult {}
impl ::core::clone::Clone for SetImageFeedResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetImageFeedResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SetImageFeedResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SetImageFeedResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetImageFeedResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SetImageFeedResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetImageFeedResult;i4)");
}
