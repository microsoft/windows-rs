#[cfg(feature = "Win32_mediaobj")]
windows_link::link!("msdmo.dll" "system" fn DMOEnum(guidcategory : *const windows_sys::core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE, ppenum : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("msdmo.dll" "system" fn DMOGetName(clsiddmo : *const windows_sys::core::GUID, szname : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("msdmo.dll" "system" fn DMOGetTypes(clsiddmo : *const windows_sys::core::GUID, ulinputtypesrequested : u32, pulinputtypessupplied : *mut u32, pinputtypes : *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested : u32, puloutputtypessupplied : *mut u32, poutputtypes : *mut DMO_PARTIAL_MEDIATYPE) -> windows_sys::core::HRESULT);
windows_link::link!("msdmo.dll" "system" fn DMORegister(szname : windows_sys::core::PCWSTR, clsiddmo : *const windows_sys::core::GUID, guidcategory : *const windows_sys::core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE) -> windows_sys::core::HRESULT);
windows_link::link!("msdmo.dll" "system" fn DMOUnregister(clsiddmo : *const windows_sys::core::GUID, guidcategory : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AGC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf665aaba_3e09_4920_aa5f_219811148f09);
pub const DMOCATEGORY_AUDIO_DECODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57f2db8b_e6bb_4513_9d43_dcd2a6593125);
pub const DMOCATEGORY_AUDIO_EFFECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3602b3f_0592_48df_a4cd_674721e7ebeb);
pub const DMOCATEGORY_AUDIO_ENCODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const DMOCATEGORY_VIDEO_DECODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4a69b442_28be_4991_969c_b500adf5d8a8);
pub const DMOCATEGORY_VIDEO_EFFECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd990ee14_776c_4723_be46_3da2f56f10b9);
pub const DMOCATEGORY_VIDEO_ENCODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = 1;
pub type DMO_ENUM_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: windows_sys::core::GUID,
    pub subtype: windows_sys::core::GUID,
}
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = 1;
pub type DMO_REGISTER_FLAGS = i32;
pub type PDMO_PARTIAL_MEDIATYPE = *mut DMO_PARTIAL_MEDIATYPE;
