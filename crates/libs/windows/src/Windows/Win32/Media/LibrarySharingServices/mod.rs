pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = 1i32;
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = 2i32;
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(pub i32);
impl windows_core::TypeKind for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    type TypeKind = windows_core::CopyType;
}
pub const WindowsMediaLibrarySharingServices: windows_core::GUID = windows_core::GUID::from_u128(0xad581b00_7b64_4e59_a38d_d2c5bf51ddb3);
