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
#[repr(transparent)]
pub struct UI_COLLECTIONCHANGE(pub i32);
pub const UI_COLLECTIONCHANGE_INSERT: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(0i32);
pub const UI_COLLECTIONCHANGE_REMOVE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(1i32);
pub const UI_COLLECTIONCHANGE_REPLACE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(2i32);
pub const UI_COLLECTIONCHANGE_RESET: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(3i32);
impl ::core::marker::Copy for UI_COLLECTIONCHANGE {}
impl ::core::clone::Clone for UI_COLLECTIONCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
#[repr(transparent)]
pub struct UI_COMMANDTYPE(pub i32);
pub const UI_COMMANDTYPE_UNKNOWN: UI_COMMANDTYPE = UI_COMMANDTYPE(0i32);
pub const UI_COMMANDTYPE_GROUP: UI_COMMANDTYPE = UI_COMMANDTYPE(1i32);
pub const UI_COMMANDTYPE_ACTION: UI_COMMANDTYPE = UI_COMMANDTYPE(2i32);
pub const UI_COMMANDTYPE_ANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(3i32);
pub const UI_COMMANDTYPE_CONTEXT: UI_COMMANDTYPE = UI_COMMANDTYPE(4i32);
pub const UI_COMMANDTYPE_COLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(5i32);
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(6i32);
pub const UI_COMMANDTYPE_DECIMAL: UI_COMMANDTYPE = UI_COMMANDTYPE(7i32);
pub const UI_COMMANDTYPE_BOOLEAN: UI_COMMANDTYPE = UI_COMMANDTYPE(8i32);
pub const UI_COMMANDTYPE_FONT: UI_COMMANDTYPE = UI_COMMANDTYPE(9i32);
pub const UI_COMMANDTYPE_RECENTITEMS: UI_COMMANDTYPE = UI_COMMANDTYPE(10i32);
pub const UI_COMMANDTYPE_COLORANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(11i32);
pub const UI_COMMANDTYPE_COLORCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(12i32);
impl ::core::marker::Copy for UI_COMMANDTYPE {}
impl ::core::clone::Clone for UI_COMMANDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_CONTEXTAVAILABILITY(pub i32);
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(0i32);
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(1i32);
pub const UI_CONTEXTAVAILABILITY_ACTIVE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(2i32);
impl ::core::marker::Copy for UI_CONTEXTAVAILABILITY {}
impl ::core::clone::Clone for UI_CONTEXTAVAILABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_CONTROLDOCK(pub i32);
pub const UI_CONTROLDOCK_TOP: UI_CONTROLDOCK = UI_CONTROLDOCK(1i32);
pub const UI_CONTROLDOCK_BOTTOM: UI_CONTROLDOCK = UI_CONTROLDOCK(3i32);
impl ::core::marker::Copy for UI_CONTROLDOCK {}
impl ::core::clone::Clone for UI_CONTROLDOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_EVENTLOCATION(pub i32);
pub const UI_EVENTLOCATION_Ribbon: UI_EVENTLOCATION = UI_EVENTLOCATION(0i32);
pub const UI_EVENTLOCATION_QAT: UI_EVENTLOCATION = UI_EVENTLOCATION(1i32);
pub const UI_EVENTLOCATION_ApplicationMenu: UI_EVENTLOCATION = UI_EVENTLOCATION(2i32);
pub const UI_EVENTLOCATION_ContextPopup: UI_EVENTLOCATION = UI_EVENTLOCATION(3i32);
impl ::core::marker::Copy for UI_EVENTLOCATION {}
impl ::core::clone::Clone for UI_EVENTLOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct UI_EVENTTYPE(pub i32);
pub const UI_EVENTTYPE_ApplicationMenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(0i32);
pub const UI_EVENTTYPE_RibbonMinimized: UI_EVENTTYPE = UI_EVENTTYPE(1i32);
pub const UI_EVENTTYPE_RibbonExpanded: UI_EVENTTYPE = UI_EVENTTYPE(2i32);
pub const UI_EVENTTYPE_ApplicationModeSwitched: UI_EVENTTYPE = UI_EVENTTYPE(3i32);
pub const UI_EVENTTYPE_TabActivated: UI_EVENTTYPE = UI_EVENTTYPE(4i32);
pub const UI_EVENTTYPE_MenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(5i32);
pub const UI_EVENTTYPE_CommandExecuted: UI_EVENTTYPE = UI_EVENTTYPE(6i32);
pub const UI_EVENTTYPE_TooltipShown: UI_EVENTTYPE = UI_EVENTTYPE(7i32);
impl ::core::marker::Copy for UI_EVENTTYPE {}
impl ::core::clone::Clone for UI_EVENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_EXECUTIONVERB(pub i32);
pub const UI_EXECUTIONVERB_EXECUTE: UI_EXECUTIONVERB = UI_EXECUTIONVERB(0i32);
pub const UI_EXECUTIONVERB_PREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(1i32);
pub const UI_EXECUTIONVERB_CANCELPREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(2i32);
impl ::core::marker::Copy for UI_EXECUTIONVERB {}
impl ::core::clone::Clone for UI_EXECUTIONVERB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_FONTDELTASIZE(pub i32);
pub const UI_FONTDELTASIZE_GROW: UI_FONTDELTASIZE = UI_FONTDELTASIZE(0i32);
pub const UI_FONTDELTASIZE_SHRINK: UI_FONTDELTASIZE = UI_FONTDELTASIZE(1i32);
impl ::core::marker::Copy for UI_FONTDELTASIZE {}
impl ::core::clone::Clone for UI_FONTDELTASIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_FONTPROPERTIES(pub i32);
pub const UI_FONTPROPERTIES_NOTAVAILABLE: UI_FONTPROPERTIES = UI_FONTPROPERTIES(0i32);
pub const UI_FONTPROPERTIES_NOTSET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(1i32);
pub const UI_FONTPROPERTIES_SET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(2i32);
impl ::core::marker::Copy for UI_FONTPROPERTIES {}
impl ::core::clone::Clone for UI_FONTPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_FONTUNDERLINE(pub i32);
pub const UI_FONTUNDERLINE_NOTAVAILABLE: UI_FONTUNDERLINE = UI_FONTUNDERLINE(0i32);
pub const UI_FONTUNDERLINE_NOTSET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(1i32);
pub const UI_FONTUNDERLINE_SET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(2i32);
impl ::core::marker::Copy for UI_FONTUNDERLINE {}
impl ::core::clone::Clone for UI_FONTUNDERLINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_FONTVERTICALPOSITION(pub i32);
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(0i32);
pub const UI_FONTVERTICALPOSITION_NOTSET: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(1i32);
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(2i32);
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(3i32);
impl ::core::marker::Copy for UI_FONTVERTICALPOSITION {}
impl ::core::clone::Clone for UI_FONTVERTICALPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_INVALIDATIONS(pub i32);
pub const UI_INVALIDATIONS_STATE: UI_INVALIDATIONS = UI_INVALIDATIONS(1i32);
pub const UI_INVALIDATIONS_VALUE: UI_INVALIDATIONS = UI_INVALIDATIONS(2i32);
pub const UI_INVALIDATIONS_PROPERTY: UI_INVALIDATIONS = UI_INVALIDATIONS(4i32);
pub const UI_INVALIDATIONS_ALLPROPERTIES: UI_INVALIDATIONS = UI_INVALIDATIONS(8i32);
impl ::core::marker::Copy for UI_INVALIDATIONS {}
impl ::core::clone::Clone for UI_INVALIDATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_OWNERSHIP(pub i32);
pub const UI_OWNERSHIP_TRANSFER: UI_OWNERSHIP = UI_OWNERSHIP(0i32);
pub const UI_OWNERSHIP_COPY: UI_OWNERSHIP = UI_OWNERSHIP(1i32);
impl ::core::marker::Copy for UI_OWNERSHIP {}
impl ::core::clone::Clone for UI_OWNERSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_SWATCHCOLORMODE(pub i32);
pub const UI_SWATCHCOLORMODE_NORMAL: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(0i32);
pub const UI_SWATCHCOLORMODE_MONOCHROME: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(1i32);
impl ::core::marker::Copy for UI_SWATCHCOLORMODE {}
impl ::core::clone::Clone for UI_SWATCHCOLORMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_SWATCHCOLORTYPE(pub i32);
pub const UI_SWATCHCOLORTYPE_NOCOLOR: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(0i32);
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(1i32);
pub const UI_SWATCHCOLORTYPE_RGB: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(2i32);
impl ::core::marker::Copy for UI_SWATCHCOLORTYPE {}
impl ::core::clone::Clone for UI_SWATCHCOLORTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_VIEWTYPE(pub i32);
pub const UI_VIEWTYPE_RIBBON: UI_VIEWTYPE = UI_VIEWTYPE(1i32);
impl ::core::marker::Copy for UI_VIEWTYPE {}
impl ::core::clone::Clone for UI_VIEWTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UI_VIEWVERB(pub i32);
pub const UI_VIEWVERB_CREATE: UI_VIEWVERB = UI_VIEWVERB(0i32);
pub const UI_VIEWVERB_DESTROY: UI_VIEWVERB = UI_VIEWVERB(1i32);
pub const UI_VIEWVERB_SIZE: UI_VIEWVERB = UI_VIEWVERB(2i32);
pub const UI_VIEWVERB_ERROR: UI_VIEWVERB = UI_VIEWVERB(3i32);
impl ::core::marker::Copy for UI_VIEWVERB {}
impl ::core::clone::Clone for UI_VIEWVERB {
    fn clone(&self) -> Self {
        *self
    }
}
