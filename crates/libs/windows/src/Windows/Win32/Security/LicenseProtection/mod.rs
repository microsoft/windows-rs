pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = 4i32;
pub const LicenseKeyCorrupted: LicenseProtectionStatus = 3i32;
pub const LicenseKeyNotFound: LicenseProtectionStatus = 1i32;
pub const LicenseKeyUnprotected: LicenseProtectionStatus = 2i32;
pub const Success: LicenseProtectionStatus = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LicenseProtectionStatus(pub i32);
impl windows_core::TypeKind for LicenseProtectionStatus {
    type TypeKind = windows_core::CopyType;
}
