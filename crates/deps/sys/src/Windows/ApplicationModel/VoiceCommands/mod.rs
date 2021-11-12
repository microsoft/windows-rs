#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IVoiceCommand(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandConfirmationResult(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandContentTile(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandDefinition(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandDefinitionManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandDisambiguationResult(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandResponse(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandResponseStatics(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandServiceConnection(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandServiceConnectionStatics(pub *mut ::core::ffi::c_void);
pub struct IVoiceCommandUserMessage(pub *mut ::core::ffi::c_void);
pub struct VoiceCommand(i32);
pub struct VoiceCommandCompletedEventArgs(i32);
pub struct VoiceCommandCompletionReason(i32);
pub struct VoiceCommandConfirmationResult(i32);
pub struct VoiceCommandContentTile(i32);
pub struct VoiceCommandContentTileType(i32);
pub struct VoiceCommandDefinition(i32);
pub struct VoiceCommandDefinitionManager(i32);
pub struct VoiceCommandDisambiguationResult(i32);
pub struct VoiceCommandResponse(i32);
pub struct VoiceCommandServiceConnection(i32);
pub struct VoiceCommandUserMessage(i32);
