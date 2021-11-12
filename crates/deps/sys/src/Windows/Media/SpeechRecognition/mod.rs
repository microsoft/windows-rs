#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionCompilationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionHypothesis(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionListConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionListConstraintFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionQualityDegradingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionSemanticInterpretation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraintFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizerStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizerTimeouts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizerUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechContinuousRecognitionCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechContinuousRecognitionMode(pub i32);
impl SpeechContinuousRecognitionMode {
    pub const Default: Self = Self(0i32);
    pub const PauseOnRecognition: Self = Self(1i32);
}
#[repr(transparent)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechContinuousRecognitionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct SpeechRecognitionCompilationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Rejected: Self = Self(3i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
    pub const Max: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: Self = Self(0i32);
    pub const List: Self = Self(1i32);
    pub const Grammar: Self = Self(2i32);
    pub const VoiceCommandDefinition: Self = Self(3i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionGrammarFileConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionHypothesis(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionHypothesisGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionListConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionQualityDegradingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: Self = Self(0i32);
    pub const Dictation: Self = Self(1i32);
    pub const FormFilling: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionSemanticInterpretation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionTopicConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct SpeechRecognizerStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizerTimeouts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizerUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandSet(pub *mut ::core::ffi::c_void);
