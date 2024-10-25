#[cfg(feature = "Win32_UI_Shell_Common")]
pub mod Common;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub mod PropertiesSystem;
pub const ABE_BOTTOM: u32 = 3u32;
pub const ABE_LEFT: u32 = 0u32;
pub const ABE_RIGHT: u32 = 2u32;
pub const ABE_TOP: u32 = 1u32;
pub const ABM_ACTIVATE: u32 = 6u32;
pub const ABM_GETAUTOHIDEBAR: u32 = 7u32;
pub const ABM_GETAUTOHIDEBAREX: u32 = 11u32;
pub const ABM_GETSTATE: u32 = 4u32;
pub const ABM_GETTASKBARPOS: u32 = 5u32;
pub const ABM_NEW: u32 = 0u32;
pub const ABM_QUERYPOS: u32 = 2u32;
pub const ABM_REMOVE: u32 = 1u32;
pub const ABM_SETAUTOHIDEBAR: u32 = 8u32;
pub const ABM_SETAUTOHIDEBAREX: u32 = 12u32;
pub const ABM_SETPOS: u32 = 3u32;
pub const ABM_SETSTATE: u32 = 10u32;
pub const ABM_WINDOWPOSCHANGED: u32 = 9u32;
pub const ABN_FULLSCREENAPP: u32 = 2u32;
pub const ABN_POSCHANGED: u32 = 1u32;
pub const ABN_STATECHANGE: u32 = 0u32;
pub const ABN_WINDOWARRANGE: u32 = 3u32;
pub const ABS_ALWAYSONTOP: u32 = 2u32;
pub const ABS_AUTOHIDE: u32 = 1u32;
pub const ACDD_VISIBLE: u32 = 1u32;
pub const ACEO_FIRSTUNUSED: ACENUMOPTION = 65536i32;
pub const ACEO_MOSTRECENTFIRST: ACENUMOPTION = 1i32;
pub const ACEO_NONE: ACENUMOPTION = 0i32;
pub const ACLO_CURRENTDIR: AUTOCOMPLETELISTOPTIONS = 1i32;
pub const ACLO_DESKTOP: AUTOCOMPLETELISTOPTIONS = 4i32;
pub const ACLO_FAVORITES: AUTOCOMPLETELISTOPTIONS = 8i32;
pub const ACLO_FILESYSDIRS: AUTOCOMPLETELISTOPTIONS = 32i32;
pub const ACLO_FILESYSONLY: AUTOCOMPLETELISTOPTIONS = 16i32;
pub const ACLO_MYCOMPUTER: AUTOCOMPLETELISTOPTIONS = 2i32;
pub const ACLO_NONE: AUTOCOMPLETELISTOPTIONS = 0i32;
pub const ACLO_VIRTUALNAMESPACE: AUTOCOMPLETELISTOPTIONS = 64i32;
pub const ACO_AUTOAPPEND: AUTOCOMPLETEOPTIONS = 2i32;
pub const ACO_AUTOSUGGEST: AUTOCOMPLETEOPTIONS = 1i32;
pub const ACO_FILTERPREFIXES: AUTOCOMPLETEOPTIONS = 8i32;
pub const ACO_NONE: AUTOCOMPLETEOPTIONS = 0i32;
pub const ACO_NOPREFIXFILTERING: AUTOCOMPLETEOPTIONS = 256i32;
pub const ACO_RTLREADING: AUTOCOMPLETEOPTIONS = 64i32;
pub const ACO_SEARCH: AUTOCOMPLETEOPTIONS = 4i32;
pub const ACO_UPDOWNKEYDROPSLIST: AUTOCOMPLETEOPTIONS = 32i32;
pub const ACO_USETAB: AUTOCOMPLETEOPTIONS = 16i32;
pub const ACO_WORD_FILTER: AUTOCOMPLETEOPTIONS = 128i32;
pub const ADDURL_SILENT: u32 = 1u32;
pub const ADE_LEFT: ADJACENT_DISPLAY_EDGES = 1i32;
pub const ADE_NONE: ADJACENT_DISPLAY_EDGES = 0i32;
pub const ADE_RIGHT: ADJACENT_DISPLAY_EDGES = 2i32;
pub const ADLT_FREQUENT: APPDOCLISTTYPE = 1i32;
pub const ADLT_RECENT: APPDOCLISTTYPE = 0i32;
pub const AD_APPLY_BUFFERED_REFRESH: u32 = 16u32;
pub const AD_APPLY_DYNAMICREFRESH: u32 = 32u32;
pub const AD_APPLY_FORCE: u32 = 8u32;
pub const AD_APPLY_HTMLGEN: u32 = 2u32;
pub const AD_APPLY_REFRESH: u32 = 4u32;
pub const AD_APPLY_SAVE: u32 = 1u32;
pub const AD_GETWP_BMP: u32 = 0u32;
pub const AD_GETWP_IMAGE: u32 = 1u32;
pub const AD_GETWP_LAST_APPLIED: u32 = 2u32;
pub const AHE_DESKTOP: AHE_TYPE = 0i32;
pub const AHE_IMMERSIVE: AHE_TYPE = 1i32;
pub const AHTYPE_ANY_APPLICATION: AHTYPE = 16i32;
pub const AHTYPE_ANY_PROGID: AHTYPE = 512i32;
pub const AHTYPE_APPLICATION: AHTYPE = 128i32;
pub const AHTYPE_CLASS_APPLICATION: AHTYPE = 256i32;
pub const AHTYPE_MACHINEDEFAULT: AHTYPE = 32i32;
pub const AHTYPE_PROGID: AHTYPE = 64i32;
pub const AHTYPE_UNDEFINED: AHTYPE = 0i32;
pub const AHTYPE_USER_APPLICATION: AHTYPE = 8i32;
pub const AIM_COMMENTS: APPINFODATAFLAGS = 32768i32;
pub const AIM_CONTACT: APPINFODATAFLAGS = 16384i32;
pub const AIM_DISPLAYNAME: APPINFODATAFLAGS = 1i32;
pub const AIM_HELPLINK: APPINFODATAFLAGS = 512i32;
pub const AIM_IMAGE: APPINFODATAFLAGS = 131072i32;
pub const AIM_INSTALLDATE: APPINFODATAFLAGS = 4096i32;
pub const AIM_INSTALLLOCATION: APPINFODATAFLAGS = 1024i32;
pub const AIM_INSTALLSOURCE: APPINFODATAFLAGS = 2048i32;
pub const AIM_LANGUAGE: APPINFODATAFLAGS = 64i32;
pub const AIM_PRODUCTID: APPINFODATAFLAGS = 8i32;
pub const AIM_PUBLISHER: APPINFODATAFLAGS = 4i32;
pub const AIM_READMEURL: APPINFODATAFLAGS = 262144i32;
pub const AIM_REGISTEREDCOMPANY: APPINFODATAFLAGS = 32i32;
pub const AIM_REGISTEREDOWNER: APPINFODATAFLAGS = 16i32;
pub const AIM_SUPPORTTELEPHONE: APPINFODATAFLAGS = 256i32;
pub const AIM_SUPPORTURL: APPINFODATAFLAGS = 128i32;
pub const AIM_UPDATEINFOURL: APPINFODATAFLAGS = 524288i32;
pub const AIM_VERSION: APPINFODATAFLAGS = 2i32;
pub const AL_EFFECTIVE: ASSOCIATIONLEVEL = 1i32;
pub const AL_MACHINE: ASSOCIATIONLEVEL = 0i32;
pub const AL_USER: ASSOCIATIONLEVEL = 2i32;
pub const AO_DESIGNMODE: ACTIVATEOPTIONS = 1i32;
pub const AO_NOERRORUI: ACTIVATEOPTIONS = 2i32;
pub const AO_NONE: ACTIVATEOPTIONS = 0i32;
pub const AO_NOSPLASHSCREEN: ACTIVATEOPTIONS = 4i32;
pub const AO_PRELAUNCH: ACTIVATEOPTIONS = 33554432i32;
pub const APPACTION_ADDLATER: APPACTIONFLAGS = 256i32;
pub const APPACTION_CANGETSIZE: APPACTIONFLAGS = 32i32;
pub const APPACTION_INSTALL: APPACTIONFLAGS = 1i32;
pub const APPACTION_MODIFY: APPACTIONFLAGS = 4i32;
pub const APPACTION_MODIFYREMOVE: APPACTIONFLAGS = 128i32;
pub const APPACTION_REPAIR: APPACTIONFLAGS = 8i32;
pub const APPACTION_UNINSTALL: APPACTIONFLAGS = 2i32;
pub const APPACTION_UNSCHEDULE: APPACTIONFLAGS = 512i32;
pub const APPACTION_UPGRADE: APPACTIONFLAGS = 16i32;
pub const APPNAMEBUFFERLEN: u32 = 40u32;
pub const ARCONTENT_AUDIOCD: u32 = 4u32;
pub const ARCONTENT_AUTOPLAYMUSIC: u32 = 256u32;
pub const ARCONTENT_AUTOPLAYPIX: u32 = 128u32;
pub const ARCONTENT_AUTOPLAYVIDEO: u32 = 512u32;
pub const ARCONTENT_AUTORUNINF: u32 = 2u32;
pub const ARCONTENT_BLANKBD: u32 = 8192u32;
pub const ARCONTENT_BLANKCD: u32 = 16u32;
pub const ARCONTENT_BLANKDVD: u32 = 32u32;
pub const ARCONTENT_BLURAY: u32 = 16384u32;
pub const ARCONTENT_CAMERASTORAGE: u32 = 32768u32;
pub const ARCONTENT_CUSTOMEVENT: u32 = 65536u32;
pub const ARCONTENT_DVDAUDIO: u32 = 4096u32;
pub const ARCONTENT_DVDMOVIE: u32 = 8u32;
pub const ARCONTENT_MASK: u32 = 131070u32;
pub const ARCONTENT_NONE: u32 = 0u32;
pub const ARCONTENT_PHASE_FINAL: u32 = 1073741824u32;
pub const ARCONTENT_PHASE_MASK: u32 = 1879048192u32;
pub const ARCONTENT_PHASE_PRESNIFF: u32 = 268435456u32;
pub const ARCONTENT_PHASE_SNIFFING: u32 = 536870912u32;
pub const ARCONTENT_PHASE_UNKNOWN: u32 = 0u32;
pub const ARCONTENT_SVCD: u32 = 2048u32;
pub const ARCONTENT_UNKNOWNCONTENT: u32 = 64u32;
pub const ARCONTENT_VCD: u32 = 1024u32;
pub const ASSOCCLASS_APP_KEY: ASSOCCLASS = 5i32;
pub const ASSOCCLASS_APP_STR: ASSOCCLASS = 6i32;
pub const ASSOCCLASS_CLSID_KEY: ASSOCCLASS = 3i32;
pub const ASSOCCLASS_CLSID_STR: ASSOCCLASS = 4i32;
pub const ASSOCCLASS_FIXED_PROGID_STR: ASSOCCLASS = 10i32;
pub const ASSOCCLASS_FOLDER: ASSOCCLASS = 8i32;
pub const ASSOCCLASS_PROGID_KEY: ASSOCCLASS = 1i32;
pub const ASSOCCLASS_PROGID_STR: ASSOCCLASS = 2i32;
pub const ASSOCCLASS_PROTOCOL_STR: ASSOCCLASS = 11i32;
pub const ASSOCCLASS_SHELL_KEY: ASSOCCLASS = 0i32;
pub const ASSOCCLASS_STAR: ASSOCCLASS = 9i32;
pub const ASSOCCLASS_SYSTEM_STR: ASSOCCLASS = 7i32;
pub const ASSOCDATA_EDITFLAGS: ASSOCDATA = 5i32;
pub const ASSOCDATA_HASPERUSERASSOC: ASSOCDATA = 4i32;
pub const ASSOCDATA_MAX: ASSOCDATA = 7i32;
pub const ASSOCDATA_MSIDESCRIPTOR: ASSOCDATA = 1i32;
pub const ASSOCDATA_NOACTIVATEHANDLER: ASSOCDATA = 2i32;
pub const ASSOCDATA_UNUSED1: ASSOCDATA = 3i32;
pub const ASSOCDATA_VALUE: ASSOCDATA = 6i32;
pub const ASSOCENUM_NONE: ASSOCENUM = 0i32;
pub const ASSOCF_APP_TO_APP: ASSOCF = 65536u32;
pub const ASSOCF_IGNOREBASECLASS: ASSOCF = 512u32;
pub const ASSOCF_INIT_BYEXENAME: ASSOCF = 2u32;
pub const ASSOCF_INIT_DEFAULTTOFOLDER: ASSOCF = 8u32;
pub const ASSOCF_INIT_DEFAULTTOSTAR: ASSOCF = 4u32;
pub const ASSOCF_INIT_FIXED_PROGID: ASSOCF = 2048u32;
pub const ASSOCF_INIT_FOR_FILE: ASSOCF = 8192u32;
pub const ASSOCF_INIT_IGNOREUNKNOWN: ASSOCF = 1024u32;
pub const ASSOCF_INIT_NOREMAPCLSID: ASSOCF = 1u32;
pub const ASSOCF_IS_FULL_URI: ASSOCF = 16384u32;
pub const ASSOCF_IS_PROTOCOL: ASSOCF = 4096u32;
pub const ASSOCF_NOFIXUPS: ASSOCF = 256u32;
pub const ASSOCF_NONE: ASSOCF = 0u32;
pub const ASSOCF_NOTRUNCATE: ASSOCF = 32u32;
pub const ASSOCF_NOUSERSETTINGS: ASSOCF = 16u32;
pub const ASSOCF_OPEN_BYEXENAME: ASSOCF = 2u32;
pub const ASSOCF_PER_MACHINE_ONLY: ASSOCF = 32768u32;
pub const ASSOCF_REMAPRUNDLL: ASSOCF = 128u32;
pub const ASSOCF_VERIFY: ASSOCF = 64u32;
pub const ASSOCKEY_APP: ASSOCKEY = 2i32;
pub const ASSOCKEY_BASECLASS: ASSOCKEY = 4i32;
pub const ASSOCKEY_CLASS: ASSOCKEY = 3i32;
pub const ASSOCKEY_MAX: ASSOCKEY = 5i32;
pub const ASSOCKEY_SHELLEXECCLASS: ASSOCKEY = 1i32;
pub const ASSOCSTR_APPICONREFERENCE: ASSOCSTR = 23i32;
pub const ASSOCSTR_APPID: ASSOCSTR = 21i32;
pub const ASSOCSTR_APPPUBLISHER: ASSOCSTR = 22i32;
pub const ASSOCSTR_COMMAND: ASSOCSTR = 1i32;
pub const ASSOCSTR_CONTENTTYPE: ASSOCSTR = 14i32;
pub const ASSOCSTR_DDEAPPLICATION: ASSOCSTR = 9i32;
pub const ASSOCSTR_DDECOMMAND: ASSOCSTR = 7i32;
pub const ASSOCSTR_DDEIFEXEC: ASSOCSTR = 8i32;
pub const ASSOCSTR_DDETOPIC: ASSOCSTR = 10i32;
pub const ASSOCSTR_DEFAULTICON: ASSOCSTR = 15i32;
pub const ASSOCSTR_DELEGATEEXECUTE: ASSOCSTR = 18i32;
pub const ASSOCSTR_DROPTARGET: ASSOCSTR = 17i32;
pub const ASSOCSTR_EXECUTABLE: ASSOCSTR = 2i32;
pub const ASSOCSTR_FRIENDLYAPPNAME: ASSOCSTR = 4i32;
pub const ASSOCSTR_FRIENDLYDOCNAME: ASSOCSTR = 3i32;
pub const ASSOCSTR_INFOTIP: ASSOCSTR = 11i32;
pub const ASSOCSTR_MAX: ASSOCSTR = 24i32;
pub const ASSOCSTR_NOOPEN: ASSOCSTR = 5i32;
pub const ASSOCSTR_PROGID: ASSOCSTR = 20i32;
pub const ASSOCSTR_QUICKTIP: ASSOCSTR = 12i32;
pub const ASSOCSTR_SHELLEXTENSION: ASSOCSTR = 16i32;
pub const ASSOCSTR_SHELLNEWVALUE: ASSOCSTR = 6i32;
pub const ASSOCSTR_SUPPORTED_URI_PROTOCOLS: ASSOCSTR = 19i32;
pub const ASSOCSTR_TILEINFO: ASSOCSTR = 13i32;
pub const ASSOC_FILTER_NONE: ASSOC_FILTER = 0i32;
pub const ASSOC_FILTER_RECOMMENDED: ASSOC_FILTER = 1i32;
pub const ATTACHMENT_ACTION_CANCEL: ATTACHMENT_ACTION = 0i32;
pub const ATTACHMENT_ACTION_EXEC: ATTACHMENT_ACTION = 2i32;
pub const ATTACHMENT_ACTION_SAVE: ATTACHMENT_ACTION = 1i32;
pub const ATTACHMENT_PROMPT_EXEC: ATTACHMENT_PROMPT = 2i32;
pub const ATTACHMENT_PROMPT_EXEC_OR_SAVE: ATTACHMENT_PROMPT = 3i32;
pub const ATTACHMENT_PROMPT_NONE: ATTACHMENT_PROMPT = 0i32;
pub const ATTACHMENT_PROMPT_SAVE: ATTACHMENT_PROMPT = 1i32;
pub const AT_FILEEXTENSION: ASSOCIATIONTYPE = 0i32;
pub const AT_MIMETYPE: ASSOCIATIONTYPE = 3i32;
pub const AT_STARTMENUCLIENT: ASSOCIATIONTYPE = 2i32;
pub const AT_URLPROTOCOL: ASSOCIATIONTYPE = 1i32;
pub const AVMW_320: APPLICATION_VIEW_MIN_WIDTH = 1i32;
pub const AVMW_500: APPLICATION_VIEW_MIN_WIDTH = 2i32;
pub const AVMW_DEFAULT: APPLICATION_VIEW_MIN_WIDTH = 0i32;
pub const AVO_LANDSCAPE: APPLICATION_VIEW_ORIENTATION = 0i32;
pub const AVO_PORTRAIT: APPLICATION_VIEW_ORIENTATION = 1i32;
pub const AVSP_CUSTOM: APPLICATION_VIEW_SIZE_PREFERENCE = 6i32;
pub const AVSP_DEFAULT: APPLICATION_VIEW_SIZE_PREFERENCE = 0i32;
pub const AVSP_USE_HALF: APPLICATION_VIEW_SIZE_PREFERENCE = 2i32;
pub const AVSP_USE_LESS: APPLICATION_VIEW_SIZE_PREFERENCE = 1i32;
pub const AVSP_USE_MINIMUM: APPLICATION_VIEW_SIZE_PREFERENCE = 4i32;
pub const AVSP_USE_MORE: APPLICATION_VIEW_SIZE_PREFERENCE = 3i32;
pub const AVSP_USE_NONE: APPLICATION_VIEW_SIZE_PREFERENCE = 5i32;
pub const AVS_FILLED: APPLICATION_VIEW_STATE = 1i32;
pub const AVS_FULLSCREEN_LANDSCAPE: APPLICATION_VIEW_STATE = 0i32;
pub const AVS_FULLSCREEN_PORTRAIT: APPLICATION_VIEW_STATE = 3i32;
pub const AVS_SNAPPED: APPLICATION_VIEW_STATE = 2i32;
pub const AllowSmallerSize: ThumbnailStreamCacheOptions = 4i32;
pub const BFFM_ENABLEOK: u32 = 1125u32;
pub const BFFM_INITIALIZED: u32 = 1u32;
pub const BFFM_IUNKNOWN: u32 = 5u32;
pub const BFFM_SELCHANGED: u32 = 2u32;
pub const BFFM_SETEXPANDED: u32 = 1130u32;
pub const BFFM_SETOKTEXT: u32 = 1129u32;
pub const BFFM_SETSELECTION: u32 = 1127u32;
pub const BFFM_SETSELECTIONA: u32 = 1126u32;
pub const BFFM_SETSELECTIONW: u32 = 1127u32;
pub const BFFM_SETSTATUSTEXT: u32 = 1128u32;
pub const BFFM_SETSTATUSTEXTA: u32 = 1124u32;
pub const BFFM_SETSTATUSTEXTW: u32 = 1128u32;
pub const BFFM_VALIDATEFAILED: u32 = 4u32;
pub const BFFM_VALIDATEFAILEDA: u32 = 3u32;
pub const BFFM_VALIDATEFAILEDW: u32 = 4u32;
pub const BFO_ADD_IE_TOCAPTIONBAR: _BROWSERFRAMEOPTIONS = 512i32;
pub const BFO_BOTH_OPTIONS: _BROWSERFRAMEOPTIONS = 4i32;
pub const BFO_BROWSER_PERSIST_SETTINGS: _BROWSERFRAMEOPTIONS = 1i32;
pub const BFO_BROWSE_NO_IN_NEW_PROCESS: _BROWSERFRAMEOPTIONS = 16i32;
pub const BFO_ENABLE_HYPERLINK_TRACKING: _BROWSERFRAMEOPTIONS = 32i32;
pub const BFO_GO_HOME_PAGE: _BROWSERFRAMEOPTIONS = 16384i32;
pub const BFO_NONE: _BROWSERFRAMEOPTIONS = 0i32;
pub const BFO_NO_PARENT_FOLDER_SUPPORT: _BROWSERFRAMEOPTIONS = 4096i32;
pub const BFO_NO_REOPEN_NEXT_RESTART: _BROWSERFRAMEOPTIONS = 8192i32;
pub const BFO_PREFER_IEPROCESS: _BROWSERFRAMEOPTIONS = 32768i32;
pub const BFO_QUERY_ALL: _BROWSERFRAMEOPTIONS = -1i32;
pub const BFO_RENAME_FOLDER_OPTIONS_TOINTERNET: _BROWSERFRAMEOPTIONS = 2i32;
pub const BFO_SHOW_NAVIGATION_CANCELLED: _BROWSERFRAMEOPTIONS = 65536i32;
pub const BFO_SUBSTITUE_INTERNET_START_PAGE: _BROWSERFRAMEOPTIONS = 128i32;
pub const BFO_USE_DIALUP_REF: _BROWSERFRAMEOPTIONS = 1024i32;
pub const BFO_USE_IE_LOGOBANDING: _BROWSERFRAMEOPTIONS = 256i32;
pub const BFO_USE_IE_OFFLINE_SUPPORT: _BROWSERFRAMEOPTIONS = 64i32;
pub const BFO_USE_IE_STATUSBAR: _BROWSERFRAMEOPTIONS = 131072i32;
pub const BFO_USE_IE_TOOLBAR: _BROWSERFRAMEOPTIONS = 2048i32;
pub const BHID_AssociationArray: windows_core::GUID = windows_core::GUID::from_u128(0xbea9ef17_82f1_4f60_9284_4f8db75c3be9);
pub const BHID_DataObject: windows_core::GUID = windows_core::GUID::from_u128(0xb8c0bd9f_ed24_455c_83e6_d5390c4fe8c4);
pub const BHID_EnumAssocHandlers: windows_core::GUID = windows_core::GUID::from_u128(0xb8ab0b9c_c2ec_4f7a_918d_314900e6280a);
pub const BHID_EnumItems: windows_core::GUID = windows_core::GUID::from_u128(0x94f60519_2850_4924_aa5a_d15e84868039);
pub const BHID_FilePlaceholder: windows_core::GUID = windows_core::GUID::from_u128(0x8677dceb_aae0_4005_8d3d_547fa852f825);
pub const BHID_Filter: windows_core::GUID = windows_core::GUID::from_u128(0x38d08778_f557_4690_9ebf_ba54706ad8f7);
pub const BHID_LinkTargetItem: windows_core::GUID = windows_core::GUID::from_u128(0x3981e228_f559_11d3_8e3a_00c04f6837d5);
pub const BHID_PropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0x0384e1a4_1523_439c_a4c8_ab911052f586);
pub const BHID_RandomAccessStream: windows_core::GUID = windows_core::GUID::from_u128(0xf16fc93b_77ae_4cfe_bda7_a866eea6878d);
pub const BHID_SFObject: windows_core::GUID = windows_core::GUID::from_u128(0x3981e224_f559_11d3_8e3a_00c04f6837d5);
pub const BHID_SFUIObject: windows_core::GUID = windows_core::GUID::from_u128(0x3981e225_f559_11d3_8e3a_00c04f6837d5);
pub const BHID_SFViewObject: windows_core::GUID = windows_core::GUID::from_u128(0x3981e226_f559_11d3_8e3a_00c04f6837d5);
pub const BHID_Storage: windows_core::GUID = windows_core::GUID::from_u128(0x3981e227_f559_11d3_8e3a_00c04f6837d5);
pub const BHID_StorageEnum: windows_core::GUID = windows_core::GUID::from_u128(0x4621a4e3_f0d6_4773_8a9c_46e77b174840);
pub const BHID_StorageItem: windows_core::GUID = windows_core::GUID::from_u128(0x404e2109_77d2_4699_a5a0_4fdf10db9837);
pub const BHID_Stream: windows_core::GUID = windows_core::GUID::from_u128(0x1cebb3ab_7c10_499a_a417_92ca16c4cb83);
pub const BHID_ThumbnailHandler: windows_core::GUID = windows_core::GUID::from_u128(0x7b2e650a_8e20_4f4a_b09e_6597afc72fb0);
pub const BHID_Transfer: windows_core::GUID = windows_core::GUID::from_u128(0xd5e346a1_f753_4932_b403_4574800e2498);
pub const BIF_BROWSEFILEJUNCTIONS: u32 = 65536u32;
pub const BIF_BROWSEFORCOMPUTER: u32 = 4096u32;
pub const BIF_BROWSEFORPRINTER: u32 = 8192u32;
pub const BIF_BROWSEINCLUDEFILES: u32 = 16384u32;
pub const BIF_BROWSEINCLUDEURLS: u32 = 128u32;
pub const BIF_DONTGOBELOWDOMAIN: u32 = 2u32;
pub const BIF_EDITBOX: u32 = 16u32;
pub const BIF_NEWDIALOGSTYLE: u32 = 64u32;
pub const BIF_NONEWFOLDERBUTTON: u32 = 512u32;
pub const BIF_NOTRANSLATETARGETS: u32 = 1024u32;
pub const BIF_PREFER_INTERNET_SHORTCUT: _BROWSERFRAMEOPTIONS = 8i32;
pub const BIF_RETURNFSANCESTORS: u32 = 8u32;
pub const BIF_RETURNONLYFSDIRS: u32 = 1u32;
pub const BIF_SHAREABLE: u32 = 32768u32;
pub const BIF_STATUSTEXT: u32 = 4u32;
pub const BIF_UAHINT: u32 = 256u32;
pub const BIF_VALIDATE: u32 = 32u32;
pub const BIND_INTERRUPTABLE: u32 = 4294967295u32;
pub const BMICON_LARGE: i32 = 0i32;
pub const BMICON_SMALL: i32 = 1i32;
pub const BNE_Button1Clicked: BANNER_NOTIFICATION_EVENT = 4i32;
pub const BNE_Button2Clicked: BANNER_NOTIFICATION_EVENT = 5i32;
pub const BNE_Closed: BANNER_NOTIFICATION_EVENT = 2i32;
pub const BNE_Dismissed: BANNER_NOTIFICATION_EVENT = 3i32;
pub const BNE_Hovered: BANNER_NOTIFICATION_EVENT = 1i32;
pub const BNE_Rendered: BANNER_NOTIFICATION_EVENT = 0i32;
pub const BNS_BEGIN_NAVIGATE: BNSTATE = 1i32;
pub const BNS_NAVIGATE: BNSTATE = 2i32;
pub const BNS_NORMAL: BNSTATE = 0i32;
pub const BSF_CANMAXIMIZE: u32 = 1024u32;
pub const BSF_DELEGATEDNAVIGATION: u32 = 65536u32;
pub const BSF_DONTSHOWNAVCANCELPAGE: u32 = 16384u32;
pub const BSF_FEEDNAVIGATION: u32 = 524288u32;
pub const BSF_FEEDSUBSCRIBED: u32 = 1048576u32;
pub const BSF_HTMLNAVCANCELED: u32 = 8192u32;
pub const BSF_MERGEDMENUS: u32 = 262144u32;
pub const BSF_NAVNOHISTORY: u32 = 4096u32;
pub const BSF_NOLOCALFILEWARNING: u32 = 16u32;
pub const BSF_REGISTERASDROPTARGET: u32 = 1u32;
pub const BSF_RESIZABLE: u32 = 512u32;
pub const BSF_SETNAVIGATABLECODEPAGE: u32 = 32768u32;
pub const BSF_THEATERMODE: u32 = 2u32;
pub const BSF_TOPBROWSER: u32 = 2048u32;
pub const BSF_TRUSTEDFORACTIVEX: u32 = 131072u32;
pub const BSF_UISETBYAUTOMATION: u32 = 256u32;
pub const BSID_BANDADDED: BANDSITECID = 0i32;
pub const BSID_BANDREMOVED: BANDSITECID = 1i32;
pub const BSIM_STATE: u32 = 1u32;
pub const BSIM_STYLE: u32 = 2u32;
pub const BSIS_ALWAYSGRIPPER: u32 = 2u32;
pub const BSIS_AUTOGRIPPER: u32 = 0u32;
pub const BSIS_FIXEDORDER: u32 = 1024u32;
pub const BSIS_LEFTALIGN: u32 = 4u32;
pub const BSIS_LOCKED: u32 = 256u32;
pub const BSIS_NOCAPTION: u32 = 64u32;
pub const BSIS_NOCONTEXTMENU: u32 = 16u32;
pub const BSIS_NODROPTARGET: u32 = 32u32;
pub const BSIS_NOGRIPPER: u32 = 1u32;
pub const BSIS_PREFERNOLINEBREAK: u32 = 128u32;
pub const BSIS_PRESERVEORDERDURINGLAYOUT: u32 = 512u32;
pub const BSIS_SINGLECLICK: u32 = 8u32;
pub const BSSF_NOTITLE: u32 = 2u32;
pub const BSSF_UNDELETEABLE: u32 = 4096u32;
pub const BSSF_VISIBLE: u32 = 1u32;
pub const BUFFLEN: u32 = 255u32;
pub const CABINETSTATE_VERSION: u32 = 2u32;
pub const CAMERAROLL_E_NO_DOWNSAMPLING_REQUIRED: windows_core::HRESULT = 0x80270120_u32 as _;
pub const CATID_BrowsableShellExt: windows_core::GUID = windows_core::GUID::from_u128(0x00021490_0000_0000_c000_000000000046);
pub const CATID_BrowseInPlace: windows_core::GUID = windows_core::GUID::from_u128(0x00021491_0000_0000_c000_000000000046);
pub const CATID_CommBand: windows_core::GUID = windows_core::GUID::from_u128(0x00021494_0000_0000_c000_000000000046);
pub const CATID_DeskBand: windows_core::GUID = windows_core::GUID::from_u128(0x00021492_0000_0000_c000_000000000046);
pub const CATID_FilePlaceholderMergeHandler: windows_core::GUID = windows_core::GUID::from_u128(0x3e9c9a51_d4aa_4870_b47c_7424b491f1cc);
pub const CATID_InfoBand: windows_core::GUID = windows_core::GUID::from_u128(0x00021493_0000_0000_c000_000000000046);
pub const CATID_LocationFactory: windows_core::GUID = windows_core::GUID::from_u128(0x965c4d51_8b76_4e57_80b7_564d2ea4b55e);
pub const CATID_LocationProvider: windows_core::GUID = windows_core::GUID::from_u128(0x1b3ca474_2614_414b_b813_1aceca3e3dd8);
pub const CATID_SearchableApplication: windows_core::GUID = windows_core::GUID::from_u128(0x366c292a_d9b3_4dbf_bb70_e62ec3d0bbbf);
pub const CATINFO_COLLAPSED: CATEGORYINFO_FLAGS = 1i32;
pub const CATINFO_EXPANDED: CATEGORYINFO_FLAGS = 4i32;
pub const CATINFO_HIDDEN: CATEGORYINFO_FLAGS = 2i32;
pub const CATINFO_NOHEADER: CATEGORYINFO_FLAGS = 8i32;
pub const CATINFO_NOHEADERCOUNT: CATEGORYINFO_FLAGS = 32i32;
pub const CATINFO_NORMAL: CATEGORYINFO_FLAGS = 0i32;
pub const CATINFO_NOTCOLLAPSIBLE: CATEGORYINFO_FLAGS = 16i32;
pub const CATINFO_SEPARATE_IMAGES: CATEGORYINFO_FLAGS = 128i32;
pub const CATINFO_SHOWEMPTY: CATEGORYINFO_FLAGS = 256i32;
pub const CATINFO_SUBSETTED: CATEGORYINFO_FLAGS = 64i32;
pub const CATSORT_DEFAULT: CATSORT_FLAGS = 0i32;
pub const CATSORT_NAME: CATSORT_FLAGS = 1i32;
pub const CDB2GVF_ADDSHIELD: u32 = 64u32;
pub const CDB2GVF_ALLOWPREVIEWPANE: u32 = 4u32;
pub const CDB2GVF_ISFILESAVE: u32 = 2u32;
pub const CDB2GVF_ISFOLDERPICKER: u32 = 32u32;
pub const CDB2GVF_NOINCLUDEITEM: u32 = 16u32;
pub const CDB2GVF_NOSELECTVERB: u32 = 8u32;
pub const CDB2GVF_SHOWALLFILES: u32 = 1u32;
pub const CDB2N_CONTEXTMENU_DONE: u32 = 1u32;
pub const CDB2N_CONTEXTMENU_START: u32 = 2u32;
pub const CDBE_RET_DEFAULT: CDBURNINGEXTENSIONRET = 0i32;
pub const CDBE_RET_DONTRUNOTHEREXTS: CDBURNINGEXTENSIONRET = 1i32;
pub const CDBE_RET_STOPWIZARD: CDBURNINGEXTENSIONRET = 2i32;
pub const CDBE_TYPE_ALL: _CDBE_ACTIONS = -1i32;
pub const CDBE_TYPE_DATA: _CDBE_ACTIONS = 2i32;
pub const CDBE_TYPE_MUSIC: _CDBE_ACTIONS = 1i32;
pub const CDBOSC_KILLFOCUS: u32 = 1u32;
pub const CDBOSC_RENAME: u32 = 3u32;
pub const CDBOSC_SELCHANGE: u32 = 2u32;
pub const CDBOSC_SETFOCUS: u32 = 0u32;
pub const CDBOSC_STATECHANGE: u32 = 4u32;
pub const CDCS_ENABLED: CDCONTROLSTATEF = 1i32;
pub const CDCS_ENABLEDVISIBLE: CDCONTROLSTATEF = 3i32;
pub const CDCS_INACTIVE: CDCONTROLSTATEF = 0i32;
pub const CDCS_VISIBLE: CDCONTROLSTATEF = 2i32;
pub const CFSTR_AUTOPLAY_SHELLIDLISTS: windows_core::PCWSTR = windows_core::w!("Autoplay Enumerated IDList Array");
pub const CFSTR_DROPDESCRIPTION: windows_core::PCWSTR = windows_core::w!("DropDescription");
pub const CFSTR_FILECONTENTS: windows_core::PCWSTR = windows_core::w!("FileContents");
pub const CFSTR_FILEDESCRIPTOR: windows_core::PCWSTR = windows_core::w!("FileGroupDescriptorW");
pub const CFSTR_FILEDESCRIPTORA: windows_core::PCWSTR = windows_core::w!("FileGroupDescriptor");
pub const CFSTR_FILEDESCRIPTORW: windows_core::PCWSTR = windows_core::w!("FileGroupDescriptorW");
pub const CFSTR_FILENAME: windows_core::PCWSTR = windows_core::w!("FileNameW");
pub const CFSTR_FILENAMEA: windows_core::PCWSTR = windows_core::w!("FileName");
pub const CFSTR_FILENAMEMAP: windows_core::PCWSTR = windows_core::w!("FileNameMapW");
pub const CFSTR_FILENAMEMAPA: windows_core::PCWSTR = windows_core::w!("FileNameMap");
pub const CFSTR_FILENAMEMAPW: windows_core::PCWSTR = windows_core::w!("FileNameMapW");
pub const CFSTR_FILENAMEW: windows_core::PCWSTR = windows_core::w!("FileNameW");
pub const CFSTR_FILE_ATTRIBUTES_ARRAY: windows_core::PCWSTR = windows_core::w!("File Attributes Array");
pub const CFSTR_INDRAGLOOP: windows_core::PCWSTR = windows_core::w!("InShellDragLoop");
pub const CFSTR_INETURL: windows_core::PCWSTR = windows_core::w!("UniformResourceLocatorW");
pub const CFSTR_INETURLA: windows_core::PCWSTR = windows_core::w!("UniformResourceLocator");
pub const CFSTR_INETURLW: windows_core::PCWSTR = windows_core::w!("UniformResourceLocatorW");
pub const CFSTR_INVOKECOMMAND_DROPPARAM: windows_core::PCWSTR = windows_core::w!("InvokeCommand DropParam");
pub const CFSTR_LOGICALPERFORMEDDROPEFFECT: windows_core::PCWSTR = windows_core::w!("Logical Performed DropEffect");
pub const CFSTR_MOUNTEDVOLUME: windows_core::PCWSTR = windows_core::w!("MountedVolume");
pub const CFSTR_NETRESOURCES: windows_core::PCWSTR = windows_core::w!("Net Resource");
pub const CFSTR_PASTESUCCEEDED: windows_core::PCWSTR = windows_core::w!("Paste Succeeded");
pub const CFSTR_PERFORMEDDROPEFFECT: windows_core::PCWSTR = windows_core::w!("Performed DropEffect");
pub const CFSTR_PERSISTEDDATAOBJECT: windows_core::PCWSTR = windows_core::w!("PersistedDataObject");
pub const CFSTR_PREFERREDDROPEFFECT: windows_core::PCWSTR = windows_core::w!("Preferred DropEffect");
pub const CFSTR_PRINTERGROUP: windows_core::PCWSTR = windows_core::w!("PrinterFriendlyName");
pub const CFSTR_SHELLDROPHANDLER: windows_core::PCWSTR = windows_core::w!("DropHandlerCLSID");
pub const CFSTR_SHELLIDLIST: windows_core::PCWSTR = windows_core::w!("Shell IDList Array");
pub const CFSTR_SHELLIDLISTOFFSET: windows_core::PCWSTR = windows_core::w!("Shell Object Offsets");
pub const CFSTR_SHELLURL: windows_core::PCWSTR = windows_core::w!("UniformResourceLocator");
pub const CFSTR_TARGETCLSID: windows_core::PCWSTR = windows_core::w!("TargetCLSID");
pub const CFSTR_UNTRUSTEDDRAGDROP: windows_core::PCWSTR = windows_core::w!("UntrustedDragDrop");
pub const CFSTR_ZONEIDENTIFIER: windows_core::PCWSTR = windows_core::w!("ZoneIdentifier");
pub const CGID_DefView: windows_core::GUID = windows_core::GUID::from_u128(0x4af07f10_d231_11d0_b942_00a0c90312e1);
pub const CGID_Explorer: windows_core::GUID = windows_core::GUID::from_u128(0x000214d0_0000_0000_c000_000000000046);
pub const CGID_ExplorerBarDoc: windows_core::GUID = windows_core::GUID::from_u128(0x000214d3_0000_0000_c000_000000000046);
pub const CGID_MENUDESKBAR: windows_core::GUID = windows_core::GUID::from_u128(0x5c9f0a12_959e_11d0_a3a4_00a0c9082636);
pub const CGID_ShellDocView: windows_core::GUID = windows_core::GUID::from_u128(0x000214d1_0000_0000_c000_000000000046);
pub const CGID_ShellServiceObject: windows_core::GUID = windows_core::GUID::from_u128(0x000214d2_0000_0000_c000_000000000046);
pub const CGID_ShortCut: windows_core::GUID = windows_core::GUID::from_u128(0x93a68750_951a_11d1_946f_000000000000);
pub const CLOSEPROPS_DISCARD: u32 = 1u32;
pub const CLOSEPROPS_NONE: u32 = 0u32;
pub const CLSID_ACLCustomMRU: windows_core::GUID = windows_core::GUID::from_u128(0x6935db93_21e8_4ccc_beb9_9fe3c77a297a);
pub const CLSID_ACLHistory: windows_core::GUID = windows_core::GUID::from_u128(0x00bb2764_6a77_11d0_a535_00c04fd7d062);
pub const CLSID_ACLMRU: windows_core::GUID = windows_core::GUID::from_u128(0x6756a641_de71_11d0_831b_00aa005b4383);
pub const CLSID_ACLMulti: windows_core::GUID = windows_core::GUID::from_u128(0x00bb2765_6a77_11d0_a535_00c04fd7d062);
pub const CLSID_ACListISF: windows_core::GUID = windows_core::GUID::from_u128(0x03c036f1_a186_11d0_824a_00aa005b4383);
pub const CLSID_ActiveDesktop: windows_core::GUID = windows_core::GUID::from_u128(0x75048700_ef1f_11d0_9888_006097deacf9);
pub const CLSID_AutoComplete: windows_core::GUID = windows_core::GUID::from_u128(0x00bb2763_6a77_11d0_a535_00c04fd7d062);
pub const CLSID_CAnchorBrowsePropertyPage: windows_core::GUID = windows_core::GUID::from_u128(0x3050f3bb_98b5_11cf_bb82_00aa00bdce0b);
pub const CLSID_CDocBrowsePropertyPage: windows_core::GUID = windows_core::GUID::from_u128(0x3050f3b4_98b5_11cf_bb82_00aa00bdce0b);
pub const CLSID_CFSIconOverlayManager: windows_core::GUID = windows_core::GUID::from_u128(0x63b51f81_c868_11d0_999c_00c04fd655e1);
pub const CLSID_CImageBrowsePropertyPage: windows_core::GUID = windows_core::GUID::from_u128(0x3050f3b3_98b5_11cf_bb82_00aa00bdce0b);
pub const CLSID_CURLSearchHook: windows_core::GUID = windows_core::GUID::from_u128(0xcfbfae00_17a6_11d0_99cb_00c04fd64497);
pub const CLSID_CUrlHistory: windows_core::GUID = windows_core::GUID::from_u128(0x3c374a40_bae4_11cf_bf7d_00aa006946ee);
pub const CLSID_CUrlHistoryBoth: windows_core::GUID = windows_core::GUID::from_u128(0x6659983c_8476_4eb4_b78c_e5968f326ba0);
pub const CLSID_ControlPanel: windows_core::GUID = windows_core::GUID::from_u128(0x21ec2020_3aea_1069_a2dd_08002b30309d);
pub const CLSID_DarwinAppPublisher: windows_core::GUID = windows_core::GUID::from_u128(0xcfccc7a0_a282_11d1_9082_006008059382);
pub const CLSID_DocHostUIHandler: windows_core::GUID = windows_core::GUID::from_u128(0x7057e952_bd1b_11d1_8919_00c04fc2c836);
pub const CLSID_DragDropHelper: windows_core::GUID = windows_core::GUID::from_u128(0x4657278a_411b_11d2_839a_00c04fd918d0);
pub const CLSID_FileTypes: windows_core::GUID = windows_core::GUID::from_u128(0xb091e540_83e3_11cf_a713_0020afd79762);
pub const CLSID_FolderItemsMultiLevel: windows_core::GUID = windows_core::GUID::from_u128(0x53c74826_ab99_4d33_aca4_3117f51d3788);
pub const CLSID_FolderShortcut: windows_core::GUID = windows_core::GUID::from_u128(0x0afaced1_e828_11d1_9187_b532f1e9575d);
pub const CLSID_HWShellExecute: windows_core::GUID = windows_core::GUID::from_u128(0xffb8655f_81b9_4fce_b89c_9a6ba76d13e7);
pub const CLSID_ISFBand: windows_core::GUID = windows_core::GUID::from_u128(0xd82be2b0_5764_11d0_a96e_00c04fd705a2);
pub const CLSID_Internet: windows_core::GUID = windows_core::GUID::from_u128(0x871c5380_42a0_1069_a2ea_08002b30309d);
pub const CLSID_InternetButtons: windows_core::GUID = windows_core::GUID::from_u128(0x1e796980_9cc5_11d1_a83f_00c04fc99d61);
pub const CLSID_InternetShortcut: windows_core::GUID = windows_core::GUID::from_u128(0xfbf23b40_e3f0_101b_8488_00aa003e56f8);
pub const CLSID_LinkColumnProvider: windows_core::GUID = windows_core::GUID::from_u128(0x24f14f02_7b1c_11d1_838f_0000f80461cf);
pub const CLSID_MSOButtons: windows_core::GUID = windows_core::GUID::from_u128(0x178f34b8_a282_11d2_86c5_00c04f8eea99);
pub const CLSID_MenuBand: windows_core::GUID = windows_core::GUID::from_u128(0x5b4dae26_b807_11d0_9815_00c04fd91972);
pub const CLSID_MenuBandSite: windows_core::GUID = windows_core::GUID::from_u128(0xe13ef4e4_d2f2_11d0_9816_00c04fd91972);
pub const CLSID_MenuToolbarBase: windows_core::GUID = windows_core::GUID::from_u128(0x40b96610_b522_11d1_b3b4_00aa006efde7);
pub const CLSID_MyComputer: windows_core::GUID = windows_core::GUID::from_u128(0x20d04fe0_3aea_1069_a2d8_08002b30309d);
pub const CLSID_MyDocuments: windows_core::GUID = windows_core::GUID::from_u128(0x450d8fba_ad25_11d0_98a8_0800361b1103);
pub const CLSID_NetworkDomain: windows_core::GUID = windows_core::GUID::from_u128(0x46e06680_4bf0_11d1_83ee_00a0c90dc849);
pub const CLSID_NetworkServer: windows_core::GUID = windows_core::GUID::from_u128(0xc0542a90_4bf0_11d1_83ee_00a0c90dc849);
pub const CLSID_NetworkShare: windows_core::GUID = windows_core::GUID::from_u128(0x54a754c0_4bf0_11d1_83ee_00a0c90dc849);
pub const CLSID_NewMenu: windows_core::GUID = windows_core::GUID::from_u128(0xd969a300_e7ff_11d0_a93b_00a0c90f2719);
pub const CLSID_Printers: windows_core::GUID = windows_core::GUID::from_u128(0x2227a280_3aea_1069_a2de_08002b30309d);
pub const CLSID_ProgressDialog: windows_core::GUID = windows_core::GUID::from_u128(0xf8383852_fcd3_11d1_a6b9_006097df5bd4);
pub const CLSID_QueryAssociations: windows_core::GUID = windows_core::GUID::from_u128(0xa07034fd_6caa_4954_ac3f_97a27216f98a);
pub const CLSID_QuickLinks: windows_core::GUID = windows_core::GUID::from_u128(0x0e5cbf21_d15f_11d0_8301_00aa005b4383);
pub const CLSID_RecycleBin: windows_core::GUID = windows_core::GUID::from_u128(0x645ff040_5081_101b_9f08_00aa002f954e);
pub const CLSID_ShellFldSetExt: windows_core::GUID = windows_core::GUID::from_u128(0x6d5313c0_8c62_11d1_b2cd_006097df8c11);
pub const CLSID_ShellThumbnailDiskCache: windows_core::GUID = windows_core::GUID::from_u128(0x1ebdcf80_a200_11d0_a3a4_00c04fd706ec);
pub const CLSID_ToolbarExtButtons: windows_core::GUID = windows_core::GUID::from_u128(0x2ce4b5d8_a28f_11d2_86c5_00c04f8eea99);
pub const CMDID_INTSHORTCUTCREATE: i32 = 1i32;
pub const CMDSTR_NEWFOLDER: windows_core::PCWSTR = windows_core::w!("NewFolder");
pub const CMDSTR_NEWFOLDERA: windows_core::PCSTR = windows_core::s!("NewFolder");
pub const CMDSTR_NEWFOLDERW: windows_core::PCWSTR = windows_core::w!("NewFolder");
pub const CMDSTR_VIEWDETAILS: windows_core::PCWSTR = windows_core::w!("ViewDetails");
pub const CMDSTR_VIEWDETAILSA: windows_core::PCSTR = windows_core::s!("ViewDetails");
pub const CMDSTR_VIEWDETAILSW: windows_core::PCWSTR = windows_core::w!("ViewDetails");
pub const CMDSTR_VIEWLIST: windows_core::PCWSTR = windows_core::w!("ViewList");
pub const CMDSTR_VIEWLISTA: windows_core::PCSTR = windows_core::s!("ViewList");
pub const CMDSTR_VIEWLISTW: windows_core::PCWSTR = windows_core::w!("ViewList");
pub const CMF_ASYNCVERBSTATE: u32 = 1024u32;
pub const CMF_CANRENAME: u32 = 16u32;
pub const CMF_DEFAULTONLY: u32 = 1u32;
pub const CMF_DISABLEDVERBS: u32 = 512u32;
pub const CMF_DONOTPICKDEFAULT: u32 = 8192u32;
pub const CMF_EXPLORE: u32 = 4u32;
pub const CMF_EXTENDEDVERBS: u32 = 256u32;
pub const CMF_INCLUDESTATIC: u32 = 64u32;
pub const CMF_ITEMMENU: u32 = 128u32;
pub const CMF_NODEFAULT: u32 = 32u32;
pub const CMF_NORMAL: u32 = 0u32;
pub const CMF_NOVERBS: u32 = 8u32;
pub const CMF_OPTIMIZEFORINVOKE: u32 = 2048u32;
pub const CMF_RESERVED: u32 = 4294901760u32;
pub const CMF_SYNCCASCADEMENU: u32 = 4096u32;
pub const CMF_VERBSONLY: u32 = 2u32;
pub const CMIC_MASK_CONTROL_DOWN: u32 = 1073741824u32;
pub const CMIC_MASK_PTINVOKE: u32 = 536870912u32;
pub const CMIC_MASK_SHIFT_DOWN: u32 = 268435456u32;
pub const CM_ENUM_ALL: CM_ENUM_FLAGS = 1i32;
pub const CM_ENUM_VISIBLE: CM_ENUM_FLAGS = 2i32;
pub const CM_MASK_DEFAULTWIDTH: CM_MASK = 2i32;
pub const CM_MASK_IDEALWIDTH: CM_MASK = 4i32;
pub const CM_MASK_NAME: CM_MASK = 8i32;
pub const CM_MASK_STATE: CM_MASK = 16i32;
pub const CM_MASK_WIDTH: CM_MASK = 1i32;
pub const CM_STATE_ALWAYSVISIBLE: CM_STATE = 8i32;
pub const CM_STATE_FIXEDWIDTH: CM_STATE = 2i32;
pub const CM_STATE_NONE: CM_STATE = 0i32;
pub const CM_STATE_NOSORTBYFOLDERNESS: CM_STATE = 4i32;
pub const CM_STATE_VISIBLE: CM_STATE = 1i32;
pub const CM_WIDTH_AUTOSIZE: CM_SET_WIDTH_VALUE = -2i32;
pub const CM_WIDTH_USEDEFAULT: CM_SET_WIDTH_VALUE = -1i32;
pub const COMPONENT_DEFAULT_LEFT: u32 = 65535u32;
pub const COMPONENT_DEFAULT_TOP: u32 = 65535u32;
pub const COMPONENT_TOP: u32 = 1073741823u32;
pub const COMP_ELEM_CHECKED: u32 = 2u32;
pub const COMP_ELEM_CURITEMSTATE: u32 = 16384u32;
pub const COMP_ELEM_DIRTY: u32 = 4u32;
pub const COMP_ELEM_FRIENDLYNAME: u32 = 1024u32;
pub const COMP_ELEM_NOSCROLL: u32 = 8u32;
pub const COMP_ELEM_ORIGINAL_CSI: u32 = 4096u32;
pub const COMP_ELEM_POS_LEFT: u32 = 16u32;
pub const COMP_ELEM_POS_TOP: u32 = 32u32;
pub const COMP_ELEM_POS_ZINDEX: u32 = 256u32;
pub const COMP_ELEM_RESTORED_CSI: u32 = 8192u32;
pub const COMP_ELEM_SIZE_HEIGHT: u32 = 128u32;
pub const COMP_ELEM_SIZE_WIDTH: u32 = 64u32;
pub const COMP_ELEM_SOURCE: u32 = 512u32;
pub const COMP_ELEM_SUBSCRIBEDURL: u32 = 2048u32;
pub const COMP_ELEM_TYPE: u32 = 1u32;
pub const COMP_TYPE_CFHTML: u32 = 4u32;
pub const COMP_TYPE_CONTROL: u32 = 3u32;
pub const COMP_TYPE_HTMLDOC: u32 = 0u32;
pub const COMP_TYPE_MAX: u32 = 4u32;
pub const COMP_TYPE_PICTURE: u32 = 1u32;
pub const COMP_TYPE_WEBSITE: u32 = 2u32;
pub const CONFLICT_RESOLUTION_CLSID_KEY: windows_core::PCWSTR = windows_core::w!("ConflictResolutionCLSID");
pub const COPYENGINE_E_ACCESSDENIED_READONLY: windows_core::HRESULT = 0x8027003F_u32 as _;
pub const COPYENGINE_E_ACCESS_DENIED_DEST: windows_core::HRESULT = 0x80270022_u32 as _;
pub const COPYENGINE_E_ACCESS_DENIED_SRC: windows_core::HRESULT = 0x80270021_u32 as _;
pub const COPYENGINE_E_ALREADY_EXISTS_FOLDER: windows_core::HRESULT = 0x8027002C_u32 as _;
pub const COPYENGINE_E_ALREADY_EXISTS_NORMAL: windows_core::HRESULT = 0x80270029_u32 as _;
pub const COPYENGINE_E_ALREADY_EXISTS_READONLY: windows_core::HRESULT = 0x8027002A_u32 as _;
pub const COPYENGINE_E_ALREADY_EXISTS_SYSTEM: windows_core::HRESULT = 0x8027002B_u32 as _;
pub const COPYENGINE_E_BLOCKED_BY_DLP_POLICY: windows_core::HRESULT = 0x8027004E_u32 as _;
pub const COPYENGINE_E_BLOCKED_BY_EDP_FOR_REMOVABLE_DRIVE: windows_core::HRESULT = 0x8027004A_u32 as _;
pub const COPYENGINE_E_BLOCKED_BY_EDP_POLICY: windows_core::HRESULT = 0x80270048_u32 as _;
pub const COPYENGINE_E_CANCELLED: windows_core::HRESULT = 0x80270001_u32 as _;
pub const COPYENGINE_E_CANNOT_MOVE_FROM_RECYCLE_BIN: windows_core::HRESULT = 0x80270043_u32 as _;
pub const COPYENGINE_E_CANNOT_MOVE_SHARED_FOLDER: windows_core::HRESULT = 0x80270044_u32 as _;
pub const COPYENGINE_E_CANT_REACH_SOURCE: windows_core::HRESULT = 0x80270035_u32 as _;
pub const COPYENGINE_E_DEST_IS_RO_CD: windows_core::HRESULT = 0x8027000F_u32 as _;
pub const COPYENGINE_E_DEST_IS_RO_DVD: windows_core::HRESULT = 0x80270012_u32 as _;
pub const COPYENGINE_E_DEST_IS_RW_CD: windows_core::HRESULT = 0x80270010_u32 as _;
pub const COPYENGINE_E_DEST_IS_RW_DVD: windows_core::HRESULT = 0x80270013_u32 as _;
pub const COPYENGINE_E_DEST_IS_R_CD: windows_core::HRESULT = 0x80270011_u32 as _;
pub const COPYENGINE_E_DEST_IS_R_DVD: windows_core::HRESULT = 0x80270014_u32 as _;
pub const COPYENGINE_E_DEST_SAME_TREE: windows_core::HRESULT = 0x8027000A_u32 as _;
pub const COPYENGINE_E_DEST_SUBTREE: windows_core::HRESULT = 0x80270009_u32 as _;
pub const COPYENGINE_E_DIFF_DIR: windows_core::HRESULT = 0x80270004_u32 as _;
pub const COPYENGINE_E_DIR_NOT_EMPTY: windows_core::HRESULT = 0x8027003D_u32 as _;
pub const COPYENGINE_E_DISK_FULL: windows_core::HRESULT = 0x80270032_u32 as _;
pub const COPYENGINE_E_DISK_FULL_CLEAN: windows_core::HRESULT = 0x80270033_u32 as _;
pub const COPYENGINE_E_EA_LOSS: windows_core::HRESULT = 0x8027002E_u32 as _;
pub const COPYENGINE_E_EA_NOT_SUPPORTED: windows_core::HRESULT = 0x80270034_u32 as _;
pub const COPYENGINE_E_ENCRYPTION_LOSS: windows_core::HRESULT = 0x80270031_u32 as _;
pub const COPYENGINE_E_FAT_MAX_IN_ROOT: windows_core::HRESULT = 0x8027003E_u32 as _;
pub const COPYENGINE_E_FILE_IS_FLD_DEST: windows_core::HRESULT = 0x8027000C_u32 as _;
pub const COPYENGINE_E_FILE_TOO_LARGE: windows_core::HRESULT = 0x8027000D_u32 as _;
pub const COPYENGINE_E_FLD_IS_FILE_DEST: windows_core::HRESULT = 0x8027000B_u32 as _;
pub const COPYENGINE_E_INTERNET_ITEM_STORAGE_PROVIDER_ERROR: windows_core::HRESULT = 0x80270045_u32 as _;
pub const COPYENGINE_E_INTERNET_ITEM_STORAGE_PROVIDER_PAUSED: windows_core::HRESULT = 0x80270046_u32 as _;
pub const COPYENGINE_E_INTERNET_ITEM_UNAVAILABLE: windows_core::HRESULT = 0x80270042_u32 as _;
pub const COPYENGINE_E_INVALID_FILES_DEST: windows_core::HRESULT = 0x8027001C_u32 as _;
pub const COPYENGINE_E_INVALID_FILES_SRC: windows_core::HRESULT = 0x8027001B_u32 as _;
pub const COPYENGINE_E_MANY_SRC_1_DEST: windows_core::HRESULT = 0x80270005_u32 as _;
pub const COPYENGINE_E_NET_DISCONNECT_DEST: windows_core::HRESULT = 0x80270026_u32 as _;
pub const COPYENGINE_E_NET_DISCONNECT_SRC: windows_core::HRESULT = 0x80270025_u32 as _;
pub const COPYENGINE_E_NEWFILE_NAME_TOO_LONG: windows_core::HRESULT = 0x8027003B_u32 as _;
pub const COPYENGINE_E_NEWFOLDER_NAME_TOO_LONG: windows_core::HRESULT = 0x8027003C_u32 as _;
pub const COPYENGINE_E_PATH_NOT_FOUND_DEST: windows_core::HRESULT = 0x80270024_u32 as _;
pub const COPYENGINE_E_PATH_NOT_FOUND_SRC: windows_core::HRESULT = 0x80270023_u32 as _;
pub const COPYENGINE_E_PATH_TOO_DEEP_DEST: windows_core::HRESULT = 0x8027001E_u32 as _;
pub const COPYENGINE_E_PATH_TOO_DEEP_SRC: windows_core::HRESULT = 0x8027001D_u32 as _;
pub const COPYENGINE_E_PROPERTIES_LOSS: windows_core::HRESULT = 0x80270030_u32 as _;
pub const COPYENGINE_E_PROPERTY_LOSS: windows_core::HRESULT = 0x8027002F_u32 as _;
pub const COPYENGINE_E_RECYCLE_BIN_NOT_FOUND: windows_core::HRESULT = 0x8027003A_u32 as _;
pub const COPYENGINE_E_RECYCLE_FORCE_NUKE: windows_core::HRESULT = 0x80270036_u32 as _;
pub const COPYENGINE_E_RECYCLE_PATH_TOO_LONG: windows_core::HRESULT = 0x80270038_u32 as _;
pub const COPYENGINE_E_RECYCLE_SIZE_TOO_BIG: windows_core::HRESULT = 0x80270037_u32 as _;
pub const COPYENGINE_E_RECYCLE_UNKNOWN_ERROR: windows_core::HRESULT = 0x80270035_u32 as _;
pub const COPYENGINE_E_REDIRECTED_TO_WEBPAGE: windows_core::HRESULT = 0x80270040_u32 as _;
pub const COPYENGINE_E_REMOVABLE_FULL: windows_core::HRESULT = 0x8027000E_u32 as _;
pub const COPYENGINE_E_REQUIRES_EDP_CONSENT: windows_core::HRESULT = 0x80270047_u32 as _;
pub const COPYENGINE_E_REQUIRES_EDP_CONSENT_FOR_REMOVABLE_DRIVE: windows_core::HRESULT = 0x80270049_u32 as _;
pub const COPYENGINE_E_REQUIRES_ELEVATION: windows_core::HRESULT = 0x80270002_u32 as _;
pub const COPYENGINE_E_RMS_BLOCKED_BY_EDP_FOR_REMOVABLE_DRIVE: windows_core::HRESULT = 0x8027004C_u32 as _;
pub const COPYENGINE_E_RMS_REQUIRES_EDP_CONSENT_FOR_REMOVABLE_DRIVE: windows_core::HRESULT = 0x8027004B_u32 as _;
pub const COPYENGINE_E_ROOT_DIR_DEST: windows_core::HRESULT = 0x80270020_u32 as _;
pub const COPYENGINE_E_ROOT_DIR_SRC: windows_core::HRESULT = 0x8027001F_u32 as _;
pub const COPYENGINE_E_SAME_FILE: windows_core::HRESULT = 0x80270003_u32 as _;
pub const COPYENGINE_E_SERVER_BAD_FILE_TYPE: windows_core::HRESULT = 0x80270041_u32 as _;
pub const COPYENGINE_E_SHARING_VIOLATION_DEST: windows_core::HRESULT = 0x80270028_u32 as _;
pub const COPYENGINE_E_SHARING_VIOLATION_SRC: windows_core::HRESULT = 0x80270027_u32 as _;
pub const COPYENGINE_E_SILENT_FAIL_BY_DLP_POLICY: windows_core::HRESULT = 0x8027004F_u32 as _;
pub const COPYENGINE_E_SRC_IS_RO_CD: windows_core::HRESULT = 0x80270015_u32 as _;
pub const COPYENGINE_E_SRC_IS_RO_DVD: windows_core::HRESULT = 0x80270018_u32 as _;
pub const COPYENGINE_E_SRC_IS_RW_CD: windows_core::HRESULT = 0x80270016_u32 as _;
pub const COPYENGINE_E_SRC_IS_RW_DVD: windows_core::HRESULT = 0x80270019_u32 as _;
pub const COPYENGINE_E_SRC_IS_R_CD: windows_core::HRESULT = 0x80270017_u32 as _;
pub const COPYENGINE_E_SRC_IS_R_DVD: windows_core::HRESULT = 0x8027001A_u32 as _;
pub const COPYENGINE_E_STREAM_LOSS: windows_core::HRESULT = 0x8027002D_u32 as _;
pub const COPYENGINE_E_USER_CANCELLED: windows_core::HRESULT = 0x80270000_u32 as _;
pub const COPYENGINE_E_WARNED_BY_DLP_POLICY: windows_core::HRESULT = 0x8027004D_u32 as _;
pub const COPYENGINE_S_ALREADY_DONE: windows_core::HRESULT = 0x27000A_u32 as _;
pub const COPYENGINE_S_CLOSE_PROGRAM: windows_core::HRESULT = 0x27000D_u32 as _;
pub const COPYENGINE_S_COLLISIONRESOLVED: windows_core::HRESULT = 0x27000E_u32 as _;
pub const COPYENGINE_S_DONT_PROCESS_CHILDREN: windows_core::HRESULT = 0x270008_u32 as _;
pub const COPYENGINE_S_KEEP_BOTH: windows_core::HRESULT = 0x27000C_u32 as _;
pub const COPYENGINE_S_MERGE: windows_core::HRESULT = 0x270006_u32 as _;
pub const COPYENGINE_S_NOT_HANDLED: windows_core::HRESULT = 0x270003_u32 as _;
pub const COPYENGINE_S_PENDING: windows_core::HRESULT = 0x27000B_u32 as _;
pub const COPYENGINE_S_PENDING_DELETE: windows_core::HRESULT = 0x270010_u32 as _;
pub const COPYENGINE_S_PROGRESS_PAUSE: windows_core::HRESULT = 0x27000F_u32 as _;
pub const COPYENGINE_S_USER_IGNORED: windows_core::HRESULT = 0x270005_u32 as _;
pub const COPYENGINE_S_USER_RETRY: windows_core::HRESULT = 0x270004_u32 as _;
pub const COPYENGINE_S_YES: windows_core::HRESULT = 0x270001_u32 as _;
pub const CPAO_EMPTY_CONNECTED: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 2i32;
pub const CPAO_EMPTY_LOCAL: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 1i32;
pub const CPAO_NONE: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 0i32;
pub const CPCFO_ENABLE_PASSWORD_REVEAL: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 1i32;
pub const CPCFO_ENABLE_TOUCH_KEYBOARD_AUTO_INVOKE: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 4i32;
pub const CPCFO_IS_EMAIL_ADDRESS: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 2i32;
pub const CPCFO_NONE: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 0i32;
pub const CPCFO_NUMBERS_ONLY: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 8i32;
pub const CPCFO_SHOW_ENGLISH_KEYBOARD: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 16i32;
pub const CPFG_CREDENTIAL_PROVIDER_LABEL: windows_core::GUID = windows_core::GUID::from_u128(0x286bbff3_bad4_438f_b007_79b7267c3d48);
pub const CPFG_CREDENTIAL_PROVIDER_LOGO: windows_core::GUID = windows_core::GUID::from_u128(0x2d837775_f6cd_464e_a745_482fd0b47493);
pub const CPFG_LOGON_PASSWORD: windows_core::GUID = windows_core::GUID::from_u128(0x60624cfa_a477_47b1_8a8e_3a4a19981827);
pub const CPFG_LOGON_USERNAME: windows_core::GUID = windows_core::GUID::from_u128(0xda15bbe8_954d_4fd3_b0f4_1fb5b90b174b);
pub const CPFG_SMARTCARD_PIN: windows_core::GUID = windows_core::GUID::from_u128(0x4fe5263b_9181_46c1_b0a4_9dedd4db7dea);
pub const CPFG_SMARTCARD_USERNAME: windows_core::GUID = windows_core::GUID::from_u128(0x3e1ecf69_568c_4d96_9d59_46444174e2d6);
pub const CPFG_STANDALONE_SUBMIT_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x0b7b0ad8_cc36_4d59_802b_82f714fa7022);
pub const CPFG_STYLE_LINK_AS_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x088fa508_94a6_4430_a4cb_6fc6e3c0b9e2);
pub const CPFIS_DISABLED: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 2i32;
pub const CPFIS_FOCUSED: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 3i32;
pub const CPFIS_NONE: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 0i32;
pub const CPFIS_READONLY: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 1i32;
pub const CPFS_DISPLAY_IN_BOTH: CREDENTIAL_PROVIDER_FIELD_STATE = 3i32;
pub const CPFS_DISPLAY_IN_DESELECTED_TILE: CREDENTIAL_PROVIDER_FIELD_STATE = 2i32;
pub const CPFS_DISPLAY_IN_SELECTED_TILE: CREDENTIAL_PROVIDER_FIELD_STATE = 1i32;
pub const CPFS_HIDDEN: CREDENTIAL_PROVIDER_FIELD_STATE = 0i32;
pub const CPFT_CHECKBOX: CREDENTIAL_PROVIDER_FIELD_TYPE = 7i32;
pub const CPFT_COMBOBOX: CREDENTIAL_PROVIDER_FIELD_TYPE = 8i32;
pub const CPFT_COMMAND_LINK: CREDENTIAL_PROVIDER_FIELD_TYPE = 3i32;
pub const CPFT_EDIT_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 4i32;
pub const CPFT_INVALID: CREDENTIAL_PROVIDER_FIELD_TYPE = 0i32;
pub const CPFT_LARGE_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 1i32;
pub const CPFT_PASSWORD_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 5i32;
pub const CPFT_SMALL_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 2i32;
pub const CPFT_SUBMIT_BUTTON: CREDENTIAL_PROVIDER_FIELD_TYPE = 9i32;
pub const CPFT_TILE_IMAGE: CREDENTIAL_PROVIDER_FIELD_TYPE = 6i32;
pub const CPGSR_NO_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 1i32;
pub const CPGSR_NO_CREDENTIAL_NOT_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 0i32;
pub const CPGSR_RETURN_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 2i32;
pub const CPGSR_RETURN_NO_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 3i32;
pub const CPLPAGE_DISPLAY_BACKGROUND: u32 = 1u32;
pub const CPLPAGE_KEYBOARD_SPEED: u32 = 1u32;
pub const CPLPAGE_MOUSE_BUTTONS: u32 = 1u32;
pub const CPLPAGE_MOUSE_PTRMOTION: u32 = 2u32;
pub const CPLPAGE_MOUSE_WHEEL: u32 = 3u32;
pub const CPL_DBLCLK: u32 = 5u32;
pub const CPL_DYNAMIC_RES: u32 = 0u32;
pub const CPL_EXIT: u32 = 7u32;
pub const CPL_GETCOUNT: u32 = 2u32;
pub const CPL_INIT: u32 = 1u32;
pub const CPL_INQUIRE: u32 = 3u32;
pub const CPL_NEWINQUIRE: u32 = 8u32;
pub const CPL_SELECT: u32 = 4u32;
pub const CPL_SETUP: u32 = 200u32;
pub const CPL_STARTWPARMS: u32 = 10u32;
pub const CPL_STARTWPARMSA: u32 = 9u32;
pub const CPL_STARTWPARMSW: u32 = 10u32;
pub const CPL_STOP: u32 = 6u32;
pub const CPSI_ERROR: CREDENTIAL_PROVIDER_STATUS_ICON = 1i32;
pub const CPSI_NONE: CREDENTIAL_PROVIDER_STATUS_ICON = 0i32;
pub const CPSI_SUCCESS: CREDENTIAL_PROVIDER_STATUS_ICON = 3i32;
pub const CPSI_WARNING: CREDENTIAL_PROVIDER_STATUS_ICON = 2i32;
pub const CPUS_CHANGE_PASSWORD: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 3i32;
pub const CPUS_CREDUI: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 4i32;
pub const CPUS_INVALID: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 0i32;
pub const CPUS_LOGON: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 1i32;
pub const CPUS_PLAP: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 5i32;
pub const CPUS_UNLOCK_WORKSTATION: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 2i32;
pub const CPVIEW_ALLITEMS: CPVIEW = 0i32;
pub const CPVIEW_CATEGORY: CPVIEW = 1i32;
pub const CPVIEW_CLASSIC: CPVIEW = 0i32;
pub const CPVIEW_HOME: CPVIEW = 1i32;
pub const CREDENTIAL_PROVIDER_NO_DEFAULT: u32 = 4294967295u32;
pub const CSC_NAVIGATEBACK: CommandStateChangeConstants = 2i32;
pub const CSC_NAVIGATEFORWARD: CommandStateChangeConstants = 1i32;
pub const CSC_UPDATECOMMANDS: CommandStateChangeConstants = -1i32;
pub const CSIDL_ADMINTOOLS: u32 = 48u32;
pub const CSIDL_ALTSTARTUP: u32 = 29u32;
pub const CSIDL_APPDATA: u32 = 26u32;
pub const CSIDL_BITBUCKET: u32 = 10u32;
pub const CSIDL_CDBURN_AREA: u32 = 59u32;
pub const CSIDL_COMMON_ADMINTOOLS: u32 = 47u32;
pub const CSIDL_COMMON_ALTSTARTUP: u32 = 30u32;
pub const CSIDL_COMMON_APPDATA: u32 = 35u32;
pub const CSIDL_COMMON_DESKTOPDIRECTORY: u32 = 25u32;
pub const CSIDL_COMMON_DOCUMENTS: u32 = 46u32;
pub const CSIDL_COMMON_FAVORITES: u32 = 31u32;
pub const CSIDL_COMMON_MUSIC: u32 = 53u32;
pub const CSIDL_COMMON_OEM_LINKS: u32 = 58u32;
pub const CSIDL_COMMON_PICTURES: u32 = 54u32;
pub const CSIDL_COMMON_PROGRAMS: u32 = 23u32;
pub const CSIDL_COMMON_STARTMENU: u32 = 22u32;
pub const CSIDL_COMMON_STARTUP: u32 = 24u32;
pub const CSIDL_COMMON_TEMPLATES: u32 = 45u32;
pub const CSIDL_COMMON_VIDEO: u32 = 55u32;
pub const CSIDL_COMPUTERSNEARME: u32 = 61u32;
pub const CSIDL_CONNECTIONS: u32 = 49u32;
pub const CSIDL_CONTROLS: u32 = 3u32;
pub const CSIDL_COOKIES: u32 = 33u32;
pub const CSIDL_DESKTOP: u32 = 0u32;
pub const CSIDL_DESKTOPDIRECTORY: u32 = 16u32;
pub const CSIDL_DRIVES: u32 = 17u32;
pub const CSIDL_FAVORITES: u32 = 6u32;
pub const CSIDL_FLAG_CREATE: u32 = 32768u32;
pub const CSIDL_FLAG_DONT_UNEXPAND: u32 = 8192u32;
pub const CSIDL_FLAG_DONT_VERIFY: u32 = 16384u32;
pub const CSIDL_FLAG_MASK: u32 = 65280u32;
pub const CSIDL_FLAG_NO_ALIAS: u32 = 4096u32;
pub const CSIDL_FLAG_PER_USER_INIT: u32 = 2048u32;
pub const CSIDL_FLAG_PFTI_TRACKTARGET: u32 = 16384u32;
pub const CSIDL_FONTS: u32 = 20u32;
pub const CSIDL_HISTORY: u32 = 34u32;
pub const CSIDL_INTERNET: u32 = 1u32;
pub const CSIDL_INTERNET_CACHE: u32 = 32u32;
pub const CSIDL_LOCAL_APPDATA: u32 = 28u32;
pub const CSIDL_MYDOCUMENTS: u32 = 5u32;
pub const CSIDL_MYMUSIC: u32 = 13u32;
pub const CSIDL_MYPICTURES: u32 = 39u32;
pub const CSIDL_MYVIDEO: u32 = 14u32;
pub const CSIDL_NETHOOD: u32 = 19u32;
pub const CSIDL_NETWORK: u32 = 18u32;
pub const CSIDL_PERSONAL: u32 = 5u32;
pub const CSIDL_PRINTERS: u32 = 4u32;
pub const CSIDL_PRINTHOOD: u32 = 27u32;
pub const CSIDL_PROFILE: u32 = 40u32;
pub const CSIDL_PROGRAMS: u32 = 2u32;
pub const CSIDL_PROGRAM_FILES: u32 = 38u32;
pub const CSIDL_PROGRAM_FILESX86: u32 = 42u32;
pub const CSIDL_PROGRAM_FILES_COMMON: u32 = 43u32;
pub const CSIDL_PROGRAM_FILES_COMMONX86: u32 = 44u32;
pub const CSIDL_RECENT: u32 = 8u32;
pub const CSIDL_RESOURCES: u32 = 56u32;
pub const CSIDL_RESOURCES_LOCALIZED: u32 = 57u32;
pub const CSIDL_SENDTO: u32 = 9u32;
pub const CSIDL_STARTMENU: u32 = 11u32;
pub const CSIDL_STARTUP: u32 = 7u32;
pub const CSIDL_SYSTEM: u32 = 37u32;
pub const CSIDL_SYSTEMX86: u32 = 41u32;
pub const CSIDL_TEMPLATES: u32 = 21u32;
pub const CSIDL_WINDOWS: u32 = 36u32;
pub const CTF_COINIT: i32 = 8i32;
pub const CTF_COINIT_MTA: i32 = 4096i32;
pub const CTF_COINIT_STA: i32 = 8i32;
pub const CTF_FREELIBANDEXIT: i32 = 16i32;
pub const CTF_INHERITWOW64: i32 = 256i32;
pub const CTF_INSIST: i32 = 1i32;
pub const CTF_KEYBOARD_LOCALE: i32 = 1024i32;
pub const CTF_NOADDREFLIB: i32 = 8192i32;
pub const CTF_OLEINITIALIZE: i32 = 2048i32;
pub const CTF_PROCESS_REF: i32 = 4i32;
pub const CTF_REF_COUNTED: i32 = 32i32;
pub const CTF_THREAD_REF: i32 = 2i32;
pub const CTF_UNUSED: i32 = 128i32;
pub const CTF_WAIT_ALLOWCOM: i32 = 64i32;
pub const CTF_WAIT_NO_REENTRANCY: i32 = 512i32;
pub const DBCID_CLSIDOFBAR: i32 = 2i32;
pub const DBCID_EMPTY: i32 = 0i32;
pub const DBCID_GETBAR: i32 = 4i32;
pub const DBCID_ONDRAG: i32 = 1i32;
pub const DBCID_RESIZE: i32 = 3i32;
pub const DBCID_UPDATESIZE: i32 = 5i32;
pub const DBC_GS_IDEAL: u32 = 0u32;
pub const DBC_GS_SIZEDOWN: u32 = 1u32;
pub const DBC_HIDE: u32 = 0u32;
pub const DBC_SHOW: u32 = 1u32;
pub const DBC_SHOWOBSCURE: u32 = 2u32;
pub const DBID_BANDINFOCHANGED: DESKBANDCID = 0i32;
pub const DBID_DELAYINIT: DESKBANDCID = 4i32;
pub const DBID_FINISHINIT: DESKBANDCID = 5i32;
pub const DBID_MAXIMIZEBAND: DESKBANDCID = 2i32;
pub const DBID_PERMITAUTOHIDE: DESKBANDCID = 7i32;
pub const DBID_PUSHCHEVRON: DESKBANDCID = 3i32;
pub const DBID_SETWINDOWTHEME: DESKBANDCID = 6i32;
pub const DBID_SHOWONLY: DESKBANDCID = 1i32;
pub const DBIF_VIEWMODE_FLOATING: u32 = 2u32;
pub const DBIF_VIEWMODE_NORMAL: u32 = 0u32;
pub const DBIF_VIEWMODE_TRANSPARENT: u32 = 4u32;
pub const DBIF_VIEWMODE_VERTICAL: u32 = 1u32;
pub const DBIMF_ADDTOFRONT: u32 = 512u32;
pub const DBIMF_ALWAYSGRIPPER: u32 = 4096u32;
pub const DBIMF_BKCOLOR: u32 = 64u32;
pub const DBIMF_BREAK: u32 = 256u32;
pub const DBIMF_DEBOSSED: u32 = 32u32;
pub const DBIMF_FIXED: u32 = 1u32;
pub const DBIMF_FIXEDBMP: u32 = 4u32;
pub const DBIMF_NOGRIPPER: u32 = 2048u32;
pub const DBIMF_NOMARGINS: u32 = 8192u32;
pub const DBIMF_NORMAL: u32 = 0u32;
pub const DBIMF_TOPALIGN: u32 = 1024u32;
pub const DBIMF_UNDELETEABLE: u32 = 16u32;
pub const DBIMF_USECHEVRON: u32 = 128u32;
pub const DBIMF_VARIABLEHEIGHT: u32 = 8u32;
pub const DBIM_ACTUAL: u32 = 8u32;
pub const DBIM_BKCOLOR: u32 = 64u32;
pub const DBIM_INTEGRAL: u32 = 4u32;
pub const DBIM_MAXSIZE: u32 = 2u32;
pub const DBIM_MINSIZE: u32 = 1u32;
pub const DBIM_MODEFLAGS: u32 = 32u32;
pub const DBIM_TITLE: u32 = 16u32;
pub const DBPC_SELECTFIRST: u32 = 4294967295u32;
pub const DEFSHAREID_PUBLIC: DEF_SHARE_ID = 2i32;
pub const DEFSHAREID_USERS: DEF_SHARE_ID = 1i32;
pub const DEVICE_IMMERSIVE: DISPLAY_DEVICE_TYPE = 1i32;
pub const DEVICE_PRIMARY: DISPLAY_DEVICE_TYPE = 0i32;
pub const DFMR_DEFAULT: DEFAULT_FOLDER_MENU_RESTRICTIONS = 0i32;
pub const DFMR_NO_ASYNC_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 1024i32;
pub const DFMR_NO_NATIVECPU_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 2048i32;
pub const DFMR_NO_NONWOW_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 4096i32;
pub const DFMR_NO_RESOURCE_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 32i32;
pub const DFMR_NO_STATIC_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 8i32;
pub const DFMR_OPTIN_HANDLERS_ONLY: DEFAULT_FOLDER_MENU_RESTRICTIONS = 64i32;
pub const DFMR_RESOURCE_AND_FOLDER_VERBS_ONLY: DEFAULT_FOLDER_MENU_RESTRICTIONS = 128i32;
pub const DFMR_STATIC_VERBS_ONLY: DEFAULT_FOLDER_MENU_RESTRICTIONS = 16i32;
pub const DFMR_USE_SPECIFIED_HANDLERS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 256i32;
pub const DFMR_USE_SPECIFIED_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 512i32;
pub const DFM_CMD_COPY: DFM_CMD = -3i32;
pub const DFM_CMD_DELETE: DFM_CMD = -1i32;
pub const DFM_CMD_LINK: DFM_CMD = -4i32;
pub const DFM_CMD_MODALPROP: DFM_CMD = -12i32;
pub const DFM_CMD_MOVE: DFM_CMD = -2i32;
pub const DFM_CMD_NEWFOLDER: DFM_CMD = -6i32;
pub const DFM_CMD_PASTE: DFM_CMD = -7i32;
pub const DFM_CMD_PASTELINK: DFM_CMD = -10i32;
pub const DFM_CMD_PASTESPECIAL: DFM_CMD = -11i32;
pub const DFM_CMD_PROPERTIES: DFM_CMD = -5i32;
pub const DFM_CMD_RENAME: DFM_CMD = -13i32;
pub const DFM_CMD_VIEWDETAILS: DFM_CMD = -9i32;
pub const DFM_CMD_VIEWLIST: DFM_CMD = -8i32;
pub const DFM_GETDEFSTATICID: DFM_MESSAGE_ID = 14i32;
pub const DFM_GETHELPTEXT: DFM_MESSAGE_ID = 5i32;
pub const DFM_GETHELPTEXTW: DFM_MESSAGE_ID = 11i32;
pub const DFM_GETVERBA: DFM_MESSAGE_ID = 16i32;
pub const DFM_GETVERBW: DFM_MESSAGE_ID = 15i32;
pub const DFM_INVOKECOMMAND: DFM_MESSAGE_ID = 2i32;
pub const DFM_INVOKECOMMANDEX: DFM_MESSAGE_ID = 12i32;
pub const DFM_MAPCOMMANDNAME: DFM_MESSAGE_ID = 13i32;
pub const DFM_MERGECONTEXTMENU: DFM_MESSAGE_ID = 1i32;
pub const DFM_MERGECONTEXTMENU_BOTTOM: DFM_MESSAGE_ID = 17i32;
pub const DFM_MERGECONTEXTMENU_TOP: DFM_MESSAGE_ID = 10i32;
pub const DFM_MODIFYQCMFLAGS: DFM_MESSAGE_ID = 18i32;
pub const DFM_VALIDATECMD: DFM_MESSAGE_ID = 9i32;
pub const DFM_WM_DRAWITEM: DFM_MESSAGE_ID = 7i32;
pub const DFM_WM_INITMENUPOPUP: DFM_MESSAGE_ID = 8i32;
pub const DFM_WM_MEASUREITEM: DFM_MESSAGE_ID = 6i32;
pub const DISPID_BEGINDRAG: u32 = 204u32;
pub const DISPID_CHECKSTATECHANGED: u32 = 209u32;
pub const DISPID_COLUMNSCHANGED: u32 = 212u32;
pub const DISPID_CONTENTSCHANGED: u32 = 207u32;
pub const DISPID_CTRLMOUSEWHEEL: u32 = 213u32;
pub const DISPID_DEFAULTVERBINVOKED: u32 = 203u32;
pub const DISPID_ENTERPRESSED: u32 = 200u32;
pub const DISPID_ENTERPRISEIDCHANGED: u32 = 224u32;
pub const DISPID_EXPLORERWINDOWREADY: u32 = 221u32;
pub const DISPID_FILELISTENUMDONE: u32 = 201u32;
pub const DISPID_FILTERINVOKED: u32 = 218u32;
pub const DISPID_FOCUSCHANGED: u32 = 208u32;
pub const DISPID_FOLDERCHANGED: u32 = 217u32;
pub const DISPID_IADCCTL_DEFAULTCAT: u32 = 262u32;
pub const DISPID_IADCCTL_DIRTY: u32 = 256u32;
pub const DISPID_IADCCTL_FORCEX86: u32 = 259u32;
pub const DISPID_IADCCTL_ONDOMAIN: u32 = 261u32;
pub const DISPID_IADCCTL_PUBCAT: u32 = 257u32;
pub const DISPID_IADCCTL_SHOWPOSTSETUP: u32 = 260u32;
pub const DISPID_IADCCTL_SORT: u32 = 258u32;
pub const DISPID_ICONSIZECHANGED: u32 = 215u32;
pub const DISPID_INITIALENUMERATIONDONE: u32 = 223u32;
pub const DISPID_NOITEMSTATE_CHANGED: u32 = 206u32;
pub const DISPID_ORDERCHANGED: u32 = 210u32;
pub const DISPID_SEARCHCOMMAND_ABORT: u32 = 3u32;
pub const DISPID_SEARCHCOMMAND_COMPLETE: u32 = 2u32;
pub const DISPID_SEARCHCOMMAND_ERROR: u32 = 6u32;
pub const DISPID_SEARCHCOMMAND_PROGRESSTEXT: u32 = 5u32;
pub const DISPID_SEARCHCOMMAND_RESTORE: u32 = 7u32;
pub const DISPID_SEARCHCOMMAND_START: u32 = 1u32;
pub const DISPID_SEARCHCOMMAND_UPDATE: u32 = 4u32;
pub const DISPID_SELECTEDITEMCHANGED: u32 = 220u32;
pub const DISPID_SELECTIONCHANGED: u32 = 200u32;
pub const DISPID_SORTDONE: u32 = 214u32;
pub const DISPID_UPDATEIMAGE: u32 = 222u32;
pub const DISPID_VERBINVOKED: u32 = 202u32;
pub const DISPID_VIEWMODECHANGED: u32 = 205u32;
pub const DISPID_VIEWPAINTDONE: u32 = 211u32;
pub const DISPID_WORDWHEELEDITED: u32 = 219u32;
pub const DI_GETDRAGIMAGE: windows_core::PCWSTR = windows_core::w!("ShellGetDragImage");
pub const DLG_SCRNSAVECONFIGURE: u32 = 2003u32;
pub const DLLVER_BUILD_MASK: u64 = 4294901760u64;
pub const DLLVER_MAJOR_MASK: u64 = 18446462598732840960u64;
pub const DLLVER_MINOR_MASK: u64 = 281470681743360u64;
pub const DLLVER_PLATFORM_NT: u32 = 2u32;
pub const DLLVER_PLATFORM_WINDOWS: u32 = 1u32;
pub const DLLVER_QFE_MASK: u64 = 65535u64;
pub const DOGIF_DEFAULT: DATAOBJ_GET_ITEM_FLAGS = 0i32;
pub const DOGIF_NO_HDROP: DATAOBJ_GET_ITEM_FLAGS = 2i32;
pub const DOGIF_NO_URL: DATAOBJ_GET_ITEM_FLAGS = 4i32;
pub const DOGIF_ONLY_IF_ONE: DATAOBJ_GET_ITEM_FLAGS = 8i32;
pub const DOGIF_TRAVERSE_LINK: DATAOBJ_GET_ITEM_FLAGS = 1i32;
pub const DROPIMAGE_COPY: DROPIMAGETYPE = 1i32;
pub const DROPIMAGE_INVALID: DROPIMAGETYPE = -1i32;
pub const DROPIMAGE_LABEL: DROPIMAGETYPE = 6i32;
pub const DROPIMAGE_LINK: DROPIMAGETYPE = 4i32;
pub const DROPIMAGE_MOVE: DROPIMAGETYPE = 2i32;
pub const DROPIMAGE_NOIMAGE: DROPIMAGETYPE = 8i32;
pub const DROPIMAGE_NONE: DROPIMAGETYPE = 0i32;
pub const DROPIMAGE_WARNING: DROPIMAGETYPE = 7i32;
pub const DSD_BACKWARD: DESKTOP_SLIDESHOW_DIRECTION = 1i32;
pub const DSD_FORWARD: DESKTOP_SLIDESHOW_DIRECTION = 0i32;
pub const DSFT_DETECT: DEFAULTSAVEFOLDERTYPE = 1i32;
pub const DSFT_PRIVATE: DEFAULTSAVEFOLDERTYPE = 2i32;
pub const DSFT_PUBLIC: DEFAULTSAVEFOLDERTYPE = 3i32;
pub const DSH_ALLOWDROPDESCRIPTIONTEXT: DSH_FLAGS = 1i32;
pub const DSO_SHUFFLEIMAGES: DESKTOP_SLIDESHOW_OPTIONS = 1i32;
pub const DSS_DISABLED_BY_REMOTE_SESSION: DESKTOP_SLIDESHOW_STATE = 4i32;
pub const DSS_ENABLED: DESKTOP_SLIDESHOW_STATE = 1i32;
pub const DSS_SLIDESHOW: DESKTOP_SLIDESHOW_STATE = 2i32;
pub const DVASPECT_COPY: u32 = 3u32;
pub const DVASPECT_LINK: u32 = 4u32;
pub const DVASPECT_SHORTNAME: u32 = 2u32;
pub const DWFAF_AUTOHIDE: u32 = 16u32;
pub const DWFAF_GROUP1: u32 = 2u32;
pub const DWFAF_GROUP2: u32 = 4u32;
pub const DWFAF_HIDDEN: u32 = 1u32;
pub const DWFRF_DELETECONFIGDATA: u32 = 1u32;
pub const DWFRF_NORMAL: u32 = 0u32;
pub const DWPOS_CENTER: DESKTOP_WALLPAPER_POSITION = 0i32;
pub const DWPOS_FILL: DESKTOP_WALLPAPER_POSITION = 4i32;
pub const DWPOS_FIT: DESKTOP_WALLPAPER_POSITION = 3i32;
pub const DWPOS_SPAN: DESKTOP_WALLPAPER_POSITION = 5i32;
pub const DWPOS_STRETCH: DESKTOP_WALLPAPER_POSITION = 2i32;
pub const DWPOS_TILE: DESKTOP_WALLPAPER_POSITION = 1i32;
pub const EBF_NODROPTARGET: EXPLORER_BROWSER_FILL_FLAGS = 512i32;
pub const EBF_NONE: EXPLORER_BROWSER_FILL_FLAGS = 0i32;
pub const EBF_SELECTFROMDATAOBJECT: EXPLORER_BROWSER_FILL_FLAGS = 256i32;
pub const EBO_ALWAYSNAVIGATE: EXPLORER_BROWSER_OPTIONS = 4i32;
pub const EBO_HTMLSHAREPOINTVIEW: EXPLORER_BROWSER_OPTIONS = 32i32;
pub const EBO_NAVIGATEONCE: EXPLORER_BROWSER_OPTIONS = 1i32;
pub const EBO_NOBORDER: EXPLORER_BROWSER_OPTIONS = 64i32;
pub const EBO_NONE: EXPLORER_BROWSER_OPTIONS = 0i32;
pub const EBO_NOPERSISTVIEWSTATE: EXPLORER_BROWSER_OPTIONS = 128i32;
pub const EBO_NOTRAVELLOG: EXPLORER_BROWSER_OPTIONS = 8i32;
pub const EBO_NOWRAPPERWINDOW: EXPLORER_BROWSER_OPTIONS = 16i32;
pub const EBO_SHOWFRAMES: EXPLORER_BROWSER_OPTIONS = 2i32;
pub const ECF_AUTOMENUICONS: _EXPCMDFLAGS = 512i32;
pub const ECF_DEFAULT: _EXPCMDFLAGS = 0i32;
pub const ECF_HASLUASHIELD: _EXPCMDFLAGS = 16i32;
pub const ECF_HASSPLITBUTTON: _EXPCMDFLAGS = 2i32;
pub const ECF_HASSUBCOMMANDS: _EXPCMDFLAGS = 1i32;
pub const ECF_HIDELABEL: _EXPCMDFLAGS = 4i32;
pub const ECF_ISDROPDOWN: _EXPCMDFLAGS = 128i32;
pub const ECF_ISSEPARATOR: _EXPCMDFLAGS = 8i32;
pub const ECF_SEPARATORAFTER: _EXPCMDFLAGS = 64i32;
pub const ECF_SEPARATORBEFORE: _EXPCMDFLAGS = 32i32;
pub const ECF_TOGGLEABLE: _EXPCMDFLAGS = 256i32;
pub const ECHUIM_DESKTOP: EC_HOST_UI_MODE = 0i32;
pub const ECHUIM_IMMERSIVE: EC_HOST_UI_MODE = 1i32;
pub const ECHUIM_SYSTEM_LAUNCHER: EC_HOST_UI_MODE = 2i32;
pub const ECS_CHECKBOX: _EXPCMDSTATE = 4i32;
pub const ECS_CHECKED: _EXPCMDSTATE = 8i32;
pub const ECS_DISABLED: _EXPCMDSTATE = 1i32;
pub const ECS_ENABLED: _EXPCMDSTATE = 0i32;
pub const ECS_HIDDEN: _EXPCMDSTATE = 2i32;
pub const ECS_RADIOCHECK: _EXPCMDSTATE = 16i32;
pub const EGK_KEYBOARD: EDGE_GESTURE_KIND = 1i32;
pub const EGK_MOUSE: EDGE_GESTURE_KIND = 2i32;
pub const EGK_TOUCH: EDGE_GESTURE_KIND = 0i32;
pub const EPS_DEFAULT_OFF: _EXPLORERPANESTATE = 2i32;
pub const EPS_DEFAULT_ON: _EXPLORERPANESTATE = 1i32;
pub const EPS_DONTCARE: _EXPLORERPANESTATE = 0i32;
pub const EPS_FORCE: _EXPLORERPANESTATE = 131072i32;
pub const EPS_INITIALSTATE: _EXPLORERPANESTATE = 65536i32;
pub const EPS_STATEMASK: _EXPLORERPANESTATE = 65535i32;
pub const EP_AdvQueryPane: windows_core::GUID = windows_core::GUID::from_u128(0xb4e9db8b_34ba_4c39_b5cc_16a1bd2c411c);
pub const EP_Commands: windows_core::GUID = windows_core::GUID::from_u128(0xd9745868_ca5f_4a76_91cd_f5a129fbb076);
pub const EP_Commands_Organize: windows_core::GUID = windows_core::GUID::from_u128(0x72e81700_e3ec_4660_bf24_3c3b7b648806);
pub const EP_Commands_View: windows_core::GUID = windows_core::GUID::from_u128(0x21f7c32d_eeaa_439b_bb51_37b96fd6a943);
pub const EP_DetailsPane: windows_core::GUID = windows_core::GUID::from_u128(0x43abf98b_89b8_472d_b9ce_e69b8229f019);
pub const EP_NavPane: windows_core::GUID = windows_core::GUID::from_u128(0xcb316b22_25f7_42b8_8a09_540d23a43c2f);
pub const EP_PreviewPane: windows_core::GUID = windows_core::GUID::from_u128(0x893c63d1_45c8_4d17_be19_223be71be365);
pub const EP_QueryPane: windows_core::GUID = windows_core::GUID::from_u128(0x65bcde4f_4f07_4f27_83a7_1afca4df7ddd);
pub const EP_Ribbon: windows_core::GUID = windows_core::GUID::from_u128(0xd27524a8_c9f2_4834_a106_df8889fd4f37);
pub const EP_StatusBar: windows_core::GUID = windows_core::GUID::from_u128(0x65fe56ce_5cfe_4bc4_ad8a_7ae3fe7e8f7c);
pub const EXECUTE_E_LAUNCH_APPLICATION: windows_core::HRESULT = 0x80270101_u32 as _;
pub const EXPPS_FILETYPES: _EXPPS = 1i32;
pub const EXP_DARWIN_ID_SIG: u32 = 2684354566u32;
pub const EXP_PROPERTYSTORAGE_SIG: u32 = 2684354569u32;
pub const EXP_SPECIAL_FOLDER_SIG: u32 = 2684354565u32;
pub const EXP_SZ_ICON_SIG: u32 = 2684354567u32;
pub const EXP_SZ_LINK_SIG: u32 = 2684354561u32;
pub const E_ACTIVATIONDENIED_SHELLERROR: windows_core::HRESULT = 0x80270131_u32 as _;
pub const E_ACTIVATIONDENIED_SHELLNOTREADY: windows_core::HRESULT = 0x80270134_u32 as _;
pub const E_ACTIVATIONDENIED_SHELLRESTART: windows_core::HRESULT = 0x80270132_u32 as _;
pub const E_ACTIVATIONDENIED_UNEXPECTED: windows_core::HRESULT = 0x80270133_u32 as _;
pub const E_ACTIVATIONDENIED_USERCLOSE: windows_core::HRESULT = 0x80270130_u32 as _;
pub const E_FILE_PLACEHOLDER_NOT_INITIALIZED: windows_core::HRESULT = 0x80270110_u32 as _;
pub const E_FILE_PLACEHOLDER_SERVER_TIMED_OUT: windows_core::HRESULT = 0x80270112_u32 as _;
pub const E_FILE_PLACEHOLDER_STORAGEPROVIDER_NOT_FOUND: windows_core::HRESULT = 0x80270113_u32 as _;
pub const E_FILE_PLACEHOLDER_VERSION_MISMATCH: windows_core::HRESULT = 0x80270111_u32 as _;
pub const E_FLAGS: windows_core::HRESULT = 0x80041000_u32 as _;
pub const E_IMAGEFEED_CHANGEDISABLED: windows_core::HRESULT = 0x80270310_u32 as _;
pub const E_NOTVALIDFORANIMATEDIMAGE: windows_core::HRESULT = 0x80040001_u32 as _;
pub const E_PREVIEWHANDLER_CORRUPT: windows_core::HRESULT = 0x86420004_u32 as _;
pub const E_PREVIEWHANDLER_DRM_FAIL: windows_core::HRESULT = 0x86420001_u32 as _;
pub const E_PREVIEWHANDLER_NOAUTH: windows_core::HRESULT = 0x86420002_u32 as _;
pub const E_PREVIEWHANDLER_NOTFOUND: windows_core::HRESULT = 0x86420003_u32 as _;
pub const E_SHELL_EXTENSION_BLOCKED: windows_core::HRESULT = 0x80270301_u32 as _;
pub const E_TILE_NOTIFICATIONS_PLATFORM_FAILURE: windows_core::HRESULT = 0x80270249_u32 as _;
pub const E_USERTILE_CHANGEDISABLED: windows_core::HRESULT = 0x80270211_u32 as _;
pub const E_USERTILE_FILESIZE: windows_core::HRESULT = 0x80270214_u32 as _;
pub const E_USERTILE_LARGEORDYNAMIC: windows_core::HRESULT = 0x80270212_u32 as _;
pub const E_USERTILE_UNSUPPORTEDFILETYPE: windows_core::HRESULT = 0x80270210_u32 as _;
pub const E_USERTILE_VIDEOFRAMESIZE: windows_core::HRESULT = 0x80270213_u32 as _;
pub const ExtractIfNotCached: ThumbnailStreamCacheOptions = 0i32;
pub const FCIDM_BROWSERFIRST: u32 = 40960u32;
pub const FCIDM_BROWSERLAST: u32 = 48896u32;
pub const FCIDM_GLOBALFIRST: u32 = 32768u32;
pub const FCIDM_GLOBALLAST: u32 = 40959u32;
pub const FCIDM_MENU_EDIT: u32 = 32832u32;
pub const FCIDM_MENU_EXPLORE: u32 = 33104u32;
pub const FCIDM_MENU_FAVORITES: u32 = 33136u32;
pub const FCIDM_MENU_FILE: u32 = 32768u32;
pub const FCIDM_MENU_FIND: u32 = 33088u32;
pub const FCIDM_MENU_HELP: u32 = 33024u32;
pub const FCIDM_MENU_TOOLS: u32 = 32960u32;
pub const FCIDM_MENU_TOOLS_SEP_GOTO: u32 = 32961u32;
pub const FCIDM_MENU_VIEW: u32 = 32896u32;
pub const FCIDM_MENU_VIEW_SEP_OPTIONS: u32 = 32897u32;
pub const FCIDM_SHVIEWFIRST: u32 = 0u32;
pub const FCIDM_SHVIEWLAST: u32 = 32767u32;
pub const FCIDM_STATUS: u32 = 40961u32;
pub const FCIDM_TOOLBAR: u32 = 40960u32;
pub const FCSM_CLSID: u32 = 8u32;
pub const FCSM_FLAGS: u32 = 64u32;
pub const FCSM_ICONFILE: u32 = 16u32;
pub const FCSM_INFOTIP: u32 = 4u32;
pub const FCSM_LOGO: u32 = 32u32;
pub const FCSM_VIEWID: u32 = 1u32;
pub const FCSM_WEBVIEWTEMPLATE: u32 = 2u32;
pub const FCS_FLAG_DRAGDROP: u32 = 2u32;
pub const FCS_FORCEWRITE: u32 = 2u32;
pub const FCS_READ: u32 = 1u32;
pub const FCT_ADDTOEND: u32 = 4u32;
pub const FCT_CONFIGABLE: u32 = 2u32;
pub const FCT_MERGE: u32 = 1u32;
pub const FCW_INTERNETBAR: u32 = 6u32;
pub const FCW_PROGRESS: u32 = 8u32;
pub const FCW_STATUS: u32 = 1u32;
pub const FCW_TOOLBAR: u32 = 2u32;
pub const FCW_TREE: u32 = 3u32;
pub const FDAP_BOTTOM: FDAP = 0i32;
pub const FDAP_TOP: FDAP = 1i32;
pub const FDEOR_ACCEPT: FDE_OVERWRITE_RESPONSE = 1i32;
pub const FDEOR_DEFAULT: FDE_OVERWRITE_RESPONSE = 0i32;
pub const FDEOR_REFUSE: FDE_OVERWRITE_RESPONSE = 2i32;
pub const FDESVR_ACCEPT: FDE_SHAREVIOLATION_RESPONSE = 1i32;
pub const FDESVR_DEFAULT: FDE_SHAREVIOLATION_RESPONSE = 0i32;
pub const FDESVR_REFUSE: FDE_SHAREVIOLATION_RESPONSE = 2i32;
pub const FDTF_LONGDATE: u32 = 4u32;
pub const FDTF_LONGTIME: u32 = 8u32;
pub const FDTF_LTRDATE: u32 = 256u32;
pub const FDTF_NOAUTOREADINGORDER: u32 = 1024u32;
pub const FDTF_RELATIVE: u32 = 16u32;
pub const FDTF_RTLDATE: u32 = 512u32;
pub const FDTF_SHORTDATE: u32 = 2u32;
pub const FDTF_SHORTTIME: u32 = 1u32;
pub const FD_ACCESSTIME: FD_FLAGS = 16i32;
pub const FD_ATTRIBUTES: FD_FLAGS = 4i32;
pub const FD_CLSID: FD_FLAGS = 1i32;
pub const FD_CREATETIME: FD_FLAGS = 8i32;
pub const FD_FILESIZE: FD_FLAGS = 64i32;
pub const FD_LINKUI: FD_FLAGS = 32768i32;
pub const FD_PROGRESSUI: FD_FLAGS = 16384i32;
pub const FD_SIZEPOINT: FD_FLAGS = 2i32;
pub const FD_UNICODE: FD_FLAGS = -2147483648i32;
pub const FD_WRITESTIME: FD_FLAGS = 32i32;
pub const FEM_NAVIGATION: FOLDER_ENUM_MODE = 1i32;
pub const FEM_VIEWRESULT: FOLDER_ENUM_MODE = 0i32;
pub const FFFP_EXACTMATCH: FFFP_MODE = 0i32;
pub const FFFP_NEARESTPARENTMATCH: FFFP_MODE = 1i32;
pub const FLVM_CONTENT: FOLDERLOGICALVIEWMODE = 5i32;
pub const FLVM_DETAILS: FOLDERLOGICALVIEWMODE = 1i32;
pub const FLVM_FIRST: FOLDERLOGICALVIEWMODE = 1i32;
pub const FLVM_ICONS: FOLDERLOGICALVIEWMODE = 3i32;
pub const FLVM_LAST: FOLDERLOGICALVIEWMODE = 5i32;
pub const FLVM_LIST: FOLDERLOGICALVIEWMODE = 4i32;
pub const FLVM_TILES: FOLDERLOGICALVIEWMODE = 2i32;
pub const FLVM_UNSPECIFIED: FOLDERLOGICALVIEWMODE = -1i32;
pub const FMTID_Briefcase: windows_core::GUID = windows_core::GUID::from_u128(0x328d8b21_7729_4bfc_954c_902b329d56b0);
pub const FMTID_CustomImageProperties: windows_core::GUID = windows_core::GUID::from_u128(0x7ecd8b0e_c136_4a9b_9411_4ebd6673ccc3);
pub const FMTID_DRM: windows_core::GUID = windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed);
pub const FMTID_Displaced: windows_core::GUID = windows_core::GUID::from_u128(0x9b174b33_40ff_11d2_a27e_00c04fc30871);
pub const FMTID_ImageProperties: windows_core::GUID = windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99);
pub const FMTID_InternetSite: windows_core::GUID = windows_core::GUID::from_u128(0x000214a1_0000_0000_c000_000000000046);
pub const FMTID_Intshcut: windows_core::GUID = windows_core::GUID::from_u128(0x000214a0_0000_0000_c000_000000000046);
pub const FMTID_LibraryProperties: windows_core::GUID = windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67);
pub const FMTID_MUSIC: windows_core::GUID = windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6);
pub const FMTID_Misc: windows_core::GUID = windows_core::GUID::from_u128(0x9b174b34_40ff_11d2_a27e_00c04fc30871);
pub const FMTID_Query: windows_core::GUID = windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9);
pub const FMTID_ShellDetails: windows_core::GUID = windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0);
pub const FMTID_Storage: windows_core::GUID = windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac);
pub const FMTID_Volume: windows_core::GUID = windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871);
pub const FMTID_WebView: windows_core::GUID = windows_core::GUID::from_u128(0xf2275480_f782_4291_bd94_f13693513aec);
pub const FOF2_MERGEFOLDERSONCOLLISION: FILE_OPERATION_FLAGS2 = 1i32;
pub const FOF2_NONE: FILE_OPERATION_FLAGS2 = 0i32;
pub const FOFX_ADDUNDORECORD: FILEOPERATION_FLAGS = 536870912u32;
pub const FOFX_COPYASDOWNLOAD: FILEOPERATION_FLAGS = 1073741824u32;
pub const FOFX_DONTDISPLAYDESTPATH: FILEOPERATION_FLAGS = 134217728u32;
pub const FOFX_DONTDISPLAYLOCATIONS: FILEOPERATION_FLAGS = 2147483648u32;
pub const FOFX_DONTDISPLAYSOURCEPATH: FILEOPERATION_FLAGS = 67108864u32;
pub const FOFX_EARLYFAILURE: FILEOPERATION_FLAGS = 1048576u32;
pub const FOFX_KEEPNEWERFILE: FILEOPERATION_FLAGS = 4194304u32;
pub const FOFX_MOVEACLSACROSSVOLUMES: FILEOPERATION_FLAGS = 33554432u32;
pub const FOFX_NOCOPYHOOKS: FILEOPERATION_FLAGS = 8388608u32;
pub const FOFX_NOMINIMIZEBOX: FILEOPERATION_FLAGS = 16777216u32;
pub const FOFX_NOSKIPJUNCTIONS: FILEOPERATION_FLAGS = 65536u32;
pub const FOFX_PREFERHARDLINK: FILEOPERATION_FLAGS = 131072u32;
pub const FOFX_PRESERVEFILEEXTENSIONS: FILEOPERATION_FLAGS = 2097152u32;
pub const FOFX_RECYCLEONDELETE: FILEOPERATION_FLAGS = 524288u32;
pub const FOFX_REQUIREELEVATION: FILEOPERATION_FLAGS = 268435456u32;
pub const FOFX_SHOWELEVATIONPROMPT: FILEOPERATION_FLAGS = 262144u32;
pub const FOF_ALLOWUNDO: FILEOPERATION_FLAGS = 64u32;
pub const FOF_CONFIRMMOUSE: FILEOPERATION_FLAGS = 2u32;
pub const FOF_FILESONLY: FILEOPERATION_FLAGS = 128u32;
pub const FOF_MULTIDESTFILES: FILEOPERATION_FLAGS = 1u32;
pub const FOF_NOCONFIRMATION: FILEOPERATION_FLAGS = 16u32;
pub const FOF_NOCONFIRMMKDIR: FILEOPERATION_FLAGS = 512u32;
pub const FOF_NOCOPYSECURITYATTRIBS: FILEOPERATION_FLAGS = 2048u32;
pub const FOF_NOERRORUI: FILEOPERATION_FLAGS = 1024u32;
pub const FOF_NORECURSEREPARSE: FILEOPERATION_FLAGS = 32768u32;
pub const FOF_NORECURSION: FILEOPERATION_FLAGS = 4096u32;
pub const FOF_NO_CONNECTED_ELEMENTS: FILEOPERATION_FLAGS = 8192u32;
pub const FOF_NO_UI: FILEOPERATION_FLAGS = 1556u32;
pub const FOF_RENAMEONCOLLISION: FILEOPERATION_FLAGS = 8u32;
pub const FOF_SILENT: FILEOPERATION_FLAGS = 4u32;
pub const FOF_SIMPLEPROGRESS: FILEOPERATION_FLAGS = 256u32;
pub const FOF_WANTMAPPINGHANDLE: FILEOPERATION_FLAGS = 32u32;
pub const FOF_WANTNUKEWARNING: FILEOPERATION_FLAGS = 16384u32;
pub const FOLDERID_AccountPictures: windows_core::GUID = windows_core::GUID::from_u128(0x008ca0b1_55b4_4c56_b8a8_4de4b299d3be);
pub const FOLDERID_AddNewPrograms: windows_core::GUID = windows_core::GUID::from_u128(0xde61d971_5ebc_4f02_a3a9_6c82895e5c04);
pub const FOLDERID_AdminTools: windows_core::GUID = windows_core::GUID::from_u128(0x724ef170_a42d_4fef_9f26_b60e846fba4f);
pub const FOLDERID_AllAppMods: windows_core::GUID = windows_core::GUID::from_u128(0x7ad67899_66af_43ba_9156_6aad42e6c596);
pub const FOLDERID_AppCaptures: windows_core::GUID = windows_core::GUID::from_u128(0xedc0fe71_98d8_4f4a_b920_c8dc133cb165);
pub const FOLDERID_AppDataDesktop: windows_core::GUID = windows_core::GUID::from_u128(0xb2c5e279_7add_439f_b28c_c41fe1bbf672);
pub const FOLDERID_AppDataDocuments: windows_core::GUID = windows_core::GUID::from_u128(0x7be16610_1f7f_44ac_bff0_83e15f2ffca1);
pub const FOLDERID_AppDataFavorites: windows_core::GUID = windows_core::GUID::from_u128(0x7cfbefbc_de1f_45aa_b843_a542ac536cc9);
pub const FOLDERID_AppDataProgramData: windows_core::GUID = windows_core::GUID::from_u128(0x559d40a3_a036_40fa_af61_84cb430a4d34);
pub const FOLDERID_AppUpdates: windows_core::GUID = windows_core::GUID::from_u128(0xa305ce99_f527_492b_8b1a_7e76fa98d6e4);
pub const FOLDERID_ApplicationShortcuts: windows_core::GUID = windows_core::GUID::from_u128(0xa3918781_e5f2_4890_b3d9_a7e54332328c);
pub const FOLDERID_AppsFolder: windows_core::GUID = windows_core::GUID::from_u128(0x1e87508d_89c2_42f0_8a7e_645a0f50ca58);
pub const FOLDERID_CDBurning: windows_core::GUID = windows_core::GUID::from_u128(0x9e52ab10_f80d_49df_acb8_4330f5687855);
pub const FOLDERID_CameraRoll: windows_core::GUID = windows_core::GUID::from_u128(0xab5fb87b_7ce2_4f83_915d_550846c9537b);
pub const FOLDERID_CameraRollLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x2b20df75_1eda_4039_8097_38798227d5b7);
pub const FOLDERID_ChangeRemovePrograms: windows_core::GUID = windows_core::GUID::from_u128(0xdf7266ac_9274_4867_8d55_3bd661de872d);
pub const FOLDERID_CommonAdminTools: windows_core::GUID = windows_core::GUID::from_u128(0xd0384e7d_bac3_4797_8f14_cba229b392b5);
pub const FOLDERID_CommonOEMLinks: windows_core::GUID = windows_core::GUID::from_u128(0xc1bae2d0_10df_4334_bedd_7aa20b227a9d);
pub const FOLDERID_CommonPrograms: windows_core::GUID = windows_core::GUID::from_u128(0x0139d44e_6afe_49f2_8690_3dafcae6ffb8);
pub const FOLDERID_CommonStartMenu: windows_core::GUID = windows_core::GUID::from_u128(0xa4115719_d62e_491d_aa7c_e74b8be3b067);
pub const FOLDERID_CommonStartMenuPlaces: windows_core::GUID = windows_core::GUID::from_u128(0xa440879f_87a0_4f7d_b700_0207b966194a);
pub const FOLDERID_CommonStartup: windows_core::GUID = windows_core::GUID::from_u128(0x82a5ea35_d9cd_47c5_9629_e15d2f714e6e);
pub const FOLDERID_CommonTemplates: windows_core::GUID = windows_core::GUID::from_u128(0xb94237e7_57ac_4347_9151_b08c6c32d1f7);
pub const FOLDERID_ComputerFolder: windows_core::GUID = windows_core::GUID::from_u128(0x0ac0837c_bbf8_452a_850d_79d08e667ca7);
pub const FOLDERID_ConflictFolder: windows_core::GUID = windows_core::GUID::from_u128(0x4bfefb45_347d_4006_a5be_ac0cb0567192);
pub const FOLDERID_ConnectionsFolder: windows_core::GUID = windows_core::GUID::from_u128(0x6f0cd92b_2e97_45d1_88ff_b0d186b8dedd);
pub const FOLDERID_Contacts: windows_core::GUID = windows_core::GUID::from_u128(0x56784854_c6cb_462b_8169_88e350acb882);
pub const FOLDERID_ControlPanelFolder: windows_core::GUID = windows_core::GUID::from_u128(0x82a74aeb_aeb4_465c_a014_d097ee346d63);
pub const FOLDERID_Cookies: windows_core::GUID = windows_core::GUID::from_u128(0x2b0f765d_c0e9_4171_908e_08a611b84ff6);
pub const FOLDERID_CurrentAppMods: windows_core::GUID = windows_core::GUID::from_u128(0x3db40b20_2a30_4dbe_917e_771dd21dd099);
pub const FOLDERID_Desktop: windows_core::GUID = windows_core::GUID::from_u128(0xb4bfcc3a_db2c_424c_b029_7fe99a87c641);
pub const FOLDERID_DevelopmentFiles: windows_core::GUID = windows_core::GUID::from_u128(0xdbe8e08e_3053_4bbc_b183_2a7b2b191e59);
pub const FOLDERID_Device: windows_core::GUID = windows_core::GUID::from_u128(0x1c2ac1dc_4358_4b6c_9733_af21156576f0);
pub const FOLDERID_DeviceMetadataStore: windows_core::GUID = windows_core::GUID::from_u128(0x5ce4a5e9_e4eb_479d_b89f_130c02886155);
pub const FOLDERID_Documents: windows_core::GUID = windows_core::GUID::from_u128(0xfdd39ad0_238f_46af_adb4_6c85480369c7);
pub const FOLDERID_DocumentsLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x7b0db17d_9cd2_4a93_9733_46cc89022e7c);
pub const FOLDERID_Downloads: windows_core::GUID = windows_core::GUID::from_u128(0x374de290_123f_4565_9164_39c4925e467b);
pub const FOLDERID_Favorites: windows_core::GUID = windows_core::GUID::from_u128(0x1777f761_68ad_4d8a_87bd_30b759fa33dd);
pub const FOLDERID_Fonts: windows_core::GUID = windows_core::GUID::from_u128(0xfd228cb7_ae11_4ae3_864c_16f3910ab8fe);
pub const FOLDERID_GameTasks: windows_core::GUID = windows_core::GUID::from_u128(0x054fae61_4dd8_4787_80b6_090220c4b700);
pub const FOLDERID_Games: windows_core::GUID = windows_core::GUID::from_u128(0xcac52c1a_b53d_4edc_92d7_6b2e8ac19434);
pub const FOLDERID_History: windows_core::GUID = windows_core::GUID::from_u128(0xd9dc8a3b_b784_432e_a781_5a1130a75963);
pub const FOLDERID_HomeGroup: windows_core::GUID = windows_core::GUID::from_u128(0x52528a6b_b9e3_4add_b60d_588c2dba842d);
pub const FOLDERID_HomeGroupCurrentUser: windows_core::GUID = windows_core::GUID::from_u128(0x9b74b6a3_0dfd_4f11_9e78_5f7800f2e772);
pub const FOLDERID_ImplicitAppShortcuts: windows_core::GUID = windows_core::GUID::from_u128(0xbcb5256f_79f6_4cee_b725_dc34e402fd46);
pub const FOLDERID_InternetCache: windows_core::GUID = windows_core::GUID::from_u128(0x352481e8_33be_4251_ba85_6007caedcf9d);
pub const FOLDERID_InternetFolder: windows_core::GUID = windows_core::GUID::from_u128(0x4d9f7874_4e0c_4904_967b_40b0d20c3e4b);
pub const FOLDERID_Libraries: windows_core::GUID = windows_core::GUID::from_u128(0x1b3ea5dc_b587_4786_b4ef_bd1dc332aeae);
pub const FOLDERID_Links: windows_core::GUID = windows_core::GUID::from_u128(0xbfb9d5e0_c6a9_404c_b2b2_ae6db6af4968);
pub const FOLDERID_LocalAppData: windows_core::GUID = windows_core::GUID::from_u128(0xf1b32785_6fba_4fcf_9d55_7b8e7f157091);
pub const FOLDERID_LocalAppDataLow: windows_core::GUID = windows_core::GUID::from_u128(0xa520a1a4_1780_4ff6_bd18_167343c5af16);
pub const FOLDERID_LocalDocuments: windows_core::GUID = windows_core::GUID::from_u128(0xf42ee2d3_909f_4907_8871_4c22fc0bf756);
pub const FOLDERID_LocalDownloads: windows_core::GUID = windows_core::GUID::from_u128(0x7d83ee9b_2244_4e70_b1f5_5393042af1e4);
pub const FOLDERID_LocalMusic: windows_core::GUID = windows_core::GUID::from_u128(0xa0c69a99_21c8_4671_8703_7934162fcf1d);
pub const FOLDERID_LocalPictures: windows_core::GUID = windows_core::GUID::from_u128(0x0ddd015d_b06c_45d5_8c4c_f59713854639);
pub const FOLDERID_LocalStorage: windows_core::GUID = windows_core::GUID::from_u128(0xb3eb08d3_a1f3_496b_865a_42b536cda0ec);
pub const FOLDERID_LocalVideos: windows_core::GUID = windows_core::GUID::from_u128(0x35286a68_3c57_41a1_bbb1_0eae73d76c95);
pub const FOLDERID_LocalizedResourcesDir: windows_core::GUID = windows_core::GUID::from_u128(0x2a00375e_224c_49de_b8d1_440df7ef3ddc);
pub const FOLDERID_Music: windows_core::GUID = windows_core::GUID::from_u128(0x4bd8d571_6d19_48d3_be97_422220080e43);
pub const FOLDERID_MusicLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x2112ab0a_c86a_4ffe_a368_0de96e47012e);
pub const FOLDERID_NetHood: windows_core::GUID = windows_core::GUID::from_u128(0xc5abbf53_e17f_4121_8900_86626fc2c973);
pub const FOLDERID_NetworkFolder: windows_core::GUID = windows_core::GUID::from_u128(0xd20beec4_5ca8_4905_ae3b_bf251ea09b53);
pub const FOLDERID_Objects3D: windows_core::GUID = windows_core::GUID::from_u128(0x31c0dd25_9439_4f12_bf41_7ff4eda38722);
pub const FOLDERID_OneDrive: windows_core::GUID = windows_core::GUID::from_u128(0xa52bba46_e9e1_435f_b3d9_28daa648c0f6);
pub const FOLDERID_OriginalImages: windows_core::GUID = windows_core::GUID::from_u128(0x2c36c0aa_5812_4b87_bfd0_4cd0dfb19b39);
pub const FOLDERID_PhotoAlbums: windows_core::GUID = windows_core::GUID::from_u128(0x69d2cf90_fc33_4fb7_9a0c_ebb0f0fcb43c);
pub const FOLDERID_Pictures: windows_core::GUID = windows_core::GUID::from_u128(0x33e28130_4e1e_4676_835a_98395c3bc3bb);
pub const FOLDERID_PicturesLibrary: windows_core::GUID = windows_core::GUID::from_u128(0xa990ae9f_a03b_4e80_94bc_9912d7504104);
pub const FOLDERID_Playlists: windows_core::GUID = windows_core::GUID::from_u128(0xde92c1c7_837f_4f69_a3bb_86e631204a23);
pub const FOLDERID_PrintHood: windows_core::GUID = windows_core::GUID::from_u128(0x9274bd8d_cfd1_41c3_b35e_b13f55a758f4);
pub const FOLDERID_PrintersFolder: windows_core::GUID = windows_core::GUID::from_u128(0x76fc4e2d_d6ad_4519_a663_37bd56068185);
pub const FOLDERID_Profile: windows_core::GUID = windows_core::GUID::from_u128(0x5e6c858f_0e22_4760_9afe_ea3317b67173);
pub const FOLDERID_ProgramData: windows_core::GUID = windows_core::GUID::from_u128(0x62ab5d82_fdc1_4dc3_a9dd_070d1d495d97);
pub const FOLDERID_ProgramFiles: windows_core::GUID = windows_core::GUID::from_u128(0x905e63b6_c1bf_494e_b29c_65b732d3d21a);
pub const FOLDERID_ProgramFilesCommon: windows_core::GUID = windows_core::GUID::from_u128(0xf7f1ed05_9f6d_47a2_aaae_29d317c6f066);
pub const FOLDERID_ProgramFilesCommonX64: windows_core::GUID = windows_core::GUID::from_u128(0x6365d5a7_0f0d_45e5_87f6_0da56b6a4f7d);
pub const FOLDERID_ProgramFilesCommonX86: windows_core::GUID = windows_core::GUID::from_u128(0xde974d24_d9c6_4d3e_bf91_f4455120b917);
pub const FOLDERID_ProgramFilesX64: windows_core::GUID = windows_core::GUID::from_u128(0x6d809377_6af0_444b_8957_a3773f02200e);
pub const FOLDERID_ProgramFilesX86: windows_core::GUID = windows_core::GUID::from_u128(0x7c5a40ef_a0fb_4bfc_874a_c0f2e0b9fa8e);
pub const FOLDERID_Programs: windows_core::GUID = windows_core::GUID::from_u128(0xa77f5d77_2e2b_44c3_a6a2_aba601054a51);
pub const FOLDERID_Public: windows_core::GUID = windows_core::GUID::from_u128(0xdfdf76a2_c82a_4d63_906a_5644ac457385);
pub const FOLDERID_PublicDesktop: windows_core::GUID = windows_core::GUID::from_u128(0xc4aa340d_f20f_4863_afef_f87ef2e6ba25);
pub const FOLDERID_PublicDocuments: windows_core::GUID = windows_core::GUID::from_u128(0xed4824af_dce4_45a8_81e2_fc7965083634);
pub const FOLDERID_PublicDownloads: windows_core::GUID = windows_core::GUID::from_u128(0x3d644c9b_1fb8_4f30_9b45_f670235f79c0);
pub const FOLDERID_PublicGameTasks: windows_core::GUID = windows_core::GUID::from_u128(0xdebf2536_e1a8_4c59_b6a2_414586476aea);
pub const FOLDERID_PublicLibraries: windows_core::GUID = windows_core::GUID::from_u128(0x48daf80b_e6cf_4f4e_b800_0e69d84ee384);
pub const FOLDERID_PublicMusic: windows_core::GUID = windows_core::GUID::from_u128(0x3214fab5_9757_4298_bb61_92a9deaa44ff);
pub const FOLDERID_PublicPictures: windows_core::GUID = windows_core::GUID::from_u128(0xb6ebfb86_6907_413c_9af7_4fc2abf07cc5);
pub const FOLDERID_PublicRingtones: windows_core::GUID = windows_core::GUID::from_u128(0xe555ab60_153b_4d17_9f04_a5fe99fc15ec);
pub const FOLDERID_PublicUserTiles: windows_core::GUID = windows_core::GUID::from_u128(0x0482af6c_08f1_4c34_8c90_e17ec98b1e17);
pub const FOLDERID_PublicVideos: windows_core::GUID = windows_core::GUID::from_u128(0x2400183a_6185_49fb_a2d8_4a392a602ba3);
pub const FOLDERID_QuickLaunch: windows_core::GUID = windows_core::GUID::from_u128(0x52a4f021_7b75_48a9_9f6b_4b87a210bc8f);
pub const FOLDERID_Recent: windows_core::GUID = windows_core::GUID::from_u128(0xae50c081_ebd2_438a_8655_8a092e34987a);
pub const FOLDERID_RecordedCalls: windows_core::GUID = windows_core::GUID::from_u128(0x2f8b40c2_83ed_48ee_b383_a1f157ec6f9a);
pub const FOLDERID_RecordedTVLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x1a6fdba2_f42d_4358_a798_b74d745926c5);
pub const FOLDERID_RecycleBinFolder: windows_core::GUID = windows_core::GUID::from_u128(0xb7534046_3ecb_4c18_be4e_64cd4cb7d6ac);
pub const FOLDERID_ResourceDir: windows_core::GUID = windows_core::GUID::from_u128(0x8ad10c31_2adb_4296_a8f7_e4701232c972);
pub const FOLDERID_RetailDemo: windows_core::GUID = windows_core::GUID::from_u128(0x12d4c69e_24ad_4923_be19_31321c43a767);
pub const FOLDERID_Ringtones: windows_core::GUID = windows_core::GUID::from_u128(0xc870044b_f49e_4126_a9c3_b52a1ff411e8);
pub const FOLDERID_RoamedTileImages: windows_core::GUID = windows_core::GUID::from_u128(0xaaa8d5a5_f1d6_4259_baa8_78e7ef60835e);
pub const FOLDERID_RoamingAppData: windows_core::GUID = windows_core::GUID::from_u128(0x3eb685db_65f9_4cf6_a03a_e3ef65729f3d);
pub const FOLDERID_RoamingTiles: windows_core::GUID = windows_core::GUID::from_u128(0x00bcfc5a_ed94_4e48_96a1_3f6217f21990);
pub const FOLDERID_SEARCH_CSC: windows_core::GUID = windows_core::GUID::from_u128(0xee32e446_31ca_4aba_814f_a5ebd2fd6d5e);
pub const FOLDERID_SEARCH_MAPI: windows_core::GUID = windows_core::GUID::from_u128(0x98ec0e18_2098_4d44_8644_66979315a281);
pub const FOLDERID_SampleMusic: windows_core::GUID = windows_core::GUID::from_u128(0xb250c668_f57d_4ee1_a63c_290ee7d1aa1f);
pub const FOLDERID_SamplePictures: windows_core::GUID = windows_core::GUID::from_u128(0xc4900540_2379_4c75_844b_64e6faf8716b);
pub const FOLDERID_SamplePlaylists: windows_core::GUID = windows_core::GUID::from_u128(0x15ca69b3_30ee_49c1_ace1_6b5ec372afb5);
pub const FOLDERID_SampleVideos: windows_core::GUID = windows_core::GUID::from_u128(0x859ead94_2e85_48ad_a71a_0969cb56a6cd);
pub const FOLDERID_SavedGames: windows_core::GUID = windows_core::GUID::from_u128(0x4c5c32ff_bb9d_43b0_b5b4_2d72e54eaaa4);
pub const FOLDERID_SavedPictures: windows_core::GUID = windows_core::GUID::from_u128(0x3b193882_d3ad_4eab_965a_69829d1fb59f);
pub const FOLDERID_SavedPicturesLibrary: windows_core::GUID = windows_core::GUID::from_u128(0xe25b5812_be88_4bd9_94b0_29233477b6c3);
pub const FOLDERID_SavedSearches: windows_core::GUID = windows_core::GUID::from_u128(0x7d1d3a04_debb_4115_95cf_2f29da2920da);
pub const FOLDERID_Screenshots: windows_core::GUID = windows_core::GUID::from_u128(0xb7bede81_df94_4682_a7d8_57a52620b86f);
pub const FOLDERID_SearchHistory: windows_core::GUID = windows_core::GUID::from_u128(0x0d4c3db6_03a3_462f_a0e6_08924c41b5d4);
pub const FOLDERID_SearchHome: windows_core::GUID = windows_core::GUID::from_u128(0x190337d1_b8ca_4121_a639_6d472d16972a);
pub const FOLDERID_SearchTemplates: windows_core::GUID = windows_core::GUID::from_u128(0x7e636bfe_dfa9_4d5e_b456_d7b39851d8a9);
pub const FOLDERID_SendTo: windows_core::GUID = windows_core::GUID::from_u128(0x8983036c_27c0_404b_8f08_102d10dcfd74);
pub const FOLDERID_SidebarDefaultParts: windows_core::GUID = windows_core::GUID::from_u128(0x7b396e54_9ec5_4300_be0a_2482ebae1a26);
pub const FOLDERID_SidebarParts: windows_core::GUID = windows_core::GUID::from_u128(0xa75d362e_50fc_4fb7_ac2c_a8beaa314493);
pub const FOLDERID_SkyDrive: windows_core::GUID = windows_core::GUID::from_u128(0xa52bba46_e9e1_435f_b3d9_28daa648c0f6);
pub const FOLDERID_SkyDriveCameraRoll: windows_core::GUID = windows_core::GUID::from_u128(0x767e6811_49cb_4273_87c2_20f355e1085b);
pub const FOLDERID_SkyDriveDocuments: windows_core::GUID = windows_core::GUID::from_u128(0x24d89e24_2f19_4534_9dde_6a6671fbb8fe);
pub const FOLDERID_SkyDriveMusic: windows_core::GUID = windows_core::GUID::from_u128(0xc3f2459e_80d6_45dc_bfef_1f769f2be730);
pub const FOLDERID_SkyDrivePictures: windows_core::GUID = windows_core::GUID::from_u128(0x339719b5_8c47_4894_94c2_d8f77add44a6);
pub const FOLDERID_StartMenu: windows_core::GUID = windows_core::GUID::from_u128(0x625b53c3_ab48_4ec1_ba1f_a1ef4146fc19);
pub const FOLDERID_StartMenuAllPrograms: windows_core::GUID = windows_core::GUID::from_u128(0xf26305ef_6948_40b9_b255_81453d09c785);
pub const FOLDERID_Startup: windows_core::GUID = windows_core::GUID::from_u128(0xb97d20bb_f46a_4c97_ba10_5e3608430854);
pub const FOLDERID_SyncManagerFolder: windows_core::GUID = windows_core::GUID::from_u128(0x43668bf8_c14e_49b2_97c9_747784d784b7);
pub const FOLDERID_SyncResultsFolder: windows_core::GUID = windows_core::GUID::from_u128(0x289a9a43_be44_4057_a41b_587a76d7e7f9);
pub const FOLDERID_SyncSetupFolder: windows_core::GUID = windows_core::GUID::from_u128(0x0f214138_b1d3_4a90_bba9_27cbc0c5389a);
pub const FOLDERID_System: windows_core::GUID = windows_core::GUID::from_u128(0x1ac14e77_02e7_4e5d_b744_2eb1ae5198b7);
pub const FOLDERID_SystemX86: windows_core::GUID = windows_core::GUID::from_u128(0xd65231b0_b2f1_4857_a4ce_a8e7c6ea7d27);
pub const FOLDERID_Templates: windows_core::GUID = windows_core::GUID::from_u128(0xa63293e8_664e_48db_a079_df759e0509f7);
pub const FOLDERID_UserPinned: windows_core::GUID = windows_core::GUID::from_u128(0x9e3995ab_1f9c_4f13_b827_48b24b6c7174);
pub const FOLDERID_UserProfiles: windows_core::GUID = windows_core::GUID::from_u128(0x0762d272_c50a_4bb0_a382_697dcd729b80);
pub const FOLDERID_UserProgramFiles: windows_core::GUID = windows_core::GUID::from_u128(0x5cd7aee2_2219_4a67_b85d_6c9ce15660cb);
pub const FOLDERID_UserProgramFilesCommon: windows_core::GUID = windows_core::GUID::from_u128(0xbcbd3057_ca5c_4622_b42d_bc56db0ae516);
pub const FOLDERID_UsersFiles: windows_core::GUID = windows_core::GUID::from_u128(0xf3ce0f7c_4901_4acc_8648_d5d44b04ef8f);
pub const FOLDERID_UsersLibraries: windows_core::GUID = windows_core::GUID::from_u128(0xa302545d_deff_464b_abe8_61c8648d939b);
pub const FOLDERID_Videos: windows_core::GUID = windows_core::GUID::from_u128(0x18989b1d_99b5_455b_841c_ab7c74e4ddfc);
pub const FOLDERID_VideosLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x491e922f_5643_4af4_a7eb_4e7a138d8174);
pub const FOLDERID_Windows: windows_core::GUID = windows_core::GUID::from_u128(0xf38bf404_1d43_42f2_9305_67de0b28fc23);
pub const FOLDERTYPEID_AccountPictures: windows_core::GUID = windows_core::GUID::from_u128(0xdb2a5d8f_06e6_4007_aba6_af877d526ea6);
pub const FOLDERTYPEID_Communications: windows_core::GUID = windows_core::GUID::from_u128(0x91475fe5_586b_4eba_8d75_d17434b8cdf6);
pub const FOLDERTYPEID_CompressedFolder: windows_core::GUID = windows_core::GUID::from_u128(0x80213e82_bcfd_4c4f_8817_bb27601267a9);
pub const FOLDERTYPEID_Contacts: windows_core::GUID = windows_core::GUID::from_u128(0xde2b70ec_9bf7_4a93_bd3d_243f7881d492);
pub const FOLDERTYPEID_ControlPanelCategory: windows_core::GUID = windows_core::GUID::from_u128(0xde4f0660_fa10_4b8f_a494_068b20b22307);
pub const FOLDERTYPEID_ControlPanelClassic: windows_core::GUID = windows_core::GUID::from_u128(0x0c3794f3_b545_43aa_a329_c37430c58d2a);
pub const FOLDERTYPEID_Documents: windows_core::GUID = windows_core::GUID::from_u128(0x7d49d726_3c21_4f05_99aa_fdc2c9474656);
pub const FOLDERTYPEID_Downloads: windows_core::GUID = windows_core::GUID::from_u128(0x885a186e_a440_4ada_812b_db871b942259);
pub const FOLDERTYPEID_Games: windows_core::GUID = windows_core::GUID::from_u128(0xb689b0d0_76d3_4cbb_87f7_585d0e0ce070);
pub const FOLDERTYPEID_Generic: windows_core::GUID = windows_core::GUID::from_u128(0x5c4f28b5_f869_4e84_8e60_f11db97c5cc7);
pub const FOLDERTYPEID_GenericLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x5f4eab9a_6833_4f61_899d_31cf46979d49);
pub const FOLDERTYPEID_GenericSearchResults: windows_core::GUID = windows_core::GUID::from_u128(0x7fde1a1e_8b31_49a5_93b8_6be14cfa4943);
pub const FOLDERTYPEID_Invalid: windows_core::GUID = windows_core::GUID::from_u128(0x57807898_8c4f_4462_bb63_71042380b109);
pub const FOLDERTYPEID_Music: windows_core::GUID = windows_core::GUID::from_u128(0x94d6ddcc_4a68_4175_a374_bd584a510b78);
pub const FOLDERTYPEID_NetworkExplorer: windows_core::GUID = windows_core::GUID::from_u128(0x25cc242b_9a7c_4f51_80e0_7a2928febe42);
pub const FOLDERTYPEID_OpenSearch: windows_core::GUID = windows_core::GUID::from_u128(0x8faf9629_1980_46ff_8023_9dceab9c3ee3);
pub const FOLDERTYPEID_OtherUsers: windows_core::GUID = windows_core::GUID::from_u128(0xb337fd00_9dd5_4635_a6d4_da33fd102b7a);
pub const FOLDERTYPEID_Pictures: windows_core::GUID = windows_core::GUID::from_u128(0xb3690e58_e961_423b_b687_386ebfd83239);
pub const FOLDERTYPEID_Printers: windows_core::GUID = windows_core::GUID::from_u128(0x2c7bbec6_c844_4a0a_91fa_cef6f59cfda1);
pub const FOLDERTYPEID_PublishedItems: windows_core::GUID = windows_core::GUID::from_u128(0x7f2f5b96_ff74_41da_afd8_1c78a5f3aea2);
pub const FOLDERTYPEID_RecordedTV: windows_core::GUID = windows_core::GUID::from_u128(0x5557a28f_5da6_4f83_8809_c2c98a11a6fa);
pub const FOLDERTYPEID_RecycleBin: windows_core::GUID = windows_core::GUID::from_u128(0xd6d9e004_cd87_442b_9d57_5e0aeb4f6f72);
pub const FOLDERTYPEID_SavedGames: windows_core::GUID = windows_core::GUID::from_u128(0xd0363307_28cb_4106_9f23_2956e3e5e0e7);
pub const FOLDERTYPEID_SearchConnector: windows_core::GUID = windows_core::GUID::from_u128(0x982725ee_6f47_479e_b447_812bfa7d2e8f);
pub const FOLDERTYPEID_SearchHome: windows_core::GUID = windows_core::GUID::from_u128(0x834d8a44_0974_4ed6_866e_f203d80b3810);
pub const FOLDERTYPEID_Searches: windows_core::GUID = windows_core::GUID::from_u128(0x0b0ba2e3_405f_415e_a6ee_cad625207853);
pub const FOLDERTYPEID_SoftwareExplorer: windows_core::GUID = windows_core::GUID::from_u128(0xd674391b_52d9_4e07_834e_67c98610f39d);
pub const FOLDERTYPEID_StartMenu: windows_core::GUID = windows_core::GUID::from_u128(0xef87b4cb_f2ce_4785_8658_4ca6c63e38c6);
pub const FOLDERTYPEID_StorageProviderDocuments: windows_core::GUID = windows_core::GUID::from_u128(0xdd61bd66_70e8_48dd_9655_65c5e1aac2d1);
pub const FOLDERTYPEID_StorageProviderGeneric: windows_core::GUID = windows_core::GUID::from_u128(0x4f01ebc5_2385_41f2_a28e_2c5c91fb56e0);
pub const FOLDERTYPEID_StorageProviderMusic: windows_core::GUID = windows_core::GUID::from_u128(0x672ecd7e_af04_4399_875c_0290845b6247);
pub const FOLDERTYPEID_StorageProviderPictures: windows_core::GUID = windows_core::GUID::from_u128(0x71d642a9_f2b1_42cd_ad92_eb9300c7cc0a);
pub const FOLDERTYPEID_StorageProviderVideos: windows_core::GUID = windows_core::GUID::from_u128(0x51294da1_d7b1_485b_9e9a_17cffe33e187);
pub const FOLDERTYPEID_UserFiles: windows_core::GUID = windows_core::GUID::from_u128(0xcd0fc69b_71e2_46e5_9690_5bcd9f57aab3);
pub const FOLDERTYPEID_UsersLibraries: windows_core::GUID = windows_core::GUID::from_u128(0xc4d98f09_6124_4fe0_9942_826416082da9);
pub const FOLDERTYPEID_Videos: windows_core::GUID = windows_core::GUID::from_u128(0x5fa96407_7e77_483c_ac93_691d05850de8);
pub const FOS_ALLNONSTORAGEITEMS: FILEOPENDIALOGOPTIONS = 128u32;
pub const FOS_ALLOWMULTISELECT: FILEOPENDIALOGOPTIONS = 512u32;
pub const FOS_CREATEPROMPT: FILEOPENDIALOGOPTIONS = 8192u32;
pub const FOS_DEFAULTNOMINIMODE: FILEOPENDIALOGOPTIONS = 536870912u32;
pub const FOS_DONTADDTORECENT: FILEOPENDIALOGOPTIONS = 33554432u32;
pub const FOS_FILEMUSTEXIST: FILEOPENDIALOGOPTIONS = 4096u32;
pub const FOS_FORCEFILESYSTEM: FILEOPENDIALOGOPTIONS = 64u32;
pub const FOS_FORCEPREVIEWPANEON: FILEOPENDIALOGOPTIONS = 1073741824u32;
pub const FOS_FORCESHOWHIDDEN: FILEOPENDIALOGOPTIONS = 268435456u32;
pub const FOS_HIDEMRUPLACES: FILEOPENDIALOGOPTIONS = 131072u32;
pub const FOS_HIDEPINNEDPLACES: FILEOPENDIALOGOPTIONS = 262144u32;
pub const FOS_NOCHANGEDIR: FILEOPENDIALOGOPTIONS = 8u32;
pub const FOS_NODEREFERENCELINKS: FILEOPENDIALOGOPTIONS = 1048576u32;
pub const FOS_NOREADONLYRETURN: FILEOPENDIALOGOPTIONS = 32768u32;
pub const FOS_NOTESTFILECREATE: FILEOPENDIALOGOPTIONS = 65536u32;
pub const FOS_NOVALIDATE: FILEOPENDIALOGOPTIONS = 256u32;
pub const FOS_OKBUTTONNEEDSINTERACTION: FILEOPENDIALOGOPTIONS = 2097152u32;
pub const FOS_OVERWRITEPROMPT: FILEOPENDIALOGOPTIONS = 2u32;
pub const FOS_PATHMUSTEXIST: FILEOPENDIALOGOPTIONS = 2048u32;
pub const FOS_PICKFOLDERS: FILEOPENDIALOGOPTIONS = 32u32;
pub const FOS_SHAREAWARE: FILEOPENDIALOGOPTIONS = 16384u32;
pub const FOS_STRICTFILETYPES: FILEOPENDIALOGOPTIONS = 4u32;
pub const FOS_SUPPORTSTREAMABLEITEMS: FILEOPENDIALOGOPTIONS = 2147483648u32;
pub const FO_COPY: u32 = 2u32;
pub const FO_DELETE: u32 = 3u32;
pub const FO_MOVE: u32 = 1u32;
pub const FO_RENAME: u32 = 4u32;
pub const FP_ABOVE: FLYOUT_PLACEMENT = 1i32;
pub const FP_BELOW: FLYOUT_PLACEMENT = 2i32;
pub const FP_DEFAULT: FLYOUT_PLACEMENT = 0i32;
pub const FP_LEFT: FLYOUT_PLACEMENT = 3i32;
pub const FP_RIGHT: FLYOUT_PLACEMENT = 4i32;
pub const FTA_AlwaysUnsafe: FILETYPEATTRIBUTEFLAGS = 131072i32;
pub const FTA_AlwaysUseDirectInvoke: FILETYPEATTRIBUTEFLAGS = 4194304i32;
pub const FTA_Exclude: FILETYPEATTRIBUTEFLAGS = 1i32;
pub const FTA_HasExtension: FILETYPEATTRIBUTEFLAGS = 4i32;
pub const FTA_NoDDE: FILETYPEATTRIBUTEFLAGS = 8192i32;
pub const FTA_NoEdit: FILETYPEATTRIBUTEFLAGS = 8i32;
pub const FTA_NoEditDesc: FILETYPEATTRIBUTEFLAGS = 256i32;
pub const FTA_NoEditDflt: FILETYPEATTRIBUTEFLAGS = 1024i32;
pub const FTA_NoEditIcon: FILETYPEATTRIBUTEFLAGS = 512i32;
pub const FTA_NoEditMIME: FILETYPEATTRIBUTEFLAGS = 32768i32;
pub const FTA_NoEditVerb: FILETYPEATTRIBUTEFLAGS = 64i32;
pub const FTA_NoEditVerbCmd: FILETYPEATTRIBUTEFLAGS = 2048i32;
pub const FTA_NoEditVerbExe: FILETYPEATTRIBUTEFLAGS = 4096i32;
pub const FTA_NoNewVerb: FILETYPEATTRIBUTEFLAGS = 32i32;
pub const FTA_NoRecentDocs: FILETYPEATTRIBUTEFLAGS = 1048576i32;
pub const FTA_NoRemove: FILETYPEATTRIBUTEFLAGS = 16i32;
pub const FTA_NoRemoveVerb: FILETYPEATTRIBUTEFLAGS = 128i32;
pub const FTA_None: FILETYPEATTRIBUTEFLAGS = 0i32;
pub const FTA_OpenIsSafe: FILETYPEATTRIBUTEFLAGS = 65536i32;
pub const FTA_SafeForElevation: FILETYPEATTRIBUTEFLAGS = 2097152i32;
pub const FTA_Show: FILETYPEATTRIBUTEFLAGS = 2i32;
pub const FUT_EDITING: FILE_USAGE_TYPE = 1i32;
pub const FUT_GENERIC: FILE_USAGE_TYPE = 2i32;
pub const FUT_PLAYING: FILE_USAGE_TYPE = 0i32;
pub const FVM_AUTO: FOLDERVIEWMODE = -1i32;
pub const FVM_CONTENT: FOLDERVIEWMODE = 8i32;
pub const FVM_DETAILS: FOLDERVIEWMODE = 4i32;
pub const FVM_FIRST: FOLDERVIEWMODE = 1i32;
pub const FVM_ICON: FOLDERVIEWMODE = 1i32;
pub const FVM_LAST: FOLDERVIEWMODE = 8i32;
pub const FVM_LIST: FOLDERVIEWMODE = 3i32;
pub const FVM_SMALLICON: FOLDERVIEWMODE = 2i32;
pub const FVM_THUMBNAIL: FOLDERVIEWMODE = 5i32;
pub const FVM_THUMBSTRIP: FOLDERVIEWMODE = 7i32;
pub const FVM_TILE: FOLDERVIEWMODE = 6i32;
pub const FVO_CUSTOMORDERING: FOLDERVIEWOPTIONS = 4i32;
pub const FVO_CUSTOMPOSITION: FOLDERVIEWOPTIONS = 2i32;
pub const FVO_DEFAULT: FOLDERVIEWOPTIONS = 0i32;
pub const FVO_NOANIMATIONS: FOLDERVIEWOPTIONS = 16i32;
pub const FVO_NOSCROLLTIPS: FOLDERVIEWOPTIONS = 32i32;
pub const FVO_SUPPORTHYPERLINKS: FOLDERVIEWOPTIONS = 8i32;
pub const FVO_VISTALAYOUT: FOLDERVIEWOPTIONS = 1i32;
pub const FVSIF_CANVIEWIT: u32 = 1073741824u32;
pub const FVSIF_NEWFAILED: u32 = 134217728u32;
pub const FVSIF_NEWFILE: u32 = 2147483648u32;
pub const FVSIF_PINNED: u32 = 2u32;
pub const FVSIF_RECT: u32 = 1u32;
pub const FVST_EMPTYTEXT: FVTEXTTYPE = 0i32;
pub const FWF_ABBREVIATEDNAMES: FOLDERFLAGS = 2i32;
pub const FWF_ALIGNLEFT: FOLDERFLAGS = 2048i32;
pub const FWF_ALLOWRTLREADING: FOLDERFLAGS = -2147483648i32;
pub const FWF_AUTOARRANGE: FOLDERFLAGS = 1i32;
pub const FWF_AUTOCHECKSELECT: FOLDERFLAGS = 134217728i32;
pub const FWF_BESTFITWINDOW: FOLDERFLAGS = 16i32;
pub const FWF_CHECKSELECT: FOLDERFLAGS = 262144i32;
pub const FWF_DESKTOP: FOLDERFLAGS = 32i32;
pub const FWF_EXTENDEDTILES: FOLDERFLAGS = 33554432i32;
pub const FWF_FULLROWSELECT: FOLDERFLAGS = 2097152i32;
pub const FWF_HIDEFILENAMES: FOLDERFLAGS = 131072i32;
pub const FWF_NOBROWSERVIEWSTATE: FOLDERFLAGS = 268435456i32;
pub const FWF_NOCLIENTEDGE: FOLDERFLAGS = 512i32;
pub const FWF_NOCOLUMNHEADER: FOLDERFLAGS = 8388608i32;
pub const FWF_NOENUMREFRESH: FOLDERFLAGS = 524288i32;
pub const FWF_NOFILTERS: FOLDERFLAGS = 4194304i32;
pub const FWF_NOGROUPING: FOLDERFLAGS = 1048576i32;
pub const FWF_NOHEADERINALLVIEWS: FOLDERFLAGS = 16777216i32;
pub const FWF_NOICONS: FOLDERFLAGS = 4096i32;
pub const FWF_NONE: FOLDERFLAGS = 0i32;
pub const FWF_NOSCROLL: FOLDERFLAGS = 1024i32;
pub const FWF_NOSUBFOLDERS: FOLDERFLAGS = 128i32;
pub const FWF_NOVISIBLE: FOLDERFLAGS = 16384i32;
pub const FWF_NOWEBVIEW: FOLDERFLAGS = 65536i32;
pub const FWF_OWNERDATA: FOLDERFLAGS = 8i32;
pub const FWF_SHOWSELALWAYS: FOLDERFLAGS = 8192i32;
pub const FWF_SINGLECLICKACTIVATE: FOLDERFLAGS = 32768i32;
pub const FWF_SINGLESEL: FOLDERFLAGS = 64i32;
pub const FWF_SNAPTOGRID: FOLDERFLAGS = 4i32;
pub const FWF_SUBSETGROUPS: FOLDERFLAGS = 536870912i32;
pub const FWF_TRANSPARENT: FOLDERFLAGS = 256i32;
pub const FWF_TRICHECKSELECT: FOLDERFLAGS = 67108864i32;
pub const FWF_USESEARCHFOLDER: FOLDERFLAGS = 1073741824i32;
pub const GADOF_DIRTY: u32 = 1u32;
pub const GCS_HELPTEXT: u32 = 5u32;
pub const GCS_HELPTEXTA: u32 = 1u32;
pub const GCS_HELPTEXTW: u32 = 5u32;
pub const GCS_UNICODE: u32 = 4u32;
pub const GCS_VALIDATE: u32 = 6u32;
pub const GCS_VALIDATEA: u32 = 2u32;
pub const GCS_VALIDATEW: u32 = 6u32;
pub const GCS_VERB: u32 = 4u32;
pub const GCS_VERBA: u32 = 0u32;
pub const GCS_VERBICONW: u32 = 20u32;
pub const GCS_VERBW: u32 = 4u32;
pub const GCT_INVALID: u32 = 0u32;
pub const GCT_LFNCHAR: u32 = 1u32;
pub const GCT_SEPARATOR: u32 = 8u32;
pub const GCT_SHORTCHAR: u32 = 2u32;
pub const GCT_WILD: u32 = 4u32;
pub const GETPROPS_NONE: u32 = 0u32;
pub const GIL_ASYNC: u32 = 32u32;
pub const GIL_CHECKSHIELD: u32 = 512u32;
pub const GIL_DEFAULTICON: u32 = 64u32;
pub const GIL_DONTCACHE: u32 = 16u32;
pub const GIL_FORCENOSHIELD: u32 = 1024u32;
pub const GIL_FORSHELL: u32 = 2u32;
pub const GIL_FORSHORTCUT: u32 = 128u32;
pub const GIL_NOTFILENAME: u32 = 8u32;
pub const GIL_OPENICON: u32 = 1u32;
pub const GIL_PERCLASS: u32 = 4u32;
pub const GIL_PERINSTANCE: u32 = 2u32;
pub const GIL_SHIELD: u32 = 512u32;
pub const GIL_SIMULATEDOC: u32 = 1u32;
pub const GLOBALCOUNTER_APPLICATION_DESTINATIONS: SHGLOBALCOUNTER = 12i32;
pub const GLOBALCOUNTER_APPROVEDSITES: SHGLOBALCOUNTER = 4i32;
pub const GLOBALCOUNTER_APPSFOLDER_FILETYPEASSOCIATION_COUNTER: SHGLOBALCOUNTER = 55i32;
pub const GLOBALCOUNTER_APP_ITEMS_STATE_STORE_CACHE: SHGLOBALCOUNTER = 53i32;
pub const GLOBALCOUNTER_ASSOCCHANGED: SHGLOBALCOUNTER = 52i32;
pub const GLOBALCOUNTER_BANNERS_DATAMODEL_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 58i32;
pub const GLOBALCOUNTER_BITBUCKETNUMDELETERS: SHGLOBALCOUNTER = 14i32;
pub const GLOBALCOUNTER_COMMONPLACES_LIST_CACHE: SHGLOBALCOUNTER = 50i32;
pub const GLOBALCOUNTER_FOLDERDEFINITION_CACHE: SHGLOBALCOUNTER = 49i32;
pub const GLOBALCOUNTER_FOLDERSETTINGSCHANGE: SHGLOBALCOUNTER = 2i32;
pub const GLOBALCOUNTER_IEONLY_SESSIONS: SHGLOBALCOUNTER = 11i32;
pub const GLOBALCOUNTER_IESESSIONS: SHGLOBALCOUNTER = 10i32;
pub const GLOBALCOUNTER_INTERNETTOOLBAR_LAYOUT: SHGLOBALCOUNTER = 48i32;
pub const GLOBALCOUNTER_MAXIMUMVALUE: SHGLOBALCOUNTER = 59i32;
pub const GLOBALCOUNTER_OVERLAYMANAGER: SHGLOBALCOUNTER = 8i32;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE: SHGLOBALCOUNTER = 47i32;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 51i32;
pub const GLOBALCOUNTER_QUERYASSOCIATIONS: SHGLOBALCOUNTER = 9i32;
pub const GLOBALCOUNTER_RATINGS: SHGLOBALCOUNTER = 3i32;
pub const GLOBALCOUNTER_RATINGS_STATECOUNTER: SHGLOBALCOUNTER = 46i32;
pub const GLOBALCOUNTER_RECYCLEBINCORRUPTED: SHGLOBALCOUNTER = 45i32;
pub const GLOBALCOUNTER_RECYCLEBINENUM: SHGLOBALCOUNTER = 44i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_A: SHGLOBALCOUNTER = 16i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_B: SHGLOBALCOUNTER = 17i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_C: SHGLOBALCOUNTER = 18i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_D: SHGLOBALCOUNTER = 19i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_E: SHGLOBALCOUNTER = 20i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_F: SHGLOBALCOUNTER = 21i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_G: SHGLOBALCOUNTER = 22i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_H: SHGLOBALCOUNTER = 23i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_I: SHGLOBALCOUNTER = 24i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_J: SHGLOBALCOUNTER = 25i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_K: SHGLOBALCOUNTER = 26i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_L: SHGLOBALCOUNTER = 27i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_M: SHGLOBALCOUNTER = 28i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_N: SHGLOBALCOUNTER = 29i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_O: SHGLOBALCOUNTER = 30i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_P: SHGLOBALCOUNTER = 31i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Q: SHGLOBALCOUNTER = 32i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_R: SHGLOBALCOUNTER = 33i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_S: SHGLOBALCOUNTER = 34i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_T: SHGLOBALCOUNTER = 35i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_U: SHGLOBALCOUNTER = 36i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_V: SHGLOBALCOUNTER = 37i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_W: SHGLOBALCOUNTER = 38i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_X: SHGLOBALCOUNTER = 39i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Y: SHGLOBALCOUNTER = 40i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Z: SHGLOBALCOUNTER = 41i32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SHARES: SHGLOBALCOUNTER = 15i32;
pub const GLOBALCOUNTER_RESTRICTIONS: SHGLOBALCOUNTER = 5i32;
pub const GLOBALCOUNTER_SEARCHMANAGER: SHGLOBALCOUNTER = 0i32;
pub const GLOBALCOUNTER_SEARCHOPTIONS: SHGLOBALCOUNTER = 1i32;
pub const GLOBALCOUNTER_SETTINGSYNC_ENABLED: SHGLOBALCOUNTER = 54i32;
pub const GLOBALCOUNTER_SHELLSETTINGSCHANGED: SHGLOBALCOUNTER = 6i32;
pub const GLOBALCOUNTER_SYNC_ENGINE_INFORMATION_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 57i32;
pub const GLOBALCOUNTER_SYSTEMPIDLCHANGE: SHGLOBALCOUNTER = 7i32;
pub const GLOBALCOUNTER_USERINFOCHANGED: SHGLOBALCOUNTER = 56i32;
pub const GPFIDL_ALTNAME: GPFIDL_FLAGS = 1u32;
pub const GPFIDL_DEFAULT: GPFIDL_FLAGS = 0u32;
pub const GPFIDL_UNCPRINTER: GPFIDL_FLAGS = 2u32;
pub const HELPINFO_MENUITEM: HELP_INFO_TYPE = 2i32;
pub const HELPINFO_WINDOW: HELP_INFO_TYPE = 1i32;
pub const HGSC_DOCUMENTSLIBRARY: HOMEGROUPSHARINGCHOICES = 8i32;
pub const HGSC_MUSICLIBRARY: HOMEGROUPSHARINGCHOICES = 1i32;
pub const HGSC_NONE: HOMEGROUPSHARINGCHOICES = 0i32;
pub const HGSC_PICTURESLIBRARY: HOMEGROUPSHARINGCHOICES = 2i32;
pub const HGSC_PRINTERS: HOMEGROUPSHARINGCHOICES = 16i32;
pub const HGSC_VIDEOSLIBRARY: HOMEGROUPSHARINGCHOICES = 4i32;
pub const HLBWIF_DOCWNDMAXIMIZED: HLBWIF_FLAGS = 8i32;
pub const HLBWIF_FRAMEWNDMAXIMIZED: HLBWIF_FLAGS = 4i32;
pub const HLBWIF_HASDOCWNDINFO: HLBWIF_FLAGS = 2i32;
pub const HLBWIF_HASFRAMEWNDINFO: HLBWIF_FLAGS = 1i32;
pub const HLBWIF_HASWEBTOOLBARINFO: HLBWIF_FLAGS = 16i32;
pub const HLBWIF_WEBTOOLBARHIDDEN: HLBWIF_FLAGS = 32i32;
pub const HLFNAMEF_DEFAULT: HLFNAMEF = 0i32;
pub const HLFNAMEF_TRYCACHE: HLFNAMEF = 1i32;
pub const HLFNAMEF_TRYFULLTARGET: HLFNAMEF = 4i32;
pub const HLFNAMEF_TRYPRETTYTARGET: HLFNAMEF = 2i32;
pub const HLFNAMEF_TRYWIN95SHORTCUT: HLFNAMEF = 8i32;
pub const HLID_CURRENT: HLID_INFO = 4294967293u32;
pub const HLID_INVALID: HLID_INFO = 0u32;
pub const HLID_NEXT: HLID_INFO = 4294967294u32;
pub const HLID_PREVIOUS: HLID_INFO = 4294967295u32;
pub const HLID_STACKBOTTOM: HLID_INFO = 4294967292u32;
pub const HLID_STACKTOP: HLID_INFO = 4294967291u32;
pub const HLINKGETREF_ABSOLUTE: HLINKGETREF = 1i32;
pub const HLINKGETREF_DEFAULT: HLINKGETREF = 0i32;
pub const HLINKGETREF_RELATIVE: HLINKGETREF = 2i32;
pub const HLINKMISC_RELATIVE: HLINKMISC = 1i32;
pub const HLINKSETF_LOCATION: HLINKSETF = 2i32;
pub const HLINKSETF_TARGET: HLINKSETF = 1i32;
pub const HLINKWHICHMK_BASE: HLINKWHICHMK = 2i32;
pub const HLINKWHICHMK_CONTAINER: HLINKWHICHMK = 1i32;
pub const HLINK_E_FIRST: windows_core::HRESULT = 0x80040100_u32 as _;
pub const HLINK_S_DONTHIDE: i32 = 262400i32;
pub const HLINK_S_FIRST: windows_core::HRESULT = 0x40100_u32 as _;
pub const HLNF_ALLOW_AUTONAVIGATE: u32 = 536870912u32;
pub const HLNF_CALLERUNTRUSTED: u32 = 2097152u32;
pub const HLNF_CREATENOHISTORY: HLNF = 32u32;
pub const HLNF_DISABLEWINDOWRESTRICTIONS: u32 = 8388608u32;
pub const HLNF_EXTERNALNAVIGATE: u32 = 268435456u32;
pub const HLNF_INTERNALJUMP: HLNF = 1u32;
pub const HLNF_NAVIGATINGBACK: HLNF = 4u32;
pub const HLNF_NAVIGATINGFORWARD: HLNF = 8u32;
pub const HLNF_NAVIGATINGTOSTACKITEM: HLNF = 16u32;
pub const HLNF_NEWWINDOWSMANAGED: u32 = 2147483648u32;
pub const HLNF_OPENINNEWWINDOW: HLNF = 2u32;
pub const HLNF_TRUSTEDFORACTIVEX: u32 = 4194304u32;
pub const HLNF_TRUSTFIRSTDOWNLOAD: u32 = 16777216u32;
pub const HLNF_UNTRUSTEDFORDOWNLOAD: u32 = 33554432u32;
pub const HLQF_ISCURRENT: HLQF_INFO = 2i32;
pub const HLQF_ISVALID: HLQF_INFO = 1i32;
pub const HLSHORTCUTF_DEFAULT: HLSHORTCUTF = 0i32;
pub const HLSHORTCUTF_DONTACTUALLYCREATE: HLSHORTCUTF = 1i32;
pub const HLSHORTCUTF_MAYUSEEXISTINGSHORTCUT: HLSHORTCUTF = 8i32;
pub const HLSHORTCUTF_USEFILENAMEFROMFRIENDLYNAME: HLSHORTCUTF = 2i32;
pub const HLSHORTCUTF_USEUNIQUEFILENAME: HLSHORTCUTF = 4i32;
pub const HLSR_HISTORYFOLDER: HLSR = 2i32;
pub const HLSR_HOME: HLSR = 0i32;
pub const HLSR_SEARCHPAGE: HLSR = 1i32;
pub const HLTB_DOCKEDBOTTOM: HLTB_INFO = 3i32;
pub const HLTB_DOCKEDLEFT: HLTB_INFO = 0i32;
pub const HLTB_DOCKEDRIGHT: HLTB_INFO = 2i32;
pub const HLTB_DOCKEDTOP: HLTB_INFO = 1i32;
pub const HLTB_FLOATING: HLTB_INFO = 4i32;
pub const HLTRANSLATEF_DEFAULT: HLTRANSLATEF = 0i32;
pub const HLTRANSLATEF_DONTAPPLYDEFAULTPREFIX: HLTRANSLATEF = 1i32;
pub const HOMEGROUP_SECURITY_GROUP: windows_core::PCWSTR = windows_core::w!("HomeUsers");
pub const HOMEGROUP_SECURITY_GROUP_MULTI: windows_core::PCWSTR = windows_core::w!("HUG");
pub const IDC_OFFLINE_HAND: u32 = 103u32;
pub const IDC_PANTOOL_HAND_CLOSED: u32 = 105u32;
pub const IDC_PANTOOL_HAND_OPEN: u32 = 104u32;
pub const IDD_WIZEXTN_FIRST: u32 = 20480u32;
pub const IDD_WIZEXTN_LAST: u32 = 20736u32;
pub const IDO_SHGIOI_DEFAULT: u64 = 4294967292u64;
pub const IDO_SHGIOI_LINK: u32 = 268435454u32;
pub const IDO_SHGIOI_SHARE: u32 = 268435455u32;
pub const IDO_SHGIOI_SLOWFILE: u64 = 4294967293u64;
pub const IDS_DESCRIPTION: u32 = 1u32;
pub const ID_APP: u32 = 100u32;
pub const IEIFLAG_ASPECT: u32 = 4u32;
pub const IEIFLAG_ASYNC: u32 = 1u32;
pub const IEIFLAG_CACHE: u32 = 2u32;
pub const IEIFLAG_GLEAM: u32 = 16u32;
pub const IEIFLAG_NOBORDER: u32 = 256u32;
pub const IEIFLAG_NOSTAMP: u32 = 128u32;
pub const IEIFLAG_OFFLINE: u32 = 8u32;
pub const IEIFLAG_ORIGSIZE: u32 = 64u32;
pub const IEIFLAG_QUALITY: u32 = 512u32;
pub const IEIFLAG_REFRESH: u32 = 1024u32;
pub const IEIFLAG_SCREEN: u32 = 32u32;
pub const IEIT_PRIORITY_NORMAL: u32 = 268435456u32;
pub const IEI_PRIORITY_MAX: u32 = 2147483647u32;
pub const IEI_PRIORITY_MIN: u32 = 0u32;
pub const IEPDN_BINDINGUI: IEPDNFLAGS = 1i32;
pub const IESHORTCUT_BACKGROUNDTAB: IESHORTCUTFLAGS = 8i32;
pub const IESHORTCUT_FORCENAVIGATE: IESHORTCUTFLAGS = 4i32;
pub const IESHORTCUT_NEWBROWSER: IESHORTCUTFLAGS = 1i32;
pub const IESHORTCUT_OPENNEWTAB: IESHORTCUTFLAGS = 2i32;
pub const ILMM_IE4: u32 = 0u32;
pub const IMM_ACC_DOCKING_E_DOCKOCCUPIED: windows_core::HRESULT = 0x80270231_u32 as _;
pub const IMM_ACC_DOCKING_E_INSUFFICIENTHEIGHT: windows_core::HRESULT = 0x80270230_u32 as _;
pub const IMSC_E_SHELL_COMPONENT_STARTUP_FAILURE: windows_core::HRESULT = 0x80270233_u32 as _;
pub const INTERNET_MAX_PATH_LENGTH: u32 = 2048u32;
pub const INTERNET_MAX_SCHEME_LENGTH: u32 = 32u32;
pub const IRTIR_TASK_FINISHED: u32 = 4u32;
pub const IRTIR_TASK_NOT_RUNNING: u32 = 0u32;
pub const IRTIR_TASK_PENDING: u32 = 3u32;
pub const IRTIR_TASK_RUNNING: u32 = 1u32;
pub const IRTIR_TASK_SUSPENDED: u32 = 2u32;
pub const ISFBVIEWMODE_LARGEICONS: u32 = 2u32;
pub const ISFBVIEWMODE_LOGOS: u32 = 3u32;
pub const ISFBVIEWMODE_SMALLICONS: u32 = 1u32;
pub const ISFB_MASK_BKCOLOR: u32 = 2u32;
pub const ISFB_MASK_COLORS: u32 = 32u32;
pub const ISFB_MASK_IDLIST: u32 = 16u32;
pub const ISFB_MASK_SHELLFOLDER: u32 = 8u32;
pub const ISFB_MASK_STATE: u32 = 1u32;
pub const ISFB_MASK_VIEWMODE: u32 = 4u32;
pub const ISFB_STATE_ALLOWRENAME: u32 = 2u32;
pub const ISFB_STATE_BTNMINSIZE: u32 = 256u32;
pub const ISFB_STATE_CHANNELBAR: u32 = 16u32;
pub const ISFB_STATE_DEBOSSED: u32 = 1u32;
pub const ISFB_STATE_DEFAULT: u32 = 0u32;
pub const ISFB_STATE_FULLOPEN: u32 = 64u32;
pub const ISFB_STATE_NONAMESORT: u32 = 128u32;
pub const ISFB_STATE_NOSHOWTEXT: u32 = 4u32;
pub const ISFB_STATE_QLINKSMODE: u32 = 32u32;
pub const ISHCUTCMDID_COMMITHISTORY: i32 = 2i32;
pub const ISHCUTCMDID_DOWNLOADICON: i32 = 0i32;
pub const ISHCUTCMDID_INTSHORTCUTCREATE: i32 = 1i32;
pub const ISHCUTCMDID_SETUSERAWURL: i32 = 3i32;
pub const ISIOI_ICONFILE: u32 = 1u32;
pub const ISIOI_ICONINDEX: u32 = 2u32;
pub const IS_E_EXEC_FAILED: windows_core::HRESULT = 0x80042002_u32 as _;
pub const IS_FULLSCREEN: u32 = 2u32;
pub const IS_NORMAL: u32 = 1u32;
pub const IS_SPLIT: u32 = 4u32;
pub const ITSAT_DEFAULT_PRIORITY: u32 = 268435456u32;
pub const ITSAT_MAX_PRIORITY: u32 = 2147483647u32;
pub const ITSAT_MIN_PRIORITY: u32 = 0u32;
pub const ITSSFLAG_COMPLETE_ON_DESTROY: u32 = 0u32;
pub const ITSSFLAG_FLAGS_MASK: u32 = 3u32;
pub const ITSSFLAG_KILL_ON_DESTROY: u32 = 1u32;
pub const ITSS_THREAD_TIMEOUT_NO_CHANGE: u32 = 4294967294u32;
pub const IURL_INVOKECOMMAND_FL_ALLOW_UI: IURL_INVOKECOMMAND_FLAGS = 1i32;
pub const IURL_INVOKECOMMAND_FL_ASYNCOK: IURL_INVOKECOMMAND_FLAGS = 8i32;
pub const IURL_INVOKECOMMAND_FL_DDEWAIT: IURL_INVOKECOMMAND_FLAGS = 4i32;
pub const IURL_INVOKECOMMAND_FL_LOG_USAGE: IURL_INVOKECOMMAND_FLAGS = 16i32;
pub const IURL_INVOKECOMMAND_FL_USE_DEFAULT_VERB: IURL_INVOKECOMMAND_FLAGS = 2i32;
pub const IURL_SETURL_FL_GUESS_PROTOCOL: IURL_SETURL_FLAGS = 1i32;
pub const IURL_SETURL_FL_USE_DEFAULT_PROTOCOL: IURL_SETURL_FLAGS = 2i32;
pub const Identity_LocalUserProvider: windows_core::GUID = windows_core::GUID::from_u128(0xa198529b_730f_4089_b646_a12557f5665e);
pub const ItemCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xabbf5c45_5ccc_47b7_bb4e_87cb87bbd162);
pub const ItemIndex_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x92a053da_2969_4021_bf27_514cfc2e4a69);
pub const KDC_FREQUENT: KNOWNDESTCATEGORY = 1i32;
pub const KDC_RECENT: KNOWNDESTCATEGORY = 2i32;
pub const KFDF_LOCAL_REDIRECT_ONLY: _KF_DEFINITION_FLAGS = 2i32;
pub const KFDF_NO_REDIRECT_UI: _KF_DEFINITION_FLAGS = 64i32;
pub const KFDF_PRECREATE: _KF_DEFINITION_FLAGS = 8i32;
pub const KFDF_PUBLISHEXPANDEDPATH: _KF_DEFINITION_FLAGS = 32i32;
pub const KFDF_ROAMABLE: _KF_DEFINITION_FLAGS = 4i32;
pub const KFDF_STREAM: _KF_DEFINITION_FLAGS = 16i32;
pub const KF_CATEGORY_COMMON: KF_CATEGORY = 3i32;
pub const KF_CATEGORY_FIXED: KF_CATEGORY = 2i32;
pub const KF_CATEGORY_PERUSER: KF_CATEGORY = 4i32;
pub const KF_CATEGORY_VIRTUAL: KF_CATEGORY = 1i32;
pub const KF_FLAG_ALIAS_ONLY: KNOWN_FOLDER_FLAG = -2147483648i32;
pub const KF_FLAG_CREATE: KNOWN_FOLDER_FLAG = 32768i32;
pub const KF_FLAG_DEFAULT: KNOWN_FOLDER_FLAG = 0i32;
pub const KF_FLAG_DEFAULT_PATH: KNOWN_FOLDER_FLAG = 1024i32;
pub const KF_FLAG_DONT_UNEXPAND: KNOWN_FOLDER_FLAG = 8192i32;
pub const KF_FLAG_DONT_VERIFY: KNOWN_FOLDER_FLAG = 16384i32;
pub const KF_FLAG_FORCE_APPCONTAINER_REDIRECTION: KNOWN_FOLDER_FLAG = 131072i32;
pub const KF_FLAG_FORCE_APP_DATA_REDIRECTION: KNOWN_FOLDER_FLAG = 524288i32;
pub const KF_FLAG_FORCE_PACKAGE_REDIRECTION: KNOWN_FOLDER_FLAG = 131072i32;
pub const KF_FLAG_INIT: KNOWN_FOLDER_FLAG = 2048i32;
pub const KF_FLAG_NOT_PARENT_RELATIVE: KNOWN_FOLDER_FLAG = 512i32;
pub const KF_FLAG_NO_ALIAS: KNOWN_FOLDER_FLAG = 4096i32;
pub const KF_FLAG_NO_APPCONTAINER_REDIRECTION: KNOWN_FOLDER_FLAG = 65536i32;
pub const KF_FLAG_NO_PACKAGE_REDIRECTION: KNOWN_FOLDER_FLAG = 65536i32;
pub const KF_FLAG_RETURN_FILTER_REDIRECTION_TARGET: KNOWN_FOLDER_FLAG = 262144i32;
pub const KF_FLAG_SIMPLE_IDLIST: KNOWN_FOLDER_FLAG = 256i32;
pub const KF_REDIRECTION_CAPABILITIES_ALLOW_ALL: _KF_REDIRECTION_CAPABILITIES = 255i32;
pub const KF_REDIRECTION_CAPABILITIES_DENY_ALL: _KF_REDIRECTION_CAPABILITIES = 1048320i32;
pub const KF_REDIRECTION_CAPABILITIES_DENY_PERMISSIONS: _KF_REDIRECTION_CAPABILITIES = 1024i32;
pub const KF_REDIRECTION_CAPABILITIES_DENY_POLICY: _KF_REDIRECTION_CAPABILITIES = 512i32;
pub const KF_REDIRECTION_CAPABILITIES_DENY_POLICY_REDIRECTED: _KF_REDIRECTION_CAPABILITIES = 256i32;
pub const KF_REDIRECTION_CAPABILITIES_REDIRECTABLE: _KF_REDIRECTION_CAPABILITIES = 1i32;
pub const KF_REDIRECT_CHECK_ONLY: _KF_REDIRECT_FLAGS = 16i32;
pub const KF_REDIRECT_COPY_CONTENTS: _KF_REDIRECT_FLAGS = 512i32;
pub const KF_REDIRECT_COPY_SOURCE_DACL: _KF_REDIRECT_FLAGS = 2i32;
pub const KF_REDIRECT_DEL_SOURCE_CONTENTS: _KF_REDIRECT_FLAGS = 1024i32;
pub const KF_REDIRECT_EXCLUDE_ALL_KNOWN_SUBFOLDERS: _KF_REDIRECT_FLAGS = 2048i32;
pub const KF_REDIRECT_OWNER_USER: _KF_REDIRECT_FLAGS = 4i32;
pub const KF_REDIRECT_PIN: _KF_REDIRECT_FLAGS = 128i32;
pub const KF_REDIRECT_SET_OWNER_EXPLICIT: _KF_REDIRECT_FLAGS = 8i32;
pub const KF_REDIRECT_UNPIN: _KF_REDIRECT_FLAGS = 64i32;
pub const KF_REDIRECT_USER_EXCLUSIVE: _KF_REDIRECT_FLAGS = 1i32;
pub const KF_REDIRECT_WITH_UI: _KF_REDIRECT_FLAGS = 32i32;
pub const LFF_ALLITEMS: LIBRARYFOLDERFILTER = 3i32;
pub const LFF_FORCEFILESYSTEM: LIBRARYFOLDERFILTER = 1i32;
pub const LFF_STORAGEITEMS: LIBRARYFOLDERFILTER = 2i32;
pub const LIBRARY_E_NO_ACCESSIBLE_LOCATION: windows_core::HRESULT = 0x80270201_u32 as _;
pub const LIBRARY_E_NO_SAVE_LOCATION: windows_core::HRESULT = 0x80270200_u32 as _;
pub const LINK_E_DELETE: windows_core::HRESULT = 0x80270103_u32 as _;
pub const LMD_ALLOWUNINDEXABLENETWORKLOCATIONS: LIBRARYMANAGEDIALOGOPTIONS = 1i32;
pub const LMD_DEFAULT: LIBRARYMANAGEDIALOGOPTIONS = 0i32;
pub const LOF_DEFAULT: LIBRARYOPTIONFLAGS = 0i32;
pub const LOF_MASK_ALL: LIBRARYOPTIONFLAGS = 1i32;
pub const LOF_PINNEDTONAVPANE: LIBRARYOPTIONFLAGS = 1i32;
pub const LSF_FAILIFTHERE: LIBRARYSAVEFLAGS = 0i32;
pub const LSF_MAKEUNIQUENAME: LIBRARYSAVEFLAGS = 2i32;
pub const LSF_OVERRIDEEXISTING: LIBRARYSAVEFLAGS = 1i32;
pub const MAV_APP_VISIBLE: MONITOR_APP_VISIBILITY = 2i32;
pub const MAV_NO_APP_VISIBLE: MONITOR_APP_VISIBILITY = 1i32;
pub const MAV_UNKNOWN: MONITOR_APP_VISIBILITY = 0i32;
pub const MAXFILELEN: u32 = 13u32;
pub const MAX_COLUMN_DESC_LEN: u32 = 128u32;
pub const MAX_COLUMN_NAME_LEN: u32 = 80u32;
pub const MAX_SYNCMGRHANDLERNAME: u32 = 32u32;
pub const MAX_SYNCMGRITEMNAME: u32 = 128u32;
pub const MAX_SYNCMGR_ID: u32 = 64u32;
pub const MAX_SYNCMGR_NAME: u32 = 128u32;
pub const MAX_SYNCMGR_PROGRESSTEXT: u32 = 260u32;
pub const MBHANDCID_PIDLSELECT: MENUBANDHANDLERCID = 0i32;
pub const MIMEASSOCDLG_FL_REGISTER_ASSOC: MIMEASSOCIATIONDIALOG_IN_FLAGS = 1i32;
pub const MM_ADDSEPARATOR: MM_FLAGS = 1u32;
pub const MM_DONTREMOVESEPS: MM_FLAGS = 4u32;
pub const MM_SUBMENUSHAVEIDS: MM_FLAGS = 2u32;
pub const MPOS_CANCELLEVEL: MENUPOPUPSELECT = 2i32;
pub const MPOS_CHILDTRACKING: MENUPOPUPSELECT = 5i32;
pub const MPOS_EXECUTE: MENUPOPUPSELECT = 0i32;
pub const MPOS_FULLCANCEL: MENUPOPUPSELECT = 1i32;
pub const MPOS_SELECTLEFT: MENUPOPUPSELECT = 3i32;
pub const MPOS_SELECTRIGHT: MENUPOPUPSELECT = 4i32;
pub const MPPF_ALIGN_LEFT: MENUPOPUPPOPUPFLAGS = 33554432i32;
pub const MPPF_ALIGN_RIGHT: MENUPOPUPPOPUPFLAGS = 67108864i32;
pub const MPPF_BOTTOM: MENUPOPUPPOPUPFLAGS = -2147483648i32;
pub const MPPF_FINALSELECT: MENUPOPUPPOPUPFLAGS = 128i32;
pub const MPPF_FORCEZORDER: MENUPOPUPPOPUPFLAGS = 64i32;
pub const MPPF_INITIALSELECT: MENUPOPUPPOPUPFLAGS = 2i32;
pub const MPPF_KEYBOARD: MENUPOPUPPOPUPFLAGS = 16i32;
pub const MPPF_LEFT: MENUPOPUPPOPUPFLAGS = 1073741824i32;
pub const MPPF_NOANIMATE: MENUPOPUPPOPUPFLAGS = 4i32;
pub const MPPF_POS_MASK: MENUPOPUPPOPUPFLAGS = -536870912i32;
pub const MPPF_REPOSITION: MENUPOPUPPOPUPFLAGS = 32i32;
pub const MPPF_RIGHT: MENUPOPUPPOPUPFLAGS = 1610612736i32;
pub const MPPF_SETFOCUS: MENUPOPUPPOPUPFLAGS = 1i32;
pub const MPPF_TOP: MENUPOPUPPOPUPFLAGS = 536870912i32;
pub const MUS_COMPLETE: MERGE_UPDATE_STATUS = 0i32;
pub const MUS_FAILED: MERGE_UPDATE_STATUS = 2i32;
pub const MUS_USERINPUTNEEDED: MERGE_UPDATE_STATUS = 1i32;
pub const NCM_DISPLAYERRORTIP: u32 = 1028u32;
pub const NCM_GETADDRESS: u32 = 1025u32;
pub const NCM_GETALLOWTYPE: u32 = 1027u32;
pub const NCM_SETALLOWTYPE: u32 = 1026u32;
pub const NDO_LANDSCAPE: NATIVE_DISPLAY_ORIENTATION = 0i32;
pub const NDO_PORTRAIT: NATIVE_DISPLAY_ORIENTATION = 1i32;
pub const NETCACHE_E_NEGATIVE_CACHE: windows_core::HRESULT = 0x80270100_u32 as _;
pub const NIF_GUID: NOTIFY_ICON_DATA_FLAGS = 32u32;
pub const NIF_ICON: NOTIFY_ICON_DATA_FLAGS = 2u32;
pub const NIF_INFO: NOTIFY_ICON_DATA_FLAGS = 16u32;
pub const NIF_MESSAGE: NOTIFY_ICON_DATA_FLAGS = 1u32;
pub const NIF_REALTIME: NOTIFY_ICON_DATA_FLAGS = 64u32;
pub const NIF_SHOWTIP: NOTIFY_ICON_DATA_FLAGS = 128u32;
pub const NIF_STATE: NOTIFY_ICON_DATA_FLAGS = 8u32;
pub const NIF_TIP: NOTIFY_ICON_DATA_FLAGS = 4u32;
pub const NIIF_ERROR: NOTIFY_ICON_INFOTIP_FLAGS = 3u32;
pub const NIIF_ICON_MASK: NOTIFY_ICON_INFOTIP_FLAGS = 15u32;
pub const NIIF_INFO: NOTIFY_ICON_INFOTIP_FLAGS = 1u32;
pub const NIIF_LARGE_ICON: NOTIFY_ICON_INFOTIP_FLAGS = 32u32;
pub const NIIF_NONE: NOTIFY_ICON_INFOTIP_FLAGS = 0u32;
pub const NIIF_NOSOUND: NOTIFY_ICON_INFOTIP_FLAGS = 16u32;
pub const NIIF_RESPECT_QUIET_TIME: NOTIFY_ICON_INFOTIP_FLAGS = 128u32;
pub const NIIF_USER: NOTIFY_ICON_INFOTIP_FLAGS = 4u32;
pub const NIIF_WARNING: NOTIFY_ICON_INFOTIP_FLAGS = 2u32;
pub const NIM_ADD: NOTIFY_ICON_MESSAGE = 0u32;
pub const NIM_DELETE: NOTIFY_ICON_MESSAGE = 2u32;
pub const NIM_MODIFY: NOTIFY_ICON_MESSAGE = 1u32;
pub const NIM_SETFOCUS: NOTIFY_ICON_MESSAGE = 3u32;
pub const NIM_SETVERSION: NOTIFY_ICON_MESSAGE = 4u32;
pub const NINF_KEY: u32 = 1u32;
pub const NIN_BALLOONHIDE: u32 = 1027u32;
pub const NIN_BALLOONSHOW: u32 = 1026u32;
pub const NIN_BALLOONTIMEOUT: u32 = 1028u32;
pub const NIN_BALLOONUSERCLICK: u32 = 1029u32;
pub const NIN_POPUPCLOSE: u32 = 1031u32;
pub const NIN_POPUPOPEN: u32 = 1030u32;
pub const NIN_SELECT: u32 = 1024u32;
pub const NIS_HIDDEN: NOTIFY_ICON_STATE = 1u32;
pub const NIS_SHAREDICON: NOTIFY_ICON_STATE = 2u32;
pub const NMCII_FOLDERS: _NMCII_FLAGS = 2i32;
pub const NMCII_ITEMS: _NMCII_FLAGS = 1i32;
pub const NMCII_NONE: _NMCII_FLAGS = 0i32;
pub const NMCSAEI_EDIT: _NMCSAEI_FLAGS = 1i32;
pub const NMCSAEI_SELECT: _NMCSAEI_FLAGS = 0i32;
pub const NOTIFYICON_VERSION: u32 = 3u32;
pub const NOTIFYICON_VERSION_4: u32 = 4u32;
pub const NSTCDHPOS_ONTOP: i32 = -1i32;
pub const NSTCECT_BUTTON: _NSTCECLICKTYPE = 3i32;
pub const NSTCECT_DBLCLICK: _NSTCECLICKTYPE = 4i32;
pub const NSTCECT_LBUTTON: _NSTCECLICKTYPE = 1i32;
pub const NSTCECT_MBUTTON: _NSTCECLICKTYPE = 2i32;
pub const NSTCECT_RBUTTON: _NSTCECLICKTYPE = 3i32;
pub const NSTCEHT_NOWHERE: _NSTCEHITTEST = 1i32;
pub const NSTCEHT_ONITEM: _NSTCEHITTEST = 70i32;
pub const NSTCEHT_ONITEMBUTTON: _NSTCEHITTEST = 16i32;
pub const NSTCEHT_ONITEMICON: _NSTCEHITTEST = 2i32;
pub const NSTCEHT_ONITEMINDENT: _NSTCEHITTEST = 8i32;
pub const NSTCEHT_ONITEMLABEL: _NSTCEHITTEST = 4i32;
pub const NSTCEHT_ONITEMRIGHT: _NSTCEHITTEST = 32i32;
pub const NSTCEHT_ONITEMSTATEICON: _NSTCEHITTEST = 64i32;
pub const NSTCEHT_ONITEMTABBUTTON: _NSTCEHITTEST = 4096i32;
pub const NSTCFC_DELAY_REGISTER_NOTIFY: NSTCFOLDERCAPABILITIES = 2i32;
pub const NSTCFC_NONE: NSTCFOLDERCAPABILITIES = 0i32;
pub const NSTCFC_PINNEDITEMFILTERING: NSTCFOLDERCAPABILITIES = 1i32;
pub const NSTCGNI_CHILD: NSTCGNI = 5i32;
pub const NSTCGNI_FIRSTVISIBLE: NSTCGNI = 6i32;
pub const NSTCGNI_LASTVISIBLE: NSTCGNI = 7i32;
pub const NSTCGNI_NEXT: NSTCGNI = 0i32;
pub const NSTCGNI_NEXTVISIBLE: NSTCGNI = 1i32;
pub const NSTCGNI_PARENT: NSTCGNI = 4i32;
pub const NSTCGNI_PREV: NSTCGNI = 2i32;
pub const NSTCGNI_PREVVISIBLE: NSTCGNI = 3i32;
pub const NSTCIS_BOLD: _NSTCITEMSTATE = 4i32;
pub const NSTCIS_DISABLED: _NSTCITEMSTATE = 8i32;
pub const NSTCIS_EXPANDED: _NSTCITEMSTATE = 2i32;
pub const NSTCIS_NONE: _NSTCITEMSTATE = 0i32;
pub const NSTCIS_SELECTED: _NSTCITEMSTATE = 1i32;
pub const NSTCIS_SELECTEDNOEXPAND: _NSTCITEMSTATE = 16i32;
pub const NSTCRS_EXPANDED: _NSTCROOTSTYLE = 2i32;
pub const NSTCRS_HIDDEN: _NSTCROOTSTYLE = 1i32;
pub const NSTCRS_VISIBLE: _NSTCROOTSTYLE = 0i32;
pub const NSTCS2_DEFAULT: NSTCSTYLE2 = 0i32;
pub const NSTCS2_DISPLAYPADDING: NSTCSTYLE2 = 4i32;
pub const NSTCS2_DISPLAYPINNEDONLY: NSTCSTYLE2 = 8i32;
pub const NSTCS2_INTERRUPTNOTIFICATIONS: NSTCSTYLE2 = 1i32;
pub const NSTCS2_SHOWNULLSPACEMENU: NSTCSTYLE2 = 2i32;
pub const NSTCS_ALLOWJUNCTIONS: _NSTCSTYLE = 268435456i32;
pub const NSTCS_AUTOHSCROLL: _NSTCSTYLE = 1048576i32;
pub const NSTCS_BORDER: _NSTCSTYLE = 32768i32;
pub const NSTCS_CHECKBOXES: _NSTCSTYLE = 8388608i32;
pub const NSTCS_DIMMEDCHECKBOXES: _NSTCSTYLE = 67108864i32;
pub const NSTCS_DISABLEDRAGDROP: _NSTCSTYLE = 4096i32;
pub const NSTCS_EMPTYTEXT: _NSTCSTYLE = 4194304i32;
pub const NSTCS_EVENHEIGHT: _NSTCSTYLE = 1024i32;
pub const NSTCS_EXCLUSIONCHECKBOXES: _NSTCSTYLE = 33554432i32;
pub const NSTCS_FADEINOUTEXPANDOS: _NSTCSTYLE = 2097152i32;
pub const NSTCS_FAVORITESMODE: _NSTCSTYLE = 524288i32;
pub const NSTCS_FULLROWSELECT: _NSTCSTYLE = 8i32;
pub const NSTCS_HASEXPANDOS: _NSTCSTYLE = 1i32;
pub const NSTCS_HASLINES: _NSTCSTYLE = 2i32;
pub const NSTCS_HORIZONTALSCROLL: _NSTCSTYLE = 32i32;
pub const NSTCS_NOEDITLABELS: _NSTCSTYLE = 65536i32;
pub const NSTCS_NOINDENTCHECKS: _NSTCSTYLE = 134217728i32;
pub const NSTCS_NOINFOTIP: _NSTCSTYLE = 512i32;
pub const NSTCS_NOORDERSTREAM: _NSTCSTYLE = 8192i32;
pub const NSTCS_NOREPLACEOPEN: _NSTCSTYLE = 2048i32;
pub const NSTCS_PARTIALCHECKBOXES: _NSTCSTYLE = 16777216i32;
pub const NSTCS_RICHTOOLTIP: _NSTCSTYLE = 16384i32;
pub const NSTCS_ROOTHASEXPANDO: _NSTCSTYLE = 64i32;
pub const NSTCS_SHOWDELETEBUTTON: _NSTCSTYLE = 1073741824i32;
pub const NSTCS_SHOWREFRESHBUTTON: _NSTCSTYLE = -2147483648i32;
pub const NSTCS_SHOWSELECTIONALWAYS: _NSTCSTYLE = 128i32;
pub const NSTCS_SHOWTABSBUTTON: _NSTCSTYLE = 536870912i32;
pub const NSTCS_SINGLECLICKEXPAND: _NSTCSTYLE = 4i32;
pub const NSTCS_SPRINGEXPAND: _NSTCSTYLE = 16i32;
pub const NSTCS_TABSTOP: _NSTCSTYLE = 131072i32;
pub const NSWF_ACCUMULATE_FOLDERS: NAMESPACEWALKFLAG = 2048i32;
pub const NSWF_ANY_IMPLIES_ALL: NAMESPACEWALKFLAG = 32768i32;
pub const NSWF_ASYNC: NAMESPACEWALKFLAG = 512i32;
pub const NSWF_DEFAULT: NAMESPACEWALKFLAG = 0i32;
pub const NSWF_DONT_ACCUMULATE_RESULT: NAMESPACEWALKFLAG = 8i32;
pub const NSWF_DONT_RESOLVE_LINKS: NAMESPACEWALKFLAG = 1024i32;
pub const NSWF_DONT_SORT: NAMESPACEWALKFLAG = 4096i32;
pub const NSWF_DONT_TRAVERSE_LINKS: NAMESPACEWALKFLAG = 4i32;
pub const NSWF_DONT_TRAVERSE_STREAM_JUNCTIONS: NAMESPACEWALKFLAG = 16384i32;
pub const NSWF_FILESYSTEM_ONLY: NAMESPACEWALKFLAG = 32i32;
pub const NSWF_FLAG_VIEWORDER: NAMESPACEWALKFLAG = 128i32;
pub const NSWF_IGNORE_AUTOPLAY_HIDA: NAMESPACEWALKFLAG = 256i32;
pub const NSWF_NONE_IMPLIES_ALL: NAMESPACEWALKFLAG = 1i32;
pub const NSWF_ONE_IMPLIES_ALL: NAMESPACEWALKFLAG = 2i32;
pub const NSWF_SHOW_PROGRESS: NAMESPACEWALKFLAG = 64i32;
pub const NSWF_TRAVERSE_STREAM_JUNCTIONS: NAMESPACEWALKFLAG = 16i32;
pub const NSWF_USE_TRANSFER_MEDIUM: NAMESPACEWALKFLAG = 8192i32;
pub const NTSCS2_NEVERINSERTNONENUMERATED: NSTCSTYLE2 = 32i32;
pub const NTSCS2_NOSINGLETONAUTOEXPAND: NSTCSTYLE2 = 16i32;
pub const NT_CONSOLE_PROPS_SIG: u32 = 2684354562u32;
pub const NT_FE_CONSOLE_PROPS_SIG: u32 = 2684354564u32;
pub const NUM_POINTS: u32 = 3u32;
pub const NWMF_FIRST: NWMF = 4i32;
pub const NWMF_FORCETAB: NWMF = 131072i32;
pub const NWMF_FORCEWINDOW: NWMF = 65536i32;
pub const NWMF_FROMDIALOGCHILD: NWMF = 64i32;
pub const NWMF_HTMLDIALOG: NWMF = 32i32;
pub const NWMF_INACTIVETAB: NWMF = 1048576i32;
pub const NWMF_OVERRIDEKEY: NWMF = 8i32;
pub const NWMF_SHOWHELP: NWMF = 16i32;
pub const NWMF_SUGGESTTAB: NWMF = 524288i32;
pub const NWMF_SUGGESTWINDOW: NWMF = 262144i32;
pub const NWMF_UNLOADING: NWMF = 1i32;
pub const NWMF_USERALLOWED: NWMF = 256i32;
pub const NWMF_USERINITED: NWMF = 2i32;
pub const NWMF_USERREQUESTED: NWMF = 128i32;
pub const OAIF_ALLOW_REGISTRATION: OPEN_AS_INFO_FLAGS = 1i32;
pub const OAIF_EXEC: OPEN_AS_INFO_FLAGS = 4i32;
pub const OAIF_FILE_IS_URI: OPEN_AS_INFO_FLAGS = 128i32;
pub const OAIF_FORCE_REGISTRATION: OPEN_AS_INFO_FLAGS = 8i32;
pub const OAIF_HIDE_REGISTRATION: OPEN_AS_INFO_FLAGS = 32i32;
pub const OAIF_REGISTER_EXT: OPEN_AS_INFO_FLAGS = 2i32;
pub const OAIF_URL_PROTOCOL: OPEN_AS_INFO_FLAGS = 64i32;
pub const OFASI_EDIT: u32 = 1u32;
pub const OFASI_OPENDESKTOP: u32 = 2u32;
pub const OFFLINE_STATUS_INCOMPLETE: u32 = 4u32;
pub const OFFLINE_STATUS_LOCAL: u32 = 1u32;
pub const OFFLINE_STATUS_REMOTE: u32 = 2u32;
pub const OFS_DIRTYCACHE: OfflineFolderStatus = 3i32;
pub const OFS_INACTIVE: OfflineFolderStatus = -1i32;
pub const OFS_OFFLINE: OfflineFolderStatus = 1i32;
pub const OFS_ONLINE: OfflineFolderStatus = 0i32;
pub const OFS_SERVERBACK: OfflineFolderStatus = 2i32;
pub const OF_CAP_CANCLOSE: u32 = 2u32;
pub const OF_CAP_CANSWITCHTO: u32 = 1u32;
pub const OI_ASYNC: u32 = 4294962926u32;
pub const OI_DEFAULT: u32 = 0u32;
pub const OPENPROPS_INHIBITPIF: u32 = 32768u32;
pub const OPENPROPS_NONE: u32 = 0u32;
pub const OPPROGDLG_ALLOWUNDO: _OPPROGDLGF = 256i32;
pub const OPPROGDLG_DEFAULT: _OPPROGDLGF = 0i32;
pub const OPPROGDLG_DONTDISPLAYDESTPATH: _OPPROGDLGF = 1024i32;
pub const OPPROGDLG_DONTDISPLAYLOCATIONS: _OPPROGDLGF = 4096i32;
pub const OPPROGDLG_DONTDISPLAYSOURCEPATH: _OPPROGDLGF = 512i32;
pub const OPPROGDLG_ENABLEPAUSE: _OPPROGDLGF = 128i32;
pub const OPPROGDLG_NOMULTIDAYESTIMATES: _OPPROGDLGF = 2048i32;
pub const OS_ADVSERVER: OS = 22u32;
pub const OS_ANYSERVER: OS = 29u32;
pub const OS_APPLIANCE: OS = 36u32;
pub const OS_DATACENTER: OS = 21u32;
pub const OS_DOMAINMEMBER: OS = 28u32;
pub const OS_EMBEDDED: OS = 13u32;
pub const OS_FASTUSERSWITCHING: OS = 26u32;
pub const OS_HOME: OS = 19u32;
pub const OS_MEDIACENTER: OS = 35u32;
pub const OS_MEORGREATER: OS = 17u32;
pub const OS_NT: OS = 1u32;
pub const OS_NT4ORGREATER: OS = 3u32;
pub const OS_PERSONALTERMINALSERVER: OS = 25u32;
pub const OS_PROFESSIONAL: OS = 20u32;
pub const OS_SERVER: OS = 23u32;
pub const OS_SERVERADMINUI: OS = 34u32;
pub const OS_SMALLBUSINESSSERVER: OS = 32u32;
pub const OS_TABLETPC: OS = 33u32;
pub const OS_TERMINALCLIENT: OS = 14u32;
pub const OS_TERMINALREMOTEADMIN: OS = 15u32;
pub const OS_TERMINALSERVER: OS = 24u32;
pub const OS_WEBSERVER: OS = 31u32;
pub const OS_WELCOMELOGONUI: OS = 27u32;
pub const OS_WIN2000ADVSERVER: OS = 10u32;
pub const OS_WIN2000DATACENTER: OS = 11u32;
pub const OS_WIN2000ORGREATER: OS = 7u32;
pub const OS_WIN2000PRO: OS = 8u32;
pub const OS_WIN2000SERVER: OS = 9u32;
pub const OS_WIN2000TERMINAL: OS = 12u32;
pub const OS_WIN95ORGREATER: OS = 2u32;
pub const OS_WIN95_GOLD: OS = 16u32;
pub const OS_WIN98ORGREATER: OS = 5u32;
pub const OS_WIN98_GOLD: OS = 6u32;
pub const OS_WINDOWS: OS = 0u32;
pub const OS_WOW6432: OS = 30u32;
pub const OS_XPORGREATER: OS = 18u32;
pub const PAI_ASSIGNEDTIME: PUBAPPINFOFLAGS = 2i32;
pub const PAI_EXPIRETIME: PUBAPPINFOFLAGS = 16i32;
pub const PAI_PUBLISHEDTIME: PUBAPPINFOFLAGS = 4i32;
pub const PAI_SCHEDULEDTIME: PUBAPPINFOFLAGS = 8i32;
pub const PAI_SOURCE: PUBAPPINFOFLAGS = 1i32;
pub const PANE_NAVIGATION: u32 = 5u32;
pub const PANE_NONE: u32 = 4294967295u32;
pub const PANE_OFFLINE: u32 = 2u32;
pub const PANE_PRINTER: u32 = 3u32;
pub const PANE_PRIVACY: u32 = 7u32;
pub const PANE_PROGRESS: u32 = 6u32;
pub const PANE_SSL: u32 = 4u32;
pub const PANE_ZONE: u32 = 1u32;
pub const PATHCCH_ALLOW_LONG_PATHS: PATHCCH_OPTIONS = 1u32;
pub const PATHCCH_CANONICALIZE_SLASHES: PATHCCH_OPTIONS = 64u32;
pub const PATHCCH_DO_NOT_NORMALIZE_SEGMENTS: PATHCCH_OPTIONS = 8u32;
pub const PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH: PATHCCH_OPTIONS = 16u32;
pub const PATHCCH_ENSURE_TRAILING_SLASH: PATHCCH_OPTIONS = 32u32;
pub const PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS: PATHCCH_OPTIONS = 4u32;
pub const PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS: PATHCCH_OPTIONS = 2u32;
pub const PATHCCH_MAX_CCH: u32 = 32768u32;
pub const PATHCCH_NONE: PATHCCH_OPTIONS = 0u32;
pub const PCS_FATAL: PCS_RET = 2147483648u32;
pub const PCS_PATHTOOLONG: PCS_RET = 8u32;
pub const PCS_REMOVEDCHAR: PCS_RET = 2u32;
pub const PCS_REPLACEDCHAR: PCS_RET = 1u32;
pub const PCS_TRUNCATED: PCS_RET = 4u32;
pub const PDM_DEFAULT: _PDMODE = 0i32;
pub const PDM_ERRORSBLOCKING: _PDMODE = 8i32;
pub const PDM_INDETERMINATE: _PDMODE = 16i32;
pub const PDM_PREFLIGHT: _PDMODE = 2i32;
pub const PDM_RUN: _PDMODE = 1i32;
pub const PDM_UNDOING: _PDMODE = 4i32;
pub const PDTIMER_PAUSE: u32 = 2u32;
pub const PDTIMER_RESET: u32 = 1u32;
pub const PDTIMER_RESUME: u32 = 3u32;
pub const PES_RUNNING: PACKAGE_EXECUTION_STATE = 1i32;
pub const PES_SUSPENDED: PACKAGE_EXECUTION_STATE = 3i32;
pub const PES_SUSPENDING: PACKAGE_EXECUTION_STATE = 2i32;
pub const PES_TERMINATED: PACKAGE_EXECUTION_STATE = 4i32;
pub const PES_UNKNOWN: PACKAGE_EXECUTION_STATE = 0i32;
pub const PIDASI_AVG_DATA_RATE: u32 = 4u32;
pub const PIDASI_CHANNEL_COUNT: u32 = 7u32;
pub const PIDASI_COMPRESSION: u32 = 10u32;
pub const PIDASI_FORMAT: u32 = 2u32;
pub const PIDASI_SAMPLE_RATE: u32 = 5u32;
pub const PIDASI_SAMPLE_SIZE: u32 = 6u32;
pub const PIDASI_STREAM_NAME: u32 = 9u32;
pub const PIDASI_STREAM_NUMBER: u32 = 8u32;
pub const PIDASI_TIMELENGTH: u32 = 3u32;
pub const PIDDRSI_DESCRIPTION: u32 = 3u32;
pub const PIDDRSI_PLAYCOUNT: u32 = 4u32;
pub const PIDDRSI_PLAYEXPIRES: u32 = 6u32;
pub const PIDDRSI_PLAYSTARTS: u32 = 5u32;
pub const PIDDRSI_PROTECTED: u32 = 2u32;
pub const PIDISF_CACHEDSTICKY: PIDISF_FLAGS = 2i32;
pub const PIDISF_CACHEIMAGES: PIDISF_FLAGS = 16i32;
pub const PIDISF_FOLLOWALLLINKS: PIDISF_FLAGS = 32i32;
pub const PIDISF_RECENTLYCHANGED: PIDISF_FLAGS = 1i32;
pub const PIDISM_DONTWATCH: PIDISM_OPTIONS = 2i32;
pub const PIDISM_GLOBAL: PIDISM_OPTIONS = 0i32;
pub const PIDISM_WATCH: PIDISM_OPTIONS = 1i32;
pub const PIDISR_NEEDS_ADD: PIDISR_INFO = 1i32;
pub const PIDISR_NEEDS_DELETE: PIDISR_INFO = 3i32;
pub const PIDISR_NEEDS_UPDATE: PIDISR_INFO = 2i32;
pub const PIDISR_UP_TO_DATE: PIDISR_INFO = 0i32;
pub const PIDSI_ALBUM: u32 = 4u32;
pub const PIDSI_ARTIST: u32 = 2u32;
pub const PIDSI_COMMENT: u32 = 6u32;
pub const PIDSI_GENRE: u32 = 11u32;
pub const PIDSI_LYRICS: u32 = 12u32;
pub const PIDSI_SONGTITLE: u32 = 3u32;
pub const PIDSI_TRACK: u32 = 7u32;
pub const PIDSI_YEAR: u32 = 5u32;
pub const PIDVSI_COMPRESSION: u32 = 10u32;
pub const PIDVSI_DATA_RATE: u32 = 8u32;
pub const PIDVSI_FRAME_COUNT: u32 = 5u32;
pub const PIDVSI_FRAME_HEIGHT: u32 = 4u32;
pub const PIDVSI_FRAME_RATE: u32 = 6u32;
pub const PIDVSI_FRAME_WIDTH: u32 = 3u32;
pub const PIDVSI_SAMPLE_SIZE: u32 = 9u32;
pub const PIDVSI_STREAM_NAME: u32 = 2u32;
pub const PIDVSI_STREAM_NUMBER: u32 = 11u32;
pub const PIDVSI_TIMELENGTH: u32 = 7u32;
pub const PID_COMPUTERNAME: u32 = 5u32;
pub const PID_CONTROLPANEL_CATEGORY: u32 = 2u32;
pub const PID_DESCRIPTIONID: u32 = 2u32;
pub const PID_DISPLACED_DATE: u32 = 3u32;
pub const PID_DISPLACED_FROM: u32 = 2u32;
pub const PID_DISPLAY_PROPERTIES: u32 = 0u32;
pub const PID_FINDDATA: u32 = 0u32;
pub const PID_HTMLINFOTIPFILE: u32 = 5u32;
pub const PID_INTROTEXT: u32 = 1u32;
pub const PID_INTSITE_AUTHOR: PID_INTSITE = 3i32;
pub const PID_INTSITE_CODEPAGE: PID_INTSITE = 18i32;
pub const PID_INTSITE_COMMENT: PID_INTSITE = 8i32;
pub const PID_INTSITE_CONTENTCODE: PID_INTSITE = 11i32;
pub const PID_INTSITE_CONTENTLEN: PID_INTSITE = 10i32;
pub const PID_INTSITE_DESCRIPTION: PID_INTSITE = 7i32;
pub const PID_INTSITE_FLAGS: PID_INTSITE = 9i32;
pub const PID_INTSITE_ICONFILE: PID_INTSITE = 21i32;
pub const PID_INTSITE_ICONINDEX: PID_INTSITE = 20i32;
pub const PID_INTSITE_LASTMOD: PID_INTSITE = 5i32;
pub const PID_INTSITE_LASTVISIT: PID_INTSITE = 4i32;
pub const PID_INTSITE_RECURSE: PID_INTSITE = 12i32;
pub const PID_INTSITE_ROAMED: PID_INTSITE = 34i32;
pub const PID_INTSITE_SUBSCRIPTION: PID_INTSITE = 14i32;
pub const PID_INTSITE_TITLE: PID_INTSITE = 16i32;
pub const PID_INTSITE_TRACKING: PID_INTSITE = 19i32;
pub const PID_INTSITE_URL: PID_INTSITE = 15i32;
pub const PID_INTSITE_VISITCOUNT: PID_INTSITE = 6i32;
pub const PID_INTSITE_WATCH: PID_INTSITE = 13i32;
pub const PID_INTSITE_WHATSNEW: PID_INTSITE = 2i32;
pub const PID_IS_AUTHOR: PID_IS = 11i32;
pub const PID_IS_COMMENT: PID_IS = 13i32;
pub const PID_IS_DESCRIPTION: PID_IS = 12i32;
pub const PID_IS_HOTKEY: PID_IS = 6i32;
pub const PID_IS_ICONFILE: PID_IS = 9i32;
pub const PID_IS_ICONINDEX: PID_IS = 8i32;
pub const PID_IS_NAME: PID_IS = 4i32;
pub const PID_IS_ROAMED: PID_IS = 15i32;
pub const PID_IS_SHOWCMD: PID_IS = 7i32;
pub const PID_IS_URL: PID_IS = 2i32;
pub const PID_IS_WHATSNEW: PID_IS = 10i32;
pub const PID_IS_WORKINGDIR: PID_IS = 5i32;
pub const PID_LINK_TARGET: u32 = 2u32;
pub const PID_LINK_TARGET_TYPE: u32 = 3u32;
pub const PID_MISC_ACCESSCOUNT: u32 = 3u32;
pub const PID_MISC_OWNER: u32 = 4u32;
pub const PID_MISC_PICS: u32 = 6u32;
pub const PID_MISC_STATUS: u32 = 2u32;
pub const PID_NETRESOURCE: u32 = 1u32;
pub const PID_NETWORKLOCATION: u32 = 4u32;
pub const PID_QUERY_RANK: u32 = 2u32;
pub const PID_SHARE_CSC_STATUS: u32 = 2u32;
pub const PID_SYNC_COPY_IN: u32 = 2u32;
pub const PID_VOLUME_CAPACITY: u32 = 3u32;
pub const PID_VOLUME_FILESYSTEM: u32 = 4u32;
pub const PID_VOLUME_FREE: u32 = 2u32;
pub const PID_WHICHFOLDER: u32 = 3u32;
pub const PIFDEFFILESIZE: u32 = 80u32;
pub const PIFDEFPATHSIZE: u32 = 64u32;
pub const PIFMAXFILEPATH: u32 = 260u32;
pub const PIFNAMESIZE: u32 = 30u32;
pub const PIFPARAMSSIZE: u32 = 64u32;
pub const PIFSHDATASIZE: u32 = 64u32;
pub const PIFSHPROGSIZE: u32 = 64u32;
pub const PIFSTARTLOCSIZE: u32 = 63u32;
pub const PLATFORM_BROWSERONLY: u32 = 1u32;
pub const PLATFORM_IE3: u32 = 1u32;
pub const PLATFORM_INTEGRATED: u32 = 2u32;
pub const PLATFORM_UNKNOWN: u32 = 0u32;
pub const PMSF_DONT_STRIP_SPACES: u32 = 65536u32;
pub const PMSF_MULTIPLE: u32 = 1u32;
pub const PMSF_NORMAL: u32 = 0u32;
pub const PO_DELETE: u32 = 19u32;
pub const PO_PORTCHANGE: u32 = 32u32;
pub const PO_RENAME: u32 = 20u32;
pub const PO_REN_PORT: u32 = 52u32;
pub const PPCF_ADDARGUMENTS: u32 = 3u32;
pub const PPCF_ADDQUOTES: u32 = 1u32;
pub const PPCF_FORCEQUALIFY: u32 = 64u32;
pub const PPCF_LONGESTPOSSIBLE: u32 = 128u32;
pub const PPCF_NODIRECTORIES: u32 = 16u32;
pub const PRF_DONTFINDLNK: PRF_FLAGS = 8i32;
pub const PRF_FIRSTDIRDEF: PRF_FLAGS = 4i32;
pub const PRF_REQUIREABSOLUTE: PRF_FLAGS = 16i32;
pub const PRF_TRYPROGRAMEXTENSIONS: PRF_FLAGS = 3i32;
pub const PRF_VERIFYEXISTS: PRF_FLAGS = 1i32;
pub const PRINTACTION_DOCUMENTDEFAULTS: u32 = 6u32;
pub const PRINTACTION_NETINSTALL: u32 = 2u32;
pub const PRINTACTION_NETINSTALLLINK: u32 = 3u32;
pub const PRINTACTION_OPEN: u32 = 0u32;
pub const PRINTACTION_OPENNETPRN: u32 = 5u32;
pub const PRINTACTION_PROPERTIES: u32 = 1u32;
pub const PRINTACTION_SERVERPROPERTIES: u32 = 7u32;
pub const PRINTACTION_TESTPAGE: u32 = 4u32;
pub const PRINT_PROP_FORCE_NAME: u32 = 1u32;
pub const PROGDLG_AUTOTIME: u32 = 2u32;
pub const PROGDLG_MARQUEEPROGRESS: u32 = 32u32;
pub const PROGDLG_MODAL: u32 = 1u32;
pub const PROGDLG_NOCANCEL: u32 = 64u32;
pub const PROGDLG_NOMINIMIZE: u32 = 8u32;
pub const PROGDLG_NOPROGRESSBAR: u32 = 16u32;
pub const PROGDLG_NORMAL: u32 = 0u32;
pub const PROGDLG_NOTIME: u32 = 4u32;
pub const PROPSTR_EXTENSIONCOMPLETIONSTATE: windows_core::PCWSTR = windows_core::w!("ExtensionCompletionState");
pub const PROP_CONTRACT_DELEGATE: windows_core::PCWSTR = windows_core::w!("ContractDelegate");
pub const PSGUID_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03);
pub const PSGUID_BRIEFCASE: windows_core::GUID = windows_core::GUID::from_u128(0x328d8b21_7729_4bfc_954c_902b329d56b0);
pub const PSGUID_CONTROLPANEL: windows_core::GUID = windows_core::GUID::from_u128(0x305ca226_d286_468e_b848_2b2e8e697b74);
pub const PSGUID_CUSTOMIMAGEPROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x7ecd8b0e_c136_4a9b_9411_4ebd6673ccc3);
pub const PSGUID_DISPLACED: windows_core::GUID = windows_core::GUID::from_u128(0x9b174b33_40ff_11d2_a27e_00c04fc30871);
pub const PSGUID_DOCUMENTSUMMARYINFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae);
pub const PSGUID_DRM: windows_core::GUID = windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed);
pub const PSGUID_IMAGEPROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99);
pub const PSGUID_IMAGESUMMARYINFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03);
pub const PSGUID_LIBRARYPROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67);
pub const PSGUID_LINK: windows_core::GUID = windows_core::GUID::from_u128(0xb9b4b3fc_2b51_4a42_b5d8_324146afcf25);
pub const PSGUID_MEDIAFILESUMMARYINFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03);
pub const PSGUID_MISC: windows_core::GUID = windows_core::GUID::from_u128(0x9b174b34_40ff_11d2_a27e_00c04fc30871);
pub const PSGUID_MUSIC: windows_core::GUID = windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6);
pub const PSGUID_QUERY_D: windows_core::GUID = windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9);
pub const PSGUID_SHARE: windows_core::GUID = windows_core::GUID::from_u128(0xd8c3986f_813b_449c_845d_87b95d674ade);
pub const PSGUID_SHELLDETAILS: windows_core::GUID = windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0);
pub const PSGUID_SUMMARYINFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9);
pub const PSGUID_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03);
pub const PSGUID_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871);
pub const PSGUID_WEBVIEW: windows_core::GUID = windows_core::GUID::from_u128(0xf2275480_f782_4291_bd94_f13693513aec);
pub const ProtectedModeRedirect: NewProcessCauseConstants = 1i32;
pub const QCMINFO_PLACE_AFTER: u32 = 1u32;
pub const QCMINFO_PLACE_BEFORE: u32 = 0u32;
pub const QIF_CACHED: QITIPF_FLAGS = 1i32;
pub const QIF_DONTEXPANDFOLDER: QITIPF_FLAGS = 2i32;
pub const QITIPF_DEFAULT: QITIPF_FLAGS = 0i32;
pub const QITIPF_LINKNOTARGET: QITIPF_FLAGS = 2i32;
pub const QITIPF_LINKUSETARGET: QITIPF_FLAGS = 4i32;
pub const QITIPF_SINGLELINE: QITIPF_FLAGS = 16i32;
pub const QITIPF_USENAME: QITIPF_FLAGS = 1i32;
pub const QITIPF_USESLOWTIP: QITIPF_FLAGS = 8i32;
pub const QUNS_ACCEPTS_NOTIFICATIONS: QUERY_USER_NOTIFICATION_STATE = 5i32;
pub const QUNS_APP: QUERY_USER_NOTIFICATION_STATE = 7i32;
pub const QUNS_BUSY: QUERY_USER_NOTIFICATION_STATE = 2i32;
pub const QUNS_NOT_PRESENT: QUERY_USER_NOTIFICATION_STATE = 1i32;
pub const QUNS_PRESENTATION_MODE: QUERY_USER_NOTIFICATION_STATE = 4i32;
pub const QUNS_QUIET_TIME: QUERY_USER_NOTIFICATION_STATE = 6i32;
pub const QUNS_RUNNING_D3D_FULL_SCREEN: QUERY_USER_NOTIFICATION_STATE = 3i32;
pub const REFRESH_COMPLETELY: RefreshConstants = 3i32;
pub const REFRESH_IFEXPIRED: RefreshConstants = 1i32;
pub const REFRESH_NORMAL: RefreshConstants = 0i32;
pub const REST_ALLOWBITBUCKDRIVES: RESTRICTIONS = 1073741905i32;
pub const REST_ALLOWCOMMENTTOGGLE: RESTRICTIONS = 1090519044i32;
pub const REST_ALLOWFILECLSIDJUNCTIONS: RESTRICTIONS = 1073741980i32;
pub const REST_ALLOWLEGACYWEBVIEW: RESTRICTIONS = 1073741955i32;
pub const REST_ALLOWUNHASHEDWEBVIEW: RESTRICTIONS = 1073741954i32;
pub const REST_ARP_DONTGROUPPATCHES: RESTRICTIONS = 1073741996i32;
pub const REST_ARP_NOADDPAGE: RESTRICTIONS = 1073741867i32;
pub const REST_ARP_NOARP: RESTRICTIONS = 1073741865i32;
pub const REST_ARP_NOCHOOSEPROGRAMSPAGE: RESTRICTIONS = 1073741997i32;
pub const REST_ARP_NOREMOVEPAGE: RESTRICTIONS = 1073741866i32;
pub const REST_ARP_NOWINSETUPPAGE: RESTRICTIONS = 1073741868i32;
pub const REST_ARP_ShowPostSetup: RESTRICTIONS = 1073741861i32;
pub const REST_BITBUCKCONFIRMDELETE: RESTRICTIONS = 1073741941i32;
pub const REST_BITBUCKNOPROP: RESTRICTIONS = 1073741942i32;
pub const REST_BITBUCKNUKEONDELETE: RESTRICTIONS = 1073741940i32;
pub const REST_CLASSICSHELL: RESTRICTIONS = 1073741832i32;
pub const REST_CLEARRECENTDOCSONEXIT: RESTRICTIONS = 1073741831i32;
pub const REST_DISALLOWCPL: RESTRICTIONS = 1073741889i32;
pub const REST_DISALLOWRUN: RESTRICTIONS = 1073741886i32;
pub const REST_DONTRETRYBADNETNAME: RESTRICTIONS = 1073741979i32;
pub const REST_DONTSHOWSUPERHIDDEN: RESTRICTIONS = 1073741879i32;
pub const REST_ENFORCESHELLEXTSECURITY: RESTRICTIONS = 1048576i32;
pub const REST_ENUMWORKGROUP: RESTRICTIONS = 1073741864i32;
pub const REST_FORCEACTIVEDESKTOPON: RESTRICTIONS = 1073741898i32;
pub const REST_FORCECOPYACLWITHFILE: RESTRICTIONS = 1073741851i32;
pub const REST_FORCESTARTMENULOGOFF: RESTRICTIONS = 1073741874i32;
pub const REST_GREYMSIADS: RESTRICTIONS = 1073741869i32;
pub const REST_HASFINDCOMPUTERS: RESTRICTIONS = 1073741858i32;
pub const REST_HIDECLOCK: RESTRICTIONS = 1073741936i32;
pub const REST_HIDERUNASVERB: RESTRICTIONS = 1073741948i32;
pub const REST_INHERITCONSOLEHANDLES: RESTRICTIONS = 1073741958i32;
pub const REST_INTELLIMENUS: RESTRICTIONS = 1073741859i32;
pub const REST_LINKRESOLVEIGNORELINKINFO: RESTRICTIONS = 2097152i32;
pub const REST_MYCOMPNOPROP: RESTRICTIONS = 1073741912i32;
pub const REST_MYDOCSNOPROP: RESTRICTIONS = 1073741913i32;
pub const REST_MYDOCSONNET: RESTRICTIONS = 262144i32;
pub const REST_MaxRecentDocs: RESTRICTIONS = 1073741872i32;
pub const REST_NOACTIVEDESKTOP: RESTRICTIONS = 1073741828i32;
pub const REST_NOACTIVEDESKTOPCHANGES: RESTRICTIONS = 1073741829i32;
pub const REST_NOADDDESKCOMP: RESTRICTIONS = 1073741843i32;
pub const REST_NOAUTOTRAYNOTIFY: RESTRICTIONS = 1073741909i32;
pub const REST_NOCDBURNING: RESTRICTIONS = 1073741911i32;
pub const REST_NOCHANGEMAPPEDDRIVECOMMENT: RESTRICTIONS = 1073741871i32;
pub const REST_NOCHANGEMAPPEDDRIVELABEL: RESTRICTIONS = 1073741870i32;
pub const REST_NOCHANGESTARMENU: RESTRICTIONS = 1073741856i32;
pub const REST_NOCHANGINGWALLPAPER: RESTRICTIONS = 1073741841i32;
pub const REST_NOCLOSE: RESTRICTIONS = 2i32;
pub const REST_NOCLOSEDESKCOMP: RESTRICTIONS = 1073741845i32;
pub const REST_NOCLOSE_DRAGDROPBAND: RESTRICTIONS = 1073741846i32;
pub const REST_NOCOLORCHOICE: RESTRICTIONS = 1073741919i32;
pub const REST_NOCOMMONGROUPS: RESTRICTIONS = 4194304i32;
pub const REST_NOCONTROLPANEL: RESTRICTIONS = 1073741863i32;
pub const REST_NOCONTROLPANELBARRICADE: RESTRICTIONS = 1073741907i32;
pub const REST_NOCSC: RESTRICTIONS = 1073741862i32;
pub const REST_NOCURRENTUSERRUN: RESTRICTIONS = 1073741895i32;
pub const REST_NOCURRENTUSERRUNONCE: RESTRICTIONS = 1073741897i32;
pub const REST_NOCUSTOMIZETHISFOLDER: RESTRICTIONS = 1073741876i32;
pub const REST_NOCUSTOMIZEWEBVIEW: RESTRICTIONS = 1073741833i32;
pub const REST_NODELDESKCOMP: RESTRICTIONS = 1073741844i32;
pub const REST_NODESKCOMP: RESTRICTIONS = 1073741842i32;
pub const REST_NODESKTOP: RESTRICTIONS = 64i32;
pub const REST_NODESKTOPCLEANUP: RESTRICTIONS = 1073741939i32;
pub const REST_NODISCONNECT: RESTRICTIONS = 1090519041i32;
pub const REST_NODISPBACKGROUND: RESTRICTIONS = 1073741943i32;
pub const REST_NODISPLAYAPPEARANCEPAGE: RESTRICTIONS = 1073741915i32;
pub const REST_NODISPLAYCPL: RESTRICTIONS = 1073741947i32;
pub const REST_NODISPSCREENSAVEPG: RESTRICTIONS = 1073741944i32;
pub const REST_NODISPSCREENSAVEPREVIEW: RESTRICTIONS = 1073741946i32;
pub const REST_NODISPSETTINGSPG: RESTRICTIONS = 1073741945i32;
pub const REST_NODRIVEAUTORUN: RESTRICTIONS = 512i32;
pub const REST_NODRIVES: RESTRICTIONS = 256i32;
pub const REST_NODRIVETYPEAUTORUN: RESTRICTIONS = 1024i32;
pub const REST_NOEDITDESKCOMP: RESTRICTIONS = 1073741848i32;
pub const REST_NOENCRYPTION: RESTRICTIONS = 1073741877i32;
pub const REST_NOENCRYPTONMOVE: RESTRICTIONS = 1073741893i32;
pub const REST_NOENTIRENETWORK: RESTRICTIONS = 1073741938i32;
pub const REST_NOENUMENTIRENETWORK: RESTRICTIONS = 1073741971i32;
pub const REST_NOEXITTODOS: RESTRICTIONS = 524288i32;
pub const REST_NOFAVORITESMENU: RESTRICTIONS = 1073741830i32;
pub const REST_NOFILEASSOCIATE: RESTRICTIONS = 1090519043i32;
pub const REST_NOFILEMENU: RESTRICTIONS = 8i32;
pub const REST_NOFIND: RESTRICTIONS = 128i32;
pub const REST_NOFOLDEROPTIONS: RESTRICTIONS = 1073741857i32;
pub const REST_NOFORGETSOFTWAREUPDATE: RESTRICTIONS = 1073741853i32;
pub const REST_NOHARDWARETAB: RESTRICTIONS = 1073741881i32;
pub const REST_NOHTMLWALLPAPER: RESTRICTIONS = 1073741840i32;
pub const REST_NOINTERNETICON: RESTRICTIONS = 1073741825i32;
pub const REST_NOINTERNETOPENWITH: RESTRICTIONS = 1073741973i32;
pub const REST_NOLOCALMACHINERUN: RESTRICTIONS = 1073741894i32;
pub const REST_NOLOCALMACHINERUNONCE: RESTRICTIONS = 1073741896i32;
pub const REST_NOLOWDISKSPACECHECKS: RESTRICTIONS = 1073741937i32;
pub const REST_NOMANAGEMYCOMPUTERVERB: RESTRICTIONS = 1073741884i32;
pub const REST_NOMOVINGBAND: RESTRICTIONS = 1073741847i32;
pub const REST_NOMYCOMPUTERICON: RESTRICTIONS = 1073741923i32;
pub const REST_NONE: RESTRICTIONS = 0i32;
pub const REST_NONETCONNECTDISCONNECT: RESTRICTIONS = 134217728i32;
pub const REST_NONETCRAWL: RESTRICTIONS = 1073741901i32;
pub const REST_NONETHOOD: RESTRICTIONS = 2048i32;
pub const REST_NONETWORKCONNECTIONS: RESTRICTIONS = 1073741873i32;
pub const REST_NONLEGACYSHELLMODE: RESTRICTIONS = 1073741906i32;
pub const REST_NOONLINEPRINTSWIZARD: RESTRICTIONS = 1073741952i32;
pub const REST_NOPRINTERADD: RESTRICTIONS = 65536i32;
pub const REST_NOPRINTERDELETE: RESTRICTIONS = 32768i32;
pub const REST_NOPRINTERTABS: RESTRICTIONS = 16384i32;
pub const REST_NOPUBLISHWIZARD: RESTRICTIONS = 1073741951i32;
pub const REST_NORECENTDOCSHISTORY: RESTRICTIONS = 1073741826i32;
pub const REST_NORECENTDOCSMENU: RESTRICTIONS = 1073741827i32;
pub const REST_NOREMOTECHANGENOTIFY: RESTRICTIONS = 1073741969i32;
pub const REST_NOREMOTERECURSIVEEVENTS: RESTRICTIONS = 1073741961i32;
pub const REST_NORESOLVESEARCH: RESTRICTIONS = 1073741849i32;
pub const REST_NORESOLVETRACK: RESTRICTIONS = 1073741850i32;
pub const REST_NORUN: RESTRICTIONS = 1i32;
pub const REST_NORUNASINSTALLPROMPT: RESTRICTIONS = 1073741882i32;
pub const REST_NOSAVESET: RESTRICTIONS = 4i32;
pub const REST_NOSECURITY: RESTRICTIONS = 1090519042i32;
pub const REST_NOSETACTIVEDESKTOP: RESTRICTIONS = 1073741854i32;
pub const REST_NOSETFOLDERS: RESTRICTIONS = 16i32;
pub const REST_NOSETTASKBAR: RESTRICTIONS = 32i32;
pub const REST_NOSETTINGSASSIST: RESTRICTIONS = 536870912i32;
pub const REST_NOSHAREDDOCUMENTS: RESTRICTIONS = 1073741902i32;
pub const REST_NOSHELLSEARCHBUTTON: RESTRICTIONS = 1073741880i32;
pub const REST_NOSIZECHOICE: RESTRICTIONS = 1073741918i32;
pub const REST_NOSMBALLOONTIP: RESTRICTIONS = 1073741890i32;
pub const REST_NOSMCONFIGUREPROGRAMS: RESTRICTIONS = 1073741935i32;
pub const REST_NOSMEJECTPC: RESTRICTIONS = 1073741927i32;
pub const REST_NOSMHELP: RESTRICTIONS = 1073741891i32;
pub const REST_NOSMMFUPROGRAMS: RESTRICTIONS = 1073741929i32;
pub const REST_NOSMMOREPROGRAMS: RESTRICTIONS = 1073741928i32;
pub const REST_NOSMMYDOCS: RESTRICTIONS = 1073741903i32;
pub const REST_NOSMMYMUSIC: RESTRICTIONS = 1073741926i32;
pub const REST_NOSMMYPICS: RESTRICTIONS = 1073741904i32;
pub const REST_NOSMNETWORKPLACES: RESTRICTIONS = 1073741924i32;
pub const REST_NOSMPINNEDLIST: RESTRICTIONS = 1073741925i32;
pub const REST_NOSTARTMENUSUBFOLDERS: RESTRICTIONS = 131072i32;
pub const REST_NOSTARTPAGE: RESTRICTIONS = 1073741908i32;
pub const REST_NOSTARTPANEL: RESTRICTIONS = 1073741914i32;
pub const REST_NOSTRCMPLOGICAL: RESTRICTIONS = 1073741950i32;
pub const REST_NOTASKGROUPING: RESTRICTIONS = 1073741910i32;
pub const REST_NOTHEMESTAB: RESTRICTIONS = 1073741916i32;
pub const REST_NOTHUMBNAILCACHE: RESTRICTIONS = 1073741949i32;
pub const REST_NOTOOLBARSONTASKBAR: RESTRICTIONS = 1073741931i32;
pub const REST_NOTRAYCONTEXTMENU: RESTRICTIONS = 33554432i32;
pub const REST_NOTRAYITEMSDISPLAY: RESTRICTIONS = 1073741930i32;
pub const REST_NOUPDATEWINDOWS: RESTRICTIONS = 1073741855i32;
pub const REST_NOUPNPINSTALL: RESTRICTIONS = 1073741981i32;
pub const REST_NOUSERNAMEINSTARTPANEL: RESTRICTIONS = 1073741922i32;
pub const REST_NOVIEWCONTEXTMENU: RESTRICTIONS = 67108864i32;
pub const REST_NOVIEWONDRIVE: RESTRICTIONS = 1073741900i32;
pub const REST_NOVISUALSTYLECHOICE: RESTRICTIONS = 1073741917i32;
pub const REST_NOWEB: RESTRICTIONS = 16777216i32;
pub const REST_NOWEBSERVICES: RESTRICTIONS = 1073741953i32;
pub const REST_NOWEBVIEW: RESTRICTIONS = 1073741875i32;
pub const REST_NOWELCOMESCREEN: RESTRICTIONS = 1073741887i32;
pub const REST_NOWINKEYS: RESTRICTIONS = 1073741892i32;
pub const REST_PROMPTRUNASINSTALLNETPATH: RESTRICTIONS = 1073741883i32;
pub const REST_RESTRICTCPL: RESTRICTIONS = 1073741888i32;
pub const REST_RESTRICTRUN: RESTRICTIONS = 8192i32;
pub const REST_REVERTWEBVIEWSECURITY: RESTRICTIONS = 1073741956i32;
pub const REST_RUNDLGMEMCHECKBOX: RESTRICTIONS = 1073741860i32;
pub const REST_SEPARATEDESKTOPPROCESS: RESTRICTIONS = 8388608i32;
pub const REST_SETVISUALSTYLE: RESTRICTIONS = 1073741920i32;
pub const REST_STARTBANNER: RESTRICTIONS = 4096i32;
pub const REST_STARTMENULOGOFF: RESTRICTIONS = 268435456i32;
pub const REST_STARTRUNNOHOMEPATH: RESTRICTIONS = 1073741921i32;
pub const ResizeThumbnail: ThumbnailStreamCacheOptions = 2i32;
pub const ReturnOnlyIfCached: ThumbnailStreamCacheOptions = 1i32;
pub const SBSC_HIDE: SHELLBROWSERSHOWCONTROL = 0i32;
pub const SBSC_QUERY: SHELLBROWSERSHOWCONTROL = 3i32;
pub const SBSC_SHOW: SHELLBROWSERSHOWCONTROL = 1i32;
pub const SBSC_TOGGLE: SHELLBROWSERSHOWCONTROL = 2i32;
pub const SBSP_ABSOLUTE: u32 = 0u32;
pub const SBSP_ACTIVATE_NOFOCUS: u32 = 524288u32;
pub const SBSP_ALLOW_AUTONAVIGATE: u32 = 65536u32;
pub const SBSP_CALLERUNTRUSTED: u32 = 8388608u32;
pub const SBSP_CREATENOHISTORY: u32 = 1048576u32;
pub const SBSP_DEFBROWSER: u32 = 0u32;
pub const SBSP_DEFMODE: u32 = 0u32;
pub const SBSP_EXPLOREMODE: u32 = 32u32;
pub const SBSP_FEEDNAVIGATION: u32 = 536870912u32;
pub const SBSP_HELPMODE: u32 = 64u32;
pub const SBSP_INITIATEDBYHLINKFRAME: u32 = 2147483648u32;
pub const SBSP_KEEPSAMETEMPLATE: u32 = 131072u32;
pub const SBSP_KEEPWORDWHEELTEXT: u32 = 262144u32;
pub const SBSP_NAVIGATEBACK: u32 = 16384u32;
pub const SBSP_NAVIGATEFORWARD: u32 = 32768u32;
pub const SBSP_NEWBROWSER: u32 = 2u32;
pub const SBSP_NOAUTOSELECT: u32 = 67108864u32;
pub const SBSP_NOTRANSFERHIST: u32 = 128u32;
pub const SBSP_OPENMODE: u32 = 16u32;
pub const SBSP_PARENT: u32 = 8192u32;
pub const SBSP_PLAYNOSOUND: u32 = 2097152u32;
pub const SBSP_REDIRECT: u32 = 1073741824u32;
pub const SBSP_RELATIVE: u32 = 4096u32;
pub const SBSP_SAMEBROWSER: u32 = 1u32;
pub const SBSP_TRUSTEDFORACTIVEX: u32 = 268435456u32;
pub const SBSP_TRUSTFIRSTDOWNLOAD: u32 = 16777216u32;
pub const SBSP_UNTRUSTEDFORDOWNLOAD: u32 = 33554432u32;
pub const SBSP_WRITENOHISTORY: u32 = 134217728u32;
pub const SCF_PHYSICAL: SCALE_CHANGE_FLAGS = 2i32;
pub const SCF_SCALE: SCALE_CHANGE_FLAGS = 1i32;
pub const SCF_VALUE_NONE: SCALE_CHANGE_FLAGS = 0i32;
pub const SCHEME_CREATE: u32 = 128u32;
pub const SCHEME_DISPLAY: u32 = 1u32;
pub const SCHEME_DONOTUSE: u32 = 64u32;
pub const SCHEME_EDIT: u32 = 2u32;
pub const SCHEME_GLOBAL: u32 = 8u32;
pub const SCHEME_LOCAL: u32 = 4u32;
pub const SCHEME_REFRESH: u32 = 16u32;
pub const SCHEME_UPDATE: u32 = 32u32;
pub const SCNRT_DISABLE: SCNRT_STATUS = 1i32;
pub const SCNRT_ENABLE: SCNRT_STATUS = 0i32;
pub const SCRM_VERIFYPW: u32 = 32768u32;
pub const SECURELOCK_FIRSTSUGGEST: SECURELOCKCODE = 7i32;
pub const SECURELOCK_NOCHANGE: SECURELOCKCODE = -1i32;
pub const SECURELOCK_SET_FORTEZZA: SECURELOCKCODE = 5i32;
pub const SECURELOCK_SET_MIXED: SECURELOCKCODE = 1i32;
pub const SECURELOCK_SET_SECURE128BIT: SECURELOCKCODE = 6i32;
pub const SECURELOCK_SET_SECURE40BIT: SECURELOCKCODE = 3i32;
pub const SECURELOCK_SET_SECURE56BIT: SECURELOCKCODE = 4i32;
pub const SECURELOCK_SET_SECUREUNKNOWNBIT: SECURELOCKCODE = 2i32;
pub const SECURELOCK_SET_UNSECURE: SECURELOCKCODE = 0i32;
pub const SECURELOCK_SUGGEST_FORTEZZA: SECURELOCKCODE = 12i32;
pub const SECURELOCK_SUGGEST_MIXED: SECURELOCKCODE = 8i32;
pub const SECURELOCK_SUGGEST_SECURE128BIT: SECURELOCKCODE = 13i32;
pub const SECURELOCK_SUGGEST_SECURE40BIT: SECURELOCKCODE = 10i32;
pub const SECURELOCK_SUGGEST_SECURE56BIT: SECURELOCKCODE = 11i32;
pub const SECURELOCK_SUGGEST_SECUREUNKNOWNBIT: SECURELOCKCODE = 9i32;
pub const SECURELOCK_SUGGEST_UNSECURE: SECURELOCKCODE = 7i32;
pub const SEE_MASK_ASYNCOK: u32 = 1048576u32;
pub const SEE_MASK_CLASSKEY: u32 = 3u32;
pub const SEE_MASK_CLASSNAME: u32 = 1u32;
pub const SEE_MASK_CONNECTNETDRV: u32 = 128u32;
pub const SEE_MASK_DEFAULT: u32 = 0u32;
pub const SEE_MASK_DOENVSUBST: u32 = 512u32;
pub const SEE_MASK_FLAG_DDEWAIT: u32 = 256u32;
pub const SEE_MASK_FLAG_HINST_IS_SITE: u32 = 134217728u32;
pub const SEE_MASK_FLAG_LOG_USAGE: u32 = 67108864u32;
pub const SEE_MASK_FLAG_NO_UI: u32 = 1024u32;
pub const SEE_MASK_HMONITOR: u32 = 2097152u32;
pub const SEE_MASK_HOTKEY: u32 = 32u32;
pub const SEE_MASK_ICON: u32 = 16u32;
pub const SEE_MASK_IDLIST: u32 = 4u32;
pub const SEE_MASK_INVOKEIDLIST: u32 = 12u32;
pub const SEE_MASK_NOASYNC: u32 = 256u32;
pub const SEE_MASK_NOCLOSEPROCESS: u32 = 64u32;
pub const SEE_MASK_NOQUERYCLASSSTORE: u32 = 16777216u32;
pub const SEE_MASK_NOZONECHECKS: u32 = 8388608u32;
pub const SEE_MASK_NO_CONSOLE: u32 = 32768u32;
pub const SEE_MASK_UNICODE: u32 = 16384u32;
pub const SEE_MASK_WAITFORINPUTIDLE: u32 = 33554432u32;
pub const SETPROPS_NONE: u32 = 0u32;
pub const SE_ERR_ACCESSDENIED: u32 = 5u32;
pub const SE_ERR_ASSOCINCOMPLETE: u32 = 27u32;
pub const SE_ERR_DDEBUSY: u32 = 30u32;
pub const SE_ERR_DDEFAIL: u32 = 29u32;
pub const SE_ERR_DDETIMEOUT: u32 = 28u32;
pub const SE_ERR_DLLNOTFOUND: u32 = 32u32;
pub const SE_ERR_FNF: u32 = 2u32;
pub const SE_ERR_NOASSOC: u32 = 31u32;
pub const SE_ERR_OOM: u32 = 8u32;
pub const SE_ERR_PNF: u32 = 3u32;
pub const SE_ERR_SHARE: u32 = 26u32;
pub const SFBID_PIDLCHANGED: i32 = 0i32;
pub const SFBS_FLAGS_ROUND_TO_NEAREST_DISPLAYED_DIGIT: SFBS_FLAGS = 1i32;
pub const SFBS_FLAGS_TRUNCATE_UNDISPLAYED_DECIMAL_DIGITS: SFBS_FLAGS = 2i32;
pub const SFVM_ADDOBJECT: u32 = 3u32;
pub const SFVM_ADDPROPERTYPAGES: SFVM_MESSAGE_ID = 47i32;
pub const SFVM_BACKGROUNDENUM: SFVM_MESSAGE_ID = 32i32;
pub const SFVM_BACKGROUNDENUMDONE: SFVM_MESSAGE_ID = 48i32;
pub const SFVM_COLUMNCLICK: SFVM_MESSAGE_ID = 24i32;
pub const SFVM_DEFITEMCOUNT: SFVM_MESSAGE_ID = 26i32;
pub const SFVM_DEFVIEWMODE: SFVM_MESSAGE_ID = 27i32;
pub const SFVM_DIDDRAGDROP: SFVM_MESSAGE_ID = 36i32;
pub const SFVM_FSNOTIFY: SFVM_MESSAGE_ID = 14i32;
pub const SFVM_GETANIMATION: SFVM_MESSAGE_ID = 68i32;
pub const SFVM_GETBUTTONINFO: SFVM_MESSAGE_ID = 5i32;
pub const SFVM_GETBUTTONS: SFVM_MESSAGE_ID = 6i32;
pub const SFVM_GETDETAILSOF: SFVM_MESSAGE_ID = 23i32;
pub const SFVM_GETHELPTEXT: SFVM_MESSAGE_ID = 3i32;
pub const SFVM_GETHELPTOPIC: SFVM_MESSAGE_ID = 63i32;
pub const SFVM_GETNOTIFY: SFVM_MESSAGE_ID = 49i32;
pub const SFVM_GETPANE: SFVM_MESSAGE_ID = 59i32;
pub const SFVM_GETSELECTEDOBJECTS: u32 = 9u32;
pub const SFVM_GETSORTDEFAULTS: SFVM_MESSAGE_ID = 53i32;
pub const SFVM_GETTOOLTIPTEXT: SFVM_MESSAGE_ID = 4i32;
pub const SFVM_GETZONE: SFVM_MESSAGE_ID = 58i32;
pub const SFVM_INITMENUPOPUP: SFVM_MESSAGE_ID = 7i32;
pub const SFVM_INVOKECOMMAND: SFVM_MESSAGE_ID = 2i32;
pub const SFVM_MERGEMENU: SFVM_MESSAGE_ID = 1i32;
pub const SFVM_QUERYFSNOTIFY: SFVM_MESSAGE_ID = 25i32;
pub const SFVM_REARRANGE: u32 = 1u32;
pub const SFVM_REMOVEOBJECT: u32 = 6u32;
pub const SFVM_SETCLIPBOARD: u32 = 16u32;
pub const SFVM_SETISFV: SFVM_MESSAGE_ID = 39i32;
pub const SFVM_SETITEMPOS: u32 = 14u32;
pub const SFVM_SETPOINTS: u32 = 23u32;
pub const SFVM_SIZE: SFVM_MESSAGE_ID = 57i32;
pub const SFVM_THISIDLIST: SFVM_MESSAGE_ID = 41i32;
pub const SFVM_UNMERGEMENU: SFVM_MESSAGE_ID = 28i32;
pub const SFVM_UPDATEOBJECT: u32 = 7u32;
pub const SFVM_UPDATESTATUSBAR: SFVM_MESSAGE_ID = 31i32;
pub const SFVM_WINDOWCREATED: SFVM_MESSAGE_ID = 15i32;
pub const SFVSOC_INVALIDATE_ALL: u32 = 1u32;
pub const SFVSOC_NOSCROLL: u32 = 2u32;
pub const SFVS_SELECT_ALLITEMS: SFVS_SELECT = 1i32;
pub const SFVS_SELECT_INVERT: SFVS_SELECT = 2i32;
pub const SFVS_SELECT_NONE: SFVS_SELECT = 0i32;
pub const SFVVO_DESKTOPHTML: ShellFolderViewOptions = 512i32;
pub const SFVVO_DOUBLECLICKINWEBVIEW: ShellFolderViewOptions = 128i32;
pub const SFVVO_SHOWALLOBJECTS: ShellFolderViewOptions = 1i32;
pub const SFVVO_SHOWCOMPCOLOR: ShellFolderViewOptions = 8i32;
pub const SFVVO_SHOWEXTENSIONS: ShellFolderViewOptions = 2i32;
pub const SFVVO_SHOWSYSFILES: ShellFolderViewOptions = 32i32;
pub const SFVVO_WIN95CLASSIC: ShellFolderViewOptions = 64i32;
pub const SHACF_AUTOAPPEND_FORCE_OFF: SHELL_AUTOCOMPLETE_FLAGS = 2147483648u32;
pub const SHACF_AUTOAPPEND_FORCE_ON: SHELL_AUTOCOMPLETE_FLAGS = 1073741824u32;
pub const SHACF_AUTOSUGGEST_FORCE_OFF: SHELL_AUTOCOMPLETE_FLAGS = 536870912u32;
pub const SHACF_AUTOSUGGEST_FORCE_ON: SHELL_AUTOCOMPLETE_FLAGS = 268435456u32;
pub const SHACF_DEFAULT: SHELL_AUTOCOMPLETE_FLAGS = 0u32;
pub const SHACF_FILESYSTEM: SHELL_AUTOCOMPLETE_FLAGS = 1u32;
pub const SHACF_FILESYS_DIRS: SHELL_AUTOCOMPLETE_FLAGS = 32u32;
pub const SHACF_FILESYS_ONLY: SHELL_AUTOCOMPLETE_FLAGS = 16u32;
pub const SHACF_URLALL: SHELL_AUTOCOMPLETE_FLAGS = 6u32;
pub const SHACF_URLHISTORY: SHELL_AUTOCOMPLETE_FLAGS = 2u32;
pub const SHACF_URLMRU: SHELL_AUTOCOMPLETE_FLAGS = 4u32;
pub const SHACF_USETAB: SHELL_AUTOCOMPLETE_FLAGS = 8u32;
pub const SHACF_VIRTUAL_NAMESPACE: SHELL_AUTOCOMPLETE_FLAGS = 64u32;
pub const SHARD_APPIDINFO: SHARD = 4i32;
pub const SHARD_APPIDINFOIDLIST: SHARD = 5i32;
pub const SHARD_APPIDINFOLINK: SHARD = 7i32;
pub const SHARD_LINK: SHARD = 6i32;
pub const SHARD_PATHA: SHARD = 2i32;
pub const SHARD_PATHW: SHARD = 3i32;
pub const SHARD_PIDL: SHARD = 1i32;
pub const SHARD_SHELLITEM: SHARD = 8i32;
pub const SHARE_ROLE_CONTRIBUTOR: SHARE_ROLE = 1i32;
pub const SHARE_ROLE_CO_OWNER: SHARE_ROLE = 2i32;
pub const SHARE_ROLE_CUSTOM: SHARE_ROLE = 4i32;
pub const SHARE_ROLE_INVALID: SHARE_ROLE = -1i32;
pub const SHARE_ROLE_MIXED: SHARE_ROLE = 5i32;
pub const SHARE_ROLE_OWNER: SHARE_ROLE = 3i32;
pub const SHARE_ROLE_READER: SHARE_ROLE = 0i32;
pub const SHCDF_UPDATEITEM: u32 = 1u32;
pub const SHCIDS_ALLFIELDS: i32 = -2147483648i32;
pub const SHCIDS_BITMASK: i32 = -65536i32;
pub const SHCIDS_CANONICALONLY: i32 = 268435456i32;
pub const SHCIDS_COLUMNMASK: i32 = 65535i32;
pub const SHCNEE_MSI_CHANGE: i32 = 4i32;
pub const SHCNEE_MSI_UNINSTALL: i32 = 5i32;
pub const SHCNEE_ORDERCHANGED: i32 = 2i32;
pub const SHCNE_ALLEVENTS: SHCNE_ID = 2147483647u32;
pub const SHCNE_ASSOCCHANGED: SHCNE_ID = 134217728u32;
pub const SHCNE_ATTRIBUTES: SHCNE_ID = 2048u32;
pub const SHCNE_CREATE: SHCNE_ID = 2u32;
pub const SHCNE_DELETE: SHCNE_ID = 4u32;
pub const SHCNE_DISKEVENTS: SHCNE_ID = 145439u32;
pub const SHCNE_DRIVEADD: SHCNE_ID = 256u32;
pub const SHCNE_DRIVEADDGUI: SHCNE_ID = 65536u32;
pub const SHCNE_DRIVEREMOVED: SHCNE_ID = 128u32;
pub const SHCNE_EXTENDED_EVENT: SHCNE_ID = 67108864u32;
pub const SHCNE_FREESPACE: SHCNE_ID = 262144u32;
pub const SHCNE_GLOBALEVENTS: SHCNE_ID = 201687520u32;
pub const SHCNE_INTERRUPT: SHCNE_ID = 2147483648u32;
pub const SHCNE_MEDIAINSERTED: SHCNE_ID = 32u32;
pub const SHCNE_MEDIAREMOVED: SHCNE_ID = 64u32;
pub const SHCNE_MKDIR: SHCNE_ID = 8u32;
pub const SHCNE_NETSHARE: SHCNE_ID = 512u32;
pub const SHCNE_NETUNSHARE: SHCNE_ID = 1024u32;
pub const SHCNE_RENAMEFOLDER: SHCNE_ID = 131072u32;
pub const SHCNE_RENAMEITEM: SHCNE_ID = 1u32;
pub const SHCNE_RMDIR: SHCNE_ID = 16u32;
pub const SHCNE_SERVERDISCONNECT: SHCNE_ID = 16384u32;
pub const SHCNE_UPDATEDIR: SHCNE_ID = 4096u32;
pub const SHCNE_UPDATEIMAGE: SHCNE_ID = 32768u32;
pub const SHCNE_UPDATEITEM: SHCNE_ID = 8192u32;
pub const SHCNF_DWORD: SHCNF_FLAGS = 3u32;
pub const SHCNF_FLUSH: SHCNF_FLAGS = 4096u32;
pub const SHCNF_FLUSHNOWAIT: SHCNF_FLAGS = 12288u32;
pub const SHCNF_IDLIST: SHCNF_FLAGS = 0u32;
pub const SHCNF_NOTIFYRECURSIVE: SHCNF_FLAGS = 65536u32;
pub const SHCNF_PATH: SHCNF_FLAGS = 5u32;
pub const SHCNF_PATHA: SHCNF_FLAGS = 1u32;
pub const SHCNF_PATHW: SHCNF_FLAGS = 5u32;
pub const SHCNF_PRINTER: SHCNF_FLAGS = 6u32;
pub const SHCNF_PRINTERA: SHCNF_FLAGS = 2u32;
pub const SHCNF_PRINTERW: SHCNF_FLAGS = 6u32;
pub const SHCNF_TYPE: SHCNF_FLAGS = 255u32;
pub const SHCNRF_InterruptLevel: SHCNRF_SOURCE = 1i32;
pub const SHCNRF_NewDelivery: SHCNRF_SOURCE = 32768i32;
pub const SHCNRF_RecursiveInterrupt: SHCNRF_SOURCE = 4096i32;
pub const SHCNRF_ShellLevel: SHCNRF_SOURCE = 2i32;
pub const SHCONTF_CHECKING_FOR_CHILDREN: _SHCONTF = 16i32;
pub const SHCONTF_ENABLE_ASYNC: _SHCONTF = 32768i32;
pub const SHCONTF_FASTITEMS: _SHCONTF = 8192i32;
pub const SHCONTF_FLATLIST: _SHCONTF = 16384i32;
pub const SHCONTF_FOLDERS: _SHCONTF = 32i32;
pub const SHCONTF_INCLUDEHIDDEN: _SHCONTF = 128i32;
pub const SHCONTF_INCLUDESUPERHIDDEN: _SHCONTF = 65536i32;
pub const SHCONTF_INIT_ON_FIRST_NEXT: _SHCONTF = 256i32;
pub const SHCONTF_NAVIGATION_ENUM: _SHCONTF = 4096i32;
pub const SHCONTF_NETPRINTERSRCH: _SHCONTF = 512i32;
pub const SHCONTF_NONFOLDERS: _SHCONTF = 64i32;
pub const SHCONTF_SHAREABLE: _SHCONTF = 1024i32;
pub const SHCONTF_STORAGE: _SHCONTF = 2048i32;
pub const SHC_E_SHELL_COMPONENT_STARTUP_FAILURE: windows_core::HRESULT = 0x80270234_u32 as _;
pub const SHDID_COMPUTER_AUDIO: SHDID_ID = 19i32;
pub const SHDID_COMPUTER_CDROM: SHDID_ID = 10i32;
pub const SHDID_COMPUTER_DRIVE35: SHDID_ID = 5i32;
pub const SHDID_COMPUTER_DRIVE525: SHDID_ID = 6i32;
pub const SHDID_COMPUTER_FIXED: SHDID_ID = 8i32;
pub const SHDID_COMPUTER_IMAGING: SHDID_ID = 18i32;
pub const SHDID_COMPUTER_NETDRIVE: SHDID_ID = 9i32;
pub const SHDID_COMPUTER_OTHER: SHDID_ID = 12i32;
pub const SHDID_COMPUTER_RAMDISK: SHDID_ID = 11i32;
pub const SHDID_COMPUTER_REMOVABLE: SHDID_ID = 7i32;
pub const SHDID_COMPUTER_SHAREDDOCS: SHDID_ID = 20i32;
pub const SHDID_FS_DIRECTORY: SHDID_ID = 3i32;
pub const SHDID_FS_FILE: SHDID_ID = 2i32;
pub const SHDID_FS_OTHER: SHDID_ID = 4i32;
pub const SHDID_MOBILE_DEVICE: SHDID_ID = 21i32;
pub const SHDID_NET_DOMAIN: SHDID_ID = 13i32;
pub const SHDID_NET_OTHER: SHDID_ID = 17i32;
pub const SHDID_NET_RESTOFNET: SHDID_ID = 16i32;
pub const SHDID_NET_SERVER: SHDID_ID = 14i32;
pub const SHDID_NET_SHARE: SHDID_ID = 15i32;
pub const SHDID_REMOTE_DESKTOP_DRIVE: SHDID_ID = 22i32;
pub const SHDID_ROOT_REGITEM: SHDID_ID = 1i32;
pub const SHELLSTATEVERSION_IE4: u32 = 9u32;
pub const SHELLSTATEVERSION_WIN2K: u32 = 10u32;
pub const SHELL_E_WRONG_BITDEPTH: windows_core::HRESULT = 0x80270102_u32 as _;
pub const SHELL_UI_COMPONENT_DESKBAND: SHELL_UI_COMPONENT = 2i32;
pub const SHELL_UI_COMPONENT_NOTIFICATIONAREA: SHELL_UI_COMPONENT = 1i32;
pub const SHELL_UI_COMPONENT_TASKBARS: SHELL_UI_COMPONENT = 0i32;
pub const SHERB_NOCONFIRMATION: u32 = 1u32;
pub const SHERB_NOPROGRESSUI: u32 = 2u32;
pub const SHERB_NOSOUND: u32 = 4u32;
pub const SHFMT_CANCEL: SHFMT_RET = 4294967294u32;
pub const SHFMT_ERROR: SHFMT_RET = 4294967295u32;
pub const SHFMT_ID_DEFAULT: SHFMT_ID = 65535u32;
pub const SHFMT_NOFORMAT: SHFMT_RET = 4294967293u32;
pub const SHFMT_OPT_FULL: SHFMT_OPT = 1i32;
pub const SHFMT_OPT_NONE: SHFMT_OPT = 0i32;
pub const SHFMT_OPT_SYSONLY: SHFMT_OPT = 2i32;
pub const SHGDFIL_DESCRIPTIONID: SHGDFIL_FORMAT = 3i32;
pub const SHGDFIL_FINDDATA: SHGDFIL_FORMAT = 1i32;
pub const SHGDFIL_NETRESOURCE: SHGDFIL_FORMAT = 2i32;
pub const SHGDN_FORADDRESSBAR: SHGDNF = 16384u32;
pub const SHGDN_FOREDITING: SHGDNF = 4096u32;
pub const SHGDN_FORPARSING: SHGDNF = 32768u32;
pub const SHGDN_INFOLDER: SHGDNF = 1u32;
pub const SHGDN_NORMAL: SHGDNF = 0u32;
pub const SHGFI_ADDOVERLAYS: SHGFI_FLAGS = 32u32;
pub const SHGFI_ATTRIBUTES: SHGFI_FLAGS = 2048u32;
pub const SHGFI_ATTR_SPECIFIED: SHGFI_FLAGS = 131072u32;
pub const SHGFI_DISPLAYNAME: SHGFI_FLAGS = 512u32;
pub const SHGFI_EXETYPE: SHGFI_FLAGS = 8192u32;
pub const SHGFI_ICON: SHGFI_FLAGS = 256u32;
pub const SHGFI_ICONLOCATION: SHGFI_FLAGS = 4096u32;
pub const SHGFI_LARGEICON: SHGFI_FLAGS = 0u32;
pub const SHGFI_LINKOVERLAY: SHGFI_FLAGS = 32768u32;
pub const SHGFI_OPENICON: SHGFI_FLAGS = 2u32;
pub const SHGFI_OVERLAYINDEX: SHGFI_FLAGS = 64u32;
pub const SHGFI_PIDL: SHGFI_FLAGS = 8u32;
pub const SHGFI_SELECTED: SHGFI_FLAGS = 65536u32;
pub const SHGFI_SHELLICONSIZE: SHGFI_FLAGS = 4u32;
pub const SHGFI_SMALLICON: SHGFI_FLAGS = 1u32;
pub const SHGFI_SYSICONINDEX: SHGFI_FLAGS = 16384u32;
pub const SHGFI_TYPENAME: SHGFI_FLAGS = 1024u32;
pub const SHGFI_USEFILEATTRIBUTES: SHGFI_FLAGS = 16u32;
pub const SHGFP_TYPE_CURRENT: SHGFP_TYPE = 0i32;
pub const SHGFP_TYPE_DEFAULT: SHGFP_TYPE = 1i32;
pub const SHGNLI_NOLNK: u64 = 8u64;
pub const SHGNLI_NOLOCNAME: u64 = 16u64;
pub const SHGNLI_NOUNIQUE: u64 = 4u64;
pub const SHGNLI_PIDL: u64 = 1u64;
pub const SHGNLI_PREFIXNAME: u64 = 2u64;
pub const SHGNLI_USEURLEXT: u64 = 32u64;
pub const SHGSI_ICON: SHGSI_FLAGS = 256u32;
pub const SHGSI_ICONLOCATION: SHGSI_FLAGS = 0u32;
pub const SHGSI_LARGEICON: SHGSI_FLAGS = 0u32;
pub const SHGSI_LINKOVERLAY: SHGSI_FLAGS = 32768u32;
pub const SHGSI_SELECTED: SHGSI_FLAGS = 65536u32;
pub const SHGSI_SHELLICONSIZE: SHGSI_FLAGS = 4u32;
pub const SHGSI_SMALLICON: SHGSI_FLAGS = 1u32;
pub const SHGSI_SYSICONINDEX: SHGSI_FLAGS = 16384u32;
pub const SHGVSPB_ALLFOLDERS: u32 = 8u32;
pub const SHGVSPB_ALLUSERS: u32 = 2u32;
pub const SHGVSPB_INHERIT: u32 = 16u32;
pub const SHGVSPB_NOAUTODEFAULTS: u32 = 2147483648u32;
pub const SHGVSPB_PERFOLDER: u32 = 4u32;
pub const SHGVSPB_PERUSER: u32 = 1u32;
pub const SHGVSPB_ROAM: u32 = 32u32;
pub const SHHLNF_NOAUTOSELECT: u32 = 67108864u32;
pub const SHHLNF_WRITENOHISTORY: u32 = 134217728u32;
pub const SHIL_EXTRALARGE: u32 = 2u32;
pub const SHIL_JUMBO: u32 = 4u32;
pub const SHIL_LARGE: u32 = 0u32;
pub const SHIL_LAST: u32 = 4u32;
pub const SHIL_SMALL: u32 = 1u32;
pub const SHIL_SYSSMALL: u32 = 3u32;
pub const SHIMGDEC_DEFAULT: u32 = 0u32;
pub const SHIMGDEC_LOADFULL: u32 = 2u32;
pub const SHIMGDEC_THUMBNAIL: u32 = 1u32;
pub const SHIMGKEY_QUALITY: windows_core::PCWSTR = windows_core::w!("Compression");
pub const SHIMGKEY_RAWFORMAT: windows_core::PCWSTR = windows_core::w!("RawDataFormat");
pub const SHIMSTCAPFLAG_LOCKABLE: u32 = 1u32;
pub const SHIMSTCAPFLAG_PURGEABLE: u32 = 2u32;
pub const SHOP_FILEPATH: SHOP_TYPE = 2i32;
pub const SHOP_PRINTERNAME: SHOP_TYPE = 1i32;
pub const SHOP_VOLUMEGUID: SHOP_TYPE = 4i32;
pub const SHPPFW_ASKDIRCREATE: u32 = 2u32;
pub const SHPPFW_DIRCREATE: u32 = 1u32;
pub const SHPPFW_IGNOREFILENAME: u32 = 4u32;
pub const SHPPFW_MEDIACHECKONLY: u32 = 16u32;
pub const SHPPFW_NONE: u32 = 0u32;
pub const SHPPFW_NOWRITECHECK: u32 = 8u32;
pub const SHPWHF_ANYLOCATION: u32 = 256u32;
pub const SHPWHF_NOFILESELECTOR: u32 = 4u32;
pub const SHPWHF_NONETPLACECREATE: u32 = 2u32;
pub const SHPWHF_NORECOMPRESS: u32 = 1u32;
pub const SHPWHF_USEMRU: u32 = 8u32;
pub const SHPWHF_VALIDATEVIAWEBFOLDERS: u32 = 65536u32;
pub const SHREGDEL_BOTH: SHREGDEL_FLAGS = 17i32;
pub const SHREGDEL_DEFAULT: SHREGDEL_FLAGS = 0i32;
pub const SHREGDEL_HKCU: SHREGDEL_FLAGS = 1i32;
pub const SHREGDEL_HKLM: SHREGDEL_FLAGS = 16i32;
pub const SHREGENUM_BOTH: SHREGENUM_FLAGS = 17i32;
pub const SHREGENUM_DEFAULT: SHREGENUM_FLAGS = 0i32;
pub const SHREGENUM_HKCU: SHREGENUM_FLAGS = 1i32;
pub const SHREGENUM_HKLM: SHREGENUM_FLAGS = 16i32;
pub const SHREGSET_FORCE_HKCU: u32 = 2u32;
pub const SHREGSET_FORCE_HKLM: u32 = 8u32;
pub const SHREGSET_HKCU: u32 = 1u32;
pub const SHREGSET_HKLM: u32 = 4u32;
pub const SIATTRIBFLAGS_ALLITEMS: SIATTRIBFLAGS = 16384i32;
pub const SIATTRIBFLAGS_AND: SIATTRIBFLAGS = 1i32;
pub const SIATTRIBFLAGS_APPCOMPAT: SIATTRIBFLAGS = 3i32;
pub const SIATTRIBFLAGS_MASK: SIATTRIBFLAGS = 3i32;
pub const SIATTRIBFLAGS_OR: SIATTRIBFLAGS = 2i32;
pub const SICHINT_ALLFIELDS: _SICHINTF = -2147483648i32;
pub const SICHINT_CANONICAL: _SICHINTF = 268435456i32;
pub const SICHINT_DISPLAY: _SICHINTF = 0i32;
pub const SICHINT_TEST_FILESYSPATH_IF_NOT_EQUAL: _SICHINTF = 536870912i32;
pub const SID_CommandsPropertyBag: windows_core::GUID = windows_core::GUID::from_u128(0x6e043250_4416_485c_b143_e62a760d9fe5);
pub const SID_CtxQueryAssociations: windows_core::GUID = windows_core::GUID::from_u128(0xfaadfc40_b777_4b69_aa81_77035ef0e6e8);
pub const SID_DefView: windows_core::GUID = windows_core::GUID::from_u128(0x6d12fe80_7911_11cf_9534_0000c05bae0b);
pub const SID_LaunchSourceAppUserModelId: windows_core::GUID = windows_core::GUID::from_u128(0x2ce78010_74db_48bc_9c6a_10f372495723);
pub const SID_LaunchSourceViewSizePreference: windows_core::GUID = windows_core::GUID::from_u128(0x80605492_67d9_414f_af89_a1cdf1242bc1);
pub const SID_LaunchTargetViewSizePreference: windows_core::GUID = windows_core::GUID::from_u128(0x26db2472_b7b7_406b_9702_730a4e20d3bf);
pub const SID_MenuShellFolder: windows_core::GUID = windows_core::GUID::from_u128(0xa6c17eb4_2d65_11d2_838f_00c04fd918d0);
pub const SID_SCommDlgBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x80f30233_b7df_11d2_a33b_006097df5bd4);
pub const SID_SCommandBarState: windows_core::GUID = windows_core::GUID::from_u128(0xb99eaa5c_3850_4400_bc33_2ce534048bf8);
pub const SID_SGetViewFromViewDual: windows_core::GUID = windows_core::GUID::from_u128(0x889a935d_971e_4b12_b90c_24dfc9e1e5e8);
pub const SID_SInPlaceBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x1d2ae02b_3655_46cc_b63a_285988153bca);
pub const SID_SMenuBandBKContextMenu: windows_core::GUID = windows_core::GUID::from_u128(0x164bbd86_1d0d_4de0_9a3b_d9729647c2b8);
pub const SID_SMenuBandBottom: windows_core::GUID = windows_core::GUID::from_u128(0x743ca664_0deb_11d1_9825_00c04fd91972);
pub const SID_SMenuBandBottomSelected: windows_core::GUID = windows_core::GUID::from_u128(0x165ebaf4_6d51_11d2_83ad_00c04fd918d0);
pub const SID_SMenuBandChild: windows_core::GUID = windows_core::GUID::from_u128(0xed9cc020_08b9_11d1_9823_00c04fd91972);
pub const SID_SMenuBandContextMenuModifier: windows_core::GUID = windows_core::GUID::from_u128(0x39545874_7162_465e_b783_2aa1874fef81);
pub const SID_SMenuBandParent: windows_core::GUID = windows_core::GUID::from_u128(0x8c278eec_3eab_11d1_8cb0_00c04fd918d0);
pub const SID_SMenuBandTop: windows_core::GUID = windows_core::GUID::from_u128(0x9493a810_ec38_11d0_bc46_00aa006ce2f5);
pub const SID_SMenuPopup: windows_core::GUID = windows_core::GUID::from_u128(0xd1e7afeb_6a2e_11d0_8c78_00c04fd918b4);
pub const SID_SSearchBoxInfo: windows_core::GUID = windows_core::GUID::from_u128(0x142daa61_516b_4713_b49c_fb985ef82998);
pub const SID_STopLevelBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x4c96be40_915c_11cf_99d3_00aa004ae837);
pub const SID_STopWindow: windows_core::GUID = windows_core::GUID::from_u128(0x49e1b500_4636_11d3_97f7_00c04f45d0b3);
pub const SID_ShellExecuteNamedPropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0xeb84ada2_00ff_4992_8324_ed5ce061cb29);
pub const SID_URLExecutionContext: windows_core::GUID = windows_core::GUID::from_u128(0xfb5f8ebc_bbb6_4d10_a461_777291a09030);
pub const SIGDN_DESKTOPABSOLUTEEDITING: SIGDN = -2147172352i32;
pub const SIGDN_DESKTOPABSOLUTEPARSING: SIGDN = -2147319808i32;
pub const SIGDN_FILESYSPATH: SIGDN = -2147123200i32;
pub const SIGDN_NORMALDISPLAY: SIGDN = 0i32;
pub const SIGDN_PARENTRELATIVE: SIGDN = -2146959359i32;
pub const SIGDN_PARENTRELATIVEEDITING: SIGDN = -2147282943i32;
pub const SIGDN_PARENTRELATIVEFORADDRESSBAR: SIGDN = -2146975743i32;
pub const SIGDN_PARENTRELATIVEFORUI: SIGDN = -2146877439i32;
pub const SIGDN_PARENTRELATIVEPARSING: SIGDN = -2147385343i32;
pub const SIGDN_URL: SIGDN = -2147057664i32;
pub const SIID_APPLICATION: SHSTOCKICONID = 2i32;
pub const SIID_AUDIOFILES: SHSTOCKICONID = 71i32;
pub const SIID_AUTOLIST: SHSTOCKICONID = 49i32;
pub const SIID_CLUSTEREDDRIVE: SHSTOCKICONID = 140i32;
pub const SIID_DELETE: SHSTOCKICONID = 84i32;
pub const SIID_DESKTOPPC: SHSTOCKICONID = 94i32;
pub const SIID_DEVICEAUDIOPLAYER: SHSTOCKICONID = 102i32;
pub const SIID_DEVICECAMERA: SHSTOCKICONID = 100i32;
pub const SIID_DEVICECELLPHONE: SHSTOCKICONID = 99i32;
pub const SIID_DEVICEVIDEOCAMERA: SHSTOCKICONID = 101i32;
pub const SIID_DOCASSOC: SHSTOCKICONID = 1i32;
pub const SIID_DOCNOASSOC: SHSTOCKICONID = 0i32;
pub const SIID_DRIVE35: SHSTOCKICONID = 6i32;
pub const SIID_DRIVE525: SHSTOCKICONID = 5i32;
pub const SIID_DRIVEBD: SHSTOCKICONID = 133i32;
pub const SIID_DRIVECD: SHSTOCKICONID = 11i32;
pub const SIID_DRIVEDVD: SHSTOCKICONID = 59i32;
pub const SIID_DRIVEFIXED: SHSTOCKICONID = 8i32;
pub const SIID_DRIVEHDDVD: SHSTOCKICONID = 132i32;
pub const SIID_DRIVENET: SHSTOCKICONID = 9i32;
pub const SIID_DRIVENETDISABLED: SHSTOCKICONID = 10i32;
pub const SIID_DRIVERAM: SHSTOCKICONID = 12i32;
pub const SIID_DRIVEREMOVE: SHSTOCKICONID = 7i32;
pub const SIID_DRIVEUNKNOWN: SHSTOCKICONID = 58i32;
pub const SIID_ERROR: SHSTOCKICONID = 80i32;
pub const SIID_FIND: SHSTOCKICONID = 22i32;
pub const SIID_FOLDER: SHSTOCKICONID = 3i32;
pub const SIID_FOLDERBACK: SHSTOCKICONID = 75i32;
pub const SIID_FOLDERFRONT: SHSTOCKICONID = 76i32;
pub const SIID_FOLDEROPEN: SHSTOCKICONID = 4i32;
pub const SIID_HELP: SHSTOCKICONID = 23i32;
pub const SIID_IMAGEFILES: SHSTOCKICONID = 72i32;
pub const SIID_INFO: SHSTOCKICONID = 79i32;
pub const SIID_INTERNET: SHSTOCKICONID = 104i32;
pub const SIID_KEY: SHSTOCKICONID = 81i32;
pub const SIID_LINK: SHSTOCKICONID = 29i32;
pub const SIID_LOCK: SHSTOCKICONID = 47i32;
pub const SIID_MAX_ICONS: SHSTOCKICONID = 181i32;
pub const SIID_MEDIAAUDIODVD: SHSTOCKICONID = 85i32;
pub const SIID_MEDIABDR: SHSTOCKICONID = 138i32;
pub const SIID_MEDIABDRE: SHSTOCKICONID = 139i32;
pub const SIID_MEDIABDROM: SHSTOCKICONID = 137i32;
pub const SIID_MEDIABLANKCD: SHSTOCKICONID = 69i32;
pub const SIID_MEDIABLURAY: SHSTOCKICONID = 90i32;
pub const SIID_MEDIACDAUDIO: SHSTOCKICONID = 40i32;
pub const SIID_MEDIACDAUDIOPLUS: SHSTOCKICONID = 65i32;
pub const SIID_MEDIACDBURN: SHSTOCKICONID = 68i32;
pub const SIID_MEDIACDR: SHSTOCKICONID = 67i32;
pub const SIID_MEDIACDROM: SHSTOCKICONID = 70i32;
pub const SIID_MEDIACDRW: SHSTOCKICONID = 66i32;
pub const SIID_MEDIACOMPACTFLASH: SHSTOCKICONID = 98i32;
pub const SIID_MEDIADVD: SHSTOCKICONID = 60i32;
pub const SIID_MEDIADVDPLUSR: SHSTOCKICONID = 92i32;
pub const SIID_MEDIADVDPLUSRW: SHSTOCKICONID = 93i32;
pub const SIID_MEDIADVDR: SHSTOCKICONID = 63i32;
pub const SIID_MEDIADVDRAM: SHSTOCKICONID = 61i32;
pub const SIID_MEDIADVDROM: SHSTOCKICONID = 64i32;
pub const SIID_MEDIADVDRW: SHSTOCKICONID = 62i32;
pub const SIID_MEDIAENHANCEDCD: SHSTOCKICONID = 87i32;
pub const SIID_MEDIAENHANCEDDVD: SHSTOCKICONID = 88i32;
pub const SIID_MEDIAHDDVD: SHSTOCKICONID = 89i32;
pub const SIID_MEDIAHDDVDR: SHSTOCKICONID = 135i32;
pub const SIID_MEDIAHDDVDRAM: SHSTOCKICONID = 136i32;
pub const SIID_MEDIAHDDVDROM: SHSTOCKICONID = 134i32;
pub const SIID_MEDIAMOVIEDVD: SHSTOCKICONID = 86i32;
pub const SIID_MEDIASMARTMEDIA: SHSTOCKICONID = 97i32;
pub const SIID_MEDIASVCD: SHSTOCKICONID = 56i32;
pub const SIID_MEDIAVCD: SHSTOCKICONID = 91i32;
pub const SIID_MIXEDFILES: SHSTOCKICONID = 74i32;
pub const SIID_MOBILEPC: SHSTOCKICONID = 95i32;
pub const SIID_MYNETWORK: SHSTOCKICONID = 17i32;
pub const SIID_NETWORKCONNECT: SHSTOCKICONID = 103i32;
pub const SIID_PRINTER: SHSTOCKICONID = 16i32;
pub const SIID_PRINTERFAX: SHSTOCKICONID = 52i32;
pub const SIID_PRINTERFAXNET: SHSTOCKICONID = 53i32;
pub const SIID_PRINTERFILE: SHSTOCKICONID = 54i32;
pub const SIID_PRINTERNET: SHSTOCKICONID = 50i32;
pub const SIID_RECYCLER: SHSTOCKICONID = 31i32;
pub const SIID_RECYCLERFULL: SHSTOCKICONID = 32i32;
pub const SIID_RENAME: SHSTOCKICONID = 83i32;
pub const SIID_SERVER: SHSTOCKICONID = 15i32;
pub const SIID_SERVERSHARE: SHSTOCKICONID = 51i32;
pub const SIID_SETTINGS: SHSTOCKICONID = 106i32;
pub const SIID_SHARE: SHSTOCKICONID = 28i32;
pub const SIID_SHIELD: SHSTOCKICONID = 77i32;
pub const SIID_SLOWFILE: SHSTOCKICONID = 30i32;
pub const SIID_SOFTWARE: SHSTOCKICONID = 82i32;
pub const SIID_STACK: SHSTOCKICONID = 55i32;
pub const SIID_STUFFEDFOLDER: SHSTOCKICONID = 57i32;
pub const SIID_USERS: SHSTOCKICONID = 96i32;
pub const SIID_VIDEOFILES: SHSTOCKICONID = 73i32;
pub const SIID_WARNING: SHSTOCKICONID = 78i32;
pub const SIID_WORLD: SHSTOCKICONID = 13i32;
pub const SIID_ZIPFILE: SHSTOCKICONID = 105i32;
pub const SIIGBF_BIGGERSIZEOK: SIIGBF = 1i32;
pub const SIIGBF_CROPTOSQUARE: SIIGBF = 32i32;
pub const SIIGBF_ICONBACKGROUND: SIIGBF = 128i32;
pub const SIIGBF_ICONONLY: SIIGBF = 4i32;
pub const SIIGBF_INCACHEONLY: SIIGBF = 16i32;
pub const SIIGBF_MEMORYONLY: SIIGBF = 2i32;
pub const SIIGBF_RESIZETOFIT: SIIGBF = 0i32;
pub const SIIGBF_SCALEUP: SIIGBF = 256i32;
pub const SIIGBF_THUMBNAILONLY: SIIGBF = 8i32;
pub const SIIGBF_WIDETHUMBNAILS: SIIGBF = 64i32;
pub const SIOM_ICONINDEX: u32 = 2u32;
pub const SIOM_OVERLAYINDEX: u32 = 1u32;
pub const SIOM_RESERVED_DEFAULT: u32 = 3u32;
pub const SIOM_RESERVED_LINK: u32 = 1u32;
pub const SIOM_RESERVED_SHARED: u32 = 0u32;
pub const SIOM_RESERVED_SLOWFILE: u32 = 2u32;
pub const SLDF_ALLOW_LINK_TO_LINK: SHELL_LINK_DATA_FLAGS = 8388608i32;
pub const SLDF_DEFAULT: SHELL_LINK_DATA_FLAGS = 0i32;
pub const SLDF_DISABLE_KNOWNFOLDER_RELATIVE_TRACKING: SHELL_LINK_DATA_FLAGS = 2097152i32;
pub const SLDF_DISABLE_LINK_PATH_TRACKING: SHELL_LINK_DATA_FLAGS = 1048576i32;
pub const SLDF_ENABLE_TARGET_METADATA: SHELL_LINK_DATA_FLAGS = 524288i32;
pub const SLDF_FORCE_NO_LINKINFO: SHELL_LINK_DATA_FLAGS = 256i32;
pub const SLDF_FORCE_NO_LINKTRACK: SHELL_LINK_DATA_FLAGS = 262144i32;
pub const SLDF_FORCE_UNCNAME: SHELL_LINK_DATA_FLAGS = 65536i32;
pub const SLDF_HAS_ARGS: SHELL_LINK_DATA_FLAGS = 32i32;
pub const SLDF_HAS_DARWINID: SHELL_LINK_DATA_FLAGS = 4096i32;
pub const SLDF_HAS_EXP_ICON_SZ: SHELL_LINK_DATA_FLAGS = 16384i32;
pub const SLDF_HAS_EXP_SZ: SHELL_LINK_DATA_FLAGS = 512i32;
pub const SLDF_HAS_ICONLOCATION: SHELL_LINK_DATA_FLAGS = 64i32;
pub const SLDF_HAS_ID_LIST: SHELL_LINK_DATA_FLAGS = 1i32;
pub const SLDF_HAS_LINK_INFO: SHELL_LINK_DATA_FLAGS = 2i32;
pub const SLDF_HAS_NAME: SHELL_LINK_DATA_FLAGS = 4i32;
pub const SLDF_HAS_RELPATH: SHELL_LINK_DATA_FLAGS = 8i32;
pub const SLDF_HAS_WORKINGDIR: SHELL_LINK_DATA_FLAGS = 16i32;
pub const SLDF_KEEP_LOCAL_IDLIST_FOR_UNC_TARGET: SHELL_LINK_DATA_FLAGS = 67108864i32;
pub const SLDF_NO_KF_ALIAS: SHELL_LINK_DATA_FLAGS = 4194304i32;
pub const SLDF_NO_PIDL_ALIAS: SHELL_LINK_DATA_FLAGS = 32768i32;
pub const SLDF_PERSIST_VOLUME_ID_RELATIVE: SHELL_LINK_DATA_FLAGS = 134217728i32;
pub const SLDF_PREFER_ENVIRONMENT_PATH: SHELL_LINK_DATA_FLAGS = 33554432i32;
pub const SLDF_RESERVED: SHELL_LINK_DATA_FLAGS = -2147483648i32;
pub const SLDF_RUNAS_USER: SHELL_LINK_DATA_FLAGS = 8192i32;
pub const SLDF_RUN_IN_SEPARATE: SHELL_LINK_DATA_FLAGS = 1024i32;
pub const SLDF_RUN_WITH_SHIMLAYER: SHELL_LINK_DATA_FLAGS = 131072i32;
pub const SLDF_UNALIAS_ON_SAVE: SHELL_LINK_DATA_FLAGS = 16777216i32;
pub const SLDF_UNICODE: SHELL_LINK_DATA_FLAGS = 128i32;
pub const SLDF_VALID: SHELL_LINK_DATA_FLAGS = 268433407i32;
pub const SLGP_RAWPATH: SLGP_FLAGS = 4i32;
pub const SLGP_RELATIVEPRIORITY: SLGP_FLAGS = 8i32;
pub const SLGP_SHORTPATH: SLGP_FLAGS = 1i32;
pub const SLGP_UNCPRIORITY: SLGP_FLAGS = 2i32;
pub const SLR_ANY_MATCH: SLR_FLAGS = 2i32;
pub const SLR_INVOKE_MSI: SLR_FLAGS = 128i32;
pub const SLR_KNOWNFOLDER: SLR_FLAGS = 1024i32;
pub const SLR_MACHINE_IN_LOCAL_TARGET: SLR_FLAGS = 2048i32;
pub const SLR_NOLINKINFO: SLR_FLAGS = 64i32;
pub const SLR_NONE: SLR_FLAGS = 0i32;
pub const SLR_NOSEARCH: SLR_FLAGS = 16i32;
pub const SLR_NOTRACK: SLR_FLAGS = 32i32;
pub const SLR_NOUPDATE: SLR_FLAGS = 8i32;
pub const SLR_NO_OBJECT_ID: SLR_FLAGS = 8192i32;
pub const SLR_NO_UI: SLR_FLAGS = 1i32;
pub const SLR_NO_UI_WITH_MSG_PUMP: SLR_FLAGS = 257i32;
pub const SLR_OFFER_DELETE_WITHOUT_FILE: SLR_FLAGS = 512i32;
pub const SLR_UPDATE: SLR_FLAGS = 4i32;
pub const SLR_UPDATE_MACHINE_AND_SID: SLR_FLAGS = 4096i32;
pub const SMAE_CONTRACTED: u32 = 2u32;
pub const SMAE_EXPANDED: u32 = 1u32;
pub const SMAE_USER: u32 = 4u32;
pub const SMAE_VALID: u32 = 7u32;
pub const SMC_AUTOEXPANDCHANGE: u32 = 66u32;
pub const SMC_CHEVRONEXPAND: u32 = 25u32;
pub const SMC_CHEVRONGETTIP: u32 = 47u32;
pub const SMC_CREATE: u32 = 2u32;
pub const SMC_DEFAULTICON: u32 = 22u32;
pub const SMC_DEMOTE: u32 = 17u32;
pub const SMC_DISPLAYCHEVRONTIP: u32 = 42u32;
pub const SMC_EXITMENU: u32 = 3u32;
pub const SMC_GETAUTOEXPANDSTATE: u32 = 65u32;
pub const SMC_GETBKCONTEXTMENU: u32 = 68u32;
pub const SMC_GETCONTEXTMENUMODIFIER: u32 = 67u32;
pub const SMC_GETINFO: u32 = 5u32;
pub const SMC_GETOBJECT: u32 = 7u32;
pub const SMC_GETSFINFO: u32 = 6u32;
pub const SMC_GETSFOBJECT: u32 = 8u32;
pub const SMC_INITMENU: u32 = 1u32;
pub const SMC_NEWITEM: u32 = 23u32;
pub const SMC_OPEN: u32 = 69u32;
pub const SMC_PROMOTE: u32 = 18u32;
pub const SMC_REFRESH: u32 = 16u32;
pub const SMC_SETSFOBJECT: u32 = 45u32;
pub const SMC_SFDDRESTRICTED: u32 = 48u32;
pub const SMC_SFEXEC: u32 = 9u32;
pub const SMC_SFEXEC_MIDDLE: u32 = 49u32;
pub const SMC_SFSELECTITEM: u32 = 10u32;
pub const SMC_SHCHANGENOTIFY: u32 = 46u32;
pub const SMDM_HMENU: u32 = 2u32;
pub const SMDM_SHELLFOLDER: u32 = 1u32;
pub const SMDM_TOOLBAR: u32 = 4u32;
pub const SMIF_ACCELERATOR: SMINFOFLAGS = 2i32;
pub const SMIF_ALTSTATE: SMINFOFLAGS = 2048i32;
pub const SMIF_CHECKED: SMINFOFLAGS = 32i32;
pub const SMIF_DEMOTED: SMINFOFLAGS = 1024i32;
pub const SMIF_DISABLED: SMINFOFLAGS = 256i32;
pub const SMIF_DRAGNDROP: SMINFOFLAGS = 4096i32;
pub const SMIF_DROPCASCADE: SMINFOFLAGS = 64i32;
pub const SMIF_DROPTARGET: SMINFOFLAGS = 4i32;
pub const SMIF_HIDDEN: SMINFOFLAGS = 128i32;
pub const SMIF_ICON: SMINFOFLAGS = 1i32;
pub const SMIF_NEW: SMINFOFLAGS = 8192i32;
pub const SMIF_SUBMENU: SMINFOFLAGS = 8i32;
pub const SMIF_TRACKPOPUP: SMINFOFLAGS = 512i32;
pub const SMIM_FLAGS: SMINFOMASK = 2i32;
pub const SMIM_ICON: SMINFOMASK = 4i32;
pub const SMIM_TYPE: SMINFOMASK = 1i32;
pub const SMINIT_AUTOEXPAND: u32 = 256u32;
pub const SMINIT_AUTOTOOLTIP: u32 = 512u32;
pub const SMINIT_CACHED: u32 = 16u32;
pub const SMINIT_DEFAULT: u32 = 0u32;
pub const SMINIT_DROPONCONTAINER: u32 = 1024u32;
pub const SMINIT_HORIZONTAL: u32 = 536870912u32;
pub const SMINIT_RESTRICT_DRAGDROP: u32 = 2u32;
pub const SMINIT_TOPLEVEL: u32 = 4u32;
pub const SMINIT_VERTICAL: u32 = 268435456u32;
pub const SMINV_ID: u32 = 8u32;
pub const SMINV_REFRESH: u32 = 1u32;
pub const SMIT_SEPARATOR: SMINFOTYPE = 1i32;
pub const SMIT_STRING: SMINFOTYPE = 2i32;
pub const SMSET_BOTTOM: u32 = 536870912u32;
pub const SMSET_DONTOWN: u32 = 1u32;
pub const SMSET_TOP: u32 = 268435456u32;
pub const SORT_ASCENDING: SORTDIRECTION = 1i32;
pub const SORT_DESCENDING: SORTDIRECTION = -1i32;
pub const SOT_DEFAULT: SORT_ORDER_TYPE = 0i32;
pub const SOT_IGNORE_FOLDERNESS: SORT_ORDER_TYPE = 1i32;
pub const SPACTION_APPLYINGATTRIBS: SPACTION = 4i32;
pub const SPACTION_CALCULATING: SPACTION = 7i32;
pub const SPACTION_COPYING: SPACTION = 2i32;
pub const SPACTION_COPY_MOVING: SPACTION = 13i32;
pub const SPACTION_DELETING: SPACTION = 10i32;
pub const SPACTION_DOWNLOADING: SPACTION = 5i32;
pub const SPACTION_FORMATTING: SPACTION = 12i32;
pub const SPACTION_MOVING: SPACTION = 1i32;
pub const SPACTION_NONE: SPACTION = 0i32;
pub const SPACTION_RECYCLING: SPACTION = 3i32;
pub const SPACTION_RENAMING: SPACTION = 11i32;
pub const SPACTION_SEARCHING_FILES: SPACTION = 9i32;
pub const SPACTION_SEARCHING_INTERNET: SPACTION = 6i32;
pub const SPACTION_UPLOADING: SPACTION = 8i32;
pub const SPBEGINF_AUTOTIME: _SPBEGINF = 2i32;
pub const SPBEGINF_MARQUEEPROGRESS: _SPBEGINF = 32i32;
pub const SPBEGINF_NOCANCELBUTTON: _SPBEGINF = 64i32;
pub const SPBEGINF_NOPROGRESSBAR: _SPBEGINF = 16i32;
pub const SPBEGINF_NORMAL: _SPBEGINF = 0i32;
pub const SPFF_CREATED_ON_THIS_DEVICE: STORAGE_PROVIDER_FILE_FLAGS = 2i32;
pub const SPFF_DOWNLOAD_BY_DEFAULT: STORAGE_PROVIDER_FILE_FLAGS = 1i32;
pub const SPFF_NONE: STORAGE_PROVIDER_FILE_FLAGS = 0i32;
pub const SPINITF_MODAL: _SPINITF = 1i32;
pub const SPINITF_NOMINIMIZE: _SPINITF = 8i32;
pub const SPINITF_NORMAL: _SPINITF = 0i32;
pub const SPMODE_BROWSER: u32 = 8u32;
pub const SPMODE_DBMON: u32 = 8192u32;
pub const SPMODE_DEBUGBREAK: u32 = 512u32;
pub const SPMODE_DEBUGOUT: u32 = 2u32;
pub const SPMODE_EVENT: u32 = 32u32;
pub const SPMODE_EVENTTRACE: u32 = 32768u32;
pub const SPMODE_FLUSH: u32 = 16u32;
pub const SPMODE_FORMATTEXT: u32 = 128u32;
pub const SPMODE_MEMWATCH: u32 = 4096u32;
pub const SPMODE_MSGTRACE: u32 = 1024u32;
pub const SPMODE_MSVM: u32 = 64u32;
pub const SPMODE_MULTISTOP: u32 = 16384u32;
pub const SPMODE_PERFTAGS: u32 = 2048u32;
pub const SPMODE_PROFILE: u32 = 256u32;
pub const SPMODE_SHELL: u32 = 1u32;
pub const SPMODE_TEST: u32 = 4u32;
pub const SPTEXT_ACTIONDESCRIPTION: SPTEXT = 1i32;
pub const SPTEXT_ACTIONDETAIL: SPTEXT = 2i32;
pub const SRRF_NOEXPAND: u32 = 268435456u32;
pub const SRRF_NOVIRT: u32 = 1073741824u32;
pub const SRRF_RM_ANY: u32 = 0u32;
pub const SRRF_RM_NORMAL: u32 = 65536u32;
pub const SRRF_RM_SAFE: u32 = 131072u32;
pub const SRRF_RM_SAFENETWORK: u32 = 262144u32;
pub const SRRF_RT_ANY: u32 = 65535u32;
pub const SRRF_RT_REG_BINARY: u32 = 8u32;
pub const SRRF_RT_REG_DWORD: u32 = 16u32;
pub const SRRF_RT_REG_EXPAND_SZ: u32 = 4u32;
pub const SRRF_RT_REG_MULTI_SZ: u32 = 32u32;
pub const SRRF_RT_REG_NONE: u32 = 1u32;
pub const SRRF_RT_REG_QWORD: u32 = 64u32;
pub const SRRF_RT_REG_SZ: u32 = 2u32;
pub const SRRF_ZEROONFAILURE: u32 = 536870912u32;
pub const SSF_AUTOCHECKSELECT: SSF_MASK = 8388608u32;
pub const SSF_DESKTOPHTML: SSF_MASK = 512u32;
pub const SSF_DONTPRETTYPATH: SSF_MASK = 2048u32;
pub const SSF_DOUBLECLICKINWEBVIEW: SSF_MASK = 128u32;
pub const SSF_FILTER: SSF_MASK = 65536u32;
pub const SSF_HIDDENFILEEXTS: SSF_MASK = 4u32;
pub const SSF_HIDEICONS: SSF_MASK = 16384u32;
pub const SSF_ICONSONLY: SSF_MASK = 16777216u32;
pub const SSF_MAPNETDRVBUTTON: SSF_MASK = 4096u32;
pub const SSF_NOCONFIRMRECYCLE: SSF_MASK = 32768u32;
pub const SSF_NONETCRAWLING: SSF_MASK = 1048576u32;
pub const SSF_SEPPROCESS: SSF_MASK = 524288u32;
pub const SSF_SERVERADMINUI: SSF_MASK = 4u32;
pub const SSF_SHOWALLOBJECTS: SSF_MASK = 1u32;
pub const SSF_SHOWATTRIBCOL: SSF_MASK = 256u32;
pub const SSF_SHOWCOMPCOLOR: SSF_MASK = 8u32;
pub const SSF_SHOWEXTENSIONS: SSF_MASK = 2u32;
pub const SSF_SHOWINFOTIP: SSF_MASK = 8192u32;
pub const SSF_SHOWSTARTPAGE: SSF_MASK = 4194304u32;
pub const SSF_SHOWSTATUSBAR: SSF_MASK = 67108864u32;
pub const SSF_SHOWSUPERHIDDEN: SSF_MASK = 262144u32;
pub const SSF_SHOWSYSFILES: SSF_MASK = 32u32;
pub const SSF_SHOWTYPEOVERLAY: SSF_MASK = 33554432u32;
pub const SSF_SORTCOLUMNS: SSF_MASK = 16u32;
pub const SSF_STARTPANELON: SSF_MASK = 2097152u32;
pub const SSF_WEBVIEW: SSF_MASK = 131072u32;
pub const SSF_WIN95CLASSIC: SSF_MASK = 1024u32;
pub const SSM_CLEAR: u32 = 0u32;
pub const SSM_REFRESH: u32 = 2u32;
pub const SSM_SET: u32 = 1u32;
pub const SSM_UPDATE: u32 = 4u32;
pub const STGOP_APPLYPROPERTIES: STGOP = 8i32;
pub const STGOP_COPY: STGOP = 2i32;
pub const STGOP_MOVE: STGOP = 1i32;
pub const STGOP_NEW: STGOP = 10i32;
pub const STGOP_REMOVE: STGOP = 5i32;
pub const STGOP_RENAME: STGOP = 6i32;
pub const STGOP_SYNC: STGOP = 3i32;
pub const STIF_DEFAULT: i32 = 0i32;
pub const STIF_SUPPORT_HEX: i32 = 1i32;
pub const STORE_E_NEWER_VERSION_AVAILABLE: windows_core::HRESULT = 0x80270104_u32 as _;
pub const STPF_NONE: STPFLAG = 0i32;
pub const STPF_USEAPPPEEKALWAYS: STPFLAG = 4i32;
pub const STPF_USEAPPPEEKWHENACTIVE: STPFLAG = 8i32;
pub const STPF_USEAPPTHUMBNAILALWAYS: STPFLAG = 1i32;
pub const STPF_USEAPPTHUMBNAILWHENACTIVE: STPFLAG = 2i32;
pub const STR_AVOID_DRIVE_RESTRICTION_POLICY: windows_core::PCWSTR = windows_core::w!("Avoid Drive Restriction Policy");
pub const STR_BIND_DELEGATE_CREATE_OBJECT: windows_core::PCWSTR = windows_core::w!("Delegate Object Creation");
pub const STR_BIND_FOLDERS_READ_ONLY: windows_core::PCWSTR = windows_core::w!("Folders As Read Only");
pub const STR_BIND_FOLDER_ENUM_MODE: windows_core::PCWSTR = windows_core::w!("Folder Enum Mode");
pub const STR_BIND_FORCE_FOLDER_SHORTCUT_RESOLVE: windows_core::PCWSTR = windows_core::w!("Force Folder Shortcut Resolve");
pub const STR_DONT_PARSE_RELATIVE: windows_core::PCWSTR = windows_core::w!("Don\'t Parse Relative");
pub const STR_DONT_RESOLVE_LINK: windows_core::PCWSTR = windows_core::w!("Don\'t Resolve Link");
pub const STR_ENUM_ITEMS_FLAGS: windows_core::PCWSTR = windows_core::w!("SHCONTF");
pub const STR_FILE_SYS_BIND_DATA: windows_core::PCWSTR = windows_core::w!("File System Bind Data");
pub const STR_FILE_SYS_BIND_DATA_WIN7_FORMAT: windows_core::PCWSTR = windows_core::w!("Win7FileSystemIdList");
pub const STR_GET_ASYNC_HANDLER: windows_core::PCWSTR = windows_core::w!("GetAsyncHandler");
pub const STR_GPS_BESTEFFORT: windows_core::PCWSTR = windows_core::w!("GPS_BESTEFFORT");
pub const STR_GPS_DELAYCREATION: windows_core::PCWSTR = windows_core::w!("GPS_DELAYCREATION");
pub const STR_GPS_FASTPROPERTIESONLY: windows_core::PCWSTR = windows_core::w!("GPS_FASTPROPERTIESONLY");
pub const STR_GPS_HANDLERPROPERTIESONLY: windows_core::PCWSTR = windows_core::w!("GPS_HANDLERPROPERTIESONLY");
pub const STR_GPS_NO_OPLOCK: windows_core::PCWSTR = windows_core::w!("GPS_NO_OPLOCK");
pub const STR_GPS_OPENSLOWITEM: windows_core::PCWSTR = windows_core::w!("GPS_OPENSLOWITEM");
pub const STR_INTERNAL_NAVIGATE: windows_core::PCWSTR = windows_core::w!("Internal Navigation");
pub const STR_INTERNETFOLDER_PARSE_ONLY_URLMON_BINDABLE: windows_core::PCWSTR = windows_core::w!("Validate URL");
pub const STR_ITEM_CACHE_CONTEXT: windows_core::PCWSTR = windows_core::w!("ItemCacheContext");
pub const STR_MYDOCS_CLSID: windows_core::PCSTR = windows_core::s!("{450D8FBA-AD25-11D0-98A8-0800361B1103}");
pub const STR_NO_VALIDATE_FILENAME_CHARS: windows_core::PCWSTR = windows_core::w!("NoValidateFilenameChars");
pub const STR_PARSE_ALLOW_INTERNET_SHELL_FOLDERS: windows_core::PCWSTR = windows_core::w!("Allow binding to Internet shell folder handlers and negate STR_PARSE_PREFER_WEB_BROWSING");
pub const STR_PARSE_AND_CREATE_ITEM: windows_core::PCWSTR = windows_core::w!("ParseAndCreateItem");
pub const STR_PARSE_DONT_REQUIRE_VALIDATED_URLS: windows_core::PCWSTR = windows_core::w!("Do not require validated URLs");
pub const STR_PARSE_EXPLICIT_ASSOCIATION_SUCCESSFUL: windows_core::PCWSTR = windows_core::w!("ExplicitAssociationSuccessful");
pub const STR_PARSE_PARTIAL_IDLIST: windows_core::PCWSTR = windows_core::w!("ParseOriginalItem");
pub const STR_PARSE_PREFER_FOLDER_BROWSING: windows_core::PCWSTR = windows_core::w!("Parse Prefer Folder Browsing");
pub const STR_PARSE_PREFER_WEB_BROWSING: windows_core::PCWSTR = windows_core::w!("Do not bind to Internet shell folder handlers");
pub const STR_PARSE_PROPERTYSTORE: windows_core::PCWSTR = windows_core::w!("DelegateNamedProperties");
pub const STR_PARSE_SHELL_PROTOCOL_TO_FILE_OBJECTS: windows_core::PCWSTR = windows_core::w!("Parse Shell Protocol To File Objects");
pub const STR_PARSE_SHOW_NET_DIAGNOSTICS_UI: windows_core::PCWSTR = windows_core::w!("Show network diagnostics UI");
pub const STR_PARSE_SKIP_NET_CACHE: windows_core::PCWSTR = windows_core::w!("Skip Net Resource Cache");
pub const STR_PARSE_TRANSLATE_ALIASES: windows_core::PCWSTR = windows_core::w!("Parse Translate Aliases");
pub const STR_PARSE_WITH_EXPLICIT_ASSOCAPP: windows_core::PCWSTR = windows_core::w!("ExplicitAssociationApp");
pub const STR_PARSE_WITH_EXPLICIT_PROGID: windows_core::PCWSTR = windows_core::w!("ExplicitProgid");
pub const STR_PARSE_WITH_PROPERTIES: windows_core::PCWSTR = windows_core::w!("ParseWithProperties");
pub const STR_PROPERTYBAG_PARAM: windows_core::PCWSTR = windows_core::w!("SHBindCtxPropertyBag");
pub const STR_REFERRER_IDENTIFIER: windows_core::PCWSTR = windows_core::w!("Referrer Identifier");
pub const STR_SKIP_BINDING_CLSID: windows_core::PCWSTR = windows_core::w!("Skip Binding CLSID");
pub const STR_STORAGEITEM_CREATION_FLAGS: windows_core::PCWSTR = windows_core::w!("SHGETSTORAGEITEM");
pub const STR_TAB_REUSE_IDENTIFIER: windows_core::PCWSTR = windows_core::w!("Tab Reuse Identifier");
pub const STR_TRACK_CLSID: windows_core::PCWSTR = windows_core::w!("Track the CLSID");
pub const SV3CVW3_DEFAULT: _SV3CVW3_FLAGS = 0i32;
pub const SV3CVW3_FORCEFOLDERFLAGS: _SV3CVW3_FLAGS = 4i32;
pub const SV3CVW3_FORCEVIEWMODE: _SV3CVW3_FLAGS = 2i32;
pub const SV3CVW3_NONINTERACTIVE: _SV3CVW3_FLAGS = 1i32;
pub const SVGIO_ALLVIEW: _SVGIO = 2i32;
pub const SVGIO_BACKGROUND: _SVGIO = 0i32;
pub const SVGIO_CHECKED: _SVGIO = 3i32;
pub const SVGIO_FLAG_VIEWORDER: _SVGIO = -2147483648i32;
pub const SVGIO_SELECTION: _SVGIO = 1i32;
pub const SVGIO_TYPE_MASK: _SVGIO = 15i32;
pub const SVSI_CHECK: _SVSIF = 256i32;
pub const SVSI_CHECK2: _SVSIF = 512i32;
pub const SVSI_DESELECT: _SVSIF = 0i32;
pub const SVSI_DESELECTOTHERS: _SVSIF = 4i32;
pub const SVSI_EDIT: _SVSIF = 3i32;
pub const SVSI_ENSUREVISIBLE: _SVSIF = 8i32;
pub const SVSI_FOCUSED: _SVSIF = 16i32;
pub const SVSI_KEYBOARDSELECT: _SVSIF = 1025i32;
pub const SVSI_NOTAKEFOCUS: _SVSIF = 1073741824i32;
pub const SVSI_POSITIONITEM: _SVSIF = 128i32;
pub const SVSI_SELECT: _SVSIF = 1i32;
pub const SVSI_SELECTIONMARK: _SVSIF = 64i32;
pub const SVSI_TRANSLATEPT: _SVSIF = 32i32;
pub const SVUIA_ACTIVATE_FOCUS: SVUIA_STATUS = 2i32;
pub const SVUIA_ACTIVATE_NOFOCUS: SVUIA_STATUS = 1i32;
pub const SVUIA_DEACTIVATE: SVUIA_STATUS = 0i32;
pub const SVUIA_INPLACEACTIVATE: SVUIA_STATUS = 3i32;
pub const SWC_3RDPARTY: ShellWindowTypeConstants = 2i32;
pub const SWC_BROWSER: ShellWindowTypeConstants = 1i32;
pub const SWC_CALLBACK: ShellWindowTypeConstants = 4i32;
pub const SWC_DESKTOP: ShellWindowTypeConstants = 8i32;
pub const SWC_EXPLORER: ShellWindowTypeConstants = 0i32;
pub const SWFO_COOKIEPASSED: ShellWindowFindWindowOptions = 4i32;
pub const SWFO_INCLUDEPENDING: ShellWindowFindWindowOptions = 2i32;
pub const SWFO_NEEDDISPATCH: ShellWindowFindWindowOptions = 1i32;
pub const SYNCMGRERRORFLAG_ENABLEJUMPTEXT: SYNCMGRERRORFLAGS = 1i32;
pub const SYNCMGRFLAG_CONNECT: SYNCMGRFLAG = 1i32;
pub const SYNCMGRFLAG_EVENTMASK: SYNCMGRFLAG = 255i32;
pub const SYNCMGRFLAG_IDLE: SYNCMGRFLAG = 4i32;
pub const SYNCMGRFLAG_INVOKE: SYNCMGRFLAG = 5i32;
pub const SYNCMGRFLAG_MANUAL: SYNCMGRFLAG = 3i32;
pub const SYNCMGRFLAG_MAYBOTHERUSER: SYNCMGRFLAG = 512i32;
pub const SYNCMGRFLAG_PENDINGDISCONNECT: SYNCMGRFLAG = 2i32;
pub const SYNCMGRFLAG_SCHEDULED: SYNCMGRFLAG = 6i32;
pub const SYNCMGRFLAG_SETTINGS: SYNCMGRFLAG = 256i32;
pub const SYNCMGRHANDLERFLAG_MASK: u32 = 15u32;
pub const SYNCMGRHANDLER_ALWAYSLISTHANDLER: SYNCMGRHANDLERFLAGS = 4i32;
pub const SYNCMGRHANDLER_HASPROPERTIES: SYNCMGRHANDLERFLAGS = 1i32;
pub const SYNCMGRHANDLER_HIDDEN: SYNCMGRHANDLERFLAGS = 8i32;
pub const SYNCMGRHANDLER_MAYESTABLISHCONNECTION: SYNCMGRHANDLERFLAGS = 2i32;
pub const SYNCMGRINVOKE_MINIMIZED: SYNCMGRINVOKEFLAGS = 4i32;
pub const SYNCMGRINVOKE_STARTSYNC: SYNCMGRINVOKEFLAGS = 2i32;
pub const SYNCMGRITEMSTATE_CHECKED: SYNCMGRITEMSTATE = 1i32;
pub const SYNCMGRITEMSTATE_UNCHECKED: SYNCMGRITEMSTATE = 0i32;
pub const SYNCMGRITEM_HASPROPERTIES: SYNCMGRITEMFLAGS = 1i32;
pub const SYNCMGRITEM_HIDDEN: SYNCMGRITEMFLAGS = 32i32;
pub const SYNCMGRITEM_ITEMFLAGMASK: u32 = 127u32;
pub const SYNCMGRITEM_LASTUPDATETIME: SYNCMGRITEMFLAGS = 8i32;
pub const SYNCMGRITEM_MAYDELETEITEM: SYNCMGRITEMFLAGS = 16i32;
pub const SYNCMGRITEM_ROAMINGUSER: SYNCMGRITEMFLAGS = 4i32;
pub const SYNCMGRITEM_TEMPORARY: SYNCMGRITEMFLAGS = 2i32;
pub const SYNCMGRLOGERROR_ERRORFLAGS: u32 = 1u32;
pub const SYNCMGRLOGERROR_ERRORID: u32 = 2u32;
pub const SYNCMGRLOGERROR_ITEMID: u32 = 4u32;
pub const SYNCMGRLOGLEVEL_ERROR: SYNCMGRLOGLEVEL = 3i32;
pub const SYNCMGRLOGLEVEL_INFORMATION: SYNCMGRLOGLEVEL = 1i32;
pub const SYNCMGRLOGLEVEL_LOGLEVELMAX: SYNCMGRLOGLEVEL = 3i32;
pub const SYNCMGRLOGLEVEL_WARNING: SYNCMGRLOGLEVEL = 2i32;
pub const SYNCMGRPROGRESSITEM_MAXVALUE: u32 = 8u32;
pub const SYNCMGRPROGRESSITEM_PROGVALUE: u32 = 4u32;
pub const SYNCMGRPROGRESSITEM_STATUSTEXT: u32 = 1u32;
pub const SYNCMGRPROGRESSITEM_STATUSTYPE: u32 = 2u32;
pub const SYNCMGRREGISTERFLAGS_MASK: u32 = 7u32;
pub const SYNCMGRREGISTERFLAG_CONNECT: SYNCMGRREGISTERFLAGS = 1i32;
pub const SYNCMGRREGISTERFLAG_IDLE: SYNCMGRREGISTERFLAGS = 4i32;
pub const SYNCMGRREGISTERFLAG_PENDINGDISCONNECT: SYNCMGRREGISTERFLAGS = 2i32;
pub const SYNCMGRSTATUS_DELETED: SYNCMGRSTATUS = 256i32;
pub const SYNCMGRSTATUS_FAILED: SYNCMGRSTATUS = 5i32;
pub const SYNCMGRSTATUS_PAUSED: SYNCMGRSTATUS = 6i32;
pub const SYNCMGRSTATUS_PENDING: SYNCMGRSTATUS = 2i32;
pub const SYNCMGRSTATUS_RESUMING: SYNCMGRSTATUS = 7i32;
pub const SYNCMGRSTATUS_SKIPPED: SYNCMGRSTATUS = 1i32;
pub const SYNCMGRSTATUS_STOPPED: SYNCMGRSTATUS = 0i32;
pub const SYNCMGRSTATUS_SUCCEEDED: SYNCMGRSTATUS = 4i32;
pub const SYNCMGRSTATUS_UPDATING: SYNCMGRSTATUS = 3i32;
pub const SYNCMGRSTATUS_UPDATING_INDETERMINATE: SYNCMGRSTATUS = 8i32;
pub const SYNCMGR_CF_NONE: SYNCMGR_CONTROL_FLAGS = 0i32;
pub const SYNCMGR_CF_NOUI: SYNCMGR_CONTROL_FLAGS = 2i32;
pub const SYNCMGR_CF_NOWAIT: SYNCMGR_CONTROL_FLAGS = 0i32;
pub const SYNCMGR_CF_VALID: SYNCMGR_CONTROL_FLAGS = 3i32;
pub const SYNCMGR_CF_WAIT: SYNCMGR_CONTROL_FLAGS = 1i32;
pub const SYNCMGR_CIT_DELETED: SYNCMGR_CONFLICT_ITEM_TYPE = 2i32;
pub const SYNCMGR_CIT_UPDATED: SYNCMGR_CONFLICT_ITEM_TYPE = 1i32;
pub const SYNCMGR_CR_CANCEL_ALL: SYNCMGR_CANCEL_REQUEST = 2i32;
pub const SYNCMGR_CR_CANCEL_ITEM: SYNCMGR_CANCEL_REQUEST = 1i32;
pub const SYNCMGR_CR_MAX: SYNCMGR_CANCEL_REQUEST = 2i32;
pub const SYNCMGR_CR_NONE: SYNCMGR_CANCEL_REQUEST = 0i32;
pub const SYNCMGR_EF_NONE: SYNCMGR_EVENT_FLAGS = 0i32;
pub const SYNCMGR_EF_VALID: SYNCMGR_EVENT_FLAGS = 0i32;
pub const SYNCMGR_EL_ERROR: SYNCMGR_EVENT_LEVEL = 3i32;
pub const SYNCMGR_EL_INFORMATION: SYNCMGR_EVENT_LEVEL = 1i32;
pub const SYNCMGR_EL_MAX: SYNCMGR_EVENT_LEVEL = 3i32;
pub const SYNCMGR_EL_WARNING: SYNCMGR_EVENT_LEVEL = 2i32;
pub const SYNCMGR_HCM_CAN_BROWSE_CONTENT: SYNCMGR_HANDLER_CAPABILITIES = 65536i32;
pub const SYNCMGR_HCM_CAN_SHOW_SCHEDULE: SYNCMGR_HANDLER_CAPABILITIES = 131072i32;
pub const SYNCMGR_HCM_CONFLICT_STORE: SYNCMGR_HANDLER_CAPABILITIES = 4i32;
pub const SYNCMGR_HCM_EVENT_STORE: SYNCMGR_HANDLER_CAPABILITIES = 2i32;
pub const SYNCMGR_HCM_NONE: SYNCMGR_HANDLER_CAPABILITIES = 0i32;
pub const SYNCMGR_HCM_PROVIDES_ICON: SYNCMGR_HANDLER_CAPABILITIES = 1i32;
pub const SYNCMGR_HCM_QUERY_BEFORE_ACTIVATE: SYNCMGR_HANDLER_CAPABILITIES = 1048576i32;
pub const SYNCMGR_HCM_QUERY_BEFORE_DEACTIVATE: SYNCMGR_HANDLER_CAPABILITIES = 2097152i32;
pub const SYNCMGR_HCM_QUERY_BEFORE_DISABLE: SYNCMGR_HANDLER_CAPABILITIES = 8388608i32;
pub const SYNCMGR_HCM_QUERY_BEFORE_ENABLE: SYNCMGR_HANDLER_CAPABILITIES = 4194304i32;
pub const SYNCMGR_HCM_SUPPORTS_CONCURRENT_SESSIONS: SYNCMGR_HANDLER_CAPABILITIES = 16i32;
pub const SYNCMGR_HCM_VALID_MASK: SYNCMGR_HANDLER_CAPABILITIES = 15925271i32;
pub const SYNCMGR_HPM_BACKGROUND_SYNC_ONLY: SYNCMGR_HANDLER_POLICIES = 48i32;
pub const SYNCMGR_HPM_DISABLE_BROWSE: SYNCMGR_HANDLER_POLICIES = 4096i32;
pub const SYNCMGR_HPM_DISABLE_DISABLE: SYNCMGR_HANDLER_POLICIES = 512i32;
pub const SYNCMGR_HPM_DISABLE_ENABLE: SYNCMGR_HANDLER_POLICIES = 256i32;
pub const SYNCMGR_HPM_DISABLE_SCHEDULE: SYNCMGR_HANDLER_POLICIES = 8192i32;
pub const SYNCMGR_HPM_DISABLE_START_SYNC: SYNCMGR_HANDLER_POLICIES = 1024i32;
pub const SYNCMGR_HPM_DISABLE_STOP_SYNC: SYNCMGR_HANDLER_POLICIES = 2048i32;
pub const SYNCMGR_HPM_HIDDEN_BY_DEFAULT: SYNCMGR_HANDLER_POLICIES = 65536i32;
pub const SYNCMGR_HPM_NONE: SYNCMGR_HANDLER_POLICIES = 0i32;
pub const SYNCMGR_HPM_PREVENT_ACTIVATE: SYNCMGR_HANDLER_POLICIES = 1i32;
pub const SYNCMGR_HPM_PREVENT_DEACTIVATE: SYNCMGR_HANDLER_POLICIES = 2i32;
pub const SYNCMGR_HPM_PREVENT_DISABLE: SYNCMGR_HANDLER_POLICIES = 8i32;
pub const SYNCMGR_HPM_PREVENT_ENABLE: SYNCMGR_HANDLER_POLICIES = 4i32;
pub const SYNCMGR_HPM_PREVENT_START_SYNC: SYNCMGR_HANDLER_POLICIES = 16i32;
pub const SYNCMGR_HPM_PREVENT_STOP_SYNC: SYNCMGR_HANDLER_POLICIES = 32i32;
pub const SYNCMGR_HPM_VALID_MASK: SYNCMGR_HANDLER_POLICIES = 77631i32;
pub const SYNCMGR_HT_APPLICATION: SYNCMGR_HANDLER_TYPE = 1i32;
pub const SYNCMGR_HT_COMPUTER: SYNCMGR_HANDLER_TYPE = 5i32;
pub const SYNCMGR_HT_DEVICE: SYNCMGR_HANDLER_TYPE = 2i32;
pub const SYNCMGR_HT_FOLDER: SYNCMGR_HANDLER_TYPE = 3i32;
pub const SYNCMGR_HT_MAX: SYNCMGR_HANDLER_TYPE = 5i32;
pub const SYNCMGR_HT_MIN: SYNCMGR_HANDLER_TYPE = 0i32;
pub const SYNCMGR_HT_SERVICE: SYNCMGR_HANDLER_TYPE = 4i32;
pub const SYNCMGR_HT_UNSPECIFIED: SYNCMGR_HANDLER_TYPE = 0i32;
pub const SYNCMGR_ICM_CAN_BROWSE_CONTENT: SYNCMGR_ITEM_CAPABILITIES = 65536i32;
pub const SYNCMGR_ICM_CAN_DELETE: SYNCMGR_ITEM_CAPABILITIES = 16i32;
pub const SYNCMGR_ICM_CONFLICT_STORE: SYNCMGR_ITEM_CAPABILITIES = 4i32;
pub const SYNCMGR_ICM_EVENT_STORE: SYNCMGR_ITEM_CAPABILITIES = 2i32;
pub const SYNCMGR_ICM_NONE: SYNCMGR_ITEM_CAPABILITIES = 0i32;
pub const SYNCMGR_ICM_PROVIDES_ICON: SYNCMGR_ITEM_CAPABILITIES = 1i32;
pub const SYNCMGR_ICM_QUERY_BEFORE_DELETE: SYNCMGR_ITEM_CAPABILITIES = 4194304i32;
pub const SYNCMGR_ICM_QUERY_BEFORE_DISABLE: SYNCMGR_ITEM_CAPABILITIES = 2097152i32;
pub const SYNCMGR_ICM_QUERY_BEFORE_ENABLE: SYNCMGR_ITEM_CAPABILITIES = 1048576i32;
pub const SYNCMGR_ICM_VALID_MASK: SYNCMGR_ITEM_CAPABILITIES = 7405591i32;
pub const SYNCMGR_IPM_DISABLE_BROWSE: SYNCMGR_ITEM_POLICIES = 256i32;
pub const SYNCMGR_IPM_DISABLE_DELETE: SYNCMGR_ITEM_POLICIES = 512i32;
pub const SYNCMGR_IPM_DISABLE_DISABLE: SYNCMGR_ITEM_POLICIES = 32i32;
pub const SYNCMGR_IPM_DISABLE_ENABLE: SYNCMGR_ITEM_POLICIES = 16i32;
pub const SYNCMGR_IPM_DISABLE_START_SYNC: SYNCMGR_ITEM_POLICIES = 64i32;
pub const SYNCMGR_IPM_DISABLE_STOP_SYNC: SYNCMGR_ITEM_POLICIES = 128i32;
pub const SYNCMGR_IPM_HIDDEN_BY_DEFAULT: SYNCMGR_ITEM_POLICIES = 65536i32;
pub const SYNCMGR_IPM_NONE: SYNCMGR_ITEM_POLICIES = 0i32;
pub const SYNCMGR_IPM_PREVENT_DISABLE: SYNCMGR_ITEM_POLICIES = 2i32;
pub const SYNCMGR_IPM_PREVENT_ENABLE: SYNCMGR_ITEM_POLICIES = 1i32;
pub const SYNCMGR_IPM_PREVENT_START_SYNC: SYNCMGR_ITEM_POLICIES = 4i32;
pub const SYNCMGR_IPM_PREVENT_STOP_SYNC: SYNCMGR_ITEM_POLICIES = 8i32;
pub const SYNCMGR_IPM_VALID_MASK: SYNCMGR_ITEM_POLICIES = 66303i32;
pub const SYNCMGR_OBJECTID_BrowseContent: windows_core::GUID = windows_core::GUID::from_u128(0x57cbb584_e9b4_47ae_a120_c4df3335dee2);
pub const SYNCMGR_OBJECTID_ConflictStore: windows_core::GUID = windows_core::GUID::from_u128(0xd78181f4_2389_47e4_a960_60bcc2ed930b);
pub const SYNCMGR_OBJECTID_EventLinkClick: windows_core::GUID = windows_core::GUID::from_u128(0x2203bdc1_1af1_4082_8c30_28399f41384c);
pub const SYNCMGR_OBJECTID_EventStore: windows_core::GUID = windows_core::GUID::from_u128(0x4bef34b9_a786_4075_ba88_0c2b9d89a98f);
pub const SYNCMGR_OBJECTID_Icon: windows_core::GUID = windows_core::GUID::from_u128(0x6dbc85c3_5d07_4c72_a777_7fec78072c06);
pub const SYNCMGR_OBJECTID_QueryBeforeActivate: windows_core::GUID = windows_core::GUID::from_u128(0xd882d80b_e7aa_49ed_86b7_e6e1f714cdfe);
pub const SYNCMGR_OBJECTID_QueryBeforeDeactivate: windows_core::GUID = windows_core::GUID::from_u128(0xa0efc282_60e0_460e_9374_ea88513cfc80);
pub const SYNCMGR_OBJECTID_QueryBeforeDelete: windows_core::GUID = windows_core::GUID::from_u128(0xf76c3397_afb3_45d7_a59f_5a49e905437e);
pub const SYNCMGR_OBJECTID_QueryBeforeDisable: windows_core::GUID = windows_core::GUID::from_u128(0xbb5f64aa_f004_4eb5_8e4d_26751966344c);
pub const SYNCMGR_OBJECTID_QueryBeforeEnable: windows_core::GUID = windows_core::GUID::from_u128(0x04cbf7f0_5beb_4de1_bc90_908345c480f6);
pub const SYNCMGR_OBJECTID_ShowSchedule: windows_core::GUID = windows_core::GUID::from_u128(0xedc6f3e3_8441_4109_adf3_6c1ca0b7de47);
pub const SYNCMGR_PC_KEEP_MULTIPLE: SYNCMGR_PRESENTER_CHOICE = 2i32;
pub const SYNCMGR_PC_KEEP_ONE: SYNCMGR_PRESENTER_CHOICE = 1i32;
pub const SYNCMGR_PC_KEEP_RECENT: SYNCMGR_PRESENTER_CHOICE = 3i32;
pub const SYNCMGR_PC_NO_CHOICE: SYNCMGR_PRESENTER_CHOICE = 0i32;
pub const SYNCMGR_PC_REMOVE_FROM_SYNC_SET: SYNCMGR_PRESENTER_CHOICE = 4i32;
pub const SYNCMGR_PC_SKIP: SYNCMGR_PRESENTER_CHOICE = 5i32;
pub const SYNCMGR_PNS_CANCEL: SYNCMGR_PRESENTER_NEXT_STEP = 2i32;
pub const SYNCMGR_PNS_CONTINUE: SYNCMGR_PRESENTER_NEXT_STEP = 0i32;
pub const SYNCMGR_PNS_DEFAULT: SYNCMGR_PRESENTER_NEXT_STEP = 1i32;
pub const SYNCMGR_PS_CANCELED: SYNCMGR_PROGRESS_STATUS = 5i32;
pub const SYNCMGR_PS_DISCONNECTED: SYNCMGR_PROGRESS_STATUS = 6i32;
pub const SYNCMGR_PS_FAILED: SYNCMGR_PROGRESS_STATUS = 4i32;
pub const SYNCMGR_PS_MAX: SYNCMGR_PROGRESS_STATUS = 6i32;
pub const SYNCMGR_PS_SUCCEEDED: SYNCMGR_PROGRESS_STATUS = 3i32;
pub const SYNCMGR_PS_UPDATING: SYNCMGR_PROGRESS_STATUS = 1i32;
pub const SYNCMGR_PS_UPDATING_INDETERMINATE: SYNCMGR_PROGRESS_STATUS = 2i32;
pub const SYNCMGR_RA_KEEPOTHER: SYNCMGR_RESOLUTION_ABILITIES = 1i32;
pub const SYNCMGR_RA_KEEPRECENT: SYNCMGR_RESOLUTION_ABILITIES = 2i32;
pub const SYNCMGR_RA_KEEP_MULTIPLE: SYNCMGR_RESOLUTION_ABILITIES = 16i32;
pub const SYNCMGR_RA_KEEP_SINGLE: SYNCMGR_RESOLUTION_ABILITIES = 8i32;
pub const SYNCMGR_RA_REMOVEFROMSYNCSET: SYNCMGR_RESOLUTION_ABILITIES = 4i32;
pub const SYNCMGR_RA_VALID: SYNCMGR_RESOLUTION_ABILITIES = 31i32;
pub const SYNCMGR_RF_CANCEL: SYNCMGR_RESOLUTION_FEEDBACK = 2i32;
pub const SYNCMGR_RF_CONTINUE: SYNCMGR_RESOLUTION_FEEDBACK = 0i32;
pub const SYNCMGR_RF_REFRESH: SYNCMGR_RESOLUTION_FEEDBACK = 1i32;
pub const SYNCMGR_SCF_IGNORE_IF_ALREADY_SYNCING: SYNCMGR_SYNC_CONTROL_FLAGS = 1i32;
pub const SYNCMGR_SCF_NONE: SYNCMGR_SYNC_CONTROL_FLAGS = 0i32;
pub const SYNCMGR_SCF_VALID: SYNCMGR_SYNC_CONTROL_FLAGS = 1i32;
pub const SYNCMGR_UR_ADDED: SYNCMGR_UPDATE_REASON = 0i32;
pub const SYNCMGR_UR_CHANGED: SYNCMGR_UPDATE_REASON = 1i32;
pub const SYNCMGR_UR_MAX: SYNCMGR_UPDATE_REASON = 2i32;
pub const SYNCMGR_UR_REMOVED: SYNCMGR_UPDATE_REASON = 2i32;
pub const SZ_CONTENTTYPE_CDF: windows_core::PCWSTR = windows_core::w!("application/x-cdf");
pub const SZ_CONTENTTYPE_CDFA: windows_core::PCSTR = windows_core::s!("application/x-cdf");
pub const SZ_CONTENTTYPE_CDFW: windows_core::PCWSTR = windows_core::w!("application/x-cdf");
pub const SZ_CONTENTTYPE_HTML: windows_core::PCWSTR = windows_core::w!("text/html");
pub const SZ_CONTENTTYPE_HTMLA: windows_core::PCSTR = windows_core::s!("text/html");
pub const SZ_CONTENTTYPE_HTMLW: windows_core::PCWSTR = windows_core::w!("text/html");
pub const S_SYNCMGR_CANCELALL: windows_core::HRESULT = 0x40204_u32 as _;
pub const S_SYNCMGR_CANCELITEM: windows_core::HRESULT = 0x40203_u32 as _;
pub const S_SYNCMGR_ENUMITEMS: windows_core::HRESULT = 0x40211_u32 as _;
pub const S_SYNCMGR_ITEMDELETED: windows_core::HRESULT = 0x40210_u32 as _;
pub const S_SYNCMGR_MISSINGITEMS: windows_core::HRESULT = 0x40201_u32 as _;
pub const S_SYNCMGR_RETRYSYNC: windows_core::HRESULT = 0x40202_u32 as _;
pub const SelectedItemCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8fe316d2_0e52_460a_9c1e_48f273d470a3);
pub const TBIF_APPEND: u32 = 0u32;
pub const TBIF_DEFAULT: u32 = 0u32;
pub const TBIF_INTERNETBAR: u32 = 65536u32;
pub const TBIF_NOTOOLBAR: u32 = 196608u32;
pub const TBIF_PREPEND: u32 = 1u32;
pub const TBIF_REPLACE: u32 = 2u32;
pub const TBIF_STANDARDTOOLBAR: u32 = 131072u32;
pub const TBPF_ERROR: TBPFLAG = 4i32;
pub const TBPF_INDETERMINATE: TBPFLAG = 1i32;
pub const TBPF_NOPROGRESS: TBPFLAG = 0i32;
pub const TBPF_NORMAL: TBPFLAG = 2i32;
pub const TBPF_PAUSED: TBPFLAG = 8i32;
pub const THBF_DISABLED: THUMBBUTTONFLAGS = 1i32;
pub const THBF_DISMISSONCLICK: THUMBBUTTONFLAGS = 2i32;
pub const THBF_ENABLED: THUMBBUTTONFLAGS = 0i32;
pub const THBF_HIDDEN: THUMBBUTTONFLAGS = 8i32;
pub const THBF_NOBACKGROUND: THUMBBUTTONFLAGS = 4i32;
pub const THBF_NONINTERACTIVE: THUMBBUTTONFLAGS = 16i32;
pub const THBN_CLICKED: u32 = 6144u32;
pub const THB_BITMAP: THUMBBUTTONMASK = 1i32;
pub const THB_FLAGS: THUMBBUTTONMASK = 8i32;
pub const THB_ICON: THUMBBUTTONMASK = 2i32;
pub const THB_TOOLTIP: THUMBBUTTONMASK = 4i32;
pub const TITLEBARNAMELEN: u32 = 40u32;
pub const TI_BITMAP: TI_FLAGS = 1i32;
pub const TI_JPEG: TI_FLAGS = 2i32;
pub const TLEF_ABSOLUTE: TLENUMF = 49i32;
pub const TLEF_EXCLUDE_ABOUT_PAGES: TLENUMF = 256i32;
pub const TLEF_EXCLUDE_SUBFRAME_ENTRIES: TLENUMF = 128i32;
pub const TLEF_INCLUDE_UNINVOKEABLE: TLENUMF = 64i32;
pub const TLEF_RELATIVE_BACK: TLENUMF = 16i32;
pub const TLEF_RELATIVE_FORE: TLENUMF = 32i32;
pub const TLEF_RELATIVE_INCLUDE_CURRENT: TLENUMF = 1i32;
pub const TLMENUF_BACK: u32 = 16u32;
pub const TLMENUF_FORE: u32 = 32u32;
pub const TLMENUF_INCLUDECURRENT: u32 = 1u32;
pub const TLOG_BACK: i32 = -1i32;
pub const TLOG_CURRENT: u32 = 0u32;
pub const TLOG_FORE: u32 = 1u32;
pub const TRANSLATEURL_FL_GUESS_PROTOCOL: TRANSLATEURL_IN_FLAGS = 1i32;
pub const TRANSLATEURL_FL_USE_DEFAULT_PROTOCOL: TRANSLATEURL_IN_FLAGS = 2i32;
pub const TSF_ALLOW_DECRYPTION: _TRANSFER_SOURCE_FLAGS = 4i32;
pub const TSF_COPY_CREATION_TIME: _TRANSFER_SOURCE_FLAGS = 16i32;
pub const TSF_COPY_HARD_LINK: _TRANSFER_SOURCE_FLAGS = 256i32;
pub const TSF_COPY_LOCALIZED_NAME: _TRANSFER_SOURCE_FLAGS = 512i32;
pub const TSF_COPY_WRITE_TIME: _TRANSFER_SOURCE_FLAGS = 32i32;
pub const TSF_DELETE_RECYCLE_IF_POSSIBLE: _TRANSFER_SOURCE_FLAGS = 128i32;
pub const TSF_FAIL_EXIST: _TRANSFER_SOURCE_FLAGS = 0i32;
pub const TSF_MOVE_AS_COPY_DELETE: _TRANSFER_SOURCE_FLAGS = 1024i32;
pub const TSF_NORMAL: _TRANSFER_SOURCE_FLAGS = 0i32;
pub const TSF_NO_SECURITY: _TRANSFER_SOURCE_FLAGS = 8i32;
pub const TSF_OVERWRITE_EXIST: _TRANSFER_SOURCE_FLAGS = 2i32;
pub const TSF_RENAME_EXIST: _TRANSFER_SOURCE_FLAGS = 1i32;
pub const TSF_SUSPEND_SHELLEVENTS: _TRANSFER_SOURCE_FLAGS = 2048i32;
pub const TSF_USE_FULL_ACCESS: _TRANSFER_SOURCE_FLAGS = 64i32;
pub const TS_INDETERMINATE: _TRANSFER_ADVISE_STATE = 4i32;
pub const TS_NONE: _TRANSFER_ADVISE_STATE = 0i32;
pub const TS_PERFORMING: _TRANSFER_ADVISE_STATE = 1i32;
pub const TS_PREPARING: _TRANSFER_ADVISE_STATE = 2i32;
pub const URLASSOCDLG_FL_REGISTER_ASSOC: URLASSOCIATIONDIALOG_IN_FLAGS = 2i32;
pub const URLASSOCDLG_FL_USE_DEFAULT_NAME: URLASSOCIATIONDIALOG_IN_FLAGS = 1i32;
pub const URLIS_APPLIABLE: URLIS = 4i32;
pub const URLIS_DIRECTORY: URLIS = 5i32;
pub const URLIS_FILEURL: URLIS = 3i32;
pub const URLIS_HASQUERY: URLIS = 6i32;
pub const URLIS_NOHISTORY: URLIS = 2i32;
pub const URLIS_OPAQUE: URLIS = 1i32;
pub const URLIS_URL: URLIS = 0i32;
pub const URL_APPLY_DEFAULT: u32 = 1u32;
pub const URL_APPLY_FORCEAPPLY: u32 = 8u32;
pub const URL_APPLY_GUESSFILE: u32 = 4u32;
pub const URL_APPLY_GUESSSCHEME: u32 = 2u32;
pub const URL_BROWSER_MODE: u32 = 33554432u32;
pub const URL_CONVERT_IF_DOSPATH: u32 = 2097152u32;
pub const URL_DONT_ESCAPE_EXTRA_INFO: u32 = 33554432u32;
pub const URL_DONT_SIMPLIFY: u32 = 134217728u32;
pub const URL_DONT_UNESCAPE: u32 = 131072u32;
pub const URL_DONT_UNESCAPE_EXTRA_INFO: u32 = 33554432u32;
pub const URL_ESCAPE_ASCII_URI_COMPONENT: u32 = 524288u32;
pub const URL_ESCAPE_AS_UTF8: u32 = 262144u32;
pub const URL_ESCAPE_PERCENT: u32 = 4096u32;
pub const URL_ESCAPE_SEGMENT_ONLY: u32 = 8192u32;
pub const URL_ESCAPE_SPACES_ONLY: u32 = 67108864u32;
pub const URL_ESCAPE_UNSAFE: u32 = 536870912u32;
pub const URL_E_INVALID_SYNTAX: windows_core::HRESULT = 0x80041001_u32 as _;
pub const URL_E_UNREGISTERED_PROTOCOL: windows_core::HRESULT = 0x80041002_u32 as _;
pub const URL_FILE_USE_PATHURL: u32 = 65536u32;
pub const URL_INTERNAL_PATH: u32 = 8388608u32;
pub const URL_NO_META: u32 = 134217728u32;
pub const URL_PARTFLAG_KEEPSCHEME: u32 = 1u32;
pub const URL_PART_HOSTNAME: URL_PART = 2i32;
pub const URL_PART_NONE: URL_PART = 0i32;
pub const URL_PART_PASSWORD: URL_PART = 4i32;
pub const URL_PART_PORT: URL_PART = 5i32;
pub const URL_PART_QUERY: URL_PART = 6i32;
pub const URL_PART_SCHEME: URL_PART = 1i32;
pub const URL_PART_USERNAME: URL_PART = 3i32;
pub const URL_PLUGGABLE_PROTOCOL: u32 = 1073741824u32;
pub const URL_SCHEME_ABOUT: URL_SCHEME = 17i32;
pub const URL_SCHEME_FILE: URL_SCHEME = 9i32;
pub const URL_SCHEME_FTP: URL_SCHEME = 1i32;
pub const URL_SCHEME_GOPHER: URL_SCHEME = 3i32;
pub const URL_SCHEME_HTTP: URL_SCHEME = 2i32;
pub const URL_SCHEME_HTTPS: URL_SCHEME = 11i32;
pub const URL_SCHEME_INVALID: URL_SCHEME = -1i32;
pub const URL_SCHEME_JAVASCRIPT: URL_SCHEME = 15i32;
pub const URL_SCHEME_KNOWNFOLDER: URL_SCHEME = 26i32;
pub const URL_SCHEME_LOCAL: URL_SCHEME = 14i32;
pub const URL_SCHEME_MAILTO: URL_SCHEME = 4i32;
pub const URL_SCHEME_MAXVALUE: URL_SCHEME = 27i32;
pub const URL_SCHEME_MK: URL_SCHEME = 10i32;
pub const URL_SCHEME_MSHELP: URL_SCHEME = 21i32;
pub const URL_SCHEME_MSSHELLDEVICE: URL_SCHEME = 22i32;
pub const URL_SCHEME_MSSHELLIDLIST: URL_SCHEME = 20i32;
pub const URL_SCHEME_MSSHELLROOTED: URL_SCHEME = 19i32;
pub const URL_SCHEME_NEWS: URL_SCHEME = 5i32;
pub const URL_SCHEME_NNTP: URL_SCHEME = 6i32;
pub const URL_SCHEME_RES: URL_SCHEME = 18i32;
pub const URL_SCHEME_SEARCH: URL_SCHEME = 25i32;
pub const URL_SCHEME_SEARCH_MS: URL_SCHEME = 24i32;
pub const URL_SCHEME_SHELL: URL_SCHEME = 12i32;
pub const URL_SCHEME_SNEWS: URL_SCHEME = 13i32;
pub const URL_SCHEME_TELNET: URL_SCHEME = 7i32;
pub const URL_SCHEME_UNKNOWN: URL_SCHEME = 0i32;
pub const URL_SCHEME_VBSCRIPT: URL_SCHEME = 16i32;
pub const URL_SCHEME_WAIS: URL_SCHEME = 8i32;
pub const URL_SCHEME_WILDCARD: URL_SCHEME = 23i32;
pub const URL_UNESCAPE: u32 = 268435456u32;
pub const URL_UNESCAPE_AS_UTF8: u32 = 262144u32;
pub const URL_UNESCAPE_HIGH_ANSI_ONLY: u32 = 4194304u32;
pub const URL_UNESCAPE_INPLACE: u32 = 1048576u32;
pub const URL_UNESCAPE_URI_COMPONENT: u32 = 262144u32;
pub const URL_WININET_COMPATIBILITY: u32 = 2147483648u32;
pub const UR_MONITOR_DISCONNECT: UNDOCK_REASON = 1i32;
pub const UR_RESOLUTION_CHANGE: UNDOCK_REASON = 0i32;
pub const VALIDATEUNC_CONNECT: VALIDATEUNC_OPTION = 1i32;
pub const VALIDATEUNC_NOUI: VALIDATEUNC_OPTION = 2i32;
pub const VALIDATEUNC_PERSIST: VALIDATEUNC_OPTION = 8i32;
pub const VALIDATEUNC_PRINT: VALIDATEUNC_OPTION = 4i32;
pub const VALIDATEUNC_VALID: VALIDATEUNC_OPTION = 15i32;
pub const VID_Content: windows_core::GUID = windows_core::GUID::from_u128(0x30c2c434_0889_4c8d_985d_a9f71830b0a9);
pub const VID_Details: windows_core::GUID = windows_core::GUID::from_u128(0x137e7700_3573_11cf_ae69_08002b2e1262);
pub const VID_LargeIcons: windows_core::GUID = windows_core::GUID::from_u128(0x0057d0e0_3573_11cf_ae69_08002b2e1262);
pub const VID_List: windows_core::GUID = windows_core::GUID::from_u128(0x0e1fa5e0_3573_11cf_ae69_08002b2e1262);
pub const VID_SmallIcons: windows_core::GUID = windows_core::GUID::from_u128(0x089000c0_3573_11cf_ae69_08002b2e1262);
pub const VID_ThumbStrip: windows_core::GUID = windows_core::GUID::from_u128(0x8eefa624_d1e9_445b_94b7_74fbce2ea11a);
pub const VID_Thumbnails: windows_core::GUID = windows_core::GUID::from_u128(0x8bebb290_52d0_11d0_b7f4_00c04fd706ec);
pub const VID_Tile: windows_core::GUID = windows_core::GUID::from_u128(0x65f125e5_7be1_4810_ba9d_d271c8432ce3);
pub const VIEW_PRIORITY_CACHEHIT: u32 = 80u32;
pub const VIEW_PRIORITY_CACHEMISS: u32 = 48u32;
pub const VIEW_PRIORITY_DESPERATE: u32 = 16u32;
pub const VIEW_PRIORITY_INHERIT: u32 = 32u32;
pub const VIEW_PRIORITY_NONE: u32 = 0u32;
pub const VIEW_PRIORITY_RESTRICTED: u32 = 112u32;
pub const VIEW_PRIORITY_SHELLEXT: u32 = 64u32;
pub const VIEW_PRIORITY_SHELLEXT_ASBACKUP: u32 = 21u32;
pub const VIEW_PRIORITY_STALECACHEHIT: u32 = 69u32;
pub const VIEW_PRIORITY_USEASDEFAULT: u32 = 67u32;
pub const VOLUME_PREFIX: windows_core::PCWSTR = windows_core::w!("\\\\?\\Volume");
pub const VPCF_BACKGROUND: VPCOLORFLAGS = 2i32;
pub const VPCF_SORTCOLUMN: VPCOLORFLAGS = 3i32;
pub const VPCF_SUBTEXT: VPCOLORFLAGS = 4i32;
pub const VPCF_TEXT: VPCOLORFLAGS = 1i32;
pub const VPCF_TEXTBACKGROUND: VPCOLORFLAGS = 5i32;
pub const VPWF_ALPHABLEND: VPWATERMARKFLAGS = 1i32;
pub const VPWF_DEFAULT: VPWATERMARKFLAGS = 0i32;
pub const WC_NETADDRESS: windows_core::PCWSTR = windows_core::w!("msctls_netaddress");
pub const WM_CPL_LAUNCH: u32 = 2024u32;
pub const WM_CPL_LAUNCHED: u32 = 2025u32;
pub const WPSTYLE_CENTER: u32 = 0u32;
pub const WPSTYLE_CROPTOFIT: u32 = 4u32;
pub const WPSTYLE_KEEPASPECT: u32 = 3u32;
pub const WPSTYLE_MAX: u32 = 6u32;
pub const WPSTYLE_SPAN: u32 = 5u32;
pub const WPSTYLE_STRETCH: u32 = 2u32;
pub const WPSTYLE_TILE: u32 = 1u32;
pub const WTSAT_ARGB: WTS_ALPHATYPE = 2i32;
pub const WTSAT_RGB: WTS_ALPHATYPE = 1i32;
pub const WTSAT_UNKNOWN: WTS_ALPHATYPE = 0i32;
pub const WTSCF_APPSTYLE: WTS_CONTEXTFLAGS = 1i32;
pub const WTSCF_DEFAULT: WTS_CONTEXTFLAGS = 0i32;
pub const WTSCF_FAST: WTS_CONTEXTFLAGS = 8i32;
pub const WTSCF_SQUARE: WTS_CONTEXTFLAGS = 2i32;
pub const WTSCF_WIDE: WTS_CONTEXTFLAGS = 4i32;
pub const WTS_APPSTYLE: WTS_FLAGS = 8192i32;
pub const WTS_CACHED: WTS_CACHEFLAGS = 2i32;
pub const WTS_CROPTOSQUARE: WTS_FLAGS = 512i32;
pub const WTS_DEFAULT: WTS_CACHEFLAGS = 0i32;
pub const WTS_EXTRACT: WTS_FLAGS = 0i32;
pub const WTS_EXTRACTDONOTCACHE: WTS_FLAGS = 32i32;
pub const WTS_EXTRACTINPROC: WTS_FLAGS = 256i32;
pub const WTS_E_DATAFILEUNAVAILABLE: windows_core::HRESULT = 0x8004B204_u32 as _;
pub const WTS_E_EXTRACTIONBLOCKED: windows_core::HRESULT = 0x8004B206_u32 as _;
pub const WTS_E_EXTRACTIONPENDING: windows_core::HRESULT = 0x8004B205_u32 as _;
pub const WTS_E_EXTRACTIONTIMEDOUT: windows_core::HRESULT = 0x8004B201_u32 as _;
pub const WTS_E_FAILEDEXTRACTION: windows_core::HRESULT = 0x8004B200_u32 as _;
pub const WTS_E_FASTEXTRACTIONNOTSUPPORTED: windows_core::HRESULT = 0x8004B203_u32 as _;
pub const WTS_E_NOSTORAGEPROVIDERTHUMBNAILHANDLER: windows_core::HRESULT = 0x8004B207_u32 as _;
pub const WTS_E_SURROGATEUNAVAILABLE: windows_core::HRESULT = 0x8004B202_u32 as _;
pub const WTS_FASTEXTRACT: WTS_FLAGS = 2i32;
pub const WTS_FORCEEXTRACTION: WTS_FLAGS = 4i32;
pub const WTS_IDEALCACHESIZEONLY: WTS_FLAGS = 32768i32;
pub const WTS_INCACHEONLY: WTS_FLAGS = 1i32;
pub const WTS_INSTANCESURROGATE: WTS_FLAGS = 1024i32;
pub const WTS_LOWQUALITY: WTS_CACHEFLAGS = 1i32;
pub const WTS_NONE: WTS_FLAGS = 0i32;
pub const WTS_REQUIRESURROGATE: WTS_FLAGS = 2048i32;
pub const WTS_SCALETOREQUESTEDSIZE: WTS_FLAGS = 64i32;
pub const WTS_SCALEUP: WTS_FLAGS = 65536i32;
pub const WTS_SKIPFASTEXTRACT: WTS_FLAGS = 128i32;
pub const WTS_SLOWRECLAIM: WTS_FLAGS = 8i32;
pub const WTS_WIDETHUMBNAILS: WTS_FLAGS = 16384i32;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_CSCSYNCINPROGRESS: SHGLOBALCOUNTER = 13i32;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SERVERDRIVE: SHGLOBALCOUNTER = 42i32;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEGLOBALDIRTYCOUNT: SHGLOBALCOUNTER = 43i32;
pub const idsAppName: u32 = 1007u32;
pub const idsBadOldPW: u32 = 1006u32;
pub const idsChangePW: u32 = 1005u32;
pub const idsDefKeyword: u32 = 1010u32;
pub const idsDifferentPW: u32 = 1004u32;
pub const idsHelpFile: u32 = 1009u32;
pub const idsIniFile: u32 = 1001u32;
pub const idsIsPassword: u32 = 1000u32;
pub const idsNoHelpMemory: u32 = 1008u32;
pub const idsPassword: u32 = 1003u32;
pub const idsScreenSaver: u32 = 1002u32;
pub const navAllowAutosearch: BrowserNavConstants = 16i32;
pub const navBlockRedirectsXDomain: BrowserNavConstants = 32768i32;
pub const navBrowserBar: BrowserNavConstants = 32i32;
pub const navDeferUnload: BrowserNavConstants = 262144i32;
pub const navEnforceRestricted: BrowserNavConstants = 128i32;
pub const navHomepageNavigate: BrowserNavConstants = 8388608i32;
pub const navHostNavigation: BrowserNavConstants = 33554432i32;
pub const navHyperlink: BrowserNavConstants = 64i32;
pub const navKeepWordWheelText: BrowserNavConstants = 8192i32;
pub const navNewWindowsManaged: BrowserNavConstants = 256i32;
pub const navNoHistory: BrowserNavConstants = 2i32;
pub const navNoReadFromCache: BrowserNavConstants = 4i32;
pub const navNoWriteToCache: BrowserNavConstants = 8i32;
pub const navOpenInBackgroundTab: BrowserNavConstants = 4096i32;
pub const navOpenInNewTab: BrowserNavConstants = 2048i32;
pub const navOpenInNewWindow: BrowserNavConstants = 1i32;
pub const navOpenNewForegroundTab: BrowserNavConstants = 65536i32;
pub const navRefresh: BrowserNavConstants = 16777216i32;
pub const navReserved1: BrowserNavConstants = 4194304i32;
pub const navReserved2: BrowserNavConstants = 67108864i32;
pub const navReserved3: BrowserNavConstants = 134217728i32;
pub const navReserved4: BrowserNavConstants = 268435456i32;
pub const navReserved5: BrowserNavConstants = 536870912i32;
pub const navReserved6: BrowserNavConstants = 1073741824i32;
pub const navReserved7: BrowserNavConstants = -2147483648i32;
pub const navSpeculative: BrowserNavConstants = 524288i32;
pub const navSuggestNewTab: BrowserNavConstants = 2097152i32;
pub const navSuggestNewWindow: BrowserNavConstants = 1048576i32;
pub const navTravelLogScreenshot: BrowserNavConstants = 131072i32;
pub const navTrustedForActiveX: BrowserNavConstants = 1024i32;
pub const navUntrustedForDownload: BrowserNavConstants = 512i32;
pub const navVirtualTab: BrowserNavConstants = 16384i32;
pub const secureLockIconMixed: SecureLockIconConstants = 1i32;
pub const secureLockIconSecure128Bit: SecureLockIconConstants = 6i32;
pub const secureLockIconSecure40Bit: SecureLockIconConstants = 3i32;
pub const secureLockIconSecure56Bit: SecureLockIconConstants = 4i32;
pub const secureLockIconSecureFortezza: SecureLockIconConstants = 5i32;
pub const secureLockIconSecureUnknownBits: SecureLockIconConstants = 2i32;
pub const secureLockIconUnsecure: SecureLockIconConstants = 0i32;
pub const ssfALTSTARTUP: ShellSpecialFolderConstants = 29i32;
pub const ssfAPPDATA: ShellSpecialFolderConstants = 26i32;
pub const ssfBITBUCKET: ShellSpecialFolderConstants = 10i32;
pub const ssfCOMMONALTSTARTUP: ShellSpecialFolderConstants = 30i32;
pub const ssfCOMMONAPPDATA: ShellSpecialFolderConstants = 35i32;
pub const ssfCOMMONDESKTOPDIR: ShellSpecialFolderConstants = 25i32;
pub const ssfCOMMONFAVORITES: ShellSpecialFolderConstants = 31i32;
pub const ssfCOMMONPROGRAMS: ShellSpecialFolderConstants = 23i32;
pub const ssfCOMMONSTARTMENU: ShellSpecialFolderConstants = 22i32;
pub const ssfCOMMONSTARTUP: ShellSpecialFolderConstants = 24i32;
pub const ssfCONTROLS: ShellSpecialFolderConstants = 3i32;
pub const ssfCOOKIES: ShellSpecialFolderConstants = 33i32;
pub const ssfDESKTOP: ShellSpecialFolderConstants = 0i32;
pub const ssfDESKTOPDIRECTORY: ShellSpecialFolderConstants = 16i32;
pub const ssfDRIVES: ShellSpecialFolderConstants = 17i32;
pub const ssfFAVORITES: ShellSpecialFolderConstants = 6i32;
pub const ssfFONTS: ShellSpecialFolderConstants = 20i32;
pub const ssfHISTORY: ShellSpecialFolderConstants = 34i32;
pub const ssfINTERNETCACHE: ShellSpecialFolderConstants = 32i32;
pub const ssfLOCALAPPDATA: ShellSpecialFolderConstants = 28i32;
pub const ssfMYPICTURES: ShellSpecialFolderConstants = 39i32;
pub const ssfNETHOOD: ShellSpecialFolderConstants = 19i32;
pub const ssfNETWORK: ShellSpecialFolderConstants = 18i32;
pub const ssfPERSONAL: ShellSpecialFolderConstants = 5i32;
pub const ssfPRINTERS: ShellSpecialFolderConstants = 4i32;
pub const ssfPRINTHOOD: ShellSpecialFolderConstants = 27i32;
pub const ssfPROFILE: ShellSpecialFolderConstants = 40i32;
pub const ssfPROGRAMFILES: ShellSpecialFolderConstants = 38i32;
pub const ssfPROGRAMFILESx86: ShellSpecialFolderConstants = 48i32;
pub const ssfPROGRAMS: ShellSpecialFolderConstants = 2i32;
pub const ssfRECENT: ShellSpecialFolderConstants = 8i32;
pub const ssfSENDTO: ShellSpecialFolderConstants = 9i32;
pub const ssfSTARTMENU: ShellSpecialFolderConstants = 11i32;
pub const ssfSTARTUP: ShellSpecialFolderConstants = 7i32;
pub const ssfSYSTEM: ShellSpecialFolderConstants = 37i32;
pub const ssfSYSTEMx86: ShellSpecialFolderConstants = 41i32;
pub const ssfTEMPLATES: ShellSpecialFolderConstants = 21i32;
pub const ssfWINDOWS: ShellSpecialFolderConstants = 36i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ACENUMOPTION(pub i32);
impl windows_core::TypeKind for ACENUMOPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ACTIVATEOPTIONS(pub i32);
impl windows_core::TypeKind for ACTIVATEOPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ADJACENT_DISPLAY_EDGES(pub i32);
impl windows_core::TypeKind for ADJACENT_DISPLAY_EDGES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AHE_TYPE(pub i32);
impl windows_core::TypeKind for AHE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AHTYPE(pub i32);
impl windows_core::TypeKind for AHTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPACTIONFLAGS(pub i32);
impl windows_core::TypeKind for APPACTIONFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPDOCLISTTYPE(pub i32);
impl windows_core::TypeKind for APPDOCLISTTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPINFODATAFLAGS(pub i32);
impl windows_core::TypeKind for APPINFODATAFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPLICATION_VIEW_MIN_WIDTH(pub i32);
impl windows_core::TypeKind for APPLICATION_VIEW_MIN_WIDTH {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPLICATION_VIEW_ORIENTATION(pub i32);
impl windows_core::TypeKind for APPLICATION_VIEW_ORIENTATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPLICATION_VIEW_SIZE_PREFERENCE(pub i32);
impl windows_core::TypeKind for APPLICATION_VIEW_SIZE_PREFERENCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPLICATION_VIEW_STATE(pub i32);
impl windows_core::TypeKind for APPLICATION_VIEW_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCCLASS(pub i32);
impl windows_core::TypeKind for ASSOCCLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCDATA(pub i32);
impl windows_core::TypeKind for ASSOCDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCENUM(pub i32);
impl windows_core::TypeKind for ASSOCENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCF(pub u32);
impl windows_core::TypeKind for ASSOCF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCIATIONLEVEL(pub i32);
impl windows_core::TypeKind for ASSOCIATIONLEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCIATIONTYPE(pub i32);
impl windows_core::TypeKind for ASSOCIATIONTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCKEY(pub i32);
impl windows_core::TypeKind for ASSOCKEY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOCSTR(pub i32);
impl windows_core::TypeKind for ASSOCSTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ASSOC_FILTER(pub i32);
impl windows_core::TypeKind for ASSOC_FILTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ATTACHMENT_ACTION(pub i32);
impl windows_core::TypeKind for ATTACHMENT_ACTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ATTACHMENT_PROMPT(pub i32);
impl windows_core::TypeKind for ATTACHMENT_PROMPT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AUTOCOMPLETELISTOPTIONS(pub i32);
impl windows_core::TypeKind for AUTOCOMPLETELISTOPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AUTOCOMPLETEOPTIONS(pub i32);
impl windows_core::TypeKind for AUTOCOMPLETEOPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BANDSITECID(pub i32);
impl windows_core::TypeKind for BANDSITECID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BANNER_NOTIFICATION_EVENT(pub i32);
impl windows_core::TypeKind for BANNER_NOTIFICATION_EVENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BNSTATE(pub i32);
impl windows_core::TypeKind for BNSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BrowserNavConstants(pub i32);
impl windows_core::TypeKind for BrowserNavConstants {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CATEGORYINFO_FLAGS(pub i32);
impl windows_core::TypeKind for CATEGORYINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CATSORT_FLAGS(pub i32);
impl windows_core::TypeKind for CATSORT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CDBURNINGEXTENSIONRET(pub i32);
impl windows_core::TypeKind for CDBURNINGEXTENSIONRET {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CDCONTROLSTATEF(pub i32);
impl windows_core::TypeKind for CDCONTROLSTATEF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CM_ENUM_FLAGS(pub i32);
impl windows_core::TypeKind for CM_ENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CM_MASK(pub i32);
impl windows_core::TypeKind for CM_MASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CM_SET_WIDTH_VALUE(pub i32);
impl windows_core::TypeKind for CM_SET_WIDTH_VALUE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CM_STATE(pub i32);
impl windows_core::TypeKind for CM_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CPVIEW(pub i32);
impl windows_core::TypeKind for CPVIEW {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_FIELD_STATE(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_FIELD_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_FIELD_TYPE(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_FIELD_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_STATUS_ICON(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_STATUS_ICON {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_USAGE_SCENARIO(pub i32);
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_USAGE_SCENARIO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CommandStateChangeConstants(pub i32);
impl windows_core::TypeKind for CommandStateChangeConstants {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DATAOBJ_GET_ITEM_FLAGS(pub i32);
impl windows_core::TypeKind for DATAOBJ_GET_ITEM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEFAULTSAVEFOLDERTYPE(pub i32);
impl windows_core::TypeKind for DEFAULTSAVEFOLDERTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEFAULT_FOLDER_MENU_RESTRICTIONS(pub i32);
impl windows_core::TypeKind for DEFAULT_FOLDER_MENU_RESTRICTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEF_SHARE_ID(pub i32);
impl windows_core::TypeKind for DEF_SHARE_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESKBANDCID(pub i32);
impl windows_core::TypeKind for DESKBANDCID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESKTOP_SLIDESHOW_DIRECTION(pub i32);
impl windows_core::TypeKind for DESKTOP_SLIDESHOW_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESKTOP_SLIDESHOW_OPTIONS(pub i32);
impl windows_core::TypeKind for DESKTOP_SLIDESHOW_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESKTOP_SLIDESHOW_STATE(pub i32);
impl windows_core::TypeKind for DESKTOP_SLIDESHOW_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESKTOP_WALLPAPER_POSITION(pub i32);
impl windows_core::TypeKind for DESKTOP_WALLPAPER_POSITION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DFM_CMD(pub i32);
impl windows_core::TypeKind for DFM_CMD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DFM_MESSAGE_ID(pub i32);
impl windows_core::TypeKind for DFM_MESSAGE_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DISPLAY_DEVICE_TYPE(pub i32);
impl windows_core::TypeKind for DISPLAY_DEVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DROPIMAGETYPE(pub i32);
impl windows_core::TypeKind for DROPIMAGETYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DSH_FLAGS(pub i32);
impl windows_core::TypeKind for DSH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EC_HOST_UI_MODE(pub i32);
impl windows_core::TypeKind for EC_HOST_UI_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EDGE_GESTURE_KIND(pub i32);
impl windows_core::TypeKind for EDGE_GESTURE_KIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EXPLORER_BROWSER_FILL_FLAGS(pub i32);
impl windows_core::TypeKind for EXPLORER_BROWSER_FILL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EXPLORER_BROWSER_OPTIONS(pub i32);
impl windows_core::TypeKind for EXPLORER_BROWSER_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FDAP(pub i32);
impl windows_core::TypeKind for FDAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FDE_OVERWRITE_RESPONSE(pub i32);
impl windows_core::TypeKind for FDE_OVERWRITE_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FDE_SHAREVIOLATION_RESPONSE(pub i32);
impl windows_core::TypeKind for FDE_SHAREVIOLATION_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FD_FLAGS(pub i32);
impl windows_core::TypeKind for FD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FFFP_MODE(pub i32);
impl windows_core::TypeKind for FFFP_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FILEOPENDIALOGOPTIONS(pub u32);
impl windows_core::TypeKind for FILEOPENDIALOGOPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FILEOPERATION_FLAGS(pub u32);
impl windows_core::TypeKind for FILEOPERATION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FILETYPEATTRIBUTEFLAGS(pub i32);
impl windows_core::TypeKind for FILETYPEATTRIBUTEFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FILE_OPERATION_FLAGS2(pub i32);
impl windows_core::TypeKind for FILE_OPERATION_FLAGS2 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FILE_USAGE_TYPE(pub i32);
impl windows_core::TypeKind for FILE_USAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FLYOUT_PLACEMENT(pub i32);
impl windows_core::TypeKind for FLYOUT_PLACEMENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FOLDERFLAGS(pub i32);
impl windows_core::TypeKind for FOLDERFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FOLDERLOGICALVIEWMODE(pub i32);
impl windows_core::TypeKind for FOLDERLOGICALVIEWMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FOLDERVIEWMODE(pub i32);
impl windows_core::TypeKind for FOLDERVIEWMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FOLDERVIEWOPTIONS(pub i32);
impl windows_core::TypeKind for FOLDERVIEWOPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FOLDER_ENUM_MODE(pub i32);
impl windows_core::TypeKind for FOLDER_ENUM_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FVTEXTTYPE(pub i32);
impl windows_core::TypeKind for FVTEXTTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPFIDL_FLAGS(pub u32);
impl windows_core::TypeKind for GPFIDL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HELP_INFO_TYPE(pub i32);
impl windows_core::TypeKind for HELP_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLBWIF_FLAGS(pub i32);
impl windows_core::TypeKind for HLBWIF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLFNAMEF(pub i32);
impl windows_core::TypeKind for HLFNAMEF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLID_INFO(pub u32);
impl windows_core::TypeKind for HLID_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLINKGETREF(pub i32);
impl windows_core::TypeKind for HLINKGETREF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLINKMISC(pub i32);
impl windows_core::TypeKind for HLINKMISC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLINKSETF(pub i32);
impl windows_core::TypeKind for HLINKSETF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLINKWHICHMK(pub i32);
impl windows_core::TypeKind for HLINKWHICHMK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLNF(pub u32);
impl windows_core::TypeKind for HLNF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLQF_INFO(pub i32);
impl windows_core::TypeKind for HLQF_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLSHORTCUTF(pub i32);
impl windows_core::TypeKind for HLSHORTCUTF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLSR(pub i32);
impl windows_core::TypeKind for HLSR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLTB_INFO(pub i32);
impl windows_core::TypeKind for HLTB_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HLTRANSLATEF(pub i32);
impl windows_core::TypeKind for HLTRANSLATEF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HOMEGROUPSHARINGCHOICES(pub i32);
impl windows_core::TypeKind for HOMEGROUPSHARINGCHOICES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IEPDNFLAGS(pub i32);
impl windows_core::TypeKind for IEPDNFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IESHORTCUTFLAGS(pub i32);
impl windows_core::TypeKind for IESHORTCUTFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IURL_INVOKECOMMAND_FLAGS(pub i32);
impl windows_core::TypeKind for IURL_INVOKECOMMAND_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IURL_SETURL_FLAGS(pub i32);
impl windows_core::TypeKind for IURL_SETURL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KF_CATEGORY(pub i32);
impl windows_core::TypeKind for KF_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KNOWNDESTCATEGORY(pub i32);
impl windows_core::TypeKind for KNOWNDESTCATEGORY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KNOWN_FOLDER_FLAG(pub i32);
impl windows_core::TypeKind for KNOWN_FOLDER_FLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LIBRARYFOLDERFILTER(pub i32);
impl windows_core::TypeKind for LIBRARYFOLDERFILTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LIBRARYMANAGEDIALOGOPTIONS(pub i32);
impl windows_core::TypeKind for LIBRARYMANAGEDIALOGOPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LIBRARYOPTIONFLAGS(pub i32);
impl windows_core::TypeKind for LIBRARYOPTIONFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LIBRARYSAVEFLAGS(pub i32);
impl windows_core::TypeKind for LIBRARYSAVEFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MENUBANDHANDLERCID(pub i32);
impl windows_core::TypeKind for MENUBANDHANDLERCID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MENUPOPUPPOPUPFLAGS(pub i32);
impl windows_core::TypeKind for MENUPOPUPPOPUPFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MENUPOPUPSELECT(pub i32);
impl windows_core::TypeKind for MENUPOPUPSELECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MERGE_UPDATE_STATUS(pub i32);
impl windows_core::TypeKind for MERGE_UPDATE_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MIMEASSOCIATIONDIALOG_IN_FLAGS(pub i32);
impl windows_core::TypeKind for MIMEASSOCIATIONDIALOG_IN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MM_FLAGS(pub u32);
impl windows_core::TypeKind for MM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MONITOR_APP_VISIBILITY(pub i32);
impl windows_core::TypeKind for MONITOR_APP_VISIBILITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NAMESPACEWALKFLAG(pub i32);
impl windows_core::TypeKind for NAMESPACEWALKFLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NATIVE_DISPLAY_ORIENTATION(pub i32);
impl windows_core::TypeKind for NATIVE_DISPLAY_ORIENTATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NOTIFY_ICON_DATA_FLAGS(pub u32);
impl windows_core::TypeKind for NOTIFY_ICON_DATA_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NOTIFY_ICON_INFOTIP_FLAGS(pub u32);
impl windows_core::TypeKind for NOTIFY_ICON_INFOTIP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NOTIFY_ICON_MESSAGE(pub u32);
impl windows_core::TypeKind for NOTIFY_ICON_MESSAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NOTIFY_ICON_STATE(pub u32);
impl windows_core::TypeKind for NOTIFY_ICON_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NSTCFOLDERCAPABILITIES(pub i32);
impl windows_core::TypeKind for NSTCFOLDERCAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NSTCGNI(pub i32);
impl windows_core::TypeKind for NSTCGNI {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NSTCSTYLE2(pub i32);
impl windows_core::TypeKind for NSTCSTYLE2 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NWMF(pub i32);
impl windows_core::TypeKind for NWMF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NewProcessCauseConstants(pub i32);
impl windows_core::TypeKind for NewProcessCauseConstants {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPEN_AS_INFO_FLAGS(pub i32);
impl windows_core::TypeKind for OPEN_AS_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OS(pub u32);
impl windows_core::TypeKind for OS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OfflineFolderStatus(pub i32);
impl windows_core::TypeKind for OfflineFolderStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PACKAGE_EXECUTION_STATE(pub i32);
impl windows_core::TypeKind for PACKAGE_EXECUTION_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PATHCCH_OPTIONS(pub u32);
impl windows_core::TypeKind for PATHCCH_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PCS_RET(pub u32);
impl windows_core::TypeKind for PCS_RET {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PIDISF_FLAGS(pub i32);
impl windows_core::TypeKind for PIDISF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PIDISM_OPTIONS(pub i32);
impl windows_core::TypeKind for PIDISM_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PIDISR_INFO(pub i32);
impl windows_core::TypeKind for PIDISR_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PID_INTSITE(pub i32);
impl windows_core::TypeKind for PID_INTSITE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PID_IS(pub i32);
impl windows_core::TypeKind for PID_IS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PRF_FLAGS(pub i32);
impl windows_core::TypeKind for PRF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PUBAPPINFOFLAGS(pub i32);
impl windows_core::TypeKind for PUBAPPINFOFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QITIPF_FLAGS(pub i32);
impl windows_core::TypeKind for QITIPF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QUERY_USER_NOTIFICATION_STATE(pub i32);
impl windows_core::TypeKind for QUERY_USER_NOTIFICATION_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RESTRICTIONS(pub i32);
impl windows_core::TypeKind for RESTRICTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshConstants(pub i32);
impl windows_core::TypeKind for RefreshConstants {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCALE_CHANGE_FLAGS(pub i32);
impl windows_core::TypeKind for SCALE_CHANGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCNRT_STATUS(pub i32);
impl windows_core::TypeKind for SCNRT_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SECURELOCKCODE(pub i32);
impl windows_core::TypeKind for SECURELOCKCODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SFBS_FLAGS(pub i32);
impl windows_core::TypeKind for SFBS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SFVM_MESSAGE_ID(pub i32);
impl windows_core::TypeKind for SFVM_MESSAGE_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SFVS_SELECT(pub i32);
impl windows_core::TypeKind for SFVS_SELECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHARD(pub i32);
impl windows_core::TypeKind for SHARD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHARE_ROLE(pub i32);
impl windows_core::TypeKind for SHARE_ROLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHCNE_ID(pub u32);
impl windows_core::TypeKind for SHCNE_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHCNF_FLAGS(pub u32);
impl windows_core::TypeKind for SHCNF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHCNRF_SOURCE(pub i32);
impl windows_core::TypeKind for SHCNRF_SOURCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHDID_ID(pub i32);
impl windows_core::TypeKind for SHDID_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHELLBROWSERSHOWCONTROL(pub i32);
impl windows_core::TypeKind for SHELLBROWSERSHOWCONTROL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHELL_AUTOCOMPLETE_FLAGS(pub u32);
impl windows_core::TypeKind for SHELL_AUTOCOMPLETE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHELL_LINK_DATA_FLAGS(pub i32);
impl windows_core::TypeKind for SHELL_LINK_DATA_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHELL_UI_COMPONENT(pub i32);
impl windows_core::TypeKind for SHELL_UI_COMPONENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHFMT_ID(pub u32);
impl windows_core::TypeKind for SHFMT_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHFMT_OPT(pub i32);
impl windows_core::TypeKind for SHFMT_OPT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHFMT_RET(pub u32);
impl windows_core::TypeKind for SHFMT_RET {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHGDFIL_FORMAT(pub i32);
impl windows_core::TypeKind for SHGDFIL_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHGDNF(pub u32);
impl windows_core::TypeKind for SHGDNF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHGFI_FLAGS(pub u32);
impl windows_core::TypeKind for SHGFI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHGFP_TYPE(pub i32);
impl windows_core::TypeKind for SHGFP_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHGLOBALCOUNTER(pub i32);
impl windows_core::TypeKind for SHGLOBALCOUNTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHGSI_FLAGS(pub u32);
impl windows_core::TypeKind for SHGSI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHOP_TYPE(pub i32);
impl windows_core::TypeKind for SHOP_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHREGDEL_FLAGS(pub i32);
impl windows_core::TypeKind for SHREGDEL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHREGENUM_FLAGS(pub i32);
impl windows_core::TypeKind for SHREGENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHSTOCKICONID(pub i32);
impl windows_core::TypeKind for SHSTOCKICONID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SIATTRIBFLAGS(pub i32);
impl windows_core::TypeKind for SIATTRIBFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SIGDN(pub i32);
impl windows_core::TypeKind for SIGDN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SIIGBF(pub i32);
impl windows_core::TypeKind for SIIGBF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SLGP_FLAGS(pub i32);
impl windows_core::TypeKind for SLGP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SLR_FLAGS(pub i32);
impl windows_core::TypeKind for SLR_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SMINFOFLAGS(pub i32);
impl windows_core::TypeKind for SMINFOFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SMINFOMASK(pub i32);
impl windows_core::TypeKind for SMINFOMASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SMINFOTYPE(pub i32);
impl windows_core::TypeKind for SMINFOTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SORTDIRECTION(pub i32);
impl windows_core::TypeKind for SORTDIRECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SORT_ORDER_TYPE(pub i32);
impl windows_core::TypeKind for SORT_ORDER_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SPACTION(pub i32);
impl windows_core::TypeKind for SPACTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SPTEXT(pub i32);
impl windows_core::TypeKind for SPTEXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SSF_MASK(pub u32);
impl windows_core::TypeKind for SSF_MASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STGOP(pub i32);
impl windows_core::TypeKind for STGOP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STORAGE_PROVIDER_FILE_FLAGS(pub i32);
impl windows_core::TypeKind for STORAGE_PROVIDER_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STPFLAG(pub i32);
impl windows_core::TypeKind for STPFLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SVUIA_STATUS(pub i32);
impl windows_core::TypeKind for SVUIA_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRERRORFLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGRERRORFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRFLAG(pub i32);
impl windows_core::TypeKind for SYNCMGRFLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRHANDLERFLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGRHANDLERFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRINVOKEFLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGRINVOKEFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRITEMFLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGRITEMFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRITEMSTATE(pub i32);
impl windows_core::TypeKind for SYNCMGRITEMSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRLOGLEVEL(pub i32);
impl windows_core::TypeKind for SYNCMGRLOGLEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRREGISTERFLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGRREGISTERFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGRSTATUS(pub i32);
impl windows_core::TypeKind for SYNCMGRSTATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_CANCEL_REQUEST(pub i32);
impl windows_core::TypeKind for SYNCMGR_CANCEL_REQUEST {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_CONFLICT_ITEM_TYPE(pub i32);
impl windows_core::TypeKind for SYNCMGR_CONFLICT_ITEM_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_CONTROL_FLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGR_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_EVENT_FLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGR_EVENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_EVENT_LEVEL(pub i32);
impl windows_core::TypeKind for SYNCMGR_EVENT_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_HANDLER_CAPABILITIES(pub i32);
impl windows_core::TypeKind for SYNCMGR_HANDLER_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_HANDLER_POLICIES(pub i32);
impl windows_core::TypeKind for SYNCMGR_HANDLER_POLICIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_HANDLER_TYPE(pub i32);
impl windows_core::TypeKind for SYNCMGR_HANDLER_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_ITEM_CAPABILITIES(pub i32);
impl windows_core::TypeKind for SYNCMGR_ITEM_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_ITEM_POLICIES(pub i32);
impl windows_core::TypeKind for SYNCMGR_ITEM_POLICIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_PRESENTER_CHOICE(pub i32);
impl windows_core::TypeKind for SYNCMGR_PRESENTER_CHOICE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_PRESENTER_NEXT_STEP(pub i32);
impl windows_core::TypeKind for SYNCMGR_PRESENTER_NEXT_STEP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_PROGRESS_STATUS(pub i32);
impl windows_core::TypeKind for SYNCMGR_PROGRESS_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_RESOLUTION_ABILITIES(pub i32);
impl windows_core::TypeKind for SYNCMGR_RESOLUTION_ABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_RESOLUTION_FEEDBACK(pub i32);
impl windows_core::TypeKind for SYNCMGR_RESOLUTION_FEEDBACK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_SYNC_CONTROL_FLAGS(pub i32);
impl windows_core::TypeKind for SYNCMGR_SYNC_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNCMGR_UPDATE_REASON(pub i32);
impl windows_core::TypeKind for SYNCMGR_UPDATE_REASON {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SecureLockIconConstants(pub i32);
impl windows_core::TypeKind for SecureLockIconConstants {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShellFolderViewOptions(pub i32);
impl windows_core::TypeKind for ShellFolderViewOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShellSpecialFolderConstants(pub i32);
impl windows_core::TypeKind for ShellSpecialFolderConstants {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShellWindowFindWindowOptions(pub i32);
impl windows_core::TypeKind for ShellWindowFindWindowOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShellWindowTypeConstants(pub i32);
impl windows_core::TypeKind for ShellWindowTypeConstants {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TBPFLAG(pub i32);
impl windows_core::TypeKind for TBPFLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct THUMBBUTTONFLAGS(pub i32);
impl windows_core::TypeKind for THUMBBUTTONFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct THUMBBUTTONMASK(pub i32);
impl windows_core::TypeKind for THUMBBUTTONMASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TI_FLAGS(pub i32);
impl windows_core::TypeKind for TI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TLENUMF(pub i32);
impl windows_core::TypeKind for TLENUMF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TRANSLATEURL_IN_FLAGS(pub i32);
impl windows_core::TypeKind for TRANSLATEURL_IN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ThumbnailStreamCacheOptions(pub i32);
impl windows_core::TypeKind for ThumbnailStreamCacheOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UNDOCK_REASON(pub i32);
impl windows_core::TypeKind for UNDOCK_REASON {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct URLASSOCIATIONDIALOG_IN_FLAGS(pub i32);
impl windows_core::TypeKind for URLASSOCIATIONDIALOG_IN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct URLIS(pub i32);
impl windows_core::TypeKind for URLIS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct URL_PART(pub i32);
impl windows_core::TypeKind for URL_PART {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct URL_SCHEME(pub i32);
impl windows_core::TypeKind for URL_SCHEME {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VALIDATEUNC_OPTION(pub i32);
impl windows_core::TypeKind for VALIDATEUNC_OPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VPCOLORFLAGS(pub i32);
impl windows_core::TypeKind for VPCOLORFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VPWATERMARKFLAGS(pub i32);
impl windows_core::TypeKind for VPWATERMARKFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WTS_ALPHATYPE(pub i32);
impl windows_core::TypeKind for WTS_ALPHATYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WTS_CACHEFLAGS(pub i32);
impl windows_core::TypeKind for WTS_CACHEFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WTS_CONTEXTFLAGS(pub i32);
impl windows_core::TypeKind for WTS_CONTEXTFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WTS_FLAGS(pub i32);
impl windows_core::TypeKind for WTS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _BROWSERFRAMEOPTIONS(pub i32);
impl windows_core::TypeKind for _BROWSERFRAMEOPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _CDBE_ACTIONS(pub i32);
impl windows_core::TypeKind for _CDBE_ACTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _EXPCMDFLAGS(pub i32);
impl windows_core::TypeKind for _EXPCMDFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _EXPCMDSTATE(pub i32);
impl windows_core::TypeKind for _EXPCMDSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _EXPLORERPANESTATE(pub i32);
impl windows_core::TypeKind for _EXPLORERPANESTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _EXPPS(pub i32);
impl windows_core::TypeKind for _EXPPS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _KF_DEFINITION_FLAGS(pub i32);
impl windows_core::TypeKind for _KF_DEFINITION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _KF_REDIRECTION_CAPABILITIES(pub i32);
impl windows_core::TypeKind for _KF_REDIRECTION_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _KF_REDIRECT_FLAGS(pub i32);
impl windows_core::TypeKind for _KF_REDIRECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _NMCII_FLAGS(pub i32);
impl windows_core::TypeKind for _NMCII_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _NMCSAEI_FLAGS(pub i32);
impl windows_core::TypeKind for _NMCSAEI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _NSTCECLICKTYPE(pub i32);
impl windows_core::TypeKind for _NSTCECLICKTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _NSTCEHITTEST(pub i32);
impl windows_core::TypeKind for _NSTCEHITTEST {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _NSTCITEMSTATE(pub i32);
impl windows_core::TypeKind for _NSTCITEMSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _NSTCROOTSTYLE(pub i32);
impl windows_core::TypeKind for _NSTCROOTSTYLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _NSTCSTYLE(pub i32);
impl windows_core::TypeKind for _NSTCSTYLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _OPPROGDLGF(pub i32);
impl windows_core::TypeKind for _OPPROGDLGF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _PDMODE(pub i32);
impl windows_core::TypeKind for _PDMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _SHCONTF(pub i32);
impl windows_core::TypeKind for _SHCONTF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _SICHINTF(pub i32);
impl windows_core::TypeKind for _SICHINTF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _SPBEGINF(pub i32);
impl windows_core::TypeKind for _SPBEGINF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _SPINITF(pub i32);
impl windows_core::TypeKind for _SPINITF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _SV3CVW3_FLAGS(pub i32);
impl windows_core::TypeKind for _SV3CVW3_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _SVGIO(pub i32);
impl windows_core::TypeKind for _SVGIO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _SVSIF(pub i32);
impl windows_core::TypeKind for _SVSIF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _TRANSFER_ADVISE_STATE(pub i32);
impl windows_core::TypeKind for _TRANSFER_ADVISE_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _TRANSFER_SOURCE_FLAGS(pub i32);
impl windows_core::TypeKind for _TRANSFER_SOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AASHELLMENUFILENAME {
    pub cbTotal: i16,
    pub rgbReserved: [u8; 12],
    pub szFileName: [u16; 1],
}
impl Default for AASHELLMENUFILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AASHELLMENUFILENAME {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AASHELLMENUITEM {
    pub lpReserved1: *mut core::ffi::c_void,
    pub iReserved: i32,
    pub uiReserved: u32,
    pub lpName: *mut AASHELLMENUFILENAME,
    pub psz: windows_core::PWSTR,
}
impl Default for AASHELLMENUITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AASHELLMENUITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPBARDATA {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uCallbackMessage: u32,
    pub uEdge: u32,
    pub rc: super::super::Foundation::RECT,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for APPBARDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for APPBARDATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPBARDATA {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uCallbackMessage: u32,
    pub uEdge: u32,
    pub rc: super::super::Foundation::RECT,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(target_arch = "x86")]
impl Default for APPBARDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for APPBARDATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPCATEGORYINFO {
    pub Locale: u32,
    pub pszDescription: windows_core::PWSTR,
    pub AppCategoryId: windows_core::GUID,
}
impl Default for APPCATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for APPCATEGORYINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPCATEGORYINFOLIST {
    pub cCategory: u32,
    pub pCategoryInfo: *mut APPCATEGORYINFO,
}
impl Default for APPCATEGORYINFOLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for APPCATEGORYINFOLIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPINFODATA {
    pub cbSize: u32,
    pub dwMask: u32,
    pub pszDisplayName: windows_core::PWSTR,
    pub pszVersion: windows_core::PWSTR,
    pub pszPublisher: windows_core::PWSTR,
    pub pszProductID: windows_core::PWSTR,
    pub pszRegisteredOwner: windows_core::PWSTR,
    pub pszRegisteredCompany: windows_core::PWSTR,
    pub pszLanguage: windows_core::PWSTR,
    pub pszSupportUrl: windows_core::PWSTR,
    pub pszSupportTelephone: windows_core::PWSTR,
    pub pszHelpLink: windows_core::PWSTR,
    pub pszInstallLocation: windows_core::PWSTR,
    pub pszInstallSource: windows_core::PWSTR,
    pub pszInstallDate: windows_core::PWSTR,
    pub pszContact: windows_core::PWSTR,
    pub pszComments: windows_core::PWSTR,
    pub pszImage: windows_core::PWSTR,
    pub pszReadmeUrl: windows_core::PWSTR,
    pub pszUpdateInfoUrl: windows_core::PWSTR,
}
impl Default for APPINFODATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for APPINFODATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ASSOCIATIONELEMENT {
    pub ac: ASSOCCLASS,
    pub hkClass: super::super::System::Registry::HKEY,
    pub pszClass: windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl Default for ASSOCIATIONELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for ASSOCIATIONELEMENT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ASSOCIATIONELEMENT {
    pub ac: ASSOCCLASS,
    pub hkClass: super::super::System::Registry::HKEY,
    pub pszClass: windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl Default for ASSOCIATIONELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for ASSOCIATIONELEMENT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTO_SCROLL_DATA {
    pub iNextSample: i32,
    pub dwLastScroll: u32,
    pub bFull: super::super::Foundation::BOOL,
    pub pts: [super::super::Foundation::POINT; 3],
    pub dwTimes: [u32; 3],
}
impl Default for AUTO_SCROLL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTO_SCROLL_DATA {
    type TypeKind = windows_core::CloneType;
}
pub const AccessibilityDockingService: windows_core::GUID = windows_core::GUID::from_u128(0x29ce1d46_b481_4aa0_a08a_d3ebc8aca402);
pub const AlphabeticalCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x3c2654c6_7372_4f6b_b310_55d6128f49d2);
pub const AppShellVerbHandler: windows_core::GUID = windows_core::GUID::from_u128(0x4ed3a719_cea8_4bd9_910d_e252f997afc2);
pub const AppStartupLink: windows_core::GUID = windows_core::GUID::from_u128(0x273eb5e7_88b0_4843_bfef_e2c81d43aae5);
pub const AppVisibility: windows_core::GUID = windows_core::GUID::from_u128(0x7e5fe3d9_985f_4908_91f9_ee19f9fd1514);
pub const ApplicationActivationManager: windows_core::GUID = windows_core::GUID::from_u128(0x45ba127d_10a8_46ea_8ab7_56ea9078943c);
pub const ApplicationAssociationRegistration: windows_core::GUID = windows_core::GUID::from_u128(0x591209c7_767b_42b2_9fba_44ee4615f2c7);
pub const ApplicationAssociationRegistrationUI: windows_core::GUID = windows_core::GUID::from_u128(0x1968106d_f3b5_44cf_890e_116fcb9ecef1);
pub const ApplicationDesignModeSettings: windows_core::GUID = windows_core::GUID::from_u128(0x958a6fb5_dcb2_4faf_aafd_7fb054ad1a3b);
pub const ApplicationDestinations: windows_core::GUID = windows_core::GUID::from_u128(0x86c14003_4d6b_4ef3_a7b4_0506663b2e68);
pub const ApplicationDocumentLists: windows_core::GUID = windows_core::GUID::from_u128(0x86bec222_30f2_47e0_9f25_60d11cd75c28);
pub const AttachmentServices: windows_core::GUID = windows_core::GUID::from_u128(0x4125dd96_e03a_4103_8f70_e0597d803b9c);
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BANDINFOSFB {
    pub dwMask: u32,
    pub dwStateMask: u32,
    pub dwState: u32,
    pub crBkgnd: super::super::Foundation::COLORREF,
    pub crBtnLt: super::super::Foundation::COLORREF,
    pub crBtnDk: super::super::Foundation::COLORREF,
    pub wViewMode: u16,
    pub wAlign: u16,
    pub psf: Option<IShellFolder>,
    pub pidl: *mut Common::ITEMIDLIST,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for BANDINFOSFB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for BANDINFOSFB {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BANDSITEINFO {
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStyle: u32,
}
impl Default for BANDSITEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BANDSITEINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BANNER_NOTIFICATION {
    pub event: BANNER_NOTIFICATION_EVENT,
    pub providerIdentity: windows_core::PCWSTR,
    pub contentId: windows_core::PCWSTR,
}
impl Default for BANNER_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BANNER_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BASEBROWSERDATALH {
    pub _hwnd: super::super::Foundation::HWND,
    pub _ptl: Option<ITravelLog>,
    pub _phlf: Option<IHlinkFrame>,
    pub _pautoWB2: Option<IWebBrowser2>,
    pub _pautoEDS: Option<IExpDispSupport>,
    pub _pautoSS: Option<IShellService>,
    pub _eSecureLockIcon: i32,
    pub _bitfield: u32,
    pub _uActivateState: u32,
    pub _pidlViewState: *mut Common::ITEMIDLIST,
    pub _pctView: Option<super::super::System::Ole::IOleCommandTarget>,
    pub _pidlCur: *mut Common::ITEMIDLIST,
    pub _psv: Option<IShellView>,
    pub _psf: Option<IShellFolder>,
    pub _hwndView: super::super::Foundation::HWND,
    pub _pszTitleCur: windows_core::PWSTR,
    pub _pidlPending: *mut Common::ITEMIDLIST,
    pub _psvPending: Option<IShellView>,
    pub _psfPending: Option<IShellFolder>,
    pub _hwndViewPending: super::super::Foundation::HWND,
    pub _pszTitlePending: windows_core::PWSTR,
    pub _fIsViewMSHTML: super::super::Foundation::BOOL,
    pub _fPrivacyImpacted: super::super::Foundation::BOOL,
    pub _clsidView: windows_core::GUID,
    pub _clsidViewPending: windows_core::GUID,
    pub _hwndFrame: super::super::Foundation::HWND,
    pub _lPhishingFilterStatus: i32,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl Default for BASEBROWSERDATALH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl windows_core::TypeKind for BASEBROWSERDATALH {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BASEBROWSERDATAXP {
    pub _hwnd: super::super::Foundation::HWND,
    pub _ptl: Option<ITravelLog>,
    pub _phlf: Option<IHlinkFrame>,
    pub _pautoWB2: Option<IWebBrowser2>,
    pub _pautoEDS: Option<IExpDispSupportXP>,
    pub _pautoSS: Option<IShellService>,
    pub _eSecureLockIcon: i32,
    pub _bitfield: u32,
    pub _uActivateState: u32,
    pub _pidlViewState: *mut Common::ITEMIDLIST,
    pub _pctView: Option<super::super::System::Ole::IOleCommandTarget>,
    pub _pidlCur: *mut Common::ITEMIDLIST,
    pub _psv: Option<IShellView>,
    pub _psf: Option<IShellFolder>,
    pub _hwndView: super::super::Foundation::HWND,
    pub _pszTitleCur: windows_core::PWSTR,
    pub _pidlPending: *mut Common::ITEMIDLIST,
    pub _psvPending: Option<IShellView>,
    pub _psfPending: Option<IShellFolder>,
    pub _hwndViewPending: super::super::Foundation::HWND,
    pub _pszTitlePending: windows_core::PWSTR,
    pub _fIsViewMSHTML: super::super::Foundation::BOOL,
    pub _fPrivacyImpacted: super::super::Foundation::BOOL,
    pub _clsidView: windows_core::GUID,
    pub _clsidViewPending: windows_core::GUID,
    pub _hwndFrame: super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl Default for BASEBROWSERDATAXP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl windows_core::TypeKind for BASEBROWSERDATAXP {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BROWSEINFOA {
    pub hwndOwner: super::super::Foundation::HWND,
    pub pidlRoot: *mut Common::ITEMIDLIST,
    pub pszDisplayName: windows_core::PSTR,
    pub lpszTitle: windows_core::PCSTR,
    pub ulFlags: u32,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for BROWSEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for BROWSEINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BROWSEINFOW {
    pub hwndOwner: super::super::Foundation::HWND,
    pub pidlRoot: *mut Common::ITEMIDLIST,
    pub pszDisplayName: windows_core::PWSTR,
    pub lpszTitle: windows_core::PCWSTR,
    pub ulFlags: u32,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for BROWSEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for BROWSEINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABINETSTATE {
    pub cLength: u16,
    pub nVersion: u16,
    pub _bitfield: i32,
    pub fMenuEnumFilter: u32,
}
impl Default for CABINETSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CABINETSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CATEGORY_INFO {
    pub cif: CATEGORYINFO_FLAGS,
    pub wszName: [u16; 260],
}
impl Default for CATEGORY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CATEGORY_INFO {
    type TypeKind = windows_core::CopyType;
}
pub const CDBurn: windows_core::GUID = windows_core::GUID::from_u128(0xfbeb8a05_beee_4442_804e_409d6c4515e9);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CIDA {
    pub cidl: u32,
    pub aoffset: [u32; 1],
}
impl Default for CIDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CIDA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMINVOKECOMMANDINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub lpVerb: windows_core::PCSTR,
    pub lpParameters: windows_core::PCSTR,
    pub lpDirectory: windows_core::PCSTR,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub hIcon: super::super::Foundation::HANDLE,
}
impl Default for CMINVOKECOMMANDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CMINVOKECOMMANDINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMINVOKECOMMANDINFOEX {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub lpVerb: windows_core::PCSTR,
    pub lpParameters: windows_core::PCSTR,
    pub lpDirectory: windows_core::PCSTR,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub hIcon: super::super::Foundation::HANDLE,
    pub lpTitle: windows_core::PCSTR,
    pub lpVerbW: windows_core::PCWSTR,
    pub lpParametersW: windows_core::PCWSTR,
    pub lpDirectoryW: windows_core::PCWSTR,
    pub lpTitleW: windows_core::PCWSTR,
    pub ptInvoke: super::super::Foundation::POINT,
}
impl Default for CMINVOKECOMMANDINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CMINVOKECOMMANDINFOEX {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMINVOKECOMMANDINFOEX_REMOTE {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub lpVerbString: windows_core::PCSTR,
    pub lpParameters: windows_core::PCSTR,
    pub lpDirectory: windows_core::PCSTR,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub lpTitle: windows_core::PCSTR,
    pub lpVerbWString: windows_core::PCWSTR,
    pub lpParametersW: windows_core::PCWSTR,
    pub lpDirectoryW: windows_core::PCWSTR,
    pub lpTitleW: windows_core::PCWSTR,
    pub ptInvoke: super::super::Foundation::POINT,
    pub lpVerbInt: u32,
    pub lpVerbWInt: u32,
}
impl Default for CMINVOKECOMMANDINFOEX_REMOTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CMINVOKECOMMANDINFOEX_REMOTE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_COLUMNINFO {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwState: u32,
    pub uWidth: u32,
    pub uDefaultWidth: u32,
    pub uIdealWidth: u32,
    pub wszName: [u16; 80],
}
impl Default for CM_COLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CM_COLUMNINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFIRM_CONFLICT_ITEM {
    pub pShellItem: Option<IShellItem2>,
    pub pszOriginalName: windows_core::PWSTR,
    pub pszAlternateName: windows_core::PWSTR,
    pub pszLocationShort: windows_core::PWSTR,
    pub pszLocationFull: windows_core::PWSTR,
    pub nType: SYNCMGR_CONFLICT_ITEM_TYPE,
}
impl Default for CONFIRM_CONFLICT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CONFIRM_CONFLICT_ITEM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFIRM_CONFLICT_RESULT_INFO {
    pub pszNewName: windows_core::PWSTR,
    pub iItemIndex: u32,
}
impl Default for CONFIRM_CONFLICT_RESULT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CONFIRM_CONFLICT_RESULT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPLINFO {
    pub idIcon: i32,
    pub idName: i32,
    pub idInfo: i32,
    pub lData: isize,
}
impl Default for CPLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CPLINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    pub ulAuthenticationPackage: u32,
    pub clsidCredentialProvider: windows_core::GUID,
    pub cbSerialization: u32,
    pub rgbSerialization: *mut u8,
}
impl Default for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    pub dwFieldID: u32,
    pub cpft: CREDENTIAL_PROVIDER_FIELD_TYPE,
    pub pszLabel: windows_core::PWSTR,
    pub guidFieldType: windows_core::GUID,
}
impl Default for CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSFV {
    pub cbSize: u32,
    pub pshf: Option<IShellFolder>,
    pub psvOuter: Option<IShellView>,
    pub pidl: *mut Common::ITEMIDLIST,
    pub lEvents: i32,
    pub pfnCallback: LPFNVIEWCALLBACK,
    pub fvm: FOLDERVIEWMODE,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for CSFV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for CSFV {
    type TypeKind = windows_core::CloneType;
}
pub const CScriptErrorList: windows_core::GUID = windows_core::GUID::from_u128(0xefd01300_160f_11d2_bb2e_00805ff7efca);
pub const ConflictFolder: windows_core::GUID = windows_core::GUID::from_u128(0x289978ac_a101_4341_a817_21eba7fd046d);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DATABLOCK_HEADER {
    pub cbSize: u32,
    pub dwSignature: u32,
}
impl Default for DATABLOCK_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DATABLOCK_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEFCONTEXTMENU {
    pub hwnd: super::super::Foundation::HWND,
    pub pcmcb: Option<IContextMenuCB>,
    pub pidlFolder: *mut Common::ITEMIDLIST,
    pub psf: Option<IShellFolder>,
    pub cidl: u32,
    pub apidl: *mut *mut Common::ITEMIDLIST,
    pub punkAssociationInfo: Option<windows_core::IUnknown>,
    pub cKeys: u32,
    pub aKeys: *const super::super::System::Registry::HKEY,
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
impl Default for DEFCONTEXTMENU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
impl windows_core::TypeKind for DEFCONTEXTMENU {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DELEGATEITEMID {
    pub cbSize: u16,
    pub wOuter: u16,
    pub cbInner: u16,
    pub rgb: [u8; 1],
}
impl Default for DELEGATEITEMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DELEGATEITEMID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DESKBANDINFO {
    pub dwMask: u32,
    pub ptMinSize: super::super::Foundation::POINTL,
    pub ptMaxSize: super::super::Foundation::POINTL,
    pub ptIntegral: super::super::Foundation::POINTL,
    pub ptActual: super::super::Foundation::POINTL,
    pub wszTitle: [u16; 256],
    pub dwModeFlags: u32,
    pub crBkgnd: super::super::Foundation::COLORREF,
}
impl Default for DESKBANDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DESKBANDINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DETAILSINFO {
    pub pidl: *mut Common::ITEMIDLIST,
    pub fmt: i32,
    pub cxChar: i32,
    pub str: Common::STRRET,
    pub iImage: i32,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for DETAILSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for DETAILSINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DFMICS {
    pub cbSize: u32,
    pub fMask: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub idCmdFirst: u32,
    pub idDefMax: u32,
    pub pici: *mut CMINVOKECOMMANDINFO,
    pub punkSite: Option<windows_core::IUnknown>,
}
impl Default for DFMICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DFMICS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DLLVERSIONINFO {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformID: u32,
}
impl Default for DLLVERSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DLLVERSIONINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DLLVERSIONINFO2 {
    pub info1: DLLVERSIONINFO,
    pub dwFlags: u32,
    pub ullVersion: u64,
}
impl Default for DLLVERSIONINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DLLVERSIONINFO2 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRAGINFOA {
    pub uSize: u32,
    pub pt: super::super::Foundation::POINT,
    pub fNC: super::super::Foundation::BOOL,
    pub lpFileList: windows_core::PSTR,
    pub grfKeyState: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DRAGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DRAGINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRAGINFOA {
    pub uSize: u32,
    pub pt: super::super::Foundation::POINT,
    pub fNC: super::super::Foundation::BOOL,
    pub lpFileList: windows_core::PSTR,
    pub grfKeyState: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DRAGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DRAGINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRAGINFOW {
    pub uSize: u32,
    pub pt: super::super::Foundation::POINT,
    pub fNC: super::super::Foundation::BOOL,
    pub lpFileList: windows_core::PWSTR,
    pub grfKeyState: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DRAGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DRAGINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRAGINFOW {
    pub uSize: u32,
    pub pt: super::super::Foundation::POINT,
    pub fNC: super::super::Foundation::BOOL,
    pub lpFileList: windows_core::PWSTR,
    pub grfKeyState: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DRAGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DRAGINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DROPDESCRIPTION {
    pub r#type: DROPIMAGETYPE,
    pub szMessage: [u16; 260],
    pub szInsert: [u16; 260],
}
impl Default for DROPDESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DROPDESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DROPFILES {
    pub pFiles: u32,
    pub pt: super::super::Foundation::POINT,
    pub fNC: super::super::Foundation::BOOL,
    pub fWide: super::super::Foundation::BOOL,
}
impl Default for DROPFILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DROPFILES {
    type TypeKind = windows_core::CloneType;
}
pub const DefFolderMenu: windows_core::GUID = windows_core::GUID::from_u128(0xc63382be_7933_48d0_9ac8_85fb46be2fdd);
pub const DesktopGadget: windows_core::GUID = windows_core::GUID::from_u128(0x924ccc1b_6562_4c85_8657_d177925222b6);
pub const DesktopWallpaper: windows_core::GUID = windows_core::GUID::from_u128(0xc2cf3110_460e_4fc1_b9d0_8a1c0c9cc4bd);
pub const DestinationList: windows_core::GUID = windows_core::GUID::from_u128(0x77f10cf0_3db5_4966_b520_b7c54fd35ed6);
pub const DocPropShellExtension: windows_core::GUID = windows_core::GUID::from_u128(0x883373c3_bf89_11d1_be35_080036b11a03);
pub const DriveSizeCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x94357b53_ca29_4b78_83ae_e8fe7409134f);
pub const DriveTypeCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0xb0a8f3cf_4333_4bab_8873_1ccb1cada48b);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXP_DARWIN_LINK {
    pub dbh: DATABLOCK_HEADER,
    pub szDarwinID: [i8; 260],
    pub szwDarwinID: [u16; 260],
}
impl Default for EXP_DARWIN_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EXP_DARWIN_LINK {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXP_PROPERTYSTORAGE {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub abPropertyStorage: [u8; 1],
}
impl Default for EXP_PROPERTYSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EXP_PROPERTYSTORAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXP_SPECIAL_FOLDER {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub idSpecialFolder: u32,
    pub cbOffset: u32,
}
impl Default for EXP_SPECIAL_FOLDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EXP_SPECIAL_FOLDER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXP_SZ_LINK {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub szTarget: [i8; 260],
    pub swzTarget: [u16; 260],
}
impl Default for EXP_SZ_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EXP_SZ_LINK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTRASEARCH {
    pub guidSearch: windows_core::GUID,
    pub wszFriendlyName: [u16; 80],
    pub wszUrl: [u16; 2084],
}
impl Default for EXTRASEARCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EXTRASEARCH {
    type TypeKind = windows_core::CopyType;
}
pub const EnumerableObjectCollection: windows_core::GUID = windows_core::GUID::from_u128(0x2d3468c1_36a7_43b6_ac24_d3f02fd9607a);
pub const ExecuteFolder: windows_core::GUID = windows_core::GUID::from_u128(0x11dbb47c_a525_400b_9e80_a54615a090c0);
pub const ExecuteUnknown: windows_core::GUID = windows_core::GUID::from_u128(0xe44e9428_bdbc_4987_a099_40dc8fd255e7);
pub const ExplorerBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x71f96385_ddd6_48d3_a0c1_ae06e8b055fb);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILEDESCRIPTORA {
    pub dwFlags: u32,
    pub clsid: windows_core::GUID,
    pub sizel: super::super::Foundation::SIZE,
    pub pointl: super::super::Foundation::POINTL,
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub cFileName: [i8; 260],
}
impl Default for FILEDESCRIPTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FILEDESCRIPTORA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILEDESCRIPTORW {
    pub dwFlags: u32,
    pub clsid: windows_core::GUID,
    pub sizel: super::super::Foundation::SIZE,
    pub pointl: super::super::Foundation::POINTL,
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub cFileName: [u16; 260],
}
impl Default for FILEDESCRIPTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FILEDESCRIPTORW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILEGROUPDESCRIPTORA {
    pub cItems: u32,
    pub fgd: [FILEDESCRIPTORA; 1],
}
impl Default for FILEGROUPDESCRIPTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FILEGROUPDESCRIPTORA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILEGROUPDESCRIPTORW {
    pub cItems: u32,
    pub fgd: [FILEDESCRIPTORW; 1],
}
impl Default for FILEGROUPDESCRIPTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FILEGROUPDESCRIPTORW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ATTRIBUTES_ARRAY {
    pub cItems: u32,
    pub dwSumFileAttributes: u32,
    pub dwProductFileAttributes: u32,
    pub rgdwFileAttributes: [u32; 1],
}
impl Default for FILE_ATTRIBUTES_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FILE_ATTRIBUTES_ARRAY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FOLDERSETDATA {
    pub _fs: FOLDERSETTINGS,
    pub _vidRestore: windows_core::GUID,
    pub _dwViewPriority: u32,
}
impl Default for FOLDERSETDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FOLDERSETDATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FOLDERSETTINGS {
    pub ViewMode: u32,
    pub fFlags: u32,
}
impl Default for FOLDERSETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FOLDERSETTINGS {
    type TypeKind = windows_core::CopyType;
}
pub const FSCopyHandler: windows_core::GUID = windows_core::GUID::from_u128(0xd197380a_0a79_4dc8_a033_ed882c2fa14b);
pub const FileOpenDialog: windows_core::GUID = windows_core::GUID::from_u128(0xdc1c5a9c_e88a_4dde_a5a1_60f82a20aef7);
pub const FileOperation: windows_core::GUID = windows_core::GUID::from_u128(0x3ad05575_8857_4850_9277_11b85bdb8e09);
pub const FileSaveDialog: windows_core::GUID = windows_core::GUID::from_u128(0xc0b4e2f3_ba21_4773_8dba_335ec946eb8b);
pub const FileSearchBand: windows_core::GUID = windows_core::GUID::from_u128(0xc4ee31f3_4768_11d2_be5c_00a0c9a83da1);
pub const FolderViewHost: windows_core::GUID = windows_core::GUID::from_u128(0x20b1cb23_6968_4eb9_b7d4_a66d00d07cee);
pub const FrameworkInputPane: windows_core::GUID = windows_core::GUID::from_u128(0xd5120aa3_46ba_44c5_822d_ca8092c1fc72);
pub const FreeSpaceCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0xb5607793_24ac_44c7_82e2_831726aa6cb7);
pub const GenericCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x25cbb996_92ed_457e_b28c_4774084bd562);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HELPINFO {
    pub cbSize: u32,
    pub iContextType: HELP_INFO_TYPE,
    pub iCtrlId: i32,
    pub hItemHandle: super::super::Foundation::HANDLE,
    pub dwContextId: usize,
    pub MousePos: super::super::Foundation::POINT,
}
impl Default for HELPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HELPINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HELPWININFOA {
    pub wStructSize: i32,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub wMax: i32,
    pub rgchMember: [i8; 2],
}
impl Default for HELPWININFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HELPWININFOA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HELPWININFOW {
    pub wStructSize: i32,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub wMax: i32,
    pub rgchMember: [u16; 2],
}
impl Default for HELPWININFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HELPWININFOW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HLBWINFO {
    pub cbSize: u32,
    pub grfHLBWIF: u32,
    pub rcFramePos: super::super::Foundation::RECT,
    pub rcDocPos: super::super::Foundation::RECT,
    pub hltbinfo: HLTBINFO,
}
impl Default for HLBWINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HLBWINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HLITEM {
    pub uHLID: u32,
    pub pwzFriendlyName: windows_core::PWSTR,
}
impl Default for HLITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HLITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HLTBINFO {
    pub uDockType: u32,
    pub rcTbPos: super::super::Foundation::RECT,
}
impl Default for HLTBINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HLTBINFO {
    type TypeKind = windows_core::CloneType;
}
pub const HideInputPaneAnimationCoordinator: windows_core::GUID = windows_core::GUID::from_u128(0x384742b1_2a77_4cb3_8cf8_1136f5e17e59);
pub const HomeGroup: windows_core::GUID = windows_core::GUID::from_u128(0xde77ba04_3c92_4d11_a1a5_42352a53e0e3);
pub const IENamespaceTreeControl: windows_core::GUID = windows_core::GUID::from_u128(0xace52d03_e5cd_4b20_82ff_e71b11beae1d);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ITEMSPACING {
    pub cxSmall: i32,
    pub cySmall: i32,
    pub cxLarge: i32,
    pub cyLarge: i32,
}
impl Default for ITEMSPACING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ITEMSPACING {
    type TypeKind = windows_core::CopyType;
}
pub const ImageProperties: windows_core::GUID = windows_core::GUID::from_u128(0x7ab770c7_0e23_4d7a_8aa2_19bfad479829);
pub const ImageRecompress: windows_core::GUID = windows_core::GUID::from_u128(0x6e33091c_d2f8_4740_b55e_2e11d1477a2c);
pub const ImageTranscode: windows_core::GUID = windows_core::GUID::from_u128(0x17b75166_928f_417d_9685_64aa135565c1);
pub const InputPanelConfiguration: windows_core::GUID = windows_core::GUID::from_u128(0x2853add3_f096_4c63_a78f_7fa3ea837fb7);
pub const InternetExplorer: windows_core::GUID = windows_core::GUID::from_u128(0x0002df01_0000_0000_c000_000000000046);
pub const InternetExplorerMedium: windows_core::GUID = windows_core::GUID::from_u128(0xd5e8041d_920f_45e9_b8fb_b1deb82c6e5e);
pub const InternetPrintOrdering: windows_core::GUID = windows_core::GUID::from_u128(0xadd36aa8_751a_4579_a266_d66f5202ccbb);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KNOWNFOLDER_DEFINITION {
    pub category: KF_CATEGORY,
    pub pszName: windows_core::PWSTR,
    pub pszDescription: windows_core::PWSTR,
    pub fidParent: windows_core::GUID,
    pub pszRelativePath: windows_core::PWSTR,
    pub pszParsingName: windows_core::PWSTR,
    pub pszTooltip: windows_core::PWSTR,
    pub pszLocalizedName: windows_core::PWSTR,
    pub pszIcon: windows_core::PWSTR,
    pub pszSecurity: windows_core::PWSTR,
    pub dwAttributes: u32,
    pub kfdFlags: u32,
    pub ftidType: windows_core::GUID,
}
impl Default for KNOWNFOLDER_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for KNOWNFOLDER_DEFINITION {
    type TypeKind = windows_core::CopyType;
}
pub const KnownFolderManager: windows_core::GUID = windows_core::GUID::from_u128(0x4df0c730_df9d_4ae3_9153_aa6b82e9795a);
pub const LocalThumbnailCache: windows_core::GUID = windows_core::GUID::from_u128(0x50ef4544_ac9f_4a8e_b21b_8a26180db13f);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MULTIKEYHELPA {
    pub mkSize: u32,
    pub mkKeylist: i8,
    pub szKeyphrase: [i8; 1],
}
impl Default for MULTIKEYHELPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MULTIKEYHELPA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MULTIKEYHELPW {
    pub mkSize: u32,
    pub mkKeylist: u16,
    pub szKeyphrase: [u16; 1],
}
impl Default for MULTIKEYHELPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MULTIKEYHELPW {
    type TypeKind = windows_core::CopyType;
}
pub const MailRecipient: windows_core::GUID = windows_core::GUID::from_u128(0x9e56be60_c50f_11cf_9a2c_00a0c90a90ce);
pub const MergedCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x8e827c11_33e7_4bc1_b242_8cd9a1c2b304);
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_IpHelper", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NC_ADDRESS {
    pub pAddrInfo: *mut super::super::NetworkManagement::IpHelper::NET_ADDRESS_INFO,
    pub PortNumber: u16,
    pub PrefixLength: u8,
}
#[cfg(all(feature = "Win32_NetworkManagement_IpHelper", feature = "Win32_Networking_WinSock"))]
impl Default for NC_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_IpHelper", feature = "Win32_Networking_WinSock"))]
impl windows_core::TypeKind for NC_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NEWCPLINFOA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHelpContext: u32,
    pub lData: isize,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub szName: [i8; 32],
    pub szInfo: [i8; 64],
    pub szHelpFile: [i8; 128],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NEWCPLINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NEWCPLINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NEWCPLINFOW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHelpContext: u32,
    pub lData: isize,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub szName: [u16; 32],
    pub szInfo: [u16; 64],
    pub szHelpFile: [u16; 128],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NEWCPLINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NEWCPLINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFYICONDATAA {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uID: u32,
    pub uFlags: NOTIFY_ICON_DATA_FLAGS,
    pub uCallbackMessage: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub szTip: [i8; 128],
    pub dwState: NOTIFY_ICON_STATE,
    pub dwStateMask: NOTIFY_ICON_STATE,
    pub szInfo: [i8; 256],
    pub Anonymous: NOTIFYICONDATAA_0,
    pub szInfoTitle: [i8; 64],
    pub dwInfoFlags: NOTIFY_ICON_INFOTIP_FLAGS,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: super::WindowsAndMessaging::HICON,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union NOTIFYICONDATAA_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAA_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFYICONDATAA {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uID: u32,
    pub uFlags: NOTIFY_ICON_DATA_FLAGS,
    pub uCallbackMessage: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub szTip: [i8; 128],
    pub dwState: NOTIFY_ICON_STATE,
    pub dwStateMask: NOTIFY_ICON_STATE,
    pub szInfo: [i8; 256],
    pub Anonymous: NOTIFYICONDATAA_0,
    pub szInfoTitle: [i8; 64],
    pub dwInfoFlags: NOTIFY_ICON_INFOTIP_FLAGS,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: super::WindowsAndMessaging::HICON,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union NOTIFYICONDATAA_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAA_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uID: u32,
    pub uFlags: NOTIFY_ICON_DATA_FLAGS,
    pub uCallbackMessage: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub szTip: [u16; 128],
    pub dwState: NOTIFY_ICON_STATE,
    pub dwStateMask: NOTIFY_ICON_STATE,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: NOTIFY_ICON_INFOTIP_FLAGS,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: super::WindowsAndMessaging::HICON,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAW_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uID: u32,
    pub uFlags: NOTIFY_ICON_DATA_FLAGS,
    pub uCallbackMessage: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub szTip: [u16; 128],
    pub dwState: NOTIFY_ICON_STATE,
    pub dwStateMask: NOTIFY_ICON_STATE,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: NOTIFY_ICON_INFOTIP_FLAGS,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: super::WindowsAndMessaging::HICON,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for NOTIFYICONDATAW_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFYICONIDENTIFIER {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uID: u32,
    pub guidItem: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for NOTIFYICONIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for NOTIFYICONIDENTIFIER {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFYICONIDENTIFIER {
    pub cbSize: u32,
    pub hWnd: super::super::Foundation::HWND,
    pub uID: u32,
    pub guidItem: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl Default for NOTIFYICONIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for NOTIFYICONIDENTIFIER {
    type TypeKind = windows_core::CloneType;
}
pub const NPCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x3dd6bec0_8193_4ffe_ae25_e08e39ea4063);
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_WNet")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NRESARRAY {
    pub cItems: u32,
    pub nr: [super::super::NetworkManagement::WNet::NETRESOURCEA; 1],
}
#[cfg(feature = "Win32_NetworkManagement_WNet")]
impl Default for NRESARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_WNet")]
impl windows_core::TypeKind for NRESARRAY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSTCCUSTOMDRAW {
    pub psi: Option<IShellItem>,
    pub uItemState: u32,
    pub nstcis: u32,
    pub pszText: windows_core::PCWSTR,
    pub iImage: i32,
    pub himl: super::Controls::HIMAGELIST,
    pub iLevel: i32,
    pub iIndent: i32,
}
#[cfg(feature = "Win32_UI_Controls")]
impl Default for NSTCCUSTOMDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Controls")]
impl windows_core::TypeKind for NSTCCUSTOMDRAW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NT_CONSOLE_PROPS {
    pub dbh: DATABLOCK_HEADER,
    pub wFillAttribute: u16,
    pub wPopupFillAttribute: u16,
    pub dwScreenBufferSize: super::super::System::Console::COORD,
    pub dwWindowSize: super::super::System::Console::COORD,
    pub dwWindowOrigin: super::super::System::Console::COORD,
    pub nFont: u32,
    pub nInputBufferSize: u32,
    pub dwFontSize: super::super::System::Console::COORD,
    pub uFontFamily: u32,
    pub uFontWeight: u32,
    pub FaceName: [u16; 32],
    pub uCursorSize: u32,
    pub bFullScreen: super::super::Foundation::BOOL,
    pub bQuickEdit: super::super::Foundation::BOOL,
    pub bInsertMode: super::super::Foundation::BOOL,
    pub bAutoPosition: super::super::Foundation::BOOL,
    pub uHistoryBufferSize: u32,
    pub uNumberOfHistoryBuffers: u32,
    pub bHistoryNoDup: super::super::Foundation::BOOL,
    pub ColorTable: [super::super::Foundation::COLORREF; 16],
}
#[cfg(feature = "Win32_System_Console")]
impl Default for NT_CONSOLE_PROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl windows_core::TypeKind for NT_CONSOLE_PROPS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NT_FE_CONSOLE_PROPS {
    pub dbh: DATABLOCK_HEADER,
    pub uCodePage: u32,
}
impl Default for NT_FE_CONSOLE_PROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NT_FE_CONSOLE_PROPS {
    type TypeKind = windows_core::CloneType;
}
pub const NamespaceTreeControl: windows_core::GUID = windows_core::GUID::from_u128(0xae054212_3535_4430_83ed_d501aa6680e6);
pub const NamespaceWalker: windows_core::GUID = windows_core::GUID::from_u128(0x72eb61e0_8672_4303_9175_f2e4c68b2e7c);
pub const NetworkConnections: windows_core::GUID = windows_core::GUID::from_u128(0x7007acc7_3202_11d1_aad2_00805fc1270e);
pub const NetworkExplorerFolder: windows_core::GUID = windows_core::GUID::from_u128(0xf02c1a0d_be21_4350_88b0_7367fc96ef3c);
pub const NetworkPlaces: windows_core::GUID = windows_core::GUID::from_u128(0x208d2c60_3aea_1069_a2d7_08002b30309d);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPENASINFO {
    pub pcszFile: windows_core::PCWSTR,
    pub pcszClass: windows_core::PCWSTR,
    pub oaifInFlags: OPEN_AS_INFO_FLAGS,
}
impl Default for OPENASINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OPENASINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPEN_PRINTER_PROPS_INFOA {
    pub dwSize: u32,
    pub pszSheetName: windows_core::PSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: super::super::Foundation::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPEN_PRINTER_PROPS_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for OPEN_PRINTER_PROPS_INFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPEN_PRINTER_PROPS_INFOA {
    pub dwSize: u32,
    pub pszSheetName: windows_core::PSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: super::super::Foundation::BOOL,
}
#[cfg(target_arch = "x86")]
impl Default for OPEN_PRINTER_PROPS_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for OPEN_PRINTER_PROPS_INFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPEN_PRINTER_PROPS_INFOW {
    pub dwSize: u32,
    pub pszSheetName: windows_core::PWSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: super::super::Foundation::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPEN_PRINTER_PROPS_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for OPEN_PRINTER_PROPS_INFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPEN_PRINTER_PROPS_INFOW {
    pub dwSize: u32,
    pub pszSheetName: windows_core::PWSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: super::super::Foundation::BOOL,
}
#[cfg(target_arch = "x86")]
impl Default for OPEN_PRINTER_PROPS_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for OPEN_PRINTER_PROPS_INFOW {
    type TypeKind = windows_core::CloneType;
}
pub const OnexCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x07aa0886_cc8d_4e19_a410_1c75af686e62);
pub const OnexPlapSmartcardCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x33c86cd6_705f_4ba1_9adb_67070b837775);
pub const OpenControlPanel: windows_core::GUID = windows_core::GUID::from_u128(0x06622d85_6856_4460_8de1_a81921b41c4b);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PARSEDURLA {
    pub cbSize: u32,
    pub pszProtocol: windows_core::PCSTR,
    pub cchProtocol: u32,
    pub pszSuffix: windows_core::PCSTR,
    pub cchSuffix: u32,
    pub nScheme: u32,
}
impl Default for PARSEDURLA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PARSEDURLA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PARSEDURLW {
    pub cbSize: u32,
    pub pszProtocol: windows_core::PCWSTR,
    pub cchProtocol: u32,
    pub pszSuffix: windows_core::PCWSTR,
    pub cchSuffix: u32,
    pub nScheme: u32,
}
impl Default for PARSEDURLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PARSEDURLW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERSIST_FOLDER_TARGET_INFO {
    pub pidlTargetFolder: *mut Common::ITEMIDLIST,
    pub szTargetParsingName: [u16; 260],
    pub szNetworkProvider: [u16; 260],
    pub dwAttributes: u32,
    pub csidl: i32,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for PERSIST_FOLDER_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for PERSIST_FOLDER_TARGET_INFO {
    type TypeKind = windows_core::CopyType;
}
pub const PINLogonCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0xcb82ea12_9f71_446d_89e1_8d0924e1256e);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PREVIEWHANDLERFRAMEINFO {
    pub haccel: super::WindowsAndMessaging::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for PREVIEWHANDLERFRAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for PREVIEWHANDLERFRAMEINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILEINFOA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: windows_core::PSTR,
    pub lpProfilePath: windows_core::PSTR,
    pub lpDefaultPath: windows_core::PSTR,
    pub lpServerName: windows_core::PSTR,
    pub lpPolicyPath: windows_core::PSTR,
    pub hProfile: super::super::Foundation::HANDLE,
}
impl Default for PROFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILEINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILEINFOW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: windows_core::PWSTR,
    pub lpProfilePath: windows_core::PWSTR,
    pub lpDefaultPath: windows_core::PWSTR,
    pub lpServerName: windows_core::PWSTR,
    pub lpPolicyPath: windows_core::PWSTR,
    pub hProfile: super::super::Foundation::HANDLE,
}
impl Default for PROFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILEINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PUBAPPINFO {
    pub cbSize: u32,
    pub dwMask: u32,
    pub pszSource: windows_core::PWSTR,
    pub stAssigned: super::super::Foundation::SYSTEMTIME,
    pub stPublished: super::super::Foundation::SYSTEMTIME,
    pub stScheduled: super::super::Foundation::SYSTEMTIME,
    pub stExpire: super::super::Foundation::SYSTEMTIME,
}
impl Default for PUBAPPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PUBAPPINFO {
    type TypeKind = windows_core::CloneType;
}
pub const PackageDebugSettings: windows_core::GUID = windows_core::GUID::from_u128(0xb1aec16f_2383_4852_b0e9_8f0b1dc66b4d);
pub const PasswordCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x60b78e88_ead8_445c_9cfd_0b87f74ea6cd);
pub const PreviousVersions: windows_core::GUID = windows_core::GUID::from_u128(0x596ab062_b4d2_4215_9f74_e9109b0a8153);
pub const PropertiesUI: windows_core::GUID = windows_core::GUID::from_u128(0xd912f8cf_0396_4915_884e_fb425d32943b);
pub const PublishDropTarget: windows_core::GUID = windows_core::GUID::from_u128(0xcc6eeffb_43f6_46c5_9619_51d571967f7d);
pub const PublishingWizard: windows_core::GUID = windows_core::GUID::from_u128(0x6b33163c_76a5_4b6c_bf21_45de9cd503a1);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QCMINFO {
    pub hmenu: super::WindowsAndMessaging::HMENU,
    pub indexMenu: u32,
    pub idCmdFirst: u32,
    pub idCmdLast: u32,
    pub pIdMap: *const QCMINFO_IDMAP,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for QCMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for QCMINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QCMINFO_IDMAP {
    pub nMaxIds: u32,
    pub pIdList: [QCMINFO_IDMAP_PLACEMENT; 1],
}
impl Default for QCMINFO_IDMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for QCMINFO_IDMAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QCMINFO_IDMAP_PLACEMENT {
    pub id: u32,
    pub fFlags: u32,
}
impl Default for QCMINFO_IDMAP_PLACEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for QCMINFO_IDMAP_PLACEMENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QITAB {
    pub piid: *const windows_core::GUID,
    pub dwOffset: u32,
}
impl Default for QITAB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for QITAB {
    type TypeKind = windows_core::CopyType;
}
pub const QueryCancelAutoPlay: windows_core::GUID = windows_core::GUID::from_u128(0x331f1768_05a9_4ddd_b86e_dae34ddc998a);
pub const RASProvider: windows_core::GUID = windows_core::GUID::from_u128(0x5537e283_b1e7_4ef8_9c6e_7ab0afe5056d);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SFVM_HELPTOPIC_DATA {
    pub wszHelpFile: [u16; 260],
    pub wszHelpTopic: [u16; 260],
}
impl Default for SFVM_HELPTOPIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SFVM_HELPTOPIC_DATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SFVM_PROPPAGE_DATA {
    pub dwReserved: u32,
    pub pfn: super::Controls::LPFNSVADDPROPSHEETPAGE,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_UI_Controls")]
impl Default for SFVM_PROPPAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Controls")]
impl windows_core::TypeKind for SFVM_PROPPAGE_DATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SFV_CREATE {
    pub cbSize: u32,
    pub pshf: Option<IShellFolder>,
    pub psvOuter: Option<IShellView>,
    pub psfvcb: Option<IShellFolderViewCB>,
}
impl Default for SFV_CREATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SFV_CREATE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SFV_SETITEMPOS {
    pub pidl: *mut Common::ITEMIDLIST,
    pub pt: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for SFV_SETITEMPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for SFV_SETITEMPOS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHARDAPPIDINFO {
    pub psi: Option<IShellItem>,
    pub pszAppID: windows_core::PCWSTR,
}
impl Default for SHARDAPPIDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHARDAPPIDINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHARDAPPIDINFOIDLIST {
    pub pidl: *mut Common::ITEMIDLIST,
    pub pszAppID: windows_core::PCWSTR,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for SHARDAPPIDINFOIDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for SHARDAPPIDINFOIDLIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHARDAPPIDINFOLINK {
    pub psl: Option<IShellLinkA>,
    pub pszAppID: windows_core::PCWSTR,
}
impl Default for SHARDAPPIDINFOLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHARDAPPIDINFOLINK {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHCOLUMNDATA {
    pub dwFlags: u32,
    pub dwFileAttributes: u32,
    pub dwReserved: u32,
    pub pwszExt: windows_core::PWSTR,
    pub wszFile: [u16; 260],
}
impl Default for SHCOLUMNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHCOLUMNDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHCOLUMNINFO {
    pub scid: PropertiesSystem::PROPERTYKEY,
    pub vt: super::super::System::Variant::VARENUM,
    pub fmt: u32,
    pub cChars: u32,
    pub csFlags: u32,
    pub wszTitle: [u16; 80],
    pub wszDescription: [u16; 128],
}
#[cfg(all(feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl Default for SHCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::TypeKind for SHCOLUMNINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHCOLUMNINIT {
    pub dwFlags: u32,
    pub dwReserved: u32,
    pub wszFolder: [u16; 260],
}
impl Default for SHCOLUMNINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHCOLUMNINIT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Threading"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHCREATEPROCESSINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub pszFile: windows_core::PCWSTR,
    pub pszParameters: windows_core::PCWSTR,
    pub pszCurrentDirectory: windows_core::PCWSTR,
    pub hUserToken: super::super::Foundation::HANDLE,
    pub lpProcessAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub lpThreadAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub bInheritHandles: super::super::Foundation::BOOL,
    pub dwCreationFlags: u32,
    pub lpStartupInfo: *mut super::super::System::Threading::STARTUPINFOW,
    pub lpProcessInformation: *mut super::super::System::Threading::PROCESS_INFORMATION,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Threading"))]
impl Default for SHCREATEPROCESSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Threading"))]
impl windows_core::TypeKind for SHCREATEPROCESSINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Threading"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHCREATEPROCESSINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub pszFile: windows_core::PCWSTR,
    pub pszParameters: windows_core::PCWSTR,
    pub pszCurrentDirectory: windows_core::PCWSTR,
    pub hUserToken: super::super::Foundation::HANDLE,
    pub lpProcessAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub lpThreadAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub bInheritHandles: super::super::Foundation::BOOL,
    pub dwCreationFlags: u32,
    pub lpStartupInfo: *mut super::super::System::Threading::STARTUPINFOW,
    pub lpProcessInformation: *mut super::super::System::Threading::PROCESS_INFORMATION,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Threading"))]
impl Default for SHCREATEPROCESSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Threading"))]
impl windows_core::TypeKind for SHCREATEPROCESSINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHChangeDWORDAsIDList {
    pub cb: u16,
    pub dwItem1: u32,
    pub dwItem2: u32,
    pub cbZero: u16,
}
impl Default for SHChangeDWORDAsIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHChangeDWORDAsIDList {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHChangeNotifyEntry {
    pub pidl: *mut Common::ITEMIDLIST,
    pub fRecursive: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for SHChangeNotifyEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for SHChangeNotifyEntry {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHChangeProductKeyAsIDList {
    pub cb: u16,
    pub wszProductKey: [u16; 39],
    pub cbZero: u16,
}
impl Default for SHChangeProductKeyAsIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHChangeProductKeyAsIDList {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHChangeUpdateImageIDList {
    pub cb: u16,
    pub iIconIndex: i32,
    pub iCurIndex: i32,
    pub uFlags: u32,
    pub dwProcessID: u32,
    pub szName: [u16; 260],
    pub cbZero: u16,
}
impl Default for SHChangeUpdateImageIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHChangeUpdateImageIDList {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHDESCRIPTIONID {
    pub dwDescriptionId: u32,
    pub clsid: windows_core::GUID,
}
impl Default for SHDESCRIPTIONID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHDESCRIPTIONID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHDRAGIMAGE {
    pub sizeDragImage: super::super::Foundation::SIZE,
    pub ptOffset: super::super::Foundation::POINT,
    pub hbmpDragImage: super::super::Graphics::Gdi::HBITMAP,
    pub crColorKey: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for SHDRAGIMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for SHDRAGIMAGE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELLEXECUTEINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub lpVerb: windows_core::PCSTR,
    pub lpFile: windows_core::PCSTR,
    pub lpParameters: windows_core::PCSTR,
    pub lpDirectory: windows_core::PCSTR,
    pub nShow: i32,
    pub hInstApp: super::super::Foundation::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_core::PCSTR,
    pub hkeyClass: super::super::System::Registry::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOA_0,
    pub hProcess: super::super::Foundation::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union SHELLEXECUTEINFOA_0 {
    pub hIcon: super::super::Foundation::HANDLE,
    pub hMonitor: super::super::Foundation::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOA_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELLEXECUTEINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub lpVerb: windows_core::PCSTR,
    pub lpFile: windows_core::PCSTR,
    pub lpParameters: windows_core::PCSTR,
    pub lpDirectory: windows_core::PCSTR,
    pub nShow: i32,
    pub hInstApp: super::super::Foundation::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_core::PCSTR,
    pub hkeyClass: super::super::System::Registry::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOA_0,
    pub hProcess: super::super::Foundation::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union SHELLEXECUTEINFOA_0 {
    pub hIcon: super::super::Foundation::HANDLE,
    pub hMonitor: super::super::Foundation::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOA_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELLEXECUTEINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub lpVerb: windows_core::PCWSTR,
    pub lpFile: windows_core::PCWSTR,
    pub lpParameters: windows_core::PCWSTR,
    pub lpDirectory: windows_core::PCWSTR,
    pub nShow: i32,
    pub hInstApp: super::super::Foundation::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_core::PCWSTR,
    pub hkeyClass: super::super::System::Registry::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOW_0,
    pub hProcess: super::super::Foundation::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union SHELLEXECUTEINFOW_0 {
    pub hIcon: super::super::Foundation::HANDLE,
    pub hMonitor: super::super::Foundation::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOW_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELLEXECUTEINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub lpVerb: windows_core::PCWSTR,
    pub lpFile: windows_core::PCWSTR,
    pub lpParameters: windows_core::PCWSTR,
    pub lpDirectory: windows_core::PCWSTR,
    pub nShow: i32,
    pub hInstApp: super::super::Foundation::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_core::PCWSTR,
    pub hkeyClass: super::super::System::Registry::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOW_0,
    pub hProcess: super::super::Foundation::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union SHELLEXECUTEINFOW_0 {
    pub hIcon: super::super::Foundation::HANDLE,
    pub hMonitor: super::super::Foundation::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl Default for SHELLEXECUTEINFOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for SHELLEXECUTEINFOW_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELLFLAGSTATE {
    pub _bitfield: i32,
}
impl Default for SHELLFLAGSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHELLFLAGSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELLSTATEA {
    pub _bitfield1: i32,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: i32,
}
impl Default for SHELLSTATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHELLSTATEA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELLSTATEW {
    pub _bitfield1: i32,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: i32,
}
impl Default for SHELLSTATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHELLSTATEW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHELL_ITEM_RESOURCE {
    pub guidType: windows_core::GUID,
    pub szName: [u16; 260],
}
impl Default for SHELL_ITEM_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHELL_ITEM_RESOURCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEINFOA {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [i8; 260],
    pub szTypeName: [i8; 80],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SHFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SHFILEINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEINFOA {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [i8; 260],
    pub szTypeName: [i8; 80],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SHFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SHFILEINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEINFOW {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [u16; 260],
    pub szTypeName: [u16; 80],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SHFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SHFILEINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEINFOW {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [u16; 260],
    pub szTypeName: [u16; 80],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SHFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SHFILEINFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEOPSTRUCTA {
    pub hwnd: super::super::Foundation::HWND,
    pub wFunc: u32,
    pub pFrom: *mut i8,
    pub pTo: *mut i8,
    pub fFlags: u16,
    pub fAnyOperationsAborted: super::super::Foundation::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SHFILEOPSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SHFILEOPSTRUCTA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEOPSTRUCTA {
    pub hwnd: super::super::Foundation::HWND,
    pub wFunc: u32,
    pub pFrom: *mut i8,
    pub pTo: *mut i8,
    pub fFlags: u16,
    pub fAnyOperationsAborted: super::super::Foundation::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for SHFILEOPSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SHFILEOPSTRUCTA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEOPSTRUCTW {
    pub hwnd: super::super::Foundation::HWND,
    pub wFunc: u32,
    pub pFrom: windows_core::PCWSTR,
    pub pTo: windows_core::PCWSTR,
    pub fFlags: u16,
    pub fAnyOperationsAborted: super::super::Foundation::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SHFILEOPSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SHFILEOPSTRUCTW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFILEOPSTRUCTW {
    pub hwnd: super::super::Foundation::HWND,
    pub wFunc: u32,
    pub pFrom: windows_core::PCWSTR,
    pub pTo: windows_core::PCWSTR,
    pub fFlags: u16,
    pub fAnyOperationsAborted: super::super::Foundation::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for SHFILEOPSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SHFILEOPSTRUCTW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFOLDERCUSTOMSETTINGS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub pvid: *mut windows_core::GUID,
    pub pszWebViewTemplate: windows_core::PWSTR,
    pub cchWebViewTemplate: u32,
    pub pszWebViewTemplateVersion: windows_core::PWSTR,
    pub pszInfoTip: windows_core::PWSTR,
    pub cchInfoTip: u32,
    pub pclsid: *mut windows_core::GUID,
    pub dwFlags: u32,
    pub pszIconFile: windows_core::PWSTR,
    pub cchIconFile: u32,
    pub iIconIndex: i32,
    pub pszLogo: windows_core::PWSTR,
    pub cchLogo: u32,
}
impl Default for SHFOLDERCUSTOMSETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHFOLDERCUSTOMSETTINGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHNAMEMAPPINGA {
    pub pszOldPath: windows_core::PSTR,
    pub pszNewPath: windows_core::PSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SHNAMEMAPPINGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SHNAMEMAPPINGA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHNAMEMAPPINGA {
    pub pszOldPath: windows_core::PSTR,
    pub pszNewPath: windows_core::PSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SHNAMEMAPPINGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SHNAMEMAPPINGA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHNAMEMAPPINGW {
    pub pszOldPath: windows_core::PWSTR,
    pub pszNewPath: windows_core::PWSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SHNAMEMAPPINGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SHNAMEMAPPINGW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHNAMEMAPPINGW {
    pub pszOldPath: windows_core::PWSTR,
    pub pszNewPath: windows_core::PWSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SHNAMEMAPPINGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SHNAMEMAPPINGW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHQUERYRBINFO {
    pub cbSize: u32,
    pub i64Size: i64,
    pub i64NumItems: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SHQUERYRBINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SHQUERYRBINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHQUERYRBINFO {
    pub cbSize: u32,
    pub i64Size: i64,
    pub i64NumItems: i64,
}
#[cfg(target_arch = "x86")]
impl Default for SHQUERYRBINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SHQUERYRBINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHSTOCKICONINFO {
    pub cbSize: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub iSysImageIndex: i32,
    pub iIcon: i32,
    pub szPath: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SHSTOCKICONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SHSTOCKICONINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHSTOCKICONINFO {
    pub cbSize: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub iSysImageIndex: i32,
    pub iIcon: i32,
    pub szPath: [u16; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SHSTOCKICONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SHSTOCKICONINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SLOWAPPINFO {
    pub ullSize: u64,
    pub ftLastUsed: super::super::Foundation::FILETIME,
    pub iTimesUsed: i32,
    pub pszImage: windows_core::PWSTR,
}
impl Default for SLOWAPPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SLOWAPPINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SMCSHCHANGENOTIFYSTRUCT {
    pub lEvent: i32,
    pub pidl1: *mut Common::ITEMIDLIST,
    pub pidl2: *mut Common::ITEMIDLIST,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for SMCSHCHANGENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for SMCSHCHANGENOTIFYSTRUCT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SMDATA {
    pub dwMask: u32,
    pub dwFlags: u32,
    pub hmenu: super::WindowsAndMessaging::HMENU,
    pub hwnd: super::super::Foundation::HWND,
    pub uId: u32,
    pub uIdParent: u32,
    pub uIdAncestor: u32,
    pub punk: Option<windows_core::IUnknown>,
    pub pidlFolder: *mut Common::ITEMIDLIST,
    pub pidlItem: *mut Common::ITEMIDLIST,
    pub psf: Option<IShellFolder>,
    pub pvUserData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for SMDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for SMDATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SMINFO {
    pub dwMask: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub iIcon: i32,
}
impl Default for SMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SMINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SORTCOLUMN {
    pub propkey: PropertiesSystem::PROPERTYKEY,
    pub direction: SORTDIRECTION,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for SORTCOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for SORTCOLUMN {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SV2CVW2_PARAMS {
    pub cbSize: u32,
    pub psvPrev: Option<IShellView>,
    pub pfs: *mut FOLDERSETTINGS,
    pub psbOwner: Option<IShellBrowser>,
    pub prcView: *mut super::super::Foundation::RECT,
    pub pvid: *const windows_core::GUID,
    pub hwndView: super::super::Foundation::HWND,
}
impl Default for SV2CVW2_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SV2CVW2_PARAMS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYNCMGRHANDLERINFO {
    pub cbSize: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub SyncMgrHandlerFlags: u32,
    pub wszHandlerName: [u16; 32],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SYNCMGRHANDLERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SYNCMGRHANDLERINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYNCMGRITEM {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub ItemID: windows_core::GUID,
    pub dwItemState: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub wszItemName: [u16; 128],
    pub ftLastUpdate: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for SYNCMGRITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for SYNCMGRITEM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYNCMGRLOGERRORINFO {
    pub cbSize: u32,
    pub mask: u32,
    pub dwSyncMgrErrorFlags: u32,
    pub ErrorID: windows_core::GUID,
    pub ItemID: windows_core::GUID,
}
impl Default for SYNCMGRLOGERRORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SYNCMGRLOGERRORINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYNCMGRPROGRESSITEM {
    pub cbSize: u32,
    pub mask: u32,
    pub lpcStatusText: windows_core::PCWSTR,
    pub dwStatusType: u32,
    pub iProgValue: i32,
    pub iMaxValue: i32,
}
impl Default for SYNCMGRPROGRESSITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SYNCMGRPROGRESSITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYNCMGR_CONFLICT_ID_INFO {
    pub pblobID: *mut super::super::System::Com::BYTE_BLOB,
    pub pblobExtra: *mut super::super::System::Com::BYTE_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SYNCMGR_CONFLICT_ID_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SYNCMGR_CONFLICT_ID_INFO {
    type TypeKind = windows_core::CopyType;
}
pub const ScheduledTasks: windows_core::GUID = windows_core::GUID::from_u128(0xd6277990_4c6a_11cf_8d87_00aa0060f5bf);
pub const SearchFolderItemFactory: windows_core::GUID = windows_core::GUID::from_u128(0x14010e02_bbbd_41f0_88e3_eda371216584);
pub const SharedBitmap: windows_core::GUID = windows_core::GUID::from_u128(0x4db26476_6787_4046_b836_e8412a9e8a27);
pub const SharingConfigurationManager: windows_core::GUID = windows_core::GUID::from_u128(0x49f371e1_8c5c_4d9c_9a3b_54a6827f513c);
pub const Shell: windows_core::GUID = windows_core::GUID::from_u128(0x13709620_c279_11ce_a49e_444553540000);
pub const ShellBrowserWindow: windows_core::GUID = windows_core::GUID::from_u128(0xc08afd90_f2a1_11d1_8455_00a0c91f3880);
pub const ShellDesktop: windows_core::GUID = windows_core::GUID::from_u128(0x00021400_0000_0000_c000_000000000046);
pub const ShellDispatchInproc: windows_core::GUID = windows_core::GUID::from_u128(0x0a89a860_d7b1_11ce_8350_444553540000);
pub const ShellFSFolder: windows_core::GUID = windows_core::GUID::from_u128(0xf3364ba0_65b9_11ce_a9ba_00aa004ae837);
pub const ShellFolderItem: windows_core::GUID = windows_core::GUID::from_u128(0x2fe352ea_fd1f_11d2_b1f4_00c04f8eeb3e);
pub const ShellFolderView: windows_core::GUID = windows_core::GUID::from_u128(0x62112aa1_ebe4_11cf_a5fb_0020afe7292d);
pub const ShellFolderViewOC: windows_core::GUID = windows_core::GUID::from_u128(0x9ba05971_f6a8_11cf_a442_00a0c90a8f39);
pub const ShellImageDataFactory: windows_core::GUID = windows_core::GUID::from_u128(0x66e4e4fb_f385_4dd0_8d74_a2efd1bc6178);
pub const ShellItem: windows_core::GUID = windows_core::GUID::from_u128(0x9ac9fbe1_e0a2_4ad6_b4ee_e212013ea917);
pub const ShellLibrary: windows_core::GUID = windows_core::GUID::from_u128(0xd9b3211d_e57f_4426_aaef_30a806add397);
pub const ShellLink: windows_core::GUID = windows_core::GUID::from_u128(0x00021401_0000_0000_c000_000000000046);
pub const ShellLinkObject: windows_core::GUID = windows_core::GUID::from_u128(0x11219420_1768_11d1_95be_00609797ea4f);
pub const ShellNameSpace: windows_core::GUID = windows_core::GUID::from_u128(0x55136805_b2de_11d1_b9f2_00a0c98bc547);
pub const ShellUIHelper: windows_core::GUID = windows_core::GUID::from_u128(0x64ab4bb7_111e_11d1_8f79_00c04fc2fbe1);
pub const ShellWindows: windows_core::GUID = windows_core::GUID::from_u128(0x9ba05972_f6a8_11cf_a442_00a0c90a8f39);
pub const ShowInputPaneAnimationCoordinator: windows_core::GUID = windows_core::GUID::from_u128(0x1f046abf_3202_4dc1_8cb5_3c67617ce1fa);
pub const SimpleConflictPresenter: windows_core::GUID = windows_core::GUID::from_u128(0x7a0f6ab7_ed84_46b6_b47e_02aa159a152b);
pub const SizeCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x55d7b852_f6d1_42f2_aa75_8728a1b2d264);
pub const SmartcardCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x8fd7e19c_3bf7_489b_a72c_846ab3678c96);
pub const SmartcardPinProvider: windows_core::GUID = windows_core::GUID::from_u128(0x94596c7e_3744_41ce_893e_bbf09122f76a);
pub const SmartcardReaderSelectionProvider: windows_core::GUID = windows_core::GUID::from_u128(0x1b283861_754f_4022_ad47_a5eaaa618894);
pub const SmartcardWinRTProvider: windows_core::GUID = windows_core::GUID::from_u128(0x1ee7337f_85ac_45e2_a23c_37c753209769);
pub const StartMenuPin: windows_core::GUID = windows_core::GUID::from_u128(0xa2a9545d_a0c2_42b4_9708_a0b2badd77c8);
pub const StorageProviderBanners: windows_core::GUID = windows_core::GUID::from_u128(0x7ccdf9f4_e576_455a_8bc7_f6ec68d6f063);
pub const SuspensionDependencyManager: windows_core::GUID = windows_core::GUID::from_u128(0x6b273fc5_61fd_4918_95a2_c3b5e9d7f581);
pub const SyncMgr: windows_core::GUID = windows_core::GUID::from_u128(0x6295df27_35ee_11d1_8707_00c04fd93327);
pub const SyncMgrClient: windows_core::GUID = windows_core::GUID::from_u128(0x1202db60_1dac_42c5_aed5_1abdd432248e);
pub const SyncMgrControl: windows_core::GUID = windows_core::GUID::from_u128(0x1a1f4206_0688_4e7f_be03_d82ec69df9a5);
pub const SyncMgrFolder: windows_core::GUID = windows_core::GUID::from_u128(0x9c73f5e5_7ae7_4e32_a8e8_8d23b85255bf);
pub const SyncMgrScheduleWizard: windows_core::GUID = windows_core::GUID::from_u128(0x8d8b8e30_c451_421b_8553_d2976afa648c);
pub const SyncResultsFolder: windows_core::GUID = windows_core::GUID::from_u128(0x71d99464_3b6b_475c_b241_e15883207529);
pub const SyncSetupFolder: windows_core::GUID = windows_core::GUID::from_u128(0x2e9e59c0_b437_4981_a647_9c34b9b90891);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TBINFO {
    pub cbuttons: u32,
    pub uFlags: u32,
}
impl Default for TBINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TBINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct THUMBBUTTON {
    pub dwMask: THUMBBUTTONMASK,
    pub iId: u32,
    pub iBitmap: u32,
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub szTip: [u16; 260],
    pub dwFlags: THUMBBUTTONFLAGS,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for THUMBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for THUMBBUTTON {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOOLBARITEM {
    pub ptbar: Option<IDockingWindow>,
    pub rcBorderTool: super::super::Foundation::RECT,
    pub pwszItem: windows_core::PWSTR,
    pub fShow: super::super::Foundation::BOOL,
    pub hMon: super::super::Graphics::Gdi::HMONITOR,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for TOOLBARITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for TOOLBARITEM {
    type TypeKind = windows_core::CloneType;
}
pub const TaskbarList: windows_core::GUID = windows_core::GUID::from_u128(0x56fdf344_fd6d_11d0_958a_006097c9a090);
pub const ThumbnailStreamCache: windows_core::GUID = windows_core::GUID::from_u128(0xcbe0fed3_4b91_4e90_8354_8a8c84ec6872);
pub const TimeCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x3bb4118f_ddfd_4d30_a348_9fb5d6bf1afe);
pub const TrackShellMenu: windows_core::GUID = windows_core::GUID::from_u128(0x8278f931_2a3e_11d2_838f_00c04fd918d0);
pub const TrayBandSiteService: windows_core::GUID = windows_core::GUID::from_u128(0xf60ad0a0_e5e1_45cb_b51a_e15b9f8b2934);
pub const TrayDeskBand: windows_core::GUID = windows_core::GUID::from_u128(0xe6442437_6c68_4f52_94dd_2cfed267efb9);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct URLINVOKECOMMANDINFOA {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pcszVerb: windows_core::PCSTR,
}
impl Default for URLINVOKECOMMANDINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for URLINVOKECOMMANDINFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct URLINVOKECOMMANDINFOW {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pcszVerb: windows_core::PCWSTR,
}
impl Default for URLINVOKECOMMANDINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for URLINVOKECOMMANDINFOW {
    type TypeKind = windows_core::CloneType;
}
pub const UserNotification: windows_core::GUID = windows_core::GUID::from_u128(0x0010890e_8789_413c_adbc_48f5b511b3af);
pub const V1PasswordCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x6f45dc1e_5384_457a_bc13_2cd81b0d28ed);
pub const V1SmartcardCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x8bf9a910_a8ff_457f_999f_a5ca10b4a885);
pub const V1WinBioCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0xac3ac249_e820_4343_a65b_377ac634dc09);
pub const VaultProvider: windows_core::GUID = windows_core::GUID::from_u128(0x503739d0_4c5e_4cfd_b3ba_d881334f0df2);
pub const VirtualDesktopManager: windows_core::GUID = windows_core::GUID::from_u128(0xaa509086_5ca9_4c25_8f95_589d3c07b48a);
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINDOWDATA {
    pub dwWindowID: u32,
    pub uiCP: u32,
    pub pidl: *mut Common::ITEMIDLIST,
    pub lpszUrl: windows_core::PWSTR,
    pub lpszUrlLocation: windows_core::PWSTR,
    pub lpszTitle: windows_core::PWSTR,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl Default for WINDOWDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::TypeKind for WINDOWDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTS_THUMBNAILID {
    pub rgbKey: [u8; 16],
}
impl Default for WTS_THUMBNAILID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WTS_THUMBNAILID {
    type TypeKind = windows_core::CopyType;
}
pub const WebBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x8856f961_340a_11d0_a96b_00c04fd705a2);
pub const WebBrowser_V1: windows_core::GUID = windows_core::GUID::from_u128(0xeab22ac3_30c1_11cf_a7eb_0000c05bae0b);
pub const WebWizardHost: windows_core::GUID = windows_core::GUID::from_u128(0xc827f149_55c1_4d28_935e_57e47caed973);
pub const WinBioCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0xbec09223_b018_416d_a0ac_523971b639f5);
pub type APPLET_PROC = Option<unsafe extern "system" fn(hwndcpl: super::super::Foundation::HWND, msg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> i32>;
pub type BFFCALLBACK = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, lparam: super::super::Foundation::LPARAM, lpdata: super::super::Foundation::LPARAM) -> i32>;
pub type DLLGETVERSIONPROC = Option<unsafe extern "system" fn(param0: *mut DLLVERSIONINFO) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_Com")]
pub type LPFNDFMCALLBACK = Option<unsafe extern "system" fn(psf: Option<IShellFolder>, hwnd: super::super::Foundation::HWND, pdtobj: Option<super::super::System::Com::IDataObject>, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT>;
pub type LPFNVIEWCALLBACK = Option<unsafe extern "system" fn(psvouter: Option<IShellView>, psf: Option<IShellFolder>, hwndmain: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT>;
pub type PAPPCONSTRAIN_CHANGE_ROUTINE = Option<unsafe extern "system" fn(constrained: super::super::Foundation::BOOLEAN, context: *const core::ffi::c_void)>;
pub type PAPPSTATE_CHANGE_ROUTINE = Option<unsafe extern "system" fn(quiesced: super::super::Foundation::BOOLEAN, context: *const core::ffi::c_void)>;
pub type PFNCANSHAREFOLDERW = Option<unsafe extern "system" fn(pszpath: windows_core::PCWSTR) -> windows_core::HRESULT>;
pub type PFNSHOWSHAREFOLDERUIW = Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT>;
pub type SUBCLASSPROC = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, uidsubclass: usize, dwrefdata: usize) -> super::super::Foundation::LRESULT>;
