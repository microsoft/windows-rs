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
    pub const Default: SpeechContinuousRecognitionMode = SpeechContinuousRecognitionMode(0i32);
    pub const PauseOnRecognition: SpeechContinuousRecognitionMode = SpeechContinuousRecognitionMode(1i32);
}
#[repr(transparent)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechContinuousRecognitionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionAudioProblem(pub i32);
impl SpeechRecognitionAudioProblem {
    pub const None: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(0i32);
    pub const TooNoisy: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(1i32);
    pub const NoSignal: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(2i32);
    pub const TooLoud: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(3i32);
    pub const TooQuiet: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(4i32);
    pub const TooFast: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(5i32);
    pub const TooSlow: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(6i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionCompilationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: SpeechRecognitionConfidence = SpeechRecognitionConfidence(0i32);
    pub const Medium: SpeechRecognitionConfidence = SpeechRecognitionConfidence(1i32);
    pub const Low: SpeechRecognitionConfidence = SpeechRecognitionConfidence(2i32);
    pub const Rejected: SpeechRecognitionConfidence = SpeechRecognitionConfidence(3i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: SpeechRecognitionConstraintProbability = SpeechRecognitionConstraintProbability(0i32);
    pub const Min: SpeechRecognitionConstraintProbability = SpeechRecognitionConstraintProbability(1i32);
    pub const Max: SpeechRecognitionConstraintProbability = SpeechRecognitionConstraintProbability(2i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(0i32);
    pub const List: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(1i32);
    pub const Grammar: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(2i32);
    pub const VoiceCommandDefinition: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(3i32);
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
    pub const Success: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(0i32);
    pub const TopicLanguageNotSupported: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(1i32);
    pub const GrammarLanguageMismatch: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(2i32);
    pub const GrammarCompilationFailure: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(3i32);
    pub const AudioQualityFailure: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(4i32);
    pub const UserCanceled: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(5i32);
    pub const Unknown: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(6i32);
    pub const TimeoutExceeded: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(7i32);
    pub const PauseLimitExceeded: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(8i32);
    pub const NetworkFailure: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(9i32);
    pub const MicrophoneUnavailable: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(10i32);
}
#[repr(transparent)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: SpeechRecognitionScenario = SpeechRecognitionScenario(0i32);
    pub const Dictation: SpeechRecognitionScenario = SpeechRecognitionScenario(1i32);
    pub const FormFilling: SpeechRecognitionScenario = SpeechRecognitionScenario(2i32);
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
    pub const Idle: SpeechRecognizerState = SpeechRecognizerState(0i32);
    pub const Capturing: SpeechRecognizerState = SpeechRecognizerState(1i32);
    pub const Processing: SpeechRecognizerState = SpeechRecognizerState(2i32);
    pub const SoundStarted: SpeechRecognizerState = SpeechRecognizerState(3i32);
    pub const SoundEnded: SpeechRecognizerState = SpeechRecognizerState(4i32);
    pub const SpeechDetected: SpeechRecognizerState = SpeechRecognizerState(5i32);
    pub const Paused: SpeechRecognizerState = SpeechRecognizerState(6i32);
}
#[repr(transparent)]
pub struct SpeechRecognizerStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizerTimeouts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizerUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandSet(pub *mut ::core::ffi::c_void);
