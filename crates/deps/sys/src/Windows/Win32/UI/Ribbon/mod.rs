#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUIApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUICollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUICollectionChangedEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUICommandHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIContextualUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIEventLogger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIEventingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIFramework(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIImageFromBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIRibbon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUISimplePropertySet(pub *mut ::core::ffi::c_void);
pub const LIBID_UIRibbon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2486121922, data2: 59451, data3: 17903, data4: [176, 133, 172, 41, 93, 214, 61, 91] };
#[repr(C)]
pub struct UIRibbonFramework(i32);
#[repr(C)]
pub struct UIRibbonImageFromBitmapFactory(i32);
pub const UI_ALL_COMMANDS: u32 = 0u32;
#[repr(C)]
pub struct UI_COLLECTIONCHANGE(i32);
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
#[repr(C)]
pub struct UI_COMMANDTYPE(i32);
#[repr(C)]
pub struct UI_CONTEXTAVAILABILITY(i32);
#[repr(C)]
pub struct UI_CONTROLDOCK(i32);
#[repr(C)]
pub struct UI_EVENTLOCATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UI_EVENTPARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UI_EVENTPARAMS_COMMAND(i32);
#[repr(C)]
pub struct UI_EVENTTYPE(i32);
#[repr(C)]
pub struct UI_EXECUTIONVERB(i32);
#[repr(C)]
pub struct UI_FONTDELTASIZE(i32);
#[repr(C)]
pub struct UI_FONTPROPERTIES(i32);
#[repr(C)]
pub struct UI_FONTUNDERLINE(i32);
#[repr(C)]
pub struct UI_FONTVERTICALPOSITION(i32);
#[repr(C)]
pub struct UI_INVALIDATIONS(i32);
#[repr(C)]
pub struct UI_OWNERSHIP(i32);
#[repr(C)]
pub struct UI_SWATCHCOLORMODE(i32);
#[repr(C)]
pub struct UI_SWATCHCOLORTYPE(i32);
#[repr(C)]
pub struct UI_VIEWTYPE(i32);
#[repr(C)]
pub struct UI_VIEWVERB(i32);
