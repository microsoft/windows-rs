pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: super::super::Foundation::BOOL,
    pub pD3D12Device: Option<windows_core::IUnknown>,
    pub ppD3D12Queues: [Option<windows_core::IUnknown>; 2],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
impl Default for D3D9ON12_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for D3D9ON12_ARGS {
    type TypeKind = windows_core::CloneType;
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12 = Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> Option<super::Direct3D9::IDirect3D9>>;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12Ex = Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut Option<super::Direct3D9::IDirect3D9Ex>) -> windows_core::HRESULT>;
