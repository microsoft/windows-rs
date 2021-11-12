#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const AUTO_WIDTH: i32 = -1i32;
#[repr(transparent)]
pub struct AppEvents(pub *mut ::core::ffi::c_void);
pub struct AppEventsDHTMLConnector(i32);
pub struct Application(i32);
pub struct CCM_COMMANDID_MASK_CONSTANTS(i32);
pub struct CCM_INSERTIONALLOWED(i32);
pub struct CCM_INSERTIONPOINTID(i32);
pub struct CCM_SPECIAL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM2(i32);
#[repr(transparent)]
pub struct Column(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Columns(pub *mut ::core::ffi::c_void);
pub struct ConsolePower(i32);
#[repr(transparent)]
pub struct ContextMenu(pub *mut ::core::ffi::c_void);
pub struct DATA_OBJECT_TYPES(i32);
#[repr(transparent)]
pub struct Document(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Extension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Extensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Frame(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const HDI_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
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
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
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
pub struct IconIdentifier(i32);
pub struct MENUBUTTONDATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MMCBUTTON(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_AUTO: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_NOICON: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_NOPARAM: i32 = -2i32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_NOPTR: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_UPDATE_NOINVALIDATEALL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_UPDATE_NOSCROLL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_VIEWSTYLE_FILTERED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_VIEWSTYLE_ICON: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_VIEWSTYLE_LIST: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_VIEWSTYLE_REPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMCLV_VIEWSTYLE_SMALLICON: u32 = 2u32;
pub struct MMCVersionInfo(i32);
pub struct MMC_ACTION_TYPE(i32);
pub struct MMC_BUTTON_STATE(i32);
pub struct MMC_COLUMN_DATA(i32);
pub struct MMC_COLUMN_SET_DATA(i32);
pub struct MMC_CONSOLE_VERB(i32);
pub struct MMC_CONTROL_TYPE(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXPANDSYNC_STRUCT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXT_VIEW_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_FILTERDATA(i32);
pub struct MMC_FILTER_CHANGE_CODE(i32);
pub struct MMC_FILTER_TYPE(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_IMAGECALLBACK: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_LISTPAD_INFO(i32);
pub struct MMC_MENU_COMMAND_IDS(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
pub struct MMC_NOTIFY_TYPE(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
pub struct MMC_PROPERTY_ACTION(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PROP_PERSIST: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PSO_HASHELP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_RESTORE_VIEW(i32);
pub struct MMC_RESULT_VIEW_STYLE(i32);
pub struct MMC_SCOPE_ITEM_STATE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct MMC_SNAPIN_PROPERTY(i32);
pub struct MMC_SORT_DATA(i32);
pub struct MMC_SORT_SET_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_BITMAP(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_OBJECT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_SYMBOL(i32);
pub struct MMC_TASK_DISPLAY_TYPE(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VER: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_CREATENEW: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_OWNERDATALIST: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const MMC_VIEW_OPTIONS_USEFONTLINKING: u32 = 32u32;
pub struct MMC_VIEW_TYPE(i32);
pub struct MMC_VISIBLE_COLUMNS(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
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
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RDCOMPARE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RDITEMHDR(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RDI_INDENT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RDI_INDEX: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RDI_PARAM: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RDI_STATE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RDI_STR: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTDATAITEM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTFINDINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RFI_PARTIAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RFI_WRAP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RSI_DESCENDING: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RSI_NOSORTICON: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_HTML_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_HTML_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_ALLOWPASTE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_MULTISELECT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_OWNERDATALIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_LIST_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_MISC_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_OCX_OPTIONS_CACHE_OCX: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_OCX_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const RVTI_OCX_OPTIONS_NONE: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SCOPEDATAITEM(i32);
pub struct SColumnSetID(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_CHILDREN: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_FIRST: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_NEXT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_OPENIMAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_PARAM: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_PARENT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_PREVIOUS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_STATE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SDI_STR: u32 = 2u32;
#[cfg(feature = "Win32_System_Com")]
pub struct SMMCDataObjects(i32);
pub struct SMMCObjectTypes(i32);
pub struct SNodeID(i32);
pub struct SNodeID2(i32);
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mmc`*"]
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
pub struct _ColumnSortOrder(i32);
pub struct _DocumentMode(i32);
#[repr(transparent)]
pub struct _EventConnector(pub *mut ::core::ffi::c_void);
pub struct _ExportListOptions(i32);
pub struct _ListViewMode(i32);
pub struct _ViewOptions(i32);
