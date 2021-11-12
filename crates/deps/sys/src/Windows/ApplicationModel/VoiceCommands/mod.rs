#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct VoiceCommandCompletionReason(pub i32);
impl VoiceCommandCompletionReason {
    pub const Unknown: Self = Self(0i32);
    pub const CommunicationFailed: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Canceled: Self = Self(3i32);
    pub const TimeoutExceeded: Self = Self(4i32);
    pub const AppLaunched: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
#[repr(transparent)]
pub struct VoiceCommandConfirmationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandContentTile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandContentTileType(pub i32);
impl VoiceCommandContentTileType {
    pub const TitleOnly: Self = Self(0i32);
    pub const TitleWithText: Self = Self(1i32);
    pub const TitleWith68x68Icon: Self = Self(2i32);
    pub const TitleWith68x68IconAndText: Self = Self(3i32);
    pub const TitleWith68x92Icon: Self = Self(4i32);
    pub const TitleWith68x92IconAndText: Self = Self(5i32);
    pub const TitleWith280x140Icon: Self = Self(6i32);
    pub const TitleWith280x140IconAndText: Self = Self(7i32);
}
#[repr(transparent)]
pub struct VoiceCommandDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandDisambiguationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceCommandUserMessage(pub *mut ::core::ffi::c_void);
