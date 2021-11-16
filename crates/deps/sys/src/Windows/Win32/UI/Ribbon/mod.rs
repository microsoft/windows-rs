#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUIApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIApplication {}
impl ::core::clone::Clone for IUIApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUICollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUICollection {}
impl ::core::clone::Clone for IUICollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUICollectionChangedEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUICollectionChangedEvent {}
impl ::core::clone::Clone for IUICollectionChangedEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUICommandHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUICommandHandler {}
impl ::core::clone::Clone for IUICommandHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIContextualUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIContextualUI {}
impl ::core::clone::Clone for IUIContextualUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIEventLogger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIEventLogger {}
impl ::core::clone::Clone for IUIEventLogger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIEventingManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIEventingManager {}
impl ::core::clone::Clone for IUIEventingManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIFramework(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIFramework {}
impl ::core::clone::Clone for IUIFramework {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIImage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIImage {}
impl ::core::clone::Clone for IUIImage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIImageFromBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIImageFromBitmap {}
impl ::core::clone::Clone for IUIImageFromBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIRibbon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIRibbon {}
impl ::core::clone::Clone for IUIRibbon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUISimplePropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUISimplePropertySet {}
impl ::core::clone::Clone for IUISimplePropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LIBID_UIRibbon: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2486121922, data2: 59451, data3: 17903, data4: [176, 133, 172, 41, 93, 214, 61, 91] };
pub const UIRibbonFramework: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2456242682, data2: 9749, data3: 18823, data4: [136, 69, 195, 62, 101, 242, 185, 87] };
pub const UIRibbonImageFromBitmapFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 259273910,
    data2: 22966,
    data3: 16976,
    data4: [153, 158, 209, 104, 214, 174, 66, 147],
};
pub const UI_ALL_COMMANDS: u32 = 0u32;
pub const UI_COLLECTIONCHANGE_INSERT: i32 = 0i32;
pub const UI_COLLECTIONCHANGE_REMOVE: i32 = 1i32;
pub const UI_COLLECTIONCHANGE_REPLACE: i32 = 2i32;
pub const UI_COLLECTIONCHANGE_RESET: i32 = 3i32;
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
pub const UI_COMMANDTYPE_UNKNOWN: i32 = 0i32;
pub const UI_COMMANDTYPE_GROUP: i32 = 1i32;
pub const UI_COMMANDTYPE_ACTION: i32 = 2i32;
pub const UI_COMMANDTYPE_ANCHOR: i32 = 3i32;
pub const UI_COMMANDTYPE_CONTEXT: i32 = 4i32;
pub const UI_COMMANDTYPE_COLLECTION: i32 = 5i32;
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: i32 = 6i32;
pub const UI_COMMANDTYPE_DECIMAL: i32 = 7i32;
pub const UI_COMMANDTYPE_BOOLEAN: i32 = 8i32;
pub const UI_COMMANDTYPE_FONT: i32 = 9i32;
pub const UI_COMMANDTYPE_RECENTITEMS: i32 = 10i32;
pub const UI_COMMANDTYPE_COLORANCHOR: i32 = 11i32;
pub const UI_COMMANDTYPE_COLORCOLLECTION: i32 = 12i32;
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: i32 = 0i32;
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: i32 = 1i32;
pub const UI_CONTEXTAVAILABILITY_ACTIVE: i32 = 2i32;
pub const UI_CONTROLDOCK_TOP: i32 = 1i32;
pub const UI_CONTROLDOCK_BOTTOM: i32 = 3i32;
pub const UI_EVENTLOCATION_Ribbon: i32 = 0i32;
pub const UI_EVENTLOCATION_QAT: i32 = 1i32;
pub const UI_EVENTLOCATION_ApplicationMenu: i32 = 2i32;
pub const UI_EVENTLOCATION_ContextPopup: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct UI_EVENTPARAMS {
    pub EventType: UI_EVENTTYPE,
    pub Anonymous: UI_EVENTPARAMS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UI_EVENTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UI_EVENTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union UI_EVENTPARAMS_0 {
    pub Modes: i32,
    pub Params: UI_EVENTPARAMS_COMMAND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UI_EVENTPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UI_EVENTPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct UI_EVENTPARAMS_COMMAND {
    pub CommandID: u32,
    pub CommandName: super::super::Foundation::PWSTR,
    pub ParentCommandID: u32,
    pub ParentCommandName: super::super::Foundation::PWSTR,
    pub SelectionIndex: u32,
    pub Location: UI_EVENTLOCATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UI_EVENTPARAMS_COMMAND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UI_EVENTPARAMS_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UI_EVENTTYPE_ApplicationMenuOpened: i32 = 0i32;
pub const UI_EVENTTYPE_RibbonMinimized: i32 = 1i32;
pub const UI_EVENTTYPE_RibbonExpanded: i32 = 2i32;
pub const UI_EVENTTYPE_ApplicationModeSwitched: i32 = 3i32;
pub const UI_EVENTTYPE_TabActivated: i32 = 4i32;
pub const UI_EVENTTYPE_MenuOpened: i32 = 5i32;
pub const UI_EVENTTYPE_CommandExecuted: i32 = 6i32;
pub const UI_EVENTTYPE_TooltipShown: i32 = 7i32;
pub const UI_EXECUTIONVERB_EXECUTE: i32 = 0i32;
pub const UI_EXECUTIONVERB_PREVIEW: i32 = 1i32;
pub const UI_EXECUTIONVERB_CANCELPREVIEW: i32 = 2i32;
pub const UI_FONTDELTASIZE_GROW: i32 = 0i32;
pub const UI_FONTDELTASIZE_SHRINK: i32 = 1i32;
pub const UI_FONTPROPERTIES_NOTAVAILABLE: i32 = 0i32;
pub const UI_FONTPROPERTIES_NOTSET: i32 = 1i32;
pub const UI_FONTPROPERTIES_SET: i32 = 2i32;
pub const UI_FONTUNDERLINE_NOTAVAILABLE: i32 = 0i32;
pub const UI_FONTUNDERLINE_NOTSET: i32 = 1i32;
pub const UI_FONTUNDERLINE_SET: i32 = 2i32;
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: i32 = 0i32;
pub const UI_FONTVERTICALPOSITION_NOTSET: i32 = 1i32;
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: i32 = 2i32;
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: i32 = 3i32;
pub const UI_INVALIDATIONS_STATE: i32 = 1i32;
pub const UI_INVALIDATIONS_VALUE: i32 = 2i32;
pub const UI_INVALIDATIONS_PROPERTY: i32 = 4i32;
pub const UI_INVALIDATIONS_ALLPROPERTIES: i32 = 8i32;
pub const UI_OWNERSHIP_TRANSFER: i32 = 0i32;
pub const UI_OWNERSHIP_COPY: i32 = 1i32;
pub const UI_SWATCHCOLORMODE_NORMAL: i32 = 0i32;
pub const UI_SWATCHCOLORMODE_MONOCHROME: i32 = 1i32;
pub const UI_SWATCHCOLORTYPE_NOCOLOR: i32 = 0i32;
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: i32 = 1i32;
pub const UI_SWATCHCOLORTYPE_RGB: i32 = 2i32;
pub const UI_VIEWTYPE_RIBBON: i32 = 1i32;
pub const UI_VIEWVERB_CREATE: i32 = 0i32;
pub const UI_VIEWVERB_DESTROY: i32 = 1i32;
pub const UI_VIEWVERB_SIZE: i32 = 2i32;
pub const UI_VIEWVERB_ERROR: i32 = 3i32;
