#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct SpeechContinuousRecognitionMode(i32);
#[repr(transparent)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechContinuousRecognitionSession(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpeechRecognitionAudioProblem(i32);
#[repr(transparent)]
pub struct SpeechRecognitionCompilationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpeechRecognitionConfidence(i32);
#[repr(C)]
pub struct SpeechRecognitionConstraintProbability(i32);
#[repr(C)]
pub struct SpeechRecognitionConstraintType(i32);
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
#[repr(C)]
pub struct SpeechRecognitionResultStatus(i32);
#[repr(C)]
pub struct SpeechRecognitionScenario(i32);
#[repr(transparent)]
pub struct SpeechRecognitionSemanticInterpretation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionTopicConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpeechRecognizerState(i32);
#[repr(transparent)]
pub struct SpeechRecognizerStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizerTimeouts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechRecognizerUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandSet(pub *mut ::core::ffi::c_void);
