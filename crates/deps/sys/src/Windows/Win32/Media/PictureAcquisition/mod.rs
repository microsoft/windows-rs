#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DEVICE_SELECTION_DEVICE_TYPE(pub i32);
pub const DST_UNKNOWN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(0i32);
pub const DST_WPD_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(1i32);
pub const DST_WIA_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(2i32);
pub const DST_STI_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(3i32);
pub const DSF_TWAIN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(4i32);
pub const DST_FS_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(5i32);
pub const DST_DV_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(6i32);
impl ::core::marker::Copy for DEVICE_SELECTION_DEVICE_TYPE {}
impl ::core::clone::Clone for DEVICE_SELECTION_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DSF_ALL_DEVICES: u32 = 65535u32;
pub const DSF_CPL_MODE: u32 = 65536u32;
pub const DSF_DV_DEVICES: u32 = 64u32;
pub const DSF_FS_DEVICES: u32 = 32u32;
pub const DSF_SHOW_OFFLINE: u32 = 131072u32;
pub const DSF_STI_DEVICES: u32 = 8u32;
pub const DSF_TWAIN_DEVICES: u32 = 16u32;
pub const DSF_WIA_CAMERAS: u32 = 2u32;
pub const DSF_WIA_SCANNERS: u32 = 4u32;
pub const DSF_WPD_DEVICES: u32 = 1u32;
#[repr(transparent)]
pub struct ERROR_ADVISE_MESSAGE_TYPE(pub i32);
pub const PHOTOACQUIRE_ERROR_SKIPRETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(0i32);
pub const PHOTOACQUIRE_ERROR_RETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(1i32);
pub const PHOTOACQUIRE_ERROR_YESNO: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(2i32);
pub const PHOTOACQUIRE_ERROR_OK: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(3i32);
impl ::core::marker::Copy for ERROR_ADVISE_MESSAGE_TYPE {}
impl ::core::clone::Clone for ERROR_ADVISE_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ERROR_ADVISE_RESULT(pub i32);
pub const PHOTOACQUIRE_RESULT_YES: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(0i32);
pub const PHOTOACQUIRE_RESULT_NO: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(1i32);
pub const PHOTOACQUIRE_RESULT_OK: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(2i32);
pub const PHOTOACQUIRE_RESULT_SKIP: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(3i32);
pub const PHOTOACQUIRE_RESULT_SKIP_ALL: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(4i32);
pub const PHOTOACQUIRE_RESULT_RETRY: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(5i32);
pub const PHOTOACQUIRE_RESULT_ABORT: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(6i32);
impl ::core::marker::Copy for ERROR_ADVISE_RESULT {}
impl ::core::clone::Clone for ERROR_ADVISE_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoAcquire(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoAcquireDeviceSelectionDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoAcquireItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoAcquireOptionsDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoAcquirePlugin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoAcquireProgressCB(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoAcquireSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoAcquireSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoProgressActionCB(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoProgressDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserInputString(pub *mut ::core::ffi::c_void);
pub const PAPS_CLEANUP: u32 = 2u32;
pub const PAPS_POSTSAVE: u32 = 1u32;
pub const PAPS_PRESAVE: u32 = 0u32;
pub const PHOTOACQ_ABORT_ON_SETTINGS_UPDATE: u32 = 2048u32;
pub const PHOTOACQ_DELETE_AFTER_ACQUIRE: u32 = 32u32;
pub const PHOTOACQ_DISABLE_AUTO_ROTATE: u32 = 2u32;
pub const PHOTOACQ_DISABLE_DB_INTEGRATION: u32 = 16u32;
pub const PHOTOACQ_DISABLE_DUPLICATE_DETECTION: u32 = 64u32;
pub const PHOTOACQ_DISABLE_GROUP_TAG_PROMPT: u32 = 8u32;
pub const PHOTOACQ_DISABLE_METADATA_WRITE: u32 = 256u32;
pub const PHOTOACQ_DISABLE_PLUGINS: u32 = 4u32;
pub const PHOTOACQ_DISABLE_SETTINGS_LINK: u32 = 1024u32;
pub const PHOTOACQ_DISABLE_THUMBNAIL_PROGRESS: u32 = 512u32;
pub const PHOTOACQ_ENABLE_THUMBNAIL_CACHING: u32 = 128u32;
pub const PHOTOACQ_ERROR_RESTART_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180543i32 as _);
pub const PHOTOACQ_IMPORT_VIDEO_AS_MULTIPLE_FILES: u32 = 4096u32;
pub const PHOTOACQ_NO_GALLERY_LAUNCH: u32 = 1u32;
pub const PHOTOACQ_RUN_DEFAULT: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_FinalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_GroupTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_RelativePathname: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_SkipImport: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_TransferResult: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 5u32,
};
#[repr(transparent)]
pub struct PROGRESS_DIALOG_CHECKBOX_ID(pub i32);
pub const PROGRESS_DIALOG_CHECKBOX_ID_DEFAULT: PROGRESS_DIALOG_CHECKBOX_ID = PROGRESS_DIALOG_CHECKBOX_ID(0i32);
impl ::core::marker::Copy for PROGRESS_DIALOG_CHECKBOX_ID {}
impl ::core::clone::Clone for PROGRESS_DIALOG_CHECKBOX_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PROGRESS_DIALOG_IMAGE_TYPE(pub i32);
pub const PROGRESS_DIALOG_ICON_SMALL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(0i32);
pub const PROGRESS_DIALOG_ICON_LARGE: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(1i32);
pub const PROGRESS_DIALOG_ICON_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(2i32);
pub const PROGRESS_DIALOG_BITMAP_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(3i32);
impl ::core::marker::Copy for PROGRESS_DIALOG_IMAGE_TYPE {}
impl ::core::clone::Clone for PROGRESS_DIALOG_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
pub const PhotoAcquire: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15887874, data2: 59890, data3: 19103, data4: [159, 221, 90, 150, 47, 178, 106, 152] };
pub const PhotoAcquireAutoPlayDropTarget: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15863477, data2: 36822, data3: 19869, data4: [183, 94, 54, 128, 23, 102, 200, 241] };
pub const PhotoAcquireAutoPlayHWEventHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15905843, data2: 17636, data3: 19848, data4: [178, 176, 38, 152, 160, 169, 29, 186] };
pub const PhotoAcquireDeviceSelectionDialog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15899188, data2: 47265, data3: 18476, data4: [188, 248, 58, 199, 176, 254, 143, 98] };
pub const PhotoAcquireOptionsDialog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15863969, data2: 25328, data3: 17291, data4: [159, 126, 150, 24, 215, 42, 24, 49] };
pub const PhotoProgressDialog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15879328, data2: 29839, data3: 20106, data4: [137, 79, 14, 3, 87, 198, 121, 159] };
#[repr(transparent)]
pub struct USER_INPUT_STRING_TYPE(pub i32);
pub const USER_INPUT_DEFAULT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(0i32);
pub const USER_INPUT_PATH_ELEMENT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(1i32);
impl ::core::marker::Copy for USER_INPUT_STRING_TYPE {}
impl ::core::clone::Clone for USER_INPUT_STRING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
