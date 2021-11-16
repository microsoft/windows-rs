#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const AUTO_WIDTH: i32 = -1i32;
#[repr(transparent)]
pub struct AppEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppEvents {}
impl ::core::clone::Clone for AppEvents {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AppEventsDHTMLConnector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2917549131, data2: 51487, data3: 20023, data4: [146, 164, 91, 180, 48, 163, 51, 64] };
pub const Application: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1236433178,
    data2: 45486,
    data3: 19600,
    data4: [155, 142, 232, 96, 186, 7, 248, 137],
};
pub const CCM_COMMANDID_MASK_RESERVED: u32 = 4294901760u32;
pub const CCM_INSERTIONALLOWED_TOP: i32 = 1i32;
pub const CCM_INSERTIONALLOWED_NEW: i32 = 2i32;
pub const CCM_INSERTIONALLOWED_TASK: i32 = 4i32;
pub const CCM_INSERTIONALLOWED_VIEW: i32 = 8i32;
pub const CCM_INSERTIONPOINTID_MASK_SPECIAL: i32 = -65536i32;
pub const CCM_INSERTIONPOINTID_MASK_SHARED: i32 = -2147483648i32;
pub const CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY: i32 = 1073741824i32;
pub const CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY: i32 = 536870912i32;
pub const CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY: i32 = 268435456i32;
pub const CCM_INSERTIONPOINTID_MASK_RESERVED: i32 = 268369920i32;
pub const CCM_INSERTIONPOINTID_MASK_FLAGINDEX: i32 = 31i32;
pub const CCM_INSERTIONPOINTID_PRIMARY_TOP: i32 = -1610612736i32;
pub const CCM_INSERTIONPOINTID_PRIMARY_NEW: i32 = -1610612735i32;
pub const CCM_INSERTIONPOINTID_PRIMARY_TASK: i32 = -1610612734i32;
pub const CCM_INSERTIONPOINTID_PRIMARY_VIEW: i32 = -1610612733i32;
pub const CCM_INSERTIONPOINTID_PRIMARY_HELP: i32 = -1610612732i32;
pub const CCM_INSERTIONPOINTID_3RDPARTY_NEW: i32 = -1879048191i32;
pub const CCM_INSERTIONPOINTID_3RDPARTY_TASK: i32 = -1879048190i32;
pub const CCM_INSERTIONPOINTID_ROOT_MENU: i32 = -2147483648i32;
pub const CCM_SPECIAL_SEPARATOR: i32 = 1i32;
pub const CCM_SPECIAL_SUBMENU: i32 = 2i32;
pub const CCM_SPECIAL_DEFAULT_ITEM: i32 = 4i32;
pub const CCM_SPECIAL_INSERTION_POINT: i32 = 8i32;
pub const CCM_SPECIAL_TESTONLY: i32 = 16i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM {
    pub strName: super::super::Foundation::PWSTR,
    pub strStatusBarText: super::super::Foundation::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONTEXTMENUITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONTEXTMENUITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM2 {
    pub strName: super::super::Foundation::PWSTR,
    pub strStatusBarText: super::super::Foundation::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
    pub strLanguageIndependentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONTEXTMENUITEM2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONTEXTMENUITEM2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Column(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Column {}
impl ::core::clone::Clone for Column {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Columns(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Columns {}
impl ::core::clone::Clone for Columns {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ConsolePower: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4029174644, data2: 57329, data3: 4563, data4: [180, 51, 0, 192, 79, 142, 205, 120] };
#[repr(transparent)]
pub struct ContextMenu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContextMenu {}
impl ::core::clone::Clone for ContextMenu {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CCT_SCOPE: i32 = 32768i32;
pub const CCT_RESULT: i32 = 32769i32;
pub const CCT_SNAPIN_MANAGER: i32 = 32770i32;
pub const CCT_UNINITIALIZED: i32 = 65535i32;
#[repr(transparent)]
pub struct Document(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Document {}
impl ::core::clone::Clone for Document {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Extension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Extension {}
impl ::core::clone::Clone for Extension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Extensions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Extensions {}
impl ::core::clone::Clone for Extensions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Frame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Frame {}
impl ::core::clone::Clone for Frame {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HDI_HIDDEN: u32 = 1u32;
pub const HIDE_COLUMN: i32 = -4i32;
#[repr(transparent)]
pub struct IColumnData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColumnData {}
impl ::core::clone::Clone for IColumnData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponent {}
impl ::core::clone::Clone for IComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponent2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponent2 {}
impl ::core::clone::Clone for IComponent2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponentData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentData {}
impl ::core::clone::Clone for IComponentData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponentData2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentData2 {}
impl ::core::clone::Clone for IComponentData2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsole(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsole {}
impl ::core::clone::Clone for IConsole {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsole2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsole2 {}
impl ::core::clone::Clone for IConsole2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsole3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsole3 {}
impl ::core::clone::Clone for IConsole3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsoleNameSpace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsoleNameSpace {}
impl ::core::clone::Clone for IConsoleNameSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsoleNameSpace2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsoleNameSpace2 {}
impl ::core::clone::Clone for IConsoleNameSpace2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsolePower(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsolePower {}
impl ::core::clone::Clone for IConsolePower {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsolePowerSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsolePowerSink {}
impl ::core::clone::Clone for IConsolePowerSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConsoleVerb(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConsoleVerb {}
impl ::core::clone::Clone for IConsoleVerb {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContextMenuCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextMenuCallback {}
impl ::core::clone::Clone for IContextMenuCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContextMenuCallback2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextMenuCallback2 {}
impl ::core::clone::Clone for IContextMenuCallback2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContextMenuProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextMenuProvider {}
impl ::core::clone::Clone for IContextMenuProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlbar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlbar {}
impl ::core::clone::Clone for IControlbar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayHelp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayHelp {}
impl ::core::clone::Clone for IDisplayHelp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumTASK(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumTASK {}
impl ::core::clone::Clone for IEnumTASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendContextMenu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendContextMenu {}
impl ::core::clone::Clone for IExtendContextMenu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendControlbar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendControlbar {}
impl ::core::clone::Clone for IExtendControlbar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendPropertySheet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendPropertySheet {}
impl ::core::clone::Clone for IExtendPropertySheet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendPropertySheet2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendPropertySheet2 {}
impl ::core::clone::Clone for IExtendPropertySheet2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendTaskPad(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendTaskPad {}
impl ::core::clone::Clone for IExtendTaskPad {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendView {}
impl ::core::clone::Clone for IExtendView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHeaderCtrl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHeaderCtrl {}
impl ::core::clone::Clone for IHeaderCtrl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHeaderCtrl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHeaderCtrl2 {}
impl ::core::clone::Clone for IHeaderCtrl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageList {}
impl ::core::clone::Clone for IImageList {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
#[repr(transparent)]
pub struct IMMCVersionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMMCVersionInfo {}
impl ::core::clone::Clone for IMMCVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuButton {}
impl ::core::clone::Clone for IMenuButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageView {}
impl ::core::clone::Clone for IMessageView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INodeProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INodeProperties {}
impl ::core::clone::Clone for INodeProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertySheetCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertySheetCallback {}
impl ::core::clone::Clone for IPropertySheetCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertySheetProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertySheetProvider {}
impl ::core::clone::Clone for IPropertySheetProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRequiredExtensions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRequiredExtensions {}
impl ::core::clone::Clone for IRequiredExtensions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResultData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResultData {}
impl ::core::clone::Clone for IResultData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResultData2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResultData2 {}
impl ::core::clone::Clone for IResultData2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResultDataCompare(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResultDataCompare {}
impl ::core::clone::Clone for IResultDataCompare {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResultDataCompareEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResultDataCompareEx {}
impl ::core::clone::Clone for IResultDataCompareEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResultOwnerData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResultOwnerData {}
impl ::core::clone::Clone for IResultOwnerData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISnapinAbout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISnapinAbout {}
impl ::core::clone::Clone for ISnapinAbout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISnapinHelp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISnapinHelp {}
impl ::core::clone::Clone for ISnapinHelp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISnapinHelp2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISnapinHelp2 {}
impl ::core::clone::Clone for ISnapinHelp2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISnapinProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISnapinProperties {}
impl ::core::clone::Clone for ISnapinProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISnapinPropertiesCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISnapinPropertiesCallback {}
impl ::core::clone::Clone for ISnapinPropertiesCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStringTable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStringTable {}
impl ::core::clone::Clone for IStringTable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolbar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolbar {}
impl ::core::clone::Clone for IToolbar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IViewExtensionCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IViewExtensionCallback {}
impl ::core::clone::Clone for IViewExtensionCallback {
    fn clone(&self) -> Self {
        *self
    }
}
pub const Icon_None: i32 = 0i32;
pub const Icon_Error: i32 = 32513i32;
pub const Icon_Question: i32 = 32514i32;
pub const Icon_Warning: i32 = 32515i32;
pub const Icon_Information: i32 = 32516i32;
pub const Icon_First: i32 = 32513i32;
pub const Icon_Last: i32 = 32516i32;
#[repr(C)]
pub struct MENUBUTTONDATA {
    pub idCommand: i32,
    pub x: i32,
    pub y: i32,
}
impl ::core::marker::Copy for MENUBUTTONDATA {}
impl ::core::clone::Clone for MENUBUTTONDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMCBUTTON {
    pub nBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsType: u8,
    pub lpButtonText: super::super::Foundation::PWSTR,
    pub lpTooltipText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMCBUTTON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMCBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MMCLV_AUTO: i32 = -1i32;
pub const MMCLV_NOICON: i32 = -1i32;
pub const MMCLV_NOPARAM: i32 = -2i32;
pub const MMCLV_NOPTR: u32 = 0u32;
pub const MMCLV_UPDATE_NOINVALIDATEALL: u32 = 1u32;
pub const MMCLV_UPDATE_NOSCROLL: u32 = 2u32;
pub const MMCLV_VIEWSTYLE_FILTERED: u32 = 4u32;
pub const MMCLV_VIEWSTYLE_ICON: u32 = 0u32;
pub const MMCLV_VIEWSTYLE_LIST: u32 = 3u32;
pub const MMCLV_VIEWSTYLE_REPORT: u32 = 1u32;
pub const MMCLV_VIEWSTYLE_SMALLICON: u32 = 2u32;
pub const MMCVersionInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3607026461,
    data2: 53025,
    data3: 19417,
    data4: [175, 59, 197, 70, 142, 156, 102, 132],
};
pub const MMC_ACTION_UNINITIALIZED: i32 = -1i32;
pub const MMC_ACTION_ID: i32 = 0i32;
pub const MMC_ACTION_LINK: i32 = 1i32;
pub const MMC_ACTION_SCRIPT: i32 = 2i32;
pub const ENABLED: i32 = 1i32;
pub const CHECKED: i32 = 2i32;
pub const HIDDEN: i32 = 4i32;
pub const INDETERMINATE: i32 = 8i32;
pub const BUTTONPRESSED: i32 = 16i32;
#[repr(C)]
pub struct MMC_COLUMN_DATA {
    pub nColIndex: i32,
    pub dwFlags: u32,
    pub nWidth: i32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_COLUMN_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MMC_COLUMN_SET_DATA {
    pub cbSize: i32,
    pub nNumCols: i32,
    pub pColData: *mut MMC_COLUMN_DATA,
}
impl ::core::marker::Copy for MMC_COLUMN_SET_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MMC_VERB_NONE: i32 = 0i32;
pub const MMC_VERB_OPEN: i32 = 32768i32;
pub const MMC_VERB_COPY: i32 = 32769i32;
pub const MMC_VERB_PASTE: i32 = 32770i32;
pub const MMC_VERB_DELETE: i32 = 32771i32;
pub const MMC_VERB_PROPERTIES: i32 = 32772i32;
pub const MMC_VERB_RENAME: i32 = 32773i32;
pub const MMC_VERB_REFRESH: i32 = 32774i32;
pub const MMC_VERB_PRINT: i32 = 32775i32;
pub const MMC_VERB_CUT: i32 = 32776i32;
pub const MMC_VERB_MAX: i32 = 32777i32;
pub const MMC_VERB_FIRST: i32 = 32768i32;
pub const MMC_VERB_LAST: i32 = 32776i32;
pub const TOOLBAR: i32 = 0i32;
pub const MENUBUTTON: i32 = 1i32;
pub const COMBOBOXBAR: i32 = 2i32;
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXPANDSYNC_STRUCT {
    pub bHandled: super::super::Foundation::BOOL,
    pub bExpanding: super::super::Foundation::BOOL,
    pub hItem: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXPANDSYNC_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXT_VIEW_DATA {
    pub viewID: ::windows_sys::core::GUID,
    pub pszURL: super::super::Foundation::PWSTR,
    pub pszViewTitle: super::super::Foundation::PWSTR,
    pub pszTooltipText: super::super::Foundation::PWSTR,
    pub bReplacesDefaultView: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXT_VIEW_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_FILTERDATA {
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub lValue: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_FILTERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_FILTERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MFCC_DISABLE: i32 = 0i32;
pub const MFCC_ENABLE: i32 = 1i32;
pub const MFCC_VALUE_CHANGE: i32 = 2i32;
pub const MMC_STRING_FILTER: i32 = 0i32;
pub const MMC_INT_FILTER: i32 = 1i32;
pub const MMC_FILTER_NOVALUE: i32 = 32768i32;
pub const MMC_IMAGECALLBACK: i32 = -1i32;
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_LISTPAD_INFO {
    pub szTitle: super::super::Foundation::PWSTR,
    pub szButtonText: super::super::Foundation::PWSTR,
    pub nCommandID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_LISTPAD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_LISTPAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MMCC_STANDARD_VIEW_SELECT: i32 = -1i32;
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
pub const MMCN_ACTIVATE: i32 = 32769i32;
pub const MMCN_ADD_IMAGES: i32 = 32770i32;
pub const MMCN_BTN_CLICK: i32 = 32771i32;
pub const MMCN_CLICK: i32 = 32772i32;
pub const MMCN_COLUMN_CLICK: i32 = 32773i32;
pub const MMCN_CONTEXTMENU: i32 = 32774i32;
pub const MMCN_CUTORMOVE: i32 = 32775i32;
pub const MMCN_DBLCLICK: i32 = 32776i32;
pub const MMCN_DELETE: i32 = 32777i32;
pub const MMCN_DESELECT_ALL: i32 = 32778i32;
pub const MMCN_EXPAND: i32 = 32779i32;
pub const MMCN_HELP: i32 = 32780i32;
pub const MMCN_MENU_BTNCLICK: i32 = 32781i32;
pub const MMCN_MINIMIZED: i32 = 32782i32;
pub const MMCN_PASTE: i32 = 32783i32;
pub const MMCN_PROPERTY_CHANGE: i32 = 32784i32;
pub const MMCN_QUERY_PASTE: i32 = 32785i32;
pub const MMCN_REFRESH: i32 = 32786i32;
pub const MMCN_REMOVE_CHILDREN: i32 = 32787i32;
pub const MMCN_RENAME: i32 = 32788i32;
pub const MMCN_SELECT: i32 = 32789i32;
pub const MMCN_SHOW: i32 = 32790i32;
pub const MMCN_VIEW_CHANGE: i32 = 32791i32;
pub const MMCN_SNAPINHELP: i32 = 32792i32;
pub const MMCN_CONTEXTHELP: i32 = 32793i32;
pub const MMCN_INITOCX: i32 = 32794i32;
pub const MMCN_FILTER_CHANGE: i32 = 32795i32;
pub const MMCN_FILTERBTN_CLICK: i32 = 32796i32;
pub const MMCN_RESTORE_VIEW: i32 = 32797i32;
pub const MMCN_PRINT: i32 = 32798i32;
pub const MMCN_PRELOAD: i32 = 32799i32;
pub const MMCN_LISTPAD: i32 = 32800i32;
pub const MMCN_EXPANDSYNC: i32 = 32801i32;
pub const MMCN_COLUMNS_CHANGED: i32 = 32802i32;
pub const MMCN_CANPASTE_OUTOFPROC: i32 = 32803i32;
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
pub const MMC_PROPACT_DELETING: i32 = 1i32;
pub const MMC_PROPACT_CHANGING: i32 = 2i32;
pub const MMC_PROPACT_INITIALIZED: i32 = 3i32;
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
pub const MMC_PROP_PERSIST: u32 = 8u32;
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
pub const MMC_PSO_HASHELP: u32 = 2u32;
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_RESTORE_VIEW {
    pub dwSize: u32,
    pub cookie: isize,
    pub pViewType: super::super::Foundation::PWSTR,
    pub lViewOptions: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_RESTORE_VIEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_RESTORE_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MMC_SINGLESEL: i32 = 1i32;
pub const MMC_SHOWSELALWAYS: i32 = 2i32;
pub const MMC_NOSORTHEADER: i32 = 4i32;
pub const MMC_ENSUREFOCUSVISIBLE: i32 = 8i32;
pub const MMC_SCOPE_ITEM_STATE_NORMAL: i32 = 1i32;
pub const MMC_SCOPE_ITEM_STATE_BOLD: i32 = 2i32;
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: i32 = 3i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct MMC_SNAPIN_PROPERTY {
    pub pszPropName: super::super::Foundation::PWSTR,
    pub varValue: super::Com::VARIANT,
    pub eAction: MMC_PROPERTY_ACTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for MMC_SNAPIN_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for MMC_SNAPIN_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MMC_SORT_DATA {
    pub nColIndex: i32,
    pub dwSortOptions: u32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_SORT_DATA {}
impl ::core::clone::Clone for MMC_SORT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MMC_SORT_SET_DATA {
    pub cbSize: i32,
    pub nNumItems: i32,
    pub pSortData: *mut MMC_SORT_DATA,
}
impl ::core::marker::Copy for MMC_SORT_SET_DATA {}
impl ::core::clone::Clone for MMC_SORT_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK {
    pub sDisplayObject: MMC_TASK_DISPLAY_OBJECT,
    pub szText: super::super::Foundation::PWSTR,
    pub szHelpString: super::super::Foundation::PWSTR,
    pub eActionType: MMC_ACTION_TYPE,
    pub Anonymous: MMC_TASK_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MMC_TASK_0 {
    pub nCommandID: isize,
    pub szActionURL: super::super::Foundation::PWSTR,
    pub szScript: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_BITMAP {
    pub szMouseOverBitmap: super::super::Foundation::PWSTR,
    pub szMouseOffBitmap: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_BITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_OBJECT {
    pub eDisplayType: MMC_TASK_DISPLAY_TYPE,
    pub Anonymous: MMC_TASK_DISPLAY_OBJECT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MMC_TASK_DISPLAY_OBJECT_0 {
    pub uBitmap: MMC_TASK_DISPLAY_BITMAP,
    pub uSymbol: MMC_TASK_DISPLAY_SYMBOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_SYMBOL {
    pub szFontFamilyName: super::super::Foundation::PWSTR,
    pub szURLtoEOT: super::super::Foundation::PWSTR,
    pub szSymbolString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_TASK_DISPLAY_SYMBOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_TASK_DISPLAY_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MMC_TASK_DISPLAY_UNINITIALIZED: i32 = 0i32;
pub const MMC_TASK_DISPLAY_TYPE_SYMBOL: i32 = 1i32;
pub const MMC_TASK_DISPLAY_TYPE_VANILLA_GIF: i32 = 2i32;
pub const MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF: i32 = 3i32;
pub const MMC_TASK_DISPLAY_TYPE_BITMAP: i32 = 4i32;
pub const MMC_VER: u32 = 512u32;
pub const MMC_VIEW_OPTIONS_CREATENEW: u32 = 16u32;
pub const MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
pub const MMC_VIEW_OPTIONS_FILTERED: u32 = 8u32;
pub const MMC_VIEW_OPTIONS_LEXICAL_SORT: u32 = 128u32;
pub const MMC_VIEW_OPTIONS_MULTISELECT: u32 = 2u32;
pub const MMC_VIEW_OPTIONS_NOLISTVIEWS: u32 = 1u32;
pub const MMC_VIEW_OPTIONS_NONE: u32 = 0u32;
pub const MMC_VIEW_OPTIONS_OWNERDATALIST: u32 = 4u32;
pub const MMC_VIEW_OPTIONS_USEFONTLINKING: u32 = 32u32;
pub const MMC_VIEW_TYPE_LIST: i32 = 0i32;
pub const MMC_VIEW_TYPE_HTML: i32 = 1i32;
pub const MMC_VIEW_TYPE_OCX: i32 = 2i32;
#[repr(C)]
pub struct MMC_VISIBLE_COLUMNS {
    pub nVisibleColumns: i32,
    pub rgVisibleCols: [i32; 1],
}
impl ::core::marker::Copy for MMC_VISIBLE_COLUMNS {}
impl ::core::clone::Clone for MMC_VISIBLE_COLUMNS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
#[repr(transparent)]
pub struct MenuItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuItem {}
impl ::core::clone::Clone for MenuItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Node(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Node {}
impl ::core::clone::Clone for Node {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Nodes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Nodes {}
impl ::core::clone::Clone for Nodes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Properties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Properties {}
impl ::core::clone::Clone for Properties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Property(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Property {}
impl ::core::clone::Clone for Property {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RDCOMPARE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub nColumn: i32,
    pub lUserParam: super::super::Foundation::LPARAM,
    pub prdch1: *mut RDITEMHDR,
    pub prdch2: *mut RDITEMHDR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDCOMPARE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RDITEMHDR {
    pub dwFlags: u32,
    pub cookie: isize,
    pub lpReserved: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDITEMHDR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RDI_IMAGE: u32 = 4u32;
pub const RDI_INDENT: u32 = 64u32;
pub const RDI_INDEX: u32 = 32u32;
pub const RDI_PARAM: u32 = 16u32;
pub const RDI_STATE: u32 = 8u32;
pub const RDI_STR: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTDATAITEM {
    pub mask: u32,
    pub bScopeItem: super::super::Foundation::BOOL,
    pub itemID: isize,
    pub nIndex: i32,
    pub nCol: i32,
    pub str: super::super::Foundation::PWSTR,
    pub nImage: i32,
    pub nState: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULTDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTFINDINFO {
    pub psz: super::super::Foundation::PWSTR,
    pub nStart: i32,
    pub dwOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULTFINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULTFINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO {
    pub pstrPersistableViewDescription: super::super::Foundation::PWSTR,
    pub eViewType: MMC_VIEW_TYPE,
    pub dwMiscOptions: u32,
    pub Anonymous: RESULT_VIEW_TYPE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RESULT_VIEW_TYPE_INFO_0 {
    pub dwListOptions: u32,
    pub Anonymous1: RESULT_VIEW_TYPE_INFO_0_0,
    pub Anonymous2: RESULT_VIEW_TYPE_INFO_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO_0_0 {
    pub dwHTMLOptions: u32,
    pub pstrURL: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO_0_1 {
    pub dwOCXOptions: u32,
    pub pUnkControl: ::windows_sys::core::IUnknown,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RFI_PARTIAL: u32 = 1u32;
pub const RFI_WRAP: u32 = 2u32;
pub const RSI_DESCENDING: u32 = 1u32;
pub const RSI_NOSORTICON: u32 = 2u32;
pub const RVTI_HTML_OPTIONS_NOLISTVIEW: u32 = 1u32;
pub const RVTI_HTML_OPTIONS_NONE: u32 = 0u32;
pub const RVTI_LIST_OPTIONS_ALLOWPASTE: u32 = 256u32;
pub const RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
pub const RVTI_LIST_OPTIONS_FILTERED: u32 = 8u32;
pub const RVTI_LIST_OPTIONS_LEXICAL_SORT: u32 = 128u32;
pub const RVTI_LIST_OPTIONS_MULTISELECT: u32 = 4u32;
pub const RVTI_LIST_OPTIONS_NONE: u32 = 0u32;
pub const RVTI_LIST_OPTIONS_OWNERDATALIST: u32 = 2u32;
pub const RVTI_LIST_OPTIONS_USEFONTLINKING: u32 = 32u32;
pub const RVTI_MISC_OPTIONS_NOLISTVIEWS: u32 = 1u32;
pub const RVTI_OCX_OPTIONS_CACHE_OCX: u32 = 2u32;
pub const RVTI_OCX_OPTIONS_NOLISTVIEW: u32 = 1u32;
pub const RVTI_OCX_OPTIONS_NONE: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCOPEDATAITEM {
    pub mask: u32,
    pub displayname: super::super::Foundation::PWSTR,
    pub nImage: i32,
    pub nOpenImage: i32,
    pub nState: u32,
    pub cChildren: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub relativeID: isize,
    pub ID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCOPEDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SColumnSetID {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SColumnSetID {}
impl ::core::clone::Clone for SColumnSetID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SDI_CHILDREN: u32 = 64u32;
pub const SDI_FIRST: u32 = 134217728u32;
pub const SDI_IMAGE: u32 = 4u32;
pub const SDI_NEXT: u32 = 536870912u32;
pub const SDI_OPENIMAGE: u32 = 8u32;
pub const SDI_PARAM: u32 = 32u32;
pub const SDI_PARENT: u32 = 0u32;
pub const SDI_PREVIOUS: u32 = 268435456u32;
pub const SDI_STATE: u32 = 16u32;
pub const SDI_STR: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct SMMCDataObjects {
    pub count: u32,
    pub lpDataObject: [super::Com::IDataObject; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SMMCDataObjects {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SMMCDataObjects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SMMCObjectTypes {
    pub count: u32,
    pub guid: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for SMMCObjectTypes {}
impl ::core::clone::Clone for SMMCObjectTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SNodeID {
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID {}
impl ::core::clone::Clone for SNodeID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SNodeID2 {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID2 {}
impl ::core::clone::Clone for SNodeID2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
#[repr(transparent)]
pub struct ScopeNamespace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScopeNamespace {}
impl ::core::clone::Clone for ScopeNamespace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SnapIn(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SnapIn {}
impl ::core::clone::Clone for SnapIn {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SnapIns(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SnapIns {}
impl ::core::clone::Clone for SnapIns {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct View(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for View {}
impl ::core::clone::Clone for View {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Views(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Views {}
impl ::core::clone::Clone for Views {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _AppEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _AppEvents {}
impl ::core::clone::Clone for _AppEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _Application(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _Application {}
impl ::core::clone::Clone for _Application {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SortOrder_Ascending: i32 = 0i32;
pub const SortOrder_Descending: i32 = 1i32;
pub const DocumentMode_Author: i32 = 0i32;
pub const DocumentMode_User: i32 = 1i32;
pub const DocumentMode_User_MDI: i32 = 2i32;
pub const DocumentMode_User_SDI: i32 = 3i32;
#[repr(transparent)]
pub struct _EventConnector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _EventConnector {}
impl ::core::clone::Clone for _EventConnector {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ExportListOptions_Default: i32 = 0i32;
pub const ExportListOptions_Unicode: i32 = 1i32;
pub const ExportListOptions_TabDelimited: i32 = 2i32;
pub const ExportListOptions_SelectedItemsOnly: i32 = 4i32;
pub const ListMode_Small_Icons: i32 = 0i32;
pub const ListMode_Large_Icons: i32 = 1i32;
pub const ListMode_List: i32 = 2i32;
pub const ListMode_Detail: i32 = 3i32;
pub const ListMode_Filtered: i32 = 4i32;
pub const ViewOption_Default: i32 = 0i32;
pub const ViewOption_ScopeTreeHidden: i32 = 1i32;
pub const ViewOption_NoToolBars: i32 = 2i32;
pub const ViewOption_NotPersistable: i32 = 4i32;
pub const ViewOption_ActionPaneHidden: i32 = 8i32;
