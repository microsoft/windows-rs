#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: windows_core::GUID,
    pub TenantId: windows_core::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for UAL_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for UAL_DATA_BLOB {
    type TypeKind = windows_core::CloneType;
}
