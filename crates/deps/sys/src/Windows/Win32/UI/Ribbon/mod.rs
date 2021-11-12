#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IUIApplication(i32);
pub struct IUICollection(i32);
pub struct IUICollectionChangedEvent(i32);
pub struct IUICommandHandler(i32);
pub struct IUIContextualUI(i32);
pub struct IUIEventLogger(i32);
pub struct IUIEventingManager(i32);
pub struct IUIFramework(i32);
pub struct IUIImage(i32);
pub struct IUIImageFromBitmap(i32);
pub struct IUIRibbon(i32);
pub struct IUISimplePropertySet(i32);
pub const LIBID_UIRibbon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2486121922, data2: 59451, data3: 17903, data4: [176, 133, 172, 41, 93, 214, 61, 91] };
pub struct UIRibbonFramework(i32);
pub struct UIRibbonImageFromBitmapFactory(i32);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
pub const UI_ALL_COMMANDS: u32 = 0u32;
pub struct UI_COLLECTIONCHANGE(i32);
#[doc = "*Required features: `Win32_UI_Ribbon`*"]
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
pub struct UI_COMMANDTYPE(i32);
pub struct UI_CONTEXTAVAILABILITY(i32);
pub struct UI_CONTROLDOCK(i32);
pub struct UI_EVENTLOCATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct UI_EVENTPARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct UI_EVENTPARAMS_COMMAND(i32);
pub struct UI_EVENTTYPE(i32);
pub struct UI_EXECUTIONVERB(i32);
pub struct UI_FONTDELTASIZE(i32);
pub struct UI_FONTPROPERTIES(i32);
pub struct UI_FONTUNDERLINE(i32);
pub struct UI_FONTVERTICALPOSITION(i32);
pub struct UI_INVALIDATIONS(i32);
pub struct UI_OWNERSHIP(i32);
pub struct UI_SWATCHCOLORMODE(i32);
pub struct UI_SWATCHCOLORTYPE(i32);
pub struct UI_VIEWTYPE(i32);
pub struct UI_VIEWVERB(i32);
