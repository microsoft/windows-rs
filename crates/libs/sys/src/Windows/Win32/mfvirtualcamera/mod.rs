#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateVirtualCamera(r#type : MFVirtualCameraType, lifetime : MFVirtualCameraLifetime, access : MFVirtualCameraAccess, friendlyname : windows_sys::core::PCWSTR, sourceid : windows_sys::core::PCWSTR, categories : *const windows_sys::core::GUID, categorycount : super::ULONG, virtualcamera : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFIsVirtualCameraTypeSupported(r#type : MFVirtualCameraType, supported : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_IsVirtualCamera: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 3 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_IsWindowsCameraEffectAvailable: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 4 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_VirtualCameraAssociatedCameras: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 5 };
pub type MFVirtualCameraAccess = i32;
pub const MFVirtualCameraAccess_AllUsers: MFVirtualCameraAccess = 1;
pub const MFVirtualCameraAccess_CurrentUser: MFVirtualCameraAccess = 0;
pub type MFVirtualCameraLifetime = i32;
pub const MFVirtualCameraLifetime_Session: MFVirtualCameraLifetime = 0;
pub const MFVirtualCameraLifetime_System: MFVirtualCameraLifetime = 1;
pub type MFVirtualCameraType = i32;
pub const MFVirtualCameraType_SoftwareCameraSource: MFVirtualCameraType = 0;
pub const MF_DEVSOURCE_ATTRIBUTE_ENABLE_MS_CAMERA_EFFECTS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x28a5531a_57dd_4fd5_aaa7_385abf57d785);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_CUSTOM_EVENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6e59489c_47d3_4467_83ef_12d34e871665);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_PIPELINE_SHUTDOWN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x45a81b31_43f8_4e5d_8ce2_22dce026996d);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_INITIALIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe52c4dff_e46d_4d0b_bc75_ddd4c8723f96);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_START: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb1eeb989_b456_4f4a_ae40_079c28e24af8);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_STOP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb7fe7a61_fe91_415e_8608_d37dedb1a58b);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_UNINITIALIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa0ebaba7_a422_4e33_8401_b37d2800aa67);
pub const MF_VIRTUALCAMERA_ASSOCIATED_CAMERA_SOURCES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1bb79e7c_5d83_438c_94d8_e5f0df6d3279);
pub const MF_VIRTUALCAMERA_CONFIGURATION_APP_PACKAGE_FAMILY_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x658abe51_8044_462e_97ea_e676fd72055f);
pub const MF_VIRTUALCAMERA_PROVIDE_ASSOCIATED_CAMERA_SOURCES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf0273718_4a4d_4ac5_a15d_305eb5e90667);
pub type PMFVirtualCameraAccess = *mut MFVirtualCameraAccess;
pub type PMFVirtualCameraLifetime = *mut MFVirtualCameraLifetime;
pub type PMFVirtualCameraType = *mut MFVirtualCameraType;
