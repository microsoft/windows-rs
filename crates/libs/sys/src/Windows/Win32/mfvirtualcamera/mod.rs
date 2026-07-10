#[cfg(feature = "mfobjects")]
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateVirtualCamera(r#type : MFVirtualCameraType, lifetime : MFVirtualCameraLifetime, access : MFVirtualCameraAccess, friendlyname : windows_sys::core::PCWSTR, sourceid : windows_sys::core::PCWSTR, categories : *const windows_sys::core::GUID, categorycount : u32, virtualcamera : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFIsVirtualCameraTypeSupported(r#type : MFVirtualCameraType, supported : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_IsVirtualCamera: super::devpropdef::DEVPROPKEY = super::devpropdef::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 3 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_IsWindowsCameraEffectAvailable: super::devpropdef::DEVPROPKEY = super::devpropdef::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 4 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_VirtualCameraAssociatedCameras: super::devpropdef::DEVPROPKEY = super::devpropdef::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 5 };
pub type MFVirtualCameraAccess = i32;
pub const MFVirtualCameraAccess_AllUsers: MFVirtualCameraAccess = 1;
pub const MFVirtualCameraAccess_CurrentUser: MFVirtualCameraAccess = 0;
pub type MFVirtualCameraLifetime = i32;
pub const MFVirtualCameraLifetime_Session: MFVirtualCameraLifetime = 0;
pub const MFVirtualCameraLifetime_System: MFVirtualCameraLifetime = 1;
pub type MFVirtualCameraType = i32;
pub const MFVirtualCameraType_SoftwareCameraSource: MFVirtualCameraType = 0;
pub type PMFVirtualCameraAccess = *mut MFVirtualCameraAccess;
pub type PMFVirtualCameraLifetime = *mut MFVirtualCameraLifetime;
pub type PMFVirtualCameraType = *mut MFVirtualCameraType;
