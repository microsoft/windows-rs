#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpeechContinuousRecognitionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechContinuousRecognitionCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3d069bb_e30c_5e18_424b_7fbe81f8fbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19091e1e_6e7e_5a46_40fb_76594f786504);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSession_Vtbl;
}
impl ::core::clone::Clone for ISpeechContinuousRecognitionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechContinuousRecognitionSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a213c04_6614_49f8_99a2_b5e9b3a085c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AutoStopSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoStopSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetAutoStopSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoStopSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartWithModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: SpeechContinuousRecognitionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartWithModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CancelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAsync: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ResultGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResultGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResultGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResultGenerated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionCompilationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResult_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionCompilationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionCompilationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407e6c5d_6ac7_4da4_9cc1_2fce32cf7489);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionCompilationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct ISpeechRecognitionConstraint(::windows::core::IUnknown);
impl ISpeechRecognitionConstraint {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Tag)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintProbability>();
            (::windows::core::Interface::vtable(this).Probability)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProbability)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
::windows::imp::interface_hierarchy!(ISpeechRecognitionConstraint, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISpeechRecognitionConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpeechRecognitionConstraint {}
impl ::core::fmt::Debug for ISpeechRecognitionConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecognitionConstraint").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISpeechRecognitionConstraint {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{79ac1628-4d68-43c4-8911-40dc4101b55b}");
}
unsafe impl ::windows::core::Interface for ISpeechRecognitionConstraint {
    type Vtable = ISpeechRecognitionConstraint_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionConstraint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79ac1628_4d68_43c4_8911_40dc4101b55b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionConstraint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> ::windows::core::HRESULT,
    pub Probability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT,
    pub SetProbability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraint_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionGrammarFileConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionGrammarFileConstraint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5031a8f_85ca_4fa4_b11a_474fc41b3835);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub GrammarFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GrammarFile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionGrammarFileConstraintFactory {
    type Vtable = ISpeechRecognitionGrammarFileConstraintFactory_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionGrammarFileConstraintFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionGrammarFileConstraintFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da770eb_c479_4c27_9f19_89974ef392d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
    #[cfg(feature = "Storage")]
    pub CreateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, tag: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateWithTag: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionHypothesis(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesis_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionHypothesis {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionHypothesis {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a7b25b0_99c5_4f7d_bf84_10aa1302b634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesis_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionHypothesisGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionHypothesisGeneratedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55161a7a_8023_5866_411d_1213bb271476);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Hypothesis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionListConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraint_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionListConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionListConstraint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09c487e9_e4ad_4526_81f2_4946fb481d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionListConstraintFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionListConstraintFactory {
    type Vtable = ISpeechRecognitionListConstraintFactory_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionListConstraintFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionListConstraintFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40f3cdc7_562a_426a_9f3b_3b4e282be1d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraintFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: *mut ::core::ffi::c_void, tag: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithTag: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionQualityDegradingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionQualityDegradingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionQualityDegradingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fe24105_8c3a_4c7e_8d0a_5bd4f5b14ad8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionQualityDegradingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Problem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionAudioProblem) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResult_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e303157_034e_4652_857e_d0454cc4beec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConfidence) -> ::windows::core::HRESULT,
    pub SemanticInterpretation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxalternates: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAlternates: usize,
    pub Constraint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RulePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RulePath: usize,
    pub RawConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionResult2 {
    type Vtable = ISpeechRecognitionResult2_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf7ed1ba_451b_4166_a0c1_1ffe84032d03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PhraseStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhraseStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub PhraseDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhraseDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionSemanticInterpretation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretation_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionSemanticInterpretation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionSemanticInterpretation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaae1da9b_7e32_4c1f_89fe_0c65f486f52e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionSemanticInterpretation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraint_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionTopicConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionTopicConstraint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6fdf19_825d_4e69_a681_36e48cf1c93e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Scenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionScenario) -> ::windows::core::HRESULT,
    pub TopicHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraintFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionTopicConstraintFactory {
    type Vtable = ISpeechRecognitionTopicConstraintFactory_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionTopicConstraintFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionTopicConstraintFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e6863df_ec05_47d7_a5df_56a3431e58d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraintFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::std::mem::MaybeUninit<::windows::core::HSTRING>, tag: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2791c2b_1ef4_4ae7_9d77_b6ff10b8a3c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizer {
    type Vtable = ISpeechRecognizer_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc3c9cb_c26a_40f2_aeb5_8096b2e48073);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub CurrentLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    CurrentLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Constraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Constraints: usize,
    pub Timeouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UIOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompileConstraintsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompileConstraintsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognizeWithUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognizeWithUIAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognitionQualityDegrading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechrecognitionqualitydegradinghandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognitionQualityDegrading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecognitionQualityDegrading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecognitionQualityDegrading: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statechangedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizer2 {
    type Vtable = ISpeechRecognizer2_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63c9baf1_91e3_4ea4_86a1_7c3867d084a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContinuousRecognitionSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StopRecognitionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecognitionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub HypothesisGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HypothesisGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHypothesisGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHypothesisGenerated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerFactory {
    type Vtable = ISpeechRecognizerFactory_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60c488dd_7fb8_4033_ac70_d046f64818e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizerStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizerStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x563d4f09_ba03_4bad_ad81_ddc6c4dab0c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerStatics {
    type Vtable = ISpeechRecognizerStatics_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a35eac_a7dc_4b0b_bcc9_24f47c0b7ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub SystemSpeechLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SystemSpeechLanguage: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub SupportedTopicLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    SupportedTopicLanguages: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub SupportedGrammarLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    SupportedGrammarLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerStatics2 {
    type Vtable = ISpeechRecognizerStatics2_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1b0d95_7565_4ef9_a2f3_ba15162a96cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    pub TrySetSystemSpeechLanguageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechlanguage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization")))]
    TrySetSystemSpeechLanguageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerTimeouts(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeouts_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizerTimeouts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizerTimeouts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ef76fca_6a3c_4dca_a153_df1bc88a79af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerTimeouts_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InitialSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetInitialSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInitialSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub EndSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndSilenceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub BabbleTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BabbleTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetBabbleTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBabbleTimeout: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerUIOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptions_Vtbl;
}
impl ::core::clone::Clone for ISpeechRecognizerUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechRecognizerUIOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7888d641_b92b_44ba_a25f_d1864630641f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerUIOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExampleText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetExampleText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AudiblePrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAudiblePrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsReadBackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsReadBackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShowConfirmation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowConfirmation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandManager {
    type Vtable = IVoiceCommandManager_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVoiceCommandManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa3a8dd5_b6e7_4ee2_baa9_dd6baced0a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub InstallCommandSetsFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    InstallCommandSetsFromStorageFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstalledCommandSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstalledCommandSets: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandSet(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandSet {
    type Vtable = IVoiceCommandSet_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVoiceCommandSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bedda75_46e6_4b11_a088_5c68632899b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandSet_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPhraseListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraselistname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, phraselist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPhraseListAsync: usize,
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionCompletedEventArgs(::windows::core::IUnknown);
impl SpeechContinuousRecognitionCompletedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionResultStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechContinuousRecognitionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechContinuousRecognitionCompletedEventArgs {}
impl ::core::fmt::Debug for SpeechContinuousRecognitionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechContinuousRecognitionCompletedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs;{e3d069bb-e30c-5e18-424b-7fbe81f8fbd0})");
}
impl ::core::clone::Clone for SpeechContinuousRecognitionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechContinuousRecognitionCompletedEventArgs {
    const IID: ::windows::core::GUID = <ISpeechContinuousRecognitionCompletedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechContinuousRecognitionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs";
}
::windows::imp::interface_hierarchy!(SpeechContinuousRecognitionCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionCompletedEventArgs {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(::windows::core::IUnknown);
impl SpeechContinuousRecognitionResultGeneratedEventArgs {
    pub fn Result(&self) -> ::windows::core::Result<SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionResult>();
            (::windows::core::Interface::vtable(this).Result)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechContinuousRecognitionResultGeneratedEventArgs {}
impl ::core::fmt::Debug for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionResultGeneratedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs;{19091e1e-6e7e-5a46-40fb-76594f786504})");
}
impl ::core::clone::Clone for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const IID: ::windows::core::GUID = <ISpeechContinuousRecognitionResultGeneratedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs";
}
::windows::imp::interface_hierarchy!(SpeechContinuousRecognitionResultGeneratedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionResultGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionResultGeneratedEventArgs {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionSession(::windows::core::IUnknown);
impl SpeechContinuousRecognitionSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoStopSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).AutoStopSilenceTimeout)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoStopSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoStopSilenceTimeout)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).StartAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartWithModeAsync(&self, mode: SpeechContinuousRecognitionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).StartWithModeAsync)(::windows::core::Interface::as_raw(this), mode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).StopAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).CancelAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).PauseAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Resume)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, value: &super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResultGenerated(&self, value: &super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ResultGenerated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResultGenerated(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveResultGenerated)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SpeechContinuousRecognitionSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechContinuousRecognitionSession {}
impl ::core::fmt::Debug for SpeechContinuousRecognitionSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechContinuousRecognitionSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession;{6a213c04-6614-49f8-99a2-b5e9b3a085c8})");
}
impl ::core::clone::Clone for SpeechContinuousRecognitionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechContinuousRecognitionSession {
    const IID: ::windows::core::GUID = <ISpeechContinuousRecognitionSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechContinuousRecognitionSession {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession";
}
::windows::imp::interface_hierarchy!(SpeechContinuousRecognitionSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionSession {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionSession {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionCompilationResult(::windows::core::IUnknown);
impl SpeechRecognitionCompilationResult {
    pub fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionResultStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionCompilationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionCompilationResult {}
impl ::core::fmt::Debug for SpeechRecognitionCompilationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionCompilationResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionCompilationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult;{407e6c5d-6ac7-4da4-9cc1-2fce32cf7489})");
}
impl ::core::clone::Clone for SpeechRecognitionCompilationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionCompilationResult {
    const IID: ::windows::core::GUID = <ISpeechRecognitionCompilationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionCompilationResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionCompilationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionCompilationResult {}
unsafe impl ::core::marker::Sync for SpeechRecognitionCompilationResult {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionGrammarFileConstraint(::windows::core::IUnknown);
impl SpeechRecognitionGrammarFileConstraint {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Tag)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintProbability>();
            (::windows::core::Interface::vtable(this).Probability)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProbability)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn GrammarFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFile>();
            (::windows::core::Interface::vtable(this).GrammarFile)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn Create(file: &super::super::Storage::StorageFile) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint> {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionGrammarFileConstraint>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(file), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CreateWithTag(file: &super::super::Storage::StorageFile, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint> {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionGrammarFileConstraint>();
            (::windows::core::Interface::vtable(this).CreateWithTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(file), ::core::mem::transmute_copy(tag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionGrammarFileConstraintFactory<R, F: FnOnce(&ISpeechRecognitionGrammarFileConstraintFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechRecognitionGrammarFileConstraint, ISpeechRecognitionGrammarFileConstraintFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionGrammarFileConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionGrammarFileConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionGrammarFileConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionGrammarFileConstraint").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionGrammarFileConstraint {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint;{b5031a8f-85ca-4fa4-b11a-474fc41b3835})");
}
impl ::core::clone::Clone for SpeechRecognitionGrammarFileConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraint_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionGrammarFileConstraint {
    const IID: ::windows::core::GUID = <ISpeechRecognitionGrammarFileConstraint as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionGrammarFileConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionGrammarFileConstraint, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionGrammarFileConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionGrammarFileConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionGrammarFileConstraint {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionHypothesis(::windows::core::IUnknown);
impl SpeechRecognitionHypothesis {
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionHypothesis {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionHypothesis {}
impl ::core::fmt::Debug for SpeechRecognitionHypothesis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionHypothesis").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionHypothesis {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis;{7a7b25b0-99c5-4f7d-bf84-10aa1302b634})");
}
impl ::core::clone::Clone for SpeechRecognitionHypothesis {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesis_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionHypothesis {
    const IID: ::windows::core::GUID = <ISpeechRecognitionHypothesis as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionHypothesis {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionHypothesis, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionHypothesis {}
unsafe impl ::core::marker::Sync for SpeechRecognitionHypothesis {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionHypothesisGeneratedEventArgs(::windows::core::IUnknown);
impl SpeechRecognitionHypothesisGeneratedEventArgs {
    pub fn Hypothesis(&self) -> ::windows::core::Result<SpeechRecognitionHypothesis> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionHypothesis>();
            (::windows::core::Interface::vtable(this).Hypothesis)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionHypothesisGeneratedEventArgs {}
impl ::core::fmt::Debug for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionHypothesisGeneratedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionHypothesisGeneratedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs;{55161a7a-8023-5866-411d-1213bb271476})");
}
impl ::core::clone::Clone for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionHypothesisGeneratedEventArgs {
    const IID: ::windows::core::GUID = <ISpeechRecognitionHypothesisGeneratedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionHypothesisGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionHypothesisGeneratedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionHypothesisGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognitionHypothesisGeneratedEventArgs {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionListConstraint(::windows::core::IUnknown);
impl SpeechRecognitionListConstraint {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Tag)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintProbability>();
            (::windows::core::Interface::vtable(this).Probability)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProbability)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Commands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(commands: P0) -> ::windows::core::Result<SpeechRecognitionListConstraint>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionListConstraint>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), commands.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithTag<P0>(commands: P0, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionListConstraint>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionListConstraint>();
            (::windows::core::Interface::vtable(this).CreateWithTag)(::windows::core::Interface::as_raw(this), commands.try_into_param()?.abi(), ::core::mem::transmute_copy(tag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionListConstraintFactory<R, F: FnOnce(&ISpeechRecognitionListConstraintFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechRecognitionListConstraint, ISpeechRecognitionListConstraintFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionListConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionListConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionListConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionListConstraint").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionListConstraint {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint;{09c487e9-e4ad-4526-81f2-4946fb481d98})");
}
impl ::core::clone::Clone for SpeechRecognitionListConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraint_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionListConstraint {
    const IID: ::windows::core::GUID = <ISpeechRecognitionListConstraint as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionListConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionListConstraint, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionListConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionListConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionListConstraint {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionQualityDegradingEventArgs(::windows::core::IUnknown);
impl SpeechRecognitionQualityDegradingEventArgs {
    pub fn Problem(&self) -> ::windows::core::Result<SpeechRecognitionAudioProblem> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionAudioProblem>();
            (::windows::core::Interface::vtable(this).Problem)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionQualityDegradingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionQualityDegradingEventArgs {}
impl ::core::fmt::Debug for SpeechRecognitionQualityDegradingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionQualityDegradingEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionQualityDegradingEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs;{4fe24105-8c3a-4c7e-8d0a-5bd4f5b14ad8})");
}
impl ::core::clone::Clone for SpeechRecognitionQualityDegradingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionQualityDegradingEventArgs {
    const IID: ::windows::core::GUID = <ISpeechRecognitionQualityDegradingEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionQualityDegradingEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionQualityDegradingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionQualityDegradingEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognitionQualityDegradingEventArgs {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionResult(::windows::core::IUnknown);
impl SpeechRecognitionResult {
    pub fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionResultStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Confidence(&self) -> ::windows::core::Result<SpeechRecognitionConfidence> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConfidence>();
            (::windows::core::Interface::vtable(this).Confidence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SemanticInterpretation(&self) -> ::windows::core::Result<SpeechRecognitionSemanticInterpretation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionSemanticInterpretation>();
            (::windows::core::Interface::vtable(this).SemanticInterpretation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAlternates(&self, maxalternates: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>>();
            (::windows::core::Interface::vtable(this).GetAlternates)(::windows::core::Interface::as_raw(this), maxalternates, &mut result__).from_abi(result__)
        }
    }
    pub fn Constraint(&self) -> ::windows::core::Result<ISpeechRecognitionConstraint> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ISpeechRecognitionConstraint>();
            (::windows::core::Interface::vtable(this).Constraint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RulePath(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).RulePath)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RawConfidence(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).RawConfidence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhraseStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).PhraseStartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhraseDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).PhraseDuration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionResult {}
impl ::core::fmt::Debug for SpeechRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionResult;{4e303157-034e-4652-857e-d0454cc4beec})");
}
impl ::core::clone::Clone for SpeechRecognitionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionResult {
    const IID: ::windows::core::GUID = <ISpeechRecognitionResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionResult";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionResult {}
unsafe impl ::core::marker::Sync for SpeechRecognitionResult {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionSemanticInterpretation(::windows::core::IUnknown);
impl SpeechRecognitionSemanticInterpretation {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionSemanticInterpretation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionSemanticInterpretation {}
impl ::core::fmt::Debug for SpeechRecognitionSemanticInterpretation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionSemanticInterpretation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionSemanticInterpretation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation;{aae1da9b-7e32-4c1f-89fe-0c65f486f52e})");
}
impl ::core::clone::Clone for SpeechRecognitionSemanticInterpretation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionSemanticInterpretation {
    const IID: ::windows::core::GUID = <ISpeechRecognitionSemanticInterpretation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionSemanticInterpretation {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionSemanticInterpretation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognitionSemanticInterpretation {}
unsafe impl ::core::marker::Sync for SpeechRecognitionSemanticInterpretation {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionTopicConstraint(::windows::core::IUnknown);
impl SpeechRecognitionTopicConstraint {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Tag)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintProbability>();
            (::windows::core::Interface::vtable(this).Probability)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProbability)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scenario(&self) -> ::windows::core::Result<SpeechRecognitionScenario> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionScenario>();
            (::windows::core::Interface::vtable(this).Scenario)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TopicHint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TopicHint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(scenario: SpeechRecognitionScenario, topichint: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionTopicConstraint>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), scenario, ::core::mem::transmute_copy(topichint), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithTag(scenario: SpeechRecognitionScenario, topichint: &::windows::core::HSTRING, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionTopicConstraint>();
            (::windows::core::Interface::vtable(this).CreateWithTag)(::windows::core::Interface::as_raw(this), scenario, ::core::mem::transmute_copy(topichint), ::core::mem::transmute_copy(tag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionTopicConstraintFactory<R, F: FnOnce(&ISpeechRecognitionTopicConstraintFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechRecognitionTopicConstraint, ISpeechRecognitionTopicConstraintFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionTopicConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionTopicConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionTopicConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionTopicConstraint").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionTopicConstraint {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint;{bf6fdf19-825d-4e69-a681-36e48cf1c93e})");
}
impl ::core::clone::Clone for SpeechRecognitionTopicConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraint_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionTopicConstraint {
    const IID: ::windows::core::GUID = <ISpeechRecognitionTopicConstraint as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionTopicConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionTopicConstraint, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionTopicConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionTopicConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionTopicConstraint {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(::windows::core::IUnknown);
impl SpeechRecognitionVoiceCommandDefinitionConstraint {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Tag)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognitionConstraintProbability>();
            (::windows::core::Interface::vtable(this).Probability)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProbability)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionVoiceCommandDefinitionConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionVoiceCommandDefinitionConstraint").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint;{f2791c2b-1ef4-4ae7-9d77-b6ff10b8a3c2})");
}
impl ::core::clone::Clone for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraint_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const IID: ::windows::core::GUID = <ISpeechRecognitionVoiceCommandDefinitionConstraint as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint";
}
::windows::imp::interface_hierarchy!(SpeechRecognitionVoiceCommandDefinitionConstraint, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISpeechRecognitionConstraint> for SpeechRecognitionVoiceCommandDefinitionConstraint {}
unsafe impl ::core::marker::Send for SpeechRecognitionVoiceCommandDefinitionConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionVoiceCommandDefinitionConstraint {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognizer(::windows::core::IUnknown);
impl SpeechRecognizer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechRecognizer, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn CurrentLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Globalization::Language>();
            (::windows::core::Interface::vtable(this).CurrentLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Constraints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>>();
            (::windows::core::Interface::vtable(this).Constraints)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Timeouts(&self) -> ::windows::core::Result<SpeechRecognizerTimeouts> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognizerTimeouts>();
            (::windows::core::Interface::vtable(this).Timeouts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UIOptions(&self) -> ::windows::core::Result<SpeechRecognizerUIOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognizerUIOptions>();
            (::windows::core::Interface::vtable(this).UIOptions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompileConstraintsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>>();
            (::windows::core::Interface::vtable(this).CompileConstraintsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecognizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>();
            (::windows::core::Interface::vtable(this).RecognizeAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecognizeWithUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>();
            (::windows::core::Interface::vtable(this).RecognizeWithUIAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecognitionQualityDegrading(&self, speechrecognitionqualitydegradinghandler: &super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).RecognitionQualityDegrading)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(speechrecognitionqualitydegradinghandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecognitionQualityDegrading(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRecognitionQualityDegrading)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged(&self, statechangedhandler: &super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).StateChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(statechangedhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ContinuousRecognitionSession(&self) -> ::windows::core::Result<SpeechContinuousRecognitionSession> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechContinuousRecognitionSession>();
            (::windows::core::Interface::vtable(this).ContinuousRecognitionSession)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<SpeechRecognizerState> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognizerState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRecognitionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).StopRecognitionAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HypothesisGenerated(&self, value: &super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).HypothesisGenerated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHypothesisGenerated(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHypothesisGenerated)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn Create(language: &super::super::Globalization::Language) -> ::windows::core::Result<SpeechRecognizer> {
        Self::ISpeechRecognizerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognizer>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(language), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn SystemSpeechLanguage() -> ::windows::core::Result<super::super::Globalization::Language> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Globalization::Language>();
            (::windows::core::Interface::vtable(this).SystemSpeechLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn SupportedTopicLanguages() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>();
            (::windows::core::Interface::vtable(this).SupportedTopicLanguages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn SupportedGrammarLanguages() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>();
            (::windows::core::Interface::vtable(this).SupportedGrammarLanguages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    pub fn TrySetSystemSpeechLanguageAsync(speechlanguage: &super::super::Globalization::Language) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISpeechRecognizerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TrySetSystemSpeechLanguageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(speechlanguage), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerFactory<R, F: FnOnce(&ISpeechRecognizerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerStatics<R, F: FnOnce(&ISpeechRecognizerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerStatics2<R, F: FnOnce(&ISpeechRecognizerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizer {}
impl ::core::fmt::Debug for SpeechRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognizer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizer;{0bc3c9cb-c26a-40f2-aeb5-8096b2e48073})");
}
impl ::core::clone::Clone for SpeechRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognizer {
    type Vtable = ISpeechRecognizer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognizer {
    const IID: ::windows::core::GUID = <ISpeechRecognizer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognizer {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizer";
}
::windows::imp::interface_hierarchy!(SpeechRecognizer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for SpeechRecognizer {}
unsafe impl ::core::marker::Send for SpeechRecognizer {}
unsafe impl ::core::marker::Sync for SpeechRecognizer {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognizerStateChangedEventArgs(::windows::core::IUnknown);
impl SpeechRecognizerStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<SpeechRecognizerState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechRecognizerState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizerStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizerStateChangedEventArgs {}
impl ::core::fmt::Debug for SpeechRecognizerStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognizerStateChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs;{563d4f09-ba03-4bad-ad81-ddc6c4dab0c3})");
}
impl ::core::clone::Clone for SpeechRecognizerStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognizerStateChangedEventArgs {
    const IID: ::windows::core::GUID = <ISpeechRecognizerStateChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognizerStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs";
}
::windows::imp::interface_hierarchy!(SpeechRecognizerStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognizerStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognizerStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognizerTimeouts(::windows::core::IUnknown);
impl SpeechRecognizerTimeouts {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitialSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).InitialSilenceTimeout)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInitialSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInitialSilenceTimeout)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).EndSilenceTimeout)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndSilenceTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndSilenceTimeout)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BabbleTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).BabbleTimeout)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBabbleTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBabbleTimeout)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizerTimeouts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizerTimeouts {}
impl ::core::fmt::Debug for SpeechRecognizerTimeouts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerTimeouts").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognizerTimeouts {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts;{2ef76fca-6a3c-4dca-a153-df1bc88a79af})");
}
impl ::core::clone::Clone for SpeechRecognizerTimeouts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeouts_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognizerTimeouts {
    const IID: ::windows::core::GUID = <ISpeechRecognizerTimeouts as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognizerTimeouts {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts";
}
::windows::imp::interface_hierarchy!(SpeechRecognizerTimeouts, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognizerTimeouts {}
unsafe impl ::core::marker::Sync for SpeechRecognizerTimeouts {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognizerUIOptions(::windows::core::IUnknown);
impl SpeechRecognizerUIOptions {
    pub fn ExampleText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ExampleText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExampleText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExampleText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AudiblePrompt(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AudiblePrompt)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudiblePrompt(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudiblePrompt)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsReadBackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsReadBackEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsReadBackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsReadBackEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowConfirmation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ShowConfirmation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowConfirmation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShowConfirmation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizerUIOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizerUIOptions {}
impl ::core::fmt::Debug for SpeechRecognizerUIOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerUIOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognizerUIOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions;{7888d641-b92b-44ba-a25f-d1864630641f})");
}
impl ::core::clone::Clone for SpeechRecognizerUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechRecognizerUIOptions {
    const IID: ::windows::core::GUID = <ISpeechRecognizerUIOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechRecognizerUIOptions {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions";
}
::windows::imp::interface_hierarchy!(SpeechRecognizerUIOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechRecognizerUIOptions {}
unsafe impl ::core::marker::Sync for SpeechRecognizerUIOptions {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
pub struct VoiceCommandManager;
impl VoiceCommandManager {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn InstallCommandSetsFromStorageFileAsync(file: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).InstallCommandSetsFromStorageFileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(file), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstalledCommandSets() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandSet>> {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandSet>>();
            (::windows::core::Interface::vtable(this).InstalledCommandSets)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandManager<R, F: FnOnce(&IVoiceCommandManager) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<VoiceCommandManager, IVoiceCommandManager> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for VoiceCommandManager {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandManager";
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct VoiceCommandSet(::windows::core::IUnknown);
impl VoiceCommandSet {
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Language)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPhraseListAsync<P0>(&self, phraselistname: &::windows::core::HSTRING, phraselist: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SetPhraseListAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(phraselistname), phraselist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandSet {}
impl ::core::fmt::Debug for VoiceCommandSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandSet").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VoiceCommandSet {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.VoiceCommandSet;{0bedda75-46e6-4b11-a088-5c68632899b5})");
}
impl ::core::clone::Clone for VoiceCommandSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VoiceCommandSet {
    type Vtable = IVoiceCommandSet_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VoiceCommandSet {
    const IID: ::windows::core::GUID = <IVoiceCommandSet as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VoiceCommandSet {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandSet";
}
::windows::imp::interface_hierarchy!(VoiceCommandSet, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandSet {}
unsafe impl ::core::marker::Sync for VoiceCommandSet {}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechContinuousRecognitionMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechContinuousRecognitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechContinuousRecognitionMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionMode;i4)");
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechRecognitionAudioProblem {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionAudioProblem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionAudioProblem").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionAudioProblem {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionAudioProblem;i4)");
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechRecognitionConfidence {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConfidence").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionConfidence {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConfidence;i4)");
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechRecognitionConstraintProbability {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionConstraintProbability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConstraintProbability").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionConstraintProbability {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintProbability;i4)");
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechRecognitionConstraintType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionConstraintType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConstraintType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionConstraintType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintType;i4)");
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechRecognitionResultStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionResultStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionResultStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionResultStatus;i4)");
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechRecognitionScenario {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionScenario").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionScenario {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionScenario;i4)");
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
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
impl ::windows::core::TypeKind for SpeechRecognizerState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognizerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognizerState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognizerState;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
