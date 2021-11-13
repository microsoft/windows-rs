#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IVoiceCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommand {}
impl ::core::clone::Clone for IVoiceCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandCompletedEventArgs {}
impl ::core::clone::Clone for IVoiceCommandCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandConfirmationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandConfirmationResult {}
impl ::core::clone::Clone for IVoiceCommandConfirmationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandContentTile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandContentTile {}
impl ::core::clone::Clone for IVoiceCommandContentTile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandDefinition {}
impl ::core::clone::Clone for IVoiceCommandDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandDefinitionManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandDefinitionManagerStatics {}
impl ::core::clone::Clone for IVoiceCommandDefinitionManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandDisambiguationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandDisambiguationResult {}
impl ::core::clone::Clone for IVoiceCommandDisambiguationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandResponse {}
impl ::core::clone::Clone for IVoiceCommandResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandResponseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandResponseStatics {}
impl ::core::clone::Clone for IVoiceCommandResponseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandServiceConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandServiceConnection {}
impl ::core::clone::Clone for IVoiceCommandServiceConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandServiceConnectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandServiceConnectionStatics {}
impl ::core::clone::Clone for IVoiceCommandServiceConnectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandUserMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandUserMessage {}
impl ::core::clone::Clone for IVoiceCommandUserMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommand {}
impl ::core::clone::Clone for VoiceCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandCompletedEventArgs {}
impl ::core::clone::Clone for VoiceCommandCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for VoiceCommandCompletionReason {}
impl ::core::clone::Clone for VoiceCommandCompletionReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandConfirmationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandConfirmationResult {}
impl ::core::clone::Clone for VoiceCommandConfirmationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandContentTile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandContentTile {}
impl ::core::clone::Clone for VoiceCommandContentTile {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for VoiceCommandContentTileType {}
impl ::core::clone::Clone for VoiceCommandContentTileType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandDefinition {}
impl ::core::clone::Clone for VoiceCommandDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandDisambiguationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandDisambiguationResult {}
impl ::core::clone::Clone for VoiceCommandDisambiguationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandResponse {}
impl ::core::clone::Clone for VoiceCommandResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandServiceConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandServiceConnection {}
impl ::core::clone::Clone for VoiceCommandServiceConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandUserMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandUserMessage {}
impl ::core::clone::Clone for VoiceCommandUserMessage {
    fn clone(&self) -> Self {
        *self
    }
}
