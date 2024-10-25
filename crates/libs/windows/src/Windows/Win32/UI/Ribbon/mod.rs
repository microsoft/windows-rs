pub const LIBID_UIRibbon: windows_core::GUID = windows_core::GUID::from_u128(0x942f35c2_e83b_45ef_b085_ac295dd63d5b);
pub const UI_ALL_COMMANDS: u32 = 0u32;
pub const UI_COLLECTIONCHANGE_INSERT: UI_COLLECTIONCHANGE = 0i32;
pub const UI_COLLECTIONCHANGE_REMOVE: UI_COLLECTIONCHANGE = 1i32;
pub const UI_COLLECTIONCHANGE_REPLACE: UI_COLLECTIONCHANGE = 2i32;
pub const UI_COLLECTIONCHANGE_RESET: UI_COLLECTIONCHANGE = 3i32;
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
pub const UI_COMMANDTYPE_ACTION: UI_COMMANDTYPE = 2i32;
pub const UI_COMMANDTYPE_ANCHOR: UI_COMMANDTYPE = 3i32;
pub const UI_COMMANDTYPE_BOOLEAN: UI_COMMANDTYPE = 8i32;
pub const UI_COMMANDTYPE_COLLECTION: UI_COMMANDTYPE = 5i32;
pub const UI_COMMANDTYPE_COLORANCHOR: UI_COMMANDTYPE = 11i32;
pub const UI_COMMANDTYPE_COLORCOLLECTION: UI_COMMANDTYPE = 12i32;
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: UI_COMMANDTYPE = 6i32;
pub const UI_COMMANDTYPE_CONTEXT: UI_COMMANDTYPE = 4i32;
pub const UI_COMMANDTYPE_DECIMAL: UI_COMMANDTYPE = 7i32;
pub const UI_COMMANDTYPE_FONT: UI_COMMANDTYPE = 9i32;
pub const UI_COMMANDTYPE_GROUP: UI_COMMANDTYPE = 1i32;
pub const UI_COMMANDTYPE_RECENTITEMS: UI_COMMANDTYPE = 10i32;
pub const UI_COMMANDTYPE_UNKNOWN: UI_COMMANDTYPE = 0i32;
pub const UI_CONTEXTAVAILABILITY_ACTIVE: UI_CONTEXTAVAILABILITY = 2i32;
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: UI_CONTEXTAVAILABILITY = 1i32;
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: UI_CONTEXTAVAILABILITY = 0i32;
pub const UI_CONTROLDOCK_BOTTOM: UI_CONTROLDOCK = 3i32;
pub const UI_CONTROLDOCK_TOP: UI_CONTROLDOCK = 1i32;
pub const UI_EVENTLOCATION_ApplicationMenu: UI_EVENTLOCATION = 2i32;
pub const UI_EVENTLOCATION_ContextPopup: UI_EVENTLOCATION = 3i32;
pub const UI_EVENTLOCATION_QAT: UI_EVENTLOCATION = 1i32;
pub const UI_EVENTLOCATION_Ribbon: UI_EVENTLOCATION = 0i32;
pub const UI_EVENTTYPE_ApplicationMenuOpened: UI_EVENTTYPE = 0i32;
pub const UI_EVENTTYPE_ApplicationModeSwitched: UI_EVENTTYPE = 3i32;
pub const UI_EVENTTYPE_CommandExecuted: UI_EVENTTYPE = 6i32;
pub const UI_EVENTTYPE_MenuOpened: UI_EVENTTYPE = 5i32;
pub const UI_EVENTTYPE_RibbonExpanded: UI_EVENTTYPE = 2i32;
pub const UI_EVENTTYPE_RibbonMinimized: UI_EVENTTYPE = 1i32;
pub const UI_EVENTTYPE_TabActivated: UI_EVENTTYPE = 4i32;
pub const UI_EVENTTYPE_TooltipShown: UI_EVENTTYPE = 7i32;
pub const UI_EXECUTIONVERB_CANCELPREVIEW: UI_EXECUTIONVERB = 2i32;
pub const UI_EXECUTIONVERB_EXECUTE: UI_EXECUTIONVERB = 0i32;
pub const UI_EXECUTIONVERB_PREVIEW: UI_EXECUTIONVERB = 1i32;
pub const UI_FONTDELTASIZE_GROW: UI_FONTDELTASIZE = 0i32;
pub const UI_FONTDELTASIZE_SHRINK: UI_FONTDELTASIZE = 1i32;
pub const UI_FONTPROPERTIES_NOTAVAILABLE: UI_FONTPROPERTIES = 0i32;
pub const UI_FONTPROPERTIES_NOTSET: UI_FONTPROPERTIES = 1i32;
pub const UI_FONTPROPERTIES_SET: UI_FONTPROPERTIES = 2i32;
pub const UI_FONTUNDERLINE_NOTAVAILABLE: UI_FONTUNDERLINE = 0i32;
pub const UI_FONTUNDERLINE_NOTSET: UI_FONTUNDERLINE = 1i32;
pub const UI_FONTUNDERLINE_SET: UI_FONTUNDERLINE = 2i32;
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: UI_FONTVERTICALPOSITION = 0i32;
pub const UI_FONTVERTICALPOSITION_NOTSET: UI_FONTVERTICALPOSITION = 1i32;
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: UI_FONTVERTICALPOSITION = 3i32;
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: UI_FONTVERTICALPOSITION = 2i32;
pub const UI_INVALIDATIONS_ALLPROPERTIES: UI_INVALIDATIONS = 8i32;
pub const UI_INVALIDATIONS_PROPERTY: UI_INVALIDATIONS = 4i32;
pub const UI_INVALIDATIONS_STATE: UI_INVALIDATIONS = 1i32;
pub const UI_INVALIDATIONS_VALUE: UI_INVALIDATIONS = 2i32;
pub const UI_OWNERSHIP_COPY: UI_OWNERSHIP = 1i32;
pub const UI_OWNERSHIP_TRANSFER: UI_OWNERSHIP = 0i32;
pub const UI_SWATCHCOLORMODE_MONOCHROME: UI_SWATCHCOLORMODE = 1i32;
pub const UI_SWATCHCOLORMODE_NORMAL: UI_SWATCHCOLORMODE = 0i32;
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: UI_SWATCHCOLORTYPE = 1i32;
pub const UI_SWATCHCOLORTYPE_NOCOLOR: UI_SWATCHCOLORTYPE = 0i32;
pub const UI_SWATCHCOLORTYPE_RGB: UI_SWATCHCOLORTYPE = 2i32;
pub const UI_VIEWTYPE_RIBBON: UI_VIEWTYPE = 1i32;
pub const UI_VIEWVERB_CREATE: UI_VIEWVERB = 0i32;
pub const UI_VIEWVERB_DESTROY: UI_VIEWVERB = 1i32;
pub const UI_VIEWVERB_ERROR: UI_VIEWVERB = 3i32;
pub const UI_VIEWVERB_SIZE: UI_VIEWVERB = 2i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_COLLECTIONCHANGE(pub i32);
impl windows_core::TypeKind for UI_COLLECTIONCHANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_COMMANDTYPE(pub i32);
impl windows_core::TypeKind for UI_COMMANDTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_CONTEXTAVAILABILITY(pub i32);
impl windows_core::TypeKind for UI_CONTEXTAVAILABILITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_CONTROLDOCK(pub i32);
impl windows_core::TypeKind for UI_CONTROLDOCK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_EVENTLOCATION(pub i32);
impl windows_core::TypeKind for UI_EVENTLOCATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_EVENTTYPE(pub i32);
impl windows_core::TypeKind for UI_EVENTTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_EXECUTIONVERB(pub i32);
impl windows_core::TypeKind for UI_EXECUTIONVERB {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_FONTDELTASIZE(pub i32);
impl windows_core::TypeKind for UI_FONTDELTASIZE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_FONTPROPERTIES(pub i32);
impl windows_core::TypeKind for UI_FONTPROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_FONTUNDERLINE(pub i32);
impl windows_core::TypeKind for UI_FONTUNDERLINE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_FONTVERTICALPOSITION(pub i32);
impl windows_core::TypeKind for UI_FONTVERTICALPOSITION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_INVALIDATIONS(pub i32);
impl windows_core::TypeKind for UI_INVALIDATIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_OWNERSHIP(pub i32);
impl windows_core::TypeKind for UI_OWNERSHIP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_SWATCHCOLORMODE(pub i32);
impl windows_core::TypeKind for UI_SWATCHCOLORMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_SWATCHCOLORTYPE(pub i32);
impl windows_core::TypeKind for UI_SWATCHCOLORTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_VIEWTYPE(pub i32);
impl windows_core::TypeKind for UI_VIEWTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_VIEWVERB(pub i32);
impl windows_core::TypeKind for UI_VIEWVERB {
    type TypeKind = windows_core::CopyType;
}
pub const UIRibbonFramework: windows_core::GUID = windows_core::GUID::from_u128(0x926749fa_2615_4987_8845_c33e65f2b957);
pub const UIRibbonImageFromBitmapFactory: windows_core::GUID = windows_core::GUID::from_u128(0x0f7434b6_59b6_4250_999e_d168d6ae4293);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UI_EVENTPARAMS {
    pub EventType: UI_EVENTTYPE,
    pub Anonymous: UI_EVENTPARAMS_0,
}
impl Default for UI_EVENTPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UI_EVENTPARAMS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union UI_EVENTPARAMS_0 {
    pub Modes: i32,
    pub Params: UI_EVENTPARAMS_COMMAND,
}
impl Default for UI_EVENTPARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UI_EVENTPARAMS_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UI_EVENTPARAMS_COMMAND {
    pub CommandID: u32,
    pub CommandName: windows_core::PCWSTR,
    pub ParentCommandID: u32,
    pub ParentCommandName: windows_core::PCWSTR,
    pub SelectionIndex: u32,
    pub Location: UI_EVENTLOCATION,
}
impl Default for UI_EVENTPARAMS_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UI_EVENTPARAMS_COMMAND {
    type TypeKind = windows_core::CopyType;
}
