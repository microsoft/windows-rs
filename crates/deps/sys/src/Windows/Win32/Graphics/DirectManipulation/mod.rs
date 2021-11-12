#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_AutoScrollBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 638741073,
    data2: 15472,
    data3: 19610,
    data4: [174, 194, 148, 136, 73, 238, 176, 147],
};
pub const CLSID_DeferContactService: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3619060980,
    data2: 33979,
    data3: 17230,
    data4: [134, 174, 101, 146, 187, 201, 171, 217],
};
pub const CLSID_DragDropConfigurationBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 162536254, data2: 47724, data3: 17741, data4: [130, 232, 149, 227, 82, 50, 159, 35] };
pub const CLSID_HorizontalIndicatorContent: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3889270005,
    data2: 16071,
    data3: 17621,
    data4: [167, 107, 55, 112, 243, 207, 144, 61],
};
pub const CLSID_VerticalIndicatorContent: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2701877015,
    data2: 45024,
    data3: 19106,
    data4: [145, 233, 62, 112, 1, 210, 230, 180],
};
pub const CLSID_VirtualViewportContent: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 839295386,
    data2: 34544,
    data3: 19636,
    data4: [167, 243, 22, 227, 183, 226, 216, 82],
};
pub struct DCompManipulationCompositor(i32);
pub struct DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(i32);
pub struct DIRECTMANIPULATION_CONFIGURATION(i32);
pub struct DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(i32);
pub struct DIRECTMANIPULATION_DRAG_DROP_STATUS(i32);
pub struct DIRECTMANIPULATION_GESTURE_CONFIGURATION(i32);
pub struct DIRECTMANIPULATION_HITTEST_TYPE(i32);
pub struct DIRECTMANIPULATION_HORIZONTALALIGNMENT(i32);
pub struct DIRECTMANIPULATION_INPUT_MODE(i32);
pub struct DIRECTMANIPULATION_INTERACTION_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
pub struct DIRECTMANIPULATION_MOTION_TYPES(i32);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
pub struct DIRECTMANIPULATION_SNAPPOINT_COORDINATE(i32);
pub struct DIRECTMANIPULATION_SNAPPOINT_TYPE(i32);
pub struct DIRECTMANIPULATION_STATUS(i32);
pub struct DIRECTMANIPULATION_VERTICALALIGNMENT(i32);
pub struct DIRECTMANIPULATION_VIEWPORT_OPTIONS(i32);
pub struct DirectManipulationManager(i32);
pub struct DirectManipulationPrimaryContent(i32);
pub struct DirectManipulationSharedManager(i32);
pub struct DirectManipulationUpdateManager(i32);
pub struct DirectManipulationViewport(i32);
#[repr(transparent)]
pub struct IDirectManipulationAutoScrollBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationCompositor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationCompositor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationDeferContactService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationDragDropBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationDragDropEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationFrameInfoProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationInteractionEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationManager3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationPrimaryContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationUpdateHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationUpdateManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationViewport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationViewport2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectManipulationViewportEventHandler(pub *mut ::core::ffi::c_void);
