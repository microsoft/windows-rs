#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalContentSuggestionSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_Vtbl;
}
impl ::core::clone::Clone for ILocalContentSuggestionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILocalContentSuggestionSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeeaeb062_743d_456e_84a3_23f06f2d15d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalContentSuggestionSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Locations: usize,
    pub SetAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PropertiesToMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PropertiesToMatch: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPane(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPane {
    type Vtable = ISearchPane_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPane {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdacec38_3700_4d73_91a1_2f998674238a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPane_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSearchHistoryEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchHistoryEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetSearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSearchHistoryContext: usize,
    #[cfg(feature = "deprecated")]
    pub SearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchHistoryContext: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaceholderText: usize,
    #[cfg(feature = "deprecated")]
    pub PlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaceholderText: usize,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
    #[cfg(feature = "deprecated")]
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Visible: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VisibilityChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVisibilityChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub QueryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    QueryChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveQueryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveQueryChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SuggestionsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSuggestionsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub QuerySubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    QuerySubmitted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveQuerySubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveQuerySubmitted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ResultSuggestionChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ResultSuggestionChosen: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveResultSuggestionChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveResultSuggestionChosen: usize,
    #[cfg(feature = "deprecated")]
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLocalContentSuggestionSettings: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOverloadDefault: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOverloadWithQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOverloadWithQuery: usize,
    #[cfg(feature = "deprecated")]
    pub SetShowOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetShowOnKeyboardInput: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOnKeyboardInput: usize,
    #[cfg(feature = "deprecated")]
    pub TrySetQueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrySetQueryText: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneQueryChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl ISearchPaneQueryChangedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(ISearchPaneQueryChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ISearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ISearchPaneQueryChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ISearchPaneQueryChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchPaneQueryChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for ISearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3c064fe9-2351-4248-a529-7110f464a785}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneQueryChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c064fe9_2351_4248_a529_7110f464a785);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
    #[cfg(feature = "deprecated")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LinguisticDetails: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneQueryLinguisticDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_Vtbl;
}
impl ::core::clone::Clone for ISearchPaneQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchPaneQueryLinguisticDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82fb460e_0940_4b6d_b8d0_642b30989e15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneQuerySubmittedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x143ba4fc_e9c5_4736_91b2_e8eb9cb88356);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    type Vtable = ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x460c92e5_4c32_4538_a4d4_b6b4400d140f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LinguisticDetails: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneResultSuggestionChosenEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneResultSuggestionChosenEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8316cc0_aed2_41e0_bce0_c26ca74f85ec);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneResultSuggestionChosenEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Tag: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneStatics {
    type Vtable = ISearchPaneStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9572adf1_8f1d_481f_a15b_c61655f16a0e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneStaticsWithHideThisApplication(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneStaticsWithHideThisApplication {
    type Vtable = ISearchPaneStaticsWithHideThisApplication_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneStaticsWithHideThisApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneStaticsWithHideThisApplication {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00732830_50f1_4d03_99ac_c6644c8ed8b5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStaticsWithHideThisApplication_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub HideThisApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HideThisApplication: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequest(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneSuggestionsRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81b10b1c_e561_4093_9b4d_2ad482794a53);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCanceled: usize,
    #[cfg(feature = "deprecated")]
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchSuggestionCollection: usize,
    #[cfg(feature = "deprecated")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestDeferral(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneSuggestionsRequestDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0d009f7_8748_4ee2_ad44_afa6be997c51);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Complete: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneSuggestionsRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc89b8a2f_ac56_4460_8d2f_80023bec4fc5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Request: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneVisibilityChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISearchPaneVisibilityChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c4d3046_ac4b_49f2_97d6_020e6182cb9c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneVisibilityChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Visible: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_Vtbl;
}
impl ::core::clone::Clone for ISearchQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchQueryLinguisticDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46a1205b_69c9_4745_b72f_a8a4fc8f24ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetailsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchQueryLinguisticDetailsFactory {
    type Vtable = ISearchQueryLinguisticDetailsFactory_Vtbl;
}
impl ::core::clone::Clone for ISearchQueryLinguisticDetailsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchQueryLinguisticDetailsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcac6c3b8_3c64_4dfd_ad9f_479e4d4065a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytextalternatives: *mut ::core::ffi::c_void, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_Vtbl;
}
impl ::core::clone::Clone for ISearchSuggestionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchSuggestionCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x323a8a4b_fbea_4446_abbc_3da7915fdd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AppendQuerySuggestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendQuerySuggestions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, suggestions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendQuerySuggestions: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendResultSuggestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, detailtext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, image: *mut ::core::ffi::c_void, imagealternatetext: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendResultSuggestion: usize,
    pub AppendSearchSeparator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_Vtbl;
}
impl ::core::clone::Clone for ISearchSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchSuggestionsRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e4e26a7_44e5_4039_9099_6000ead1f0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_Vtbl;
}
impl ::core::clone::Clone for ISearchSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISearchSuggestionsRequestDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb71598a9_c065_456d_a845_1eccec5dc28b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct LocalContentSuggestionSettings(::windows_core::IUnknown);
impl LocalContentSuggestionSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LocalContentSuggestionSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Locations(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Locations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAqsFilter(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAqsFilter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AqsFilter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AqsFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PropertiesToMatch(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesToMatch)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LocalContentSuggestionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalContentSuggestionSettings {}
impl ::core::fmt::Debug for LocalContentSuggestionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalContentSuggestionSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocalContentSuggestionSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.LocalContentSuggestionSettings;{eeaeb062-743d-456e-84a3-23f06f2d15d7})");
}
impl ::core::clone::Clone for LocalContentSuggestionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LocalContentSuggestionSettings {
    const IID: ::windows_core::GUID = <ILocalContentSuggestionSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LocalContentSuggestionSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
}
::windows_core::imp::interface_hierarchy!(LocalContentSuggestionSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPane(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPane {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchHistoryEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SearchHistoryEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchHistoryEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSearchHistoryContext(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchHistoryContext)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SearchHistoryContext(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchHistoryContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetPlaceholderText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaceholderText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PlaceholderText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaceholderText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Visible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Visible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn VisibilityChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VisibilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveVisibilityChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVisibilityChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn QueryChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveQueryChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveQueryChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SuggestionsRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestionsRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSuggestionsRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSuggestionsRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn QuerySubmitted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QuerySubmitted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveQuerySubmitted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveQuerySubmitted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ResultSuggestionChosen<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultSuggestionChosen)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveResultSuggestionChosen(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResultSuggestionChosen)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetLocalContentSuggestionSettings<P0>(&self, settings: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LocalContentSuggestionSettings>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalContentSuggestionSettings)(::windows_core::Interface::as_raw(this), settings.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowOverloadDefault(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowOverloadDefault)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowOverloadWithQuery(&self, query: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowOverloadWithQuery)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(query)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetShowOnKeyboardInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowOnKeyboardInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowOnKeyboardInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowOnKeyboardInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TrySetQueryText(&self, query: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetQueryText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(query), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows_core::Result<SearchPane> {
        Self::ISearchPaneStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn HideThisApplication() -> ::windows_core::Result<()> {
        Self::ISearchPaneStaticsWithHideThisApplication(|this| unsafe { (::windows_core::Interface::vtable(this).HideThisApplication)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISearchPaneStatics<R, F: FnOnce(&ISearchPaneStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SearchPane, ISearchPaneStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISearchPaneStaticsWithHideThisApplication<R, F: FnOnce(&ISearchPaneStaticsWithHideThisApplication) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SearchPane, ISearchPaneStaticsWithHideThisApplication> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPane {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPane").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPane {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPane;{fdacec38-3700-4d73-91a1-2f998674238a})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPane {
    type Vtable = ISearchPane_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPane {
    const IID: ::windows_core::GUID = <ISearchPane as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPane {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPane";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPane, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneQueryChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneQueryChangedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneQueryChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneQueryChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQueryChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs;{3c064fe9-2351-4248-a529-7110f464a785})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPaneQueryChangedEventArgs {
    const IID: ::windows_core::GUID = <ISearchPaneQueryChangedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPaneQueryChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<ISearchPaneQueryChangedEventArgs> for SearchPaneQueryChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneQueryChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneQueryChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchPaneQueryLinguisticDetails(::windows_core::IUnknown);
impl SearchPaneQueryLinguisticDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryTextAlternatives(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextAlternatives)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionStart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SearchPaneQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneQueryLinguisticDetails {}
impl ::core::fmt::Debug for SearchPaneQueryLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQueryLinguisticDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchPaneQueryLinguisticDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails;{82fb460e-0940-4b6d-b8d0-642b30989e15})");
}
impl ::core::clone::Clone for SearchPaneQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchPaneQueryLinguisticDetails {
    const IID: ::windows_core::GUID = <ISearchPaneQueryLinguisticDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchPaneQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
}
::windows_core::imp::interface_hierarchy!(SearchPaneQueryLinguisticDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SearchPaneQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchPaneQueryLinguisticDetails {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneQuerySubmittedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneQuerySubmittedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::ComInterface::cast::<ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneQuerySubmittedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneQuerySubmittedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneQuerySubmittedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQuerySubmittedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPaneQuerySubmittedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs;{143ba4fc-e9c5-4736-91b2-e8eb9cb88356})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPaneQuerySubmittedEventArgs {
    const IID: ::windows_core::GUID = <ISearchPaneQuerySubmittedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPaneQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPaneQuerySubmittedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneQuerySubmittedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneQuerySubmittedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneResultSuggestionChosenEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneResultSuggestionChosenEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneResultSuggestionChosenEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneResultSuggestionChosenEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneResultSuggestionChosenEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneResultSuggestionChosenEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPaneResultSuggestionChosenEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs;{c8316cc0-aed2-41e0-bce0-c26ca74f85ec})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPaneResultSuggestionChosenEventArgs {
    const IID: ::windows_core::GUID = <ISearchPaneResultSuggestionChosenEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPaneResultSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPaneResultSuggestionChosenEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneResultSuggestionChosenEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneResultSuggestionChosenEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequest(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneSuggestionsRequest {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsCanceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SearchSuggestionCollection(&self) -> ::windows_core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchSuggestionCollection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<SearchPaneSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequest {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPaneSuggestionsRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest;{81b10b1c-e561-4093-9b4d-2ad482794a53})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPaneSuggestionsRequest {
    const IID: ::windows_core::GUID = <ISearchPaneSuggestionsRequest as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPaneSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPaneSuggestionsRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequest {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequest {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestDeferral(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneSuggestionsRequestDeferral {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestDeferral {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequestDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPaneSuggestionsRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral;{a0d009f7-8748-4ee2-ad44-afa6be997c51})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPaneSuggestionsRequestDeferral {
    const IID: ::windows_core::GUID = <ISearchPaneSuggestionsRequestDeferral as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPaneSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPaneSuggestionsRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestDeferral {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneSuggestionsRequestedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::ComInterface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Request(&self) -> ::windows_core::Result<SearchPaneSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPaneSuggestionsRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs;{c89b8a2f-ac56-4460-8d2f-80023bec4fc5})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPaneSuggestionsRequestedEventArgs {
    const IID: ::windows_core::GUID = <ISearchPaneSuggestionsRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPaneSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPaneSuggestionsRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<ISearchPaneQueryChangedEventArgs> for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneVisibilityChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneVisibilityChangedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Visible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Visible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneVisibilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneVisibilityChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneVisibilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneVisibilityChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SearchPaneVisibilityChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs;{3c4d3046-ac4b-49f2-97d6-020e6182cb9c})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SearchPaneVisibilityChangedEventArgs {
    const IID: ::windows_core::GUID = <ISearchPaneVisibilityChangedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SearchPaneVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SearchPaneVisibilityChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneVisibilityChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneVisibilityChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchQueryLinguisticDetails(::windows_core::IUnknown);
impl SearchQueryLinguisticDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryTextAlternatives(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextAlternatives)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionStart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<P0>(querytextalternatives: P0, querytextcompositionstart: u32, querytextcompositionlength: u32) -> ::windows_core::Result<SearchQueryLinguisticDetails>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ISearchQueryLinguisticDetailsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), querytextalternatives.try_into_param()?.abi(), querytextcompositionstart, querytextcompositionlength, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISearchQueryLinguisticDetailsFactory<R, F: FnOnce(&ISearchQueryLinguisticDetailsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SearchQueryLinguisticDetails, ISearchQueryLinguisticDetailsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SearchQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchQueryLinguisticDetails {}
impl ::core::fmt::Debug for SearchQueryLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchQueryLinguisticDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchQueryLinguisticDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchQueryLinguisticDetails;{46a1205b-69c9-4745-b72f-a8a4fc8f24ae})");
}
impl ::core::clone::Clone for SearchQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchQueryLinguisticDetails {
    const IID: ::windows_core::GUID = <ISearchQueryLinguisticDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
}
::windows_core::imp::interface_hierarchy!(SearchQueryLinguisticDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SearchQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchQueryLinguisticDetails {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionCollection(::windows_core::IUnknown);
impl SearchSuggestionCollection {
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendQuerySuggestion(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendQuerySuggestion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppendQuerySuggestions<P0>(&self, suggestions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendQuerySuggestions)(::windows_core::Interface::as_raw(this), suggestions.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendResultSuggestion<P0>(&self, text: &::windows_core::HSTRING, detailtext: &::windows_core::HSTRING, tag: &::windows_core::HSTRING, image: P0, imagealternatetext: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendResultSuggestion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), ::core::mem::transmute_copy(detailtext), ::core::mem::transmute_copy(tag), image.try_into_param()?.abi(), ::core::mem::transmute_copy(imagealternatetext)).ok() }
    }
    pub fn AppendSearchSeparator(&self, label: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendSearchSeparator)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(label)).ok() }
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionCollection {}
impl ::core::fmt::Debug for SearchSuggestionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchSuggestionCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionCollection;{323a8a4b-fbea-4446-abbc-3da7915fdd3a})");
}
impl ::core::clone::Clone for SearchSuggestionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchSuggestionCollection {
    const IID: ::windows_core::GUID = <ISearchSuggestionCollection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionCollection {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionCollection";
}
::windows_core::imp::interface_hierarchy!(SearchSuggestionCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SearchSuggestionCollection {}
unsafe impl ::core::marker::Sync for SearchSuggestionCollection {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionsRequest(::windows_core::IUnknown);
impl SearchSuggestionsRequest {
    pub fn IsCanceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SearchSuggestionCollection(&self) -> ::windows_core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchSuggestionCollection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<SearchSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequest {}
impl ::core::fmt::Debug for SearchSuggestionsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionsRequest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchSuggestionsRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequest;{4e4e26a7-44e5-4039-9099-6000ead1f0c6})");
}
impl ::core::clone::Clone for SearchSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchSuggestionsRequest {
    const IID: ::windows_core::GUID = <ISearchSuggestionsRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequest";
}
::windows_core::imp::interface_hierarchy!(SearchSuggestionsRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SearchSuggestionsRequest {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequest {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionsRequestDeferral(::windows_core::IUnknown);
impl SearchSuggestionsRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequestDeferral {}
impl ::core::fmt::Debug for SearchSuggestionsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionsRequestDeferral").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SearchSuggestionsRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral;{b71598a9-c065-456d-a845-1eccec5dc28b})");
}
impl ::core::clone::Clone for SearchSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchSuggestionsRequestDeferral {
    const IID: ::windows_core::GUID = <ISearchSuggestionsRequestDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
}
::windows_core::imp::interface_hierarchy!(SearchSuggestionsRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SearchSuggestionsRequestDeferral {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequestDeferral {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
