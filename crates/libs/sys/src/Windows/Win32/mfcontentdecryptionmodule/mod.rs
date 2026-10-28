#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "objidlbase"))]
windows_link::link!("mf.dll" "system" fn MFCreateEncryptedMediaExtensionsStoreActivate(pmphost : *mut core::ffi::c_void, objectstream : *mut core::ffi::c_void, classid : windows_sys::core::PCWSTR, activate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MF_CONTENTDECRYPTIONMODULE_INPRIVATESTOREPATH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x730cb3ac_51dc_49da_a578_b95386b62afe), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MF_CONTENTDECRYPTIONMODULE_PMPSTORECONTEXT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x6d2a2835_c3a9_4681_97f2_0af56be93446), pid: 3 };
pub const MF_CONTENTDECRYPTIONMODULE_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x15320c45_ff80_484a_9dcb_0df894e69a01);
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MF_CONTENTDECRYPTIONMODULE_STOREPATH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x77d993b9_ba61_4bb7_92c6_18c86a189c06), pid: 2 };
pub const MF_ENCRYPTEDMEDIAEXTENSIONS_ACTIVATABLE_CLASS_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x77631a31_e5e7_4785_bf17_20f57b224802);
pub const MF_ENCRYPTEDMEDIAEXTENSIONS_ACTIVATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2df7b51e_797b_4d06_be71_d14a52cf8421);
pub const MF_ENCRYPTEDMEDIAEXTENSIONS_INITIALIZATION_DATA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3e73735c_e6c0_481d_8260_ee5db1343b5f);
