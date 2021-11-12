#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const AUTO_WIDTH: i32 = -1i32;
#[repr(transparent)]
pub struct AppEvents(pub *mut ::core::ffi::c_void);
pub const AppEventsDHTMLConnector: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2917549131, data2: 51487, data3: 20023, data4: [146, 164, 91, 180, 48, 163, 51, 64] };
pub const Application: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1236433178,
    data2: 45486,
    data3: 19600,
    data4: [155, 142, 232, 96, 186, 7, 248, 137],
};
#[repr(transparent)]
pub struct CCM_COMMANDID_MASK_CONSTANTS(pub u32);
pub const CCM_COMMANDID_MASK_RESERVED: CCM_COMMANDID_MASK_CONSTANTS = CCM_COMMANDID_MASK_CONSTANTS(4294901760u32);
#[repr(transparent)]
pub struct CCM_INSERTIONALLOWED(pub i32);
pub const CCM_INSERTIONALLOWED_TOP: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(1i32);
pub const CCM_INSERTIONALLOWED_NEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(2i32);
pub const CCM_INSERTIONALLOWED_TASK: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(4i32);
pub const CCM_INSERTIONALLOWED_VIEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(8i32);
#[repr(transparent)]
pub struct CCM_INSERTIONPOINTID(pub i32);
pub const CCM_INSERTIONPOINTID_MASK_SPECIAL: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-65536i32);
pub const CCM_INSERTIONPOINTID_MASK_SHARED: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-2147483648i32);
pub const CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(1073741824i32);
pub const CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(536870912i32);
pub const CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(268435456i32);
pub const CCM_INSERTIONPOINTID_MASK_RESERVED: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(268369920i32);
pub const CCM_INSERTIONPOINTID_MASK_FLAGINDEX: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(31i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_TOP: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612736i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_NEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612735i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_TASK: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612734i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_VIEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612733i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_HELP: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612732i32);
pub const CCM_INSERTIONPOINTID_3RDPARTY_NEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1879048191i32);
pub const CCM_INSERTIONPOINTID_3RDPARTY_TASK: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1879048190i32);
pub const CCM_INSERTIONPOINTID_ROOT_MENU: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-2147483648i32);
#[repr(transparent)]
pub struct CCM_SPECIAL(pub i32);
pub const CCM_SPECIAL_SEPARATOR: CCM_SPECIAL = CCM_SPECIAL(1i32);
pub const CCM_SPECIAL_SUBMENU: CCM_SPECIAL = CCM_SPECIAL(2i32);
pub const CCM_SPECIAL_DEFAULT_ITEM: CCM_SPECIAL = CCM_SPECIAL(4i32);
pub const CCM_SPECIAL_INSERTION_POINT: CCM_SPECIAL = CCM_SPECIAL(8i32);
pub const CCM_SPECIAL_TESTONLY: CCM_SPECIAL = CCM_SPECIAL(16i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONTEXTMENUITEM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONTEXTMENUITEM2(i32);
#[repr(transparent)]
pub struct Column(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Columns(pub *mut ::core::ffi::c_void);
pub const ConsolePower: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4029174644, data2: 57329, data3: 4563, data4: [180, 51, 0, 192, 79, 142, 205, 120] };
#[repr(transparent)]
pub struct ContextMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DATA_OBJECT_TYPES(pub i32);
pub const CCT_SCOPE: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32768i32);
pub const CCT_RESULT: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32769i32);
pub const CCT_SNAPIN_MANAGER: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32770i32);
pub const CCT_UNINITIALIZED: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(65535i32);
#[repr(transparent)]
pub struct Document(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Extension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Extensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Frame(pub *mut ::core::ffi::c_void);
pub const HDI_HIDDEN: u32 = 1u32;
pub const HIDE_COLUMN: i32 = -4i32;
#[repr(transparent)]
pub struct IColumnData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentData2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsole(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsole2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsole3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsoleNameSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsoleNameSpace2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsolePower(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsolePowerSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConsoleVerb(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenuCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenuCallback2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenuProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayHelp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumTASK(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendContextMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendControlbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendPropertySheet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendPropertySheet2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendTaskPad(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHeaderCtrl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHeaderCtrl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageList(pub *mut ::core::ffi::c_void);
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
#[repr(transparent)]
pub struct IMMCVersionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INodeProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertySheetCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertySheetProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRequiredExtensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResultData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResultData2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResultDataCompare(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResultDataCompareEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResultOwnerData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISnapinAbout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISnapinHelp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISnapinHelp2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISnapinProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISnapinPropertiesCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStringTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewExtensionCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IconIdentifier(pub i32);
pub const Icon_None: IconIdentifier = IconIdentifier(0i32);
pub const Icon_Error: IconIdentifier = IconIdentifier(32513i32);
pub const Icon_Question: IconIdentifier = IconIdentifier(32514i32);
pub const Icon_Warning: IconIdentifier = IconIdentifier(32515i32);
pub const Icon_Information: IconIdentifier = IconIdentifier(32516i32);
pub const Icon_First: IconIdentifier = IconIdentifier(32513i32);
pub const Icon_Last: IconIdentifier = IconIdentifier(32516i32);
#[repr(C)]
pub struct MENUBUTTONDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMCBUTTON(i32);
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
pub const MMCVersionInfo: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3607026461,
    data2: 53025,
    data3: 19417,
    data4: [175, 59, 197, 70, 142, 156, 102, 132],
};
#[repr(transparent)]
pub struct MMC_ACTION_TYPE(pub i32);
pub const MMC_ACTION_UNINITIALIZED: MMC_ACTION_TYPE = MMC_ACTION_TYPE(-1i32);
pub const MMC_ACTION_ID: MMC_ACTION_TYPE = MMC_ACTION_TYPE(0i32);
pub const MMC_ACTION_LINK: MMC_ACTION_TYPE = MMC_ACTION_TYPE(1i32);
pub const MMC_ACTION_SCRIPT: MMC_ACTION_TYPE = MMC_ACTION_TYPE(2i32);
#[repr(transparent)]
pub struct MMC_BUTTON_STATE(pub i32);
pub const ENABLED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(1i32);
pub const CHECKED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(2i32);
pub const HIDDEN: MMC_BUTTON_STATE = MMC_BUTTON_STATE(4i32);
pub const INDETERMINATE: MMC_BUTTON_STATE = MMC_BUTTON_STATE(8i32);
pub const BUTTONPRESSED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(16i32);
#[repr(C)]
pub struct MMC_COLUMN_DATA(i32);
#[repr(C)]
pub struct MMC_COLUMN_SET_DATA(i32);
#[repr(transparent)]
pub struct MMC_CONSOLE_VERB(pub i32);
pub const MMC_VERB_NONE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(0i32);
pub const MMC_VERB_OPEN: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
pub const MMC_VERB_COPY: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32769i32);
pub const MMC_VERB_PASTE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32770i32);
pub const MMC_VERB_DELETE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32771i32);
pub const MMC_VERB_PROPERTIES: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32772i32);
pub const MMC_VERB_RENAME: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32773i32);
pub const MMC_VERB_REFRESH: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32774i32);
pub const MMC_VERB_PRINT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32775i32);
pub const MMC_VERB_CUT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
pub const MMC_VERB_MAX: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32777i32);
pub const MMC_VERB_FIRST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
pub const MMC_VERB_LAST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
#[repr(transparent)]
pub struct MMC_CONTROL_TYPE(pub i32);
pub const TOOLBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(0i32);
pub const MENUBUTTON: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(1i32);
pub const COMBOBOXBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(2i32);
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_EXPANDSYNC_STRUCT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_EXT_VIEW_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_FILTERDATA(i32);
#[repr(transparent)]
pub struct MMC_FILTER_CHANGE_CODE(pub i32);
pub const MFCC_DISABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(0i32);
pub const MFCC_ENABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(1i32);
pub const MFCC_VALUE_CHANGE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(2i32);
#[repr(transparent)]
pub struct MMC_FILTER_TYPE(pub i32);
pub const MMC_STRING_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(0i32);
pub const MMC_INT_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(1i32);
pub const MMC_FILTER_NOVALUE: MMC_FILTER_TYPE = MMC_FILTER_TYPE(32768i32);
pub const MMC_IMAGECALLBACK: i32 = -1i32;
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_LISTPAD_INFO(i32);
#[repr(transparent)]
pub struct MMC_MENU_COMMAND_IDS(pub i32);
pub const MMCC_STANDARD_VIEW_SELECT: MMC_MENU_COMMAND_IDS = MMC_MENU_COMMAND_IDS(-1i32);
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
#[repr(transparent)]
pub struct MMC_NOTIFY_TYPE(pub i32);
pub const MMCN_ACTIVATE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32769i32);
pub const MMCN_ADD_IMAGES: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32770i32);
pub const MMCN_BTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32771i32);
pub const MMCN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32772i32);
pub const MMCN_COLUMN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32773i32);
pub const MMCN_CONTEXTMENU: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32774i32);
pub const MMCN_CUTORMOVE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32775i32);
pub const MMCN_DBLCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32776i32);
pub const MMCN_DELETE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32777i32);
pub const MMCN_DESELECT_ALL: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32778i32);
pub const MMCN_EXPAND: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32779i32);
pub const MMCN_HELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32780i32);
pub const MMCN_MENU_BTNCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32781i32);
pub const MMCN_MINIMIZED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32782i32);
pub const MMCN_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32783i32);
pub const MMCN_PROPERTY_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32784i32);
pub const MMCN_QUERY_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32785i32);
pub const MMCN_REFRESH: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32786i32);
pub const MMCN_REMOVE_CHILDREN: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32787i32);
pub const MMCN_RENAME: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32788i32);
pub const MMCN_SELECT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32789i32);
pub const MMCN_SHOW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32790i32);
pub const MMCN_VIEW_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32791i32);
pub const MMCN_SNAPINHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32792i32);
pub const MMCN_CONTEXTHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32793i32);
pub const MMCN_INITOCX: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32794i32);
pub const MMCN_FILTER_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32795i32);
pub const MMCN_FILTERBTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32796i32);
pub const MMCN_RESTORE_VIEW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32797i32);
pub const MMCN_PRINT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32798i32);
pub const MMCN_PRELOAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32799i32);
pub const MMCN_LISTPAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32800i32);
pub const MMCN_EXPANDSYNC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32801i32);
pub const MMCN_COLUMNS_CHANGED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32802i32);
pub const MMCN_CANPASTE_OUTOFPROC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32803i32);
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
#[repr(transparent)]
pub struct MMC_PROPERTY_ACTION(pub i32);
pub const MMC_PROPACT_DELETING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(1i32);
pub const MMC_PROPACT_CHANGING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(2i32);
pub const MMC_PROPACT_INITIALIZED: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(3i32);
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
pub const MMC_PROP_PERSIST: u32 = 8u32;
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
pub const MMC_PSO_HASHELP: u32 = 2u32;
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_RESTORE_VIEW(i32);
#[repr(transparent)]
pub struct MMC_RESULT_VIEW_STYLE(pub i32);
pub const MMC_SINGLESEL: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(1i32);
pub const MMC_SHOWSELALWAYS: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(2i32);
pub const MMC_NOSORTHEADER: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(4i32);
pub const MMC_ENSUREFOCUSVISIBLE: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(8i32);
#[repr(transparent)]
pub struct MMC_SCOPE_ITEM_STATE(pub i32);
pub const MMC_SCOPE_ITEM_STATE_NORMAL: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(1i32);
pub const MMC_SCOPE_ITEM_STATE_BOLD: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(2i32);
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(3i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct MMC_SNAPIN_PROPERTY(i32);
#[repr(C)]
pub struct MMC_SORT_DATA(i32);
#[repr(C)]
pub struct MMC_SORT_SET_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_TASK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_TASK_DISPLAY_BITMAP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_TASK_DISPLAY_OBJECT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MMC_TASK_DISPLAY_SYMBOL(i32);
#[repr(transparent)]
pub struct MMC_TASK_DISPLAY_TYPE(pub i32);
pub const MMC_TASK_DISPLAY_UNINITIALIZED: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(0i32);
pub const MMC_TASK_DISPLAY_TYPE_SYMBOL: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(1i32);
pub const MMC_TASK_DISPLAY_TYPE_VANILLA_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(2i32);
pub const MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(3i32);
pub const MMC_TASK_DISPLAY_TYPE_BITMAP: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(4i32);
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
#[repr(transparent)]
pub struct MMC_VIEW_TYPE(pub i32);
pub const MMC_VIEW_TYPE_LIST: MMC_VIEW_TYPE = MMC_VIEW_TYPE(0i32);
pub const MMC_VIEW_TYPE_HTML: MMC_VIEW_TYPE = MMC_VIEW_TYPE(1i32);
pub const MMC_VIEW_TYPE_OCX: MMC_VIEW_TYPE = MMC_VIEW_TYPE(2i32);
#[repr(C)]
pub struct MMC_VISIBLE_COLUMNS(i32);
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
#[repr(transparent)]
pub struct MenuItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Node(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Nodes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Properties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Property(pub *mut ::core::ffi::c_void);
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RDCOMPARE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RDITEMHDR(i32);
pub const RDI_IMAGE: u32 = 4u32;
pub const RDI_INDENT: u32 = 64u32;
pub const RDI_INDEX: u32 = 32u32;
pub const RDI_PARAM: u32 = 16u32;
pub const RDI_STATE: u32 = 8u32;
pub const RDI_STR: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESULTDATAITEM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESULTFINDINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESULT_VIEW_TYPE_INFO(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SCOPEDATAITEM(i32);
#[repr(C)]
pub struct SColumnSetID(i32);
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
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SMMCDataObjects(i32);
#[repr(C)]
pub struct SMMCObjectTypes(i32);
#[repr(C)]
pub struct SNodeID(i32);
#[repr(C)]
pub struct SNodeID2(i32);
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
#[repr(transparent)]
pub struct ScopeNamespace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SnapIn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SnapIns(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct View(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Views(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _AppEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _Application(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ColumnSortOrder(pub i32);
pub const SortOrder_Ascending: _ColumnSortOrder = _ColumnSortOrder(0i32);
pub const SortOrder_Descending: _ColumnSortOrder = _ColumnSortOrder(1i32);
#[repr(transparent)]
pub struct _DocumentMode(pub i32);
pub const DocumentMode_Author: _DocumentMode = _DocumentMode(0i32);
pub const DocumentMode_User: _DocumentMode = _DocumentMode(1i32);
pub const DocumentMode_User_MDI: _DocumentMode = _DocumentMode(2i32);
pub const DocumentMode_User_SDI: _DocumentMode = _DocumentMode(3i32);
#[repr(transparent)]
pub struct _EventConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ExportListOptions(pub i32);
pub const ExportListOptions_Default: _ExportListOptions = _ExportListOptions(0i32);
pub const ExportListOptions_Unicode: _ExportListOptions = _ExportListOptions(1i32);
pub const ExportListOptions_TabDelimited: _ExportListOptions = _ExportListOptions(2i32);
pub const ExportListOptions_SelectedItemsOnly: _ExportListOptions = _ExportListOptions(4i32);
#[repr(transparent)]
pub struct _ListViewMode(pub i32);
pub const ListMode_Small_Icons: _ListViewMode = _ListViewMode(0i32);
pub const ListMode_Large_Icons: _ListViewMode = _ListViewMode(1i32);
pub const ListMode_List: _ListViewMode = _ListViewMode(2i32);
pub const ListMode_Detail: _ListViewMode = _ListViewMode(3i32);
pub const ListMode_Filtered: _ListViewMode = _ListViewMode(4i32);
#[repr(transparent)]
pub struct _ViewOptions(pub i32);
pub const ViewOption_Default: _ViewOptions = _ViewOptions(0i32);
pub const ViewOption_ScopeTreeHidden: _ViewOptions = _ViewOptions(1i32);
pub const ViewOption_NoToolBars: _ViewOptions = _ViewOptions(2i32);
pub const ViewOption_NotPersistable: _ViewOptions = _ViewOptions(4i32);
pub const ViewOption_ActionPaneHidden: _ViewOptions = _ViewOptions(8i32);
