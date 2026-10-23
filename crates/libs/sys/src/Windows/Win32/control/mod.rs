pub const CLSID_FilgraphManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436ebb3_524f_11ce_9f53_0020af0ba770);
pub const FilgraphManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe436ebb3_524f_11ce_9f53_0020af0ba770);
pub const IID_IAMCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b9_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IAMStats: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc9bcf80_dcd2_11d2_abf6_00a0c905f375);
pub const IID_IBasicAudio: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b3_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IBasicVideo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b5_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IBasicVideo2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x329bb360_f6ea_11d1_9038_00a0c9697298);
pub const IID_IDeferredCommand: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b8_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IFilterInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868ba_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b1_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaEvent: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b6_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaEventEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868c0_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaPosition: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b2_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaTypeInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868bc_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IPinInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868bd_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IQueueCommand: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b7_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IRegFilterInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868bb_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IVideoWindow: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b4_0ad4_11ce_b03a_0020af0ba770);
pub const LIBID_QuartzTypeLib: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868b0_0ad4_11ce_b03a_0020af0ba770);
#[cfg(target_arch = "x86")]
pub type OAEVENT = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type OAEVENT = i64;
pub type OAFilterState = i32;
#[cfg(target_arch = "x86")]
pub type OAHWND = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type OAHWND = i64;
