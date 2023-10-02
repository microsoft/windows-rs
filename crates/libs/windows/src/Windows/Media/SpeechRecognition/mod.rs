#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechContinuousRecognitionCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3d069bb_e30c_5e18_424b_7fbe81f8fbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19091e1e_6e7e_5a46_40fb_76594f786504);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechContinuousRecognitionSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechContinuousRecognitionSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a213c04_6614_49f8_99a2_b5e9b3a085c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AutoStopSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoStopSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetAutoStopSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoStopSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartWithModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: SpeechContinuousRecognitionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartWithModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CancelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAsync: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ResultGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResultGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResultGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResultGenerated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionCompilationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionCompilationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x407e6c5d_6ac7_4da4_9cc1_2fce32cf7489);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionCompilationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionConstraint(::windows_core::IUnknown);
impl ISpeechRecognitionConstraint {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<SpeechRecognitionConstraintType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Probability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProbability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ISpeechRecognitionConstraint, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ISpeechRecognitionConstraint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{79ac1628-4d68-43c4-8911-40dc4101b55b}");
}
unsafe impl ::windows_core::Interface for ISpeechRecognitionConstraint {
    type Vtable = ISpeechRecognitionConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionConstraint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79ac1628_4d68_43c4_8911_40dc4101b55b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionConstraint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> ::windows_core::HRESULT,
    pub Probability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows_core::HRESULT,
    pub SetProbability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionGrammarFileConstraint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionGrammarFileConstraint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5031a8f_85ca_4fa4_b11a_474fc41b3835);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub GrammarFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GrammarFile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionGrammarFileConstraintFactory {
    type Vtable = ISpeechRecognitionGrammarFileConstraintFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionGrammarFileConstraintFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3da770eb_c479_4c27_9f19_89974ef392d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
    #[cfg(feature = "Storage")]
    pub CreateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateWithTag: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionHypothesis(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesis_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionHypothesis {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a7b25b0_99c5_4f7d_bf84_10aa1302b634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesis_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionHypothesisGeneratedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55161a7a_8023_5866_411d_1213bb271476);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Hypothesis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionListConstraint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionListConstraint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09c487e9_e4ad_4526_81f2_4946fb481d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionListConstraintFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionListConstraintFactory {
    type Vtable = ISpeechRecognitionListConstraintFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionListConstraintFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40f3cdc7_562a_426a_9f3b_3b4e282be1d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraintFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: *mut ::core::ffi::c_void, tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithTag: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionQualityDegradingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionQualityDegradingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fe24105_8c3a_4c7e_8d0a_5bd4f5b14ad8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionQualityDegradingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Problem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionAudioProblem) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e303157_034e_4652_857e_d0454cc4beec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConfidence) -> ::windows_core::HRESULT,
    pub SemanticInterpretation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxalternates: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAlternates: usize,
    pub Constraint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RulePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RulePath: usize,
    pub RawConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionResult2 {
    type Vtable = ISpeechRecognitionResult2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionResult2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf7ed1ba_451b_4166_a0c1_1ffe84032d03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PhraseStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhraseStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub PhraseDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhraseDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionSemanticInterpretation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionSemanticInterpretation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaae1da9b_7e32_4c1f_89fe_0c65f486f52e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionSemanticInterpretation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionTopicConstraint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionTopicConstraint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf6fdf19_825d_4e69_a681_36e48cf1c93e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Scenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionScenario) -> ::windows_core::HRESULT,
    pub TopicHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionTopicConstraintFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionTopicConstraintFactory {
    type Vtable = ISpeechRecognitionTopicConstraintFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionTopicConstraintFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e6863df_ec05_47d7_a5df_56a3431e58d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraintFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::std::mem::MaybeUninit<::windows_core::HSTRING>, tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2791c2b_1ef4_4ae7_9d77_b6ff10b8a3c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizer {
    type Vtable = ISpeechRecognizer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bc3c9cb_c26a_40f2_aeb5_8096b2e48073);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub CurrentLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    CurrentLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Constraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Constraints: usize,
    pub Timeouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UIOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompileConstraintsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompileConstraintsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognizeWithUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognizeWithUIAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognitionQualityDegrading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechrecognitionqualitydegradinghandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognitionQualityDegrading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecognitionQualityDegrading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecognitionQualityDegrading: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statechangedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizer2 {
    type Vtable = ISpeechRecognizer2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63c9baf1_91e3_4ea4_86a1_7c3867d084a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContinuousRecognitionSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StopRecognitionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecognitionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub HypothesisGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HypothesisGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHypothesisGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHypothesisGenerated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizerFactory {
    type Vtable = ISpeechRecognizerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60c488dd_7fb8_4033_ac70_d046f64818e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizerStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizerStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x563d4f09_ba03_4bad_ad81_ddc6c4dab0c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizerStatics {
    type Vtable = ISpeechRecognizerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87a35eac_a7dc_4b0b_bcc9_24f47c0b7ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub SystemSpeechLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SystemSpeechLanguage: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub SupportedTopicLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    SupportedTopicLanguages: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub SupportedGrammarLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    SupportedGrammarLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizerStatics2 {
    type Vtable = ISpeechRecognizerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d1b0d95_7565_4ef9_a2f3_ba15162a96cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    pub TrySetSystemSpeechLanguageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechlanguage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization")))]
    TrySetSystemSpeechLanguageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizerTimeouts(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeouts_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizerTimeouts {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ef76fca_6a3c_4dca_a153_df1bc88a79af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerTimeouts_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InitialSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetInitialSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInitialSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub EndSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub BabbleTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BabbleTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetBabbleTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBabbleTimeout: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpeechRecognizerUIOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpeechRecognizerUIOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7888d641_b92b_44ba_a25f_d1864630641f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerUIOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExampleText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExampleText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudiblePrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAudiblePrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsReadBackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsReadBackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShowConfirmation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShowConfirmation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVoiceCommandManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandManager {
    type Vtable = IVoiceCommandManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa3a8dd5_b6e7_4ee2_baa9_dd6baced0a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub InstallCommandSetsFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    InstallCommandSetsFromStorageFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstalledCommandSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstalledCommandSets: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVoiceCommandSet(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandSet {
    type Vtable = IVoiceCommandSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bedda75_46e6_4b11_a088_5c68632899b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandSet_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPhraseListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraselistname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, phraselist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPhraseListAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechContinuousRecognitionCompletedEventArgs(::windows_core::IUnknown);
impl SpeechContinuousRecognitionCompletedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechContinuousRecognitionCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs;{e3d069bb-e30c-5e18-424b-7fbe81f8fbd0})");
}
unsafe impl ::windows_core::Interface for SpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechContinuousRecognitionCompletedEventArgs {
    const IID: ::windows_core::GUID = <ISpeechContinuousRecognitionCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechContinuousRecognitionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpeechContinuousRecognitionCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(::windows_core::IUnknown);
impl SpeechContinuousRecognitionResultGeneratedEventArgs {
    pub fn Result(&self) -> ::windows_core::Result<SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs;{19091e1e-6e7e-5a46-40fb-76594f786504})");
}
unsafe impl ::windows_core::Interface for SpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const IID: ::windows_core::GUID = <ISpeechContinuousRecognitionResultGeneratedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpeechContinuousRecognitionResultGeneratedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionResultGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionResultGeneratedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechContinuousRecognitionSession(::windows_core::IUnknown);
impl SpeechContinuousRecognitionSession {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AutoStopSilenceTimeout(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoStopSilenceTimeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoStopSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoStopSilenceTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartWithModeAsync(&self, mode: SpeechContinuousRecognitionMode) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartWithModeAsync)(::windows_core::Interface::as_raw(this), mode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CancelAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CancelAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PauseAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PauseAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ResultGenerated<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultGenerated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResultGenerated(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResultGenerated)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for SpeechContinuousRecognitionSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession;{6a213c04-6614-49f8-99a2-b5e9b3a085c8})");
}
unsafe impl ::windows_core::Interface for SpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechContinuousRecognitionSession {
    const IID: ::windows_core::GUID = <ISpeechContinuousRecognitionSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechContinuousRecognitionSession {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession";
}
::windows_core::imp::interface_hierarchy!(SpeechContinuousRecognitionSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionSession {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionCompilationResult(::windows_core::IUnknown);
impl SpeechRecognitionCompilationResult {
    pub fn Status(&self) -> ::windows_core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionCompilationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult;{407e6c5d-6ac7-4da4-9cc1-2fce32cf7489})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionCompilationResult {
    const IID: ::windows_core::GUID = <ISpeechRecognitionCompilationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionCompilationResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionCompilationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionCompilationResult {}
unsafe impl ::core::marker::Sync for SpeechRecognitionCompilationResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionGrammarFileConstraint(::windows_core::IUnknown);
impl SpeechRecognitionGrammarFileConstraint {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Probability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProbability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn GrammarFile(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GrammarFile)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn Create<P0>(file: P0) -> ::windows_core::Result<SpeechRecognitionGrammarFileConstraint>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn CreateWithTag<P0>(file: P0, tag: &::windows_core::HSTRING) -> ::windows_core::Result<SpeechRecognitionGrammarFileConstraint>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithTag)(::windows_core::Interface::as_raw(this), file.into_param().abi(), ::core::mem::transmute_copy(tag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionGrammarFileConstraintFactory<R, F: FnOnce(&ISpeechRecognitionGrammarFileConstraintFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechRecognitionGrammarFileConstraint, ISpeechRecognitionGrammarFileConstraintFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionGrammarFileConstraint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint;{b5031a8f-85ca-4fa4-b11a-474fc41b3835})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionGrammarFileConstraint {
    const IID: ::windows_core::GUID = <ISpeechRecognitionGrammarFileConstraint as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionGrammarFileConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionGrammarFileConstraint, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionGrammarFileConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionGrammarFileConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionGrammarFileConstraint {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionHypothesis(::windows_core::IUnknown);
impl SpeechRecognitionHypothesis {
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionHypothesis {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis;{7a7b25b0-99c5-4f7d-bf84-10aa1302b634})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesis_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionHypothesis {
    const IID: ::windows_core::GUID = <ISpeechRecognitionHypothesis as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionHypothesis {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionHypothesis, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionHypothesis {}
unsafe impl ::core::marker::Sync for SpeechRecognitionHypothesis {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionHypothesisGeneratedEventArgs(::windows_core::IUnknown);
impl SpeechRecognitionHypothesisGeneratedEventArgs {
    pub fn Hypothesis(&self) -> ::windows_core::Result<SpeechRecognitionHypothesis> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hypothesis)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionHypothesisGeneratedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs;{55161a7a-8023-5866-411d-1213bb271476})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionHypothesisGeneratedEventArgs {
    const IID: ::windows_core::GUID = <ISpeechRecognitionHypothesisGeneratedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionHypothesisGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionHypothesisGeneratedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionHypothesisGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognitionHypothesisGeneratedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionListConstraint(::windows_core::IUnknown);
impl SpeechRecognitionListConstraint {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Probability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProbability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Commands(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Commands)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(commands: P0) -> ::windows_core::Result<SpeechRecognitionListConstraint>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), commands.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithTag<P0>(commands: P0, tag: &::windows_core::HSTRING) -> ::windows_core::Result<SpeechRecognitionListConstraint>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithTag)(::windows_core::Interface::as_raw(this), commands.try_into_param()?.abi(), ::core::mem::transmute_copy(tag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionListConstraintFactory<R, F: FnOnce(&ISpeechRecognitionListConstraintFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechRecognitionListConstraint, ISpeechRecognitionListConstraintFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionListConstraint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint;{09c487e9-e4ad-4526-81f2-4946fb481d98})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionListConstraint {
    const IID: ::windows_core::GUID = <ISpeechRecognitionListConstraint as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionListConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionListConstraint, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionListConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionListConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionListConstraint {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionQualityDegradingEventArgs(::windows_core::IUnknown);
impl SpeechRecognitionQualityDegradingEventArgs {
    pub fn Problem(&self) -> ::windows_core::Result<SpeechRecognitionAudioProblem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Problem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionQualityDegradingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs;{4fe24105-8c3a-4c7e-8d0a-5bd4f5b14ad8})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionQualityDegradingEventArgs {
    const IID: ::windows_core::GUID = <ISpeechRecognitionQualityDegradingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionQualityDegradingEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionQualityDegradingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionQualityDegradingEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognitionQualityDegradingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionResult(::windows_core::IUnknown);
impl SpeechRecognitionResult {
    pub fn Status(&self) -> ::windows_core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Confidence(&self) -> ::windows_core::Result<SpeechRecognitionConfidence> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Confidence)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SemanticInterpretation(&self) -> ::windows_core::Result<SpeechRecognitionSemanticInterpretation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SemanticInterpretation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAlternates(&self, maxalternates: u32) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAlternates)(::windows_core::Interface::as_raw(this), maxalternates, &mut result__).from_abi(result__)
        }
    }
    pub fn Constraint(&self) -> ::windows_core::Result<ISpeechRecognitionConstraint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Constraint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RulePath(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RulePath)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RawConfidence(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawConfidence)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PhraseStartTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhraseStartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PhraseDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhraseDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionResult;{4e303157-034e-4652-857e-d0454cc4beec})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionResult {
    const IID: ::windows_core::GUID = <ISpeechRecognitionResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionResult";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionResult {}
unsafe impl ::core::marker::Sync for SpeechRecognitionResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionSemanticInterpretation(::windows_core::IUnknown);
impl SpeechRecognitionSemanticInterpretation {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionSemanticInterpretation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation;{aae1da9b-7e32-4c1f-89fe-0c65f486f52e})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionSemanticInterpretation {
    const IID: ::windows_core::GUID = <ISpeechRecognitionSemanticInterpretation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionSemanticInterpretation {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionSemanticInterpretation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionSemanticInterpretation {}
unsafe impl ::core::marker::Sync for SpeechRecognitionSemanticInterpretation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionTopicConstraint(::windows_core::IUnknown);
impl SpeechRecognitionTopicConstraint {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Probability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProbability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scenario(&self) -> ::windows_core::Result<SpeechRecognitionScenario> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scenario)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TopicHint(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TopicHint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(scenario: SpeechRecognitionScenario, topichint: &::windows_core::HSTRING) -> ::windows_core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), scenario, ::core::mem::transmute_copy(topichint), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithTag(scenario: SpeechRecognitionScenario, topichint: &::windows_core::HSTRING, tag: &::windows_core::HSTRING) -> ::windows_core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithTag)(::windows_core::Interface::as_raw(this), scenario, ::core::mem::transmute_copy(topichint), ::core::mem::transmute_copy(tag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionTopicConstraintFactory<R, F: FnOnce(&ISpeechRecognitionTopicConstraintFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechRecognitionTopicConstraint, ISpeechRecognitionTopicConstraintFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionTopicConstraint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint;{bf6fdf19-825d-4e69-a681-36e48cf1c93e})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionTopicConstraint {
    const IID: ::windows_core::GUID = <ISpeechRecognitionTopicConstraint as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionTopicConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionTopicConstraint, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionTopicConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionTopicConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionTopicConstraint {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(::windows_core::IUnknown);
impl SpeechRecognitionVoiceCommandDefinitionConstraint {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows_core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Probability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProbability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint;{f2791c2b-1ef4-4ae7-9d77-b6ff10b8a3c2})");
}
unsafe impl ::windows_core::Interface for SpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const IID: ::windows_core::GUID = <ISpeechRecognitionVoiceCommandDefinitionConstraint as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognitionVoiceCommandDefinitionConstraint, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionVoiceCommandDefinitionConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionVoiceCommandDefinitionConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionVoiceCommandDefinitionConstraint {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognizer(::windows_core::IUnknown);
impl SpeechRecognizer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechRecognizer, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Globalization\"`"]
    #[cfg(feature = "Globalization")]
    pub fn CurrentLanguage(&self) -> ::windows_core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Constraints(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Constraints)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Timeouts(&self) -> ::windows_core::Result<SpeechRecognizerTimeouts> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timeouts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UIOptions(&self) -> ::windows_core::Result<SpeechRecognizerUIOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UIOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CompileConstraintsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompileConstraintsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RecognizeAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecognizeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RecognizeWithUIAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecognizeWithUIAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RecognitionQualityDegrading<P0>(&self, speechrecognitionqualitydegradinghandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecognitionQualityDegrading)(::windows_core::Interface::as_raw(this), speechrecognitionqualitydegradinghandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecognitionQualityDegrading(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRecognitionQualityDegrading)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, statechangedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), statechangedhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ContinuousRecognitionSession(&self) -> ::windows_core::Result<SpeechContinuousRecognitionSession> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuousRecognitionSession)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<SpeechRecognizerState> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StopRecognitionAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopRecognitionAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn HypothesisGenerated<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HypothesisGenerated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHypothesisGenerated(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHypothesisGenerated)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Globalization\"`"]
    #[cfg(feature = "Globalization")]
    pub fn Create<P0>(language: P0) -> ::windows_core::Result<SpeechRecognizer>
    where
        P0: ::windows_core::IntoParam<super::super::Globalization::Language>,
    {
        Self::ISpeechRecognizerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), language.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Globalization\"`"]
    #[cfg(feature = "Globalization")]
    pub fn SystemSpeechLanguage() -> ::windows_core::Result<super::super::Globalization::Language> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemSpeechLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Globalization\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn SupportedTopicLanguages() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTopicLanguages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Globalization\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn SupportedGrammarLanguages() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedGrammarLanguages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Globalization\"`"]
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    pub fn TrySetSystemSpeechLanguageAsync<P0>(speechlanguage: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::Globalization::Language>,
    {
        Self::ISpeechRecognizerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetSystemSpeechLanguageAsync)(::windows_core::Interface::as_raw(this), speechlanguage.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerFactory<R, F: FnOnce(&ISpeechRecognizerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerStatics<R, F: FnOnce(&ISpeechRecognizerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerStatics2<R, F: FnOnce(&ISpeechRecognizerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SpeechRecognizer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizer;{0bc3c9cb-c26a-40f2-aeb5-8096b2e48073})");
}
unsafe impl ::windows_core::Interface for SpeechRecognizer {
    type Vtable = ISpeechRecognizer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognizer {
    const IID: ::windows_core::GUID = <ISpeechRecognizer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognizer {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizer";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognizer, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for SpeechRecognizer {}
unsafe impl ::core::marker::Send for SpeechRecognizer {}
unsafe impl ::core::marker::Sync for SpeechRecognizer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognizerStateChangedEventArgs(::windows_core::IUnknown);
impl SpeechRecognizerStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<SpeechRecognizerState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognizerStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs;{563d4f09-ba03-4bad-ad81-ddc6c4dab0c3})");
}
unsafe impl ::windows_core::Interface for SpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognizerStateChangedEventArgs {
    const IID: ::windows_core::GUID = <ISpeechRecognizerStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognizerStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognizerStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognizerStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognizerStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognizerTimeouts(::windows_core::IUnknown);
impl SpeechRecognizerTimeouts {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InitialSilenceTimeout(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitialSilenceTimeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetInitialSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitialSilenceTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EndSilenceTimeout(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndSilenceTimeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndSilenceTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BabbleTimeout(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BabbleTimeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetBabbleTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBabbleTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognizerTimeouts {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts;{2ef76fca-6a3c-4dca-a153-df1bc88a79af})");
}
unsafe impl ::windows_core::Interface for SpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeouts_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognizerTimeouts {
    const IID: ::windows_core::GUID = <ISpeechRecognizerTimeouts as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognizerTimeouts {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognizerTimeouts, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognizerTimeouts {}
unsafe impl ::core::marker::Sync for SpeechRecognizerTimeouts {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechRecognizerUIOptions(::windows_core::IUnknown);
impl SpeechRecognizerUIOptions {
    pub fn ExampleText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExampleText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExampleText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExampleText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AudiblePrompt(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudiblePrompt)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudiblePrompt(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudiblePrompt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsReadBackEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadBackEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsReadBackEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsReadBackEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowConfirmation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowConfirmation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowConfirmation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowConfirmation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for SpeechRecognizerUIOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions;{7888d641-b92b-44ba-a25f-d1864630641f})");
}
unsafe impl ::windows_core::Interface for SpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechRecognizerUIOptions {
    const IID: ::windows_core::GUID = <ISpeechRecognizerUIOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechRecognizerUIOptions {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions";
}
::windows_core::imp::interface_hierarchy!(SpeechRecognizerUIOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognizerUIOptions {}
unsafe impl ::core::marker::Sync for SpeechRecognizerUIOptions {}
pub struct VoiceCommandManager;
impl VoiceCommandManager {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn InstallCommandSetsFromStorageFileAsync<P0>(file: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallCommandSetsFromStorageFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstalledCommandSets() -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, VoiceCommandSet>> {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstalledCommandSets)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandManager<R, F: FnOnce(&IVoiceCommandManager) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VoiceCommandManager, IVoiceCommandManager> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for VoiceCommandManager {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VoiceCommandSet(::windows_core::IUnknown);
impl VoiceCommandSet {
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPhraseListAsync<P0>(&self, phraselistname: &::windows_core::HSTRING, phraselist: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPhraseListAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(phraselistname), phraselist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for VoiceCommandSet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.VoiceCommandSet;{0bedda75-46e6-4b11-a088-5c68632899b5})");
}
unsafe impl ::windows_core::Interface for VoiceCommandSet {
    type Vtable = IVoiceCommandSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandSet {
    const IID: ::windows_core::GUID = <IVoiceCommandSet as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandSet {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandSet";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandSet, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandSet {}
unsafe impl ::core::marker::Sync for VoiceCommandSet {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechContinuousRecognitionMode(pub i32);
impl SpeechContinuousRecognitionMode {
    pub const Default: Self = Self(0i32);
    pub const PauseOnRecognition: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechContinuousRecognitionMode {}
impl ::core::clone::Clone for SpeechContinuousRecognitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechContinuousRecognitionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechContinuousRecognitionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechContinuousRecognitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechContinuousRecognitionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognitionAudioProblem(pub i32);
impl SpeechRecognitionAudioProblem {
    pub const None: Self = Self(0i32);
    pub const TooNoisy: Self = Self(1i32);
    pub const NoSignal: Self = Self(2i32);
    pub const TooLoud: Self = Self(3i32);
    pub const TooQuiet: Self = Self(4i32);
    pub const TooFast: Self = Self(5i32);
    pub const TooSlow: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognitionAudioProblem {}
impl ::core::clone::Clone for SpeechRecognitionAudioProblem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognitionAudioProblem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechRecognitionAudioProblem {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionAudioProblem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionAudioProblem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionAudioProblem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionAudioProblem;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Rejected: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConfidence {}
impl ::core::clone::Clone for SpeechRecognitionConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognitionConfidence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechRecognitionConfidence {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConfidence").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionConfidence {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConfidence;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
    pub const Max: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintProbability {}
impl ::core::clone::Clone for SpeechRecognitionConstraintProbability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognitionConstraintProbability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechRecognitionConstraintProbability {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionConstraintProbability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConstraintProbability").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionConstraintProbability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintProbability;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: Self = Self(0i32);
    pub const List: Self = Self(1i32);
    pub const Grammar: Self = Self(2i32);
    pub const VoiceCommandDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintType {}
impl ::core::clone::Clone for SpeechRecognitionConstraintType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognitionConstraintType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechRecognitionConstraintType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionConstraintType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConstraintType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionConstraintType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognitionResultStatus(pub i32);
impl SpeechRecognitionResultStatus {
    pub const Success: Self = Self(0i32);
    pub const TopicLanguageNotSupported: Self = Self(1i32);
    pub const GrammarLanguageMismatch: Self = Self(2i32);
    pub const GrammarCompilationFailure: Self = Self(3i32);
    pub const AudioQualityFailure: Self = Self(4i32);
    pub const UserCanceled: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
    pub const TimeoutExceeded: Self = Self(7i32);
    pub const PauseLimitExceeded: Self = Self(8i32);
    pub const NetworkFailure: Self = Self(9i32);
    pub const MicrophoneUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for SpeechRecognitionResultStatus {}
impl ::core::clone::Clone for SpeechRecognitionResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognitionResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechRecognitionResultStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionResultStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionResultStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionResultStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: Self = Self(0i32);
    pub const Dictation: Self = Self(1i32);
    pub const FormFilling: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionScenario {}
impl ::core::clone::Clone for SpeechRecognitionScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognitionScenario {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechRecognitionScenario {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionScenario").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechRecognitionScenario {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionScenario;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognizerState(pub i32);
impl SpeechRecognizerState {
    pub const Idle: Self = Self(0i32);
    pub const Capturing: Self = Self(1i32);
    pub const Processing: Self = Self(2i32);
    pub const SoundStarted: Self = Self(3i32);
    pub const SoundEnded: Self = Self(4i32);
    pub const SpeechDetected: Self = Self(5i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognizerState {}
impl ::core::clone::Clone for SpeechRecognizerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognizerState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SpeechRecognizerState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognizerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechRecognizerState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognizerState;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
