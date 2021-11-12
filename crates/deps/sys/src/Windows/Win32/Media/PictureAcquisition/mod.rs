#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DEVICE_SELECTION_DEVICE_TYPE(i32);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_ALL_DEVICES: u32 = 65535u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_CPL_MODE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_DV_DEVICES: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_FS_DEVICES: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_SHOW_OFFLINE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_STI_DEVICES: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_TWAIN_DEVICES: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_WIA_CAMERAS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_WIA_SCANNERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_WPD_DEVICES: u32 = 1u32;
pub struct ERROR_ADVISE_MESSAGE_TYPE(i32);
pub struct ERROR_ADVISE_RESULT(i32);
pub struct IPhotoAcquire(i32);
pub struct IPhotoAcquireDeviceSelectionDialog(i32);
pub struct IPhotoAcquireItem(i32);
pub struct IPhotoAcquireOptionsDialog(i32);
pub struct IPhotoAcquirePlugin(i32);
pub struct IPhotoAcquireProgressCB(i32);
pub struct IPhotoAcquireSettings(i32);
pub struct IPhotoAcquireSource(i32);
pub struct IPhotoProgressActionCB(i32);
pub struct IPhotoProgressDialog(i32);
pub struct IUserInputString(i32);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PAPS_CLEANUP: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PAPS_POSTSAVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PAPS_PRESAVE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_ABORT_ON_SETTINGS_UPDATE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DELETE_AFTER_ACQUIRE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_AUTO_ROTATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_DB_INTEGRATION: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_DUPLICATE_DETECTION: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_GROUP_TAG_PROMPT: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_METADATA_WRITE: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_PLUGINS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_SETTINGS_LINK: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_THUMBNAIL_PROGRESS: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_ENABLE_THUMBNAIL_CACHING: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_ERROR_RESTART_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180543i32 as _);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_IMPORT_VIDEO_AS_MULTIPLE_FILES: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_NO_GALLERY_LAUNCH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_RUN_DEFAULT: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_FinalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_GroupTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_RelativePathname: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_SkipImport: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PhotoAcquire_TransferResult: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] },
    pid: 5u32,
};
pub struct PROGRESS_DIALOG_CHECKBOX_ID(i32);
pub struct PROGRESS_DIALOG_IMAGE_TYPE(i32);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
pub struct PhotoAcquire(i32);
pub struct PhotoAcquireAutoPlayDropTarget(i32);
pub struct PhotoAcquireAutoPlayHWEventHandler(i32);
pub struct PhotoAcquireDeviceSelectionDialog(i32);
pub struct PhotoAcquireOptionsDialog(i32);
pub struct PhotoProgressDialog(i32);
pub struct USER_INPUT_STRING_TYPE(i32);
