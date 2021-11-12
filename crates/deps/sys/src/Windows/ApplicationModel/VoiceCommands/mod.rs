#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IVoiceCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandConfirmationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandContentTile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandDefinitionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandDisambiguationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandResponseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandServiceConnectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceCommandUserMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct VoiceCommandCompletionReason(i32);
#[repr(transparent)]
pub struct VoiceCommandConfirmationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandContentTile(pub *mut ::core::ffi::c_void);
pub struct VoiceCommandContentTileType(i32);
#[repr(transparent)]
pub struct VoiceCommandDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandDefinitionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandDisambiguationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandUserMessage(pub *mut ::core::ffi::c_void);
