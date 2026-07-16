pub type AudioSessionDisconnectReason = i32;
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = 0;
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = 5;
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = 2;
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = 1;
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = 4;
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = 3;
