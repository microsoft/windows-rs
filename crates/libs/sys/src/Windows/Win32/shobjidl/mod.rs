#[cfg(feature = "Win32_propsys")]
windows_link::link!("shell32.dll" "system" fn SHAddDefaultPropertiesByExt(pszext : windows_sys::core::PCWSTR, ppropstore : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_shobjidl_core")]
windows_link::link!("shell32.dll" "system" fn SHCreateDefaultPropertiesOp(psi : *mut core::ffi::c_void, ppfileop : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_shobjidl_core", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn SHSetDefaultProperties(hwnd : super::windef::HWND, psi : *mut core::ffi::c_void, dwfileopflags : u32, pfops : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const ACDD_VISIBLE: u32 = 1;
pub const AccessibilityDockingService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x29ce1d46_b481_4aa0_a08a_d3ebc8aca402);
pub const AlphabeticalCategorizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3c2654c6_7372_4f6b_b310_55d6128f49d2);
pub const ApplicationAssociationRegistrationUI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1968106d_f3b5_44cf_890e_116fcb9ecef1);
pub const AttachmentServices: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4125dd96_e03a_4103_8f70_e0597d803b9c);
pub type CDBE_ACTIONS = u32;
pub const CDBE_RET_DEFAULT: tagCDBURNINGEXTENSIONRET = 0;
pub const CDBE_RET_DONTRUNOTHEREXTS: tagCDBURNINGEXTENSIONRET = 1;
pub const CDBE_RET_STOPWIZARD: tagCDBURNINGEXTENSIONRET = 2;
pub const CDBE_TYPE_ALL: _CDBE_ACTIONS = -1;
pub const CDBE_TYPE_DATA: _CDBE_ACTIONS = 2;
pub const CDBE_TYPE_MUSIC: _CDBE_ACTIONS = 1;
pub const CDBurn: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfbeb8a05_beee_4442_804e_409d6c4515e9);
pub const DSH_ALLOWDROPDESCRIPTIONTEXT: DSH_FLAGS = 1;
pub type DSH_FLAGS = u32;
pub const DesktopGadget: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x924ccc1b_6562_4c85_8657_d177925222b6);
pub const DocPropShellExtension: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x883373c3_bf89_11d1_be35_080036b11a03);
pub const ExecuteFolder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x11dbb47c_a525_400b_9e80_a54615a090c0);
pub const ExplorerBrowser: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x71f96385_ddd6_48d3_a0c1_ae06e8b055fb);
pub type FOLDERVIEWOPTIONS = u32;
pub const FSCopyHandler: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd197380a_0a79_4dc8_a033_ed882c2fa14b);
pub const FVO_CUSTOMORDERING: FOLDERVIEWOPTIONS = 4;
pub const FVO_CUSTOMPOSITION: FOLDERVIEWOPTIONS = 2;
pub const FVO_DEFAULT: FOLDERVIEWOPTIONS = 0;
pub const FVO_NOANIMATIONS: FOLDERVIEWOPTIONS = 16;
pub const FVO_NOSCROLLTIPS: FOLDERVIEWOPTIONS = 32;
pub const FVO_SUPPORTHYPERLINKS: FOLDERVIEWOPTIONS = 8;
pub const FVO_VISTALAYOUT: FOLDERVIEWOPTIONS = 1;
pub const FolderViewHost: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x20b1cb23_6968_4eb9_b7d4_a66d00d07cee);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IControlMarkup(pub u8);
pub const IDD_WIZEXTN_FIRST: u32 = 20480;
pub const IDD_WIZEXTN_LAST: u32 = 20736;
pub const IENamespaceTreeControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xace52d03_e5cd_4b20_82ff_e71b11beae1d);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMarkupCallback(pub u8);
pub const ImageProperties: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7ab770c7_0e23_4d7a_8aa2_19bfad479829);
pub const ImageRecompress: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6e33091c_d2f8_4740_b55e_2e11d1477a2c);
pub const InternetPrintOrdering: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xadd36aa8_751a_4579_a266_d66f5202ccbb);
pub type LPVIEWSETTINGS = *mut i8;
pub const MergedCategorizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8e827c11_33e7_4bc1_b242_8cd9a1c2b304);
#[repr(C)]
#[cfg(all(feature = "Win32_commctrl", feature = "Win32_shobjidl_core"))]
#[derive(Clone, Copy)]
pub struct NSTCCUSTOMDRAW {
    pub psi: *mut core::ffi::c_void,
    pub uItemState: u32,
    pub nstcis: super::shobjidl_core::NSTCITEMSTATE,
    pub pszText: windows_sys::core::PCWSTR,
    pub iImage: i32,
    pub himl: super::commctrl::HIMAGELIST,
    pub iLevel: i32,
    pub iIndent: i32,
}
#[cfg(all(feature = "Win32_commctrl", feature = "Win32_shobjidl_core"))]
impl Default for NSTCCUSTOMDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NSTCDHPOS_ONTOP: i32 = -1;
pub type NSTCECLICKTYPE = u32;
pub const NSTCECT_BUTTON: _NSTCECLICKTYPE = 3;
pub const NSTCECT_DBLCLICK: _NSTCECLICKTYPE = 4;
pub const NSTCECT_LBUTTON: _NSTCECLICKTYPE = 1;
pub const NSTCECT_MBUTTON: _NSTCECLICKTYPE = 2;
pub const NSTCECT_RBUTTON: _NSTCECLICKTYPE = 3;
pub type NSTCEHITTEST = u32;
pub const NSTCEHT_NOWHERE: _NSTCEHITTEST = 1;
pub const NSTCEHT_ONITEM: _NSTCEHITTEST = 70;
pub const NSTCEHT_ONITEMBUTTON: _NSTCEHITTEST = 16;
pub const NSTCEHT_ONITEMICON: _NSTCEHITTEST = 2;
pub const NSTCEHT_ONITEMINDENT: _NSTCEHITTEST = 8;
pub const NSTCEHT_ONITEMLABEL: _NSTCEHITTEST = 4;
pub const NSTCEHT_ONITEMRIGHT: _NSTCEHITTEST = 32;
pub const NSTCEHT_ONITEMSTATEICON: _NSTCEHITTEST = 64;
pub const NSTCEHT_ONITEMTABBUTTON: _NSTCEHITTEST = 4096;
pub const NSTCS2_ALLMASK: u32 = 7;
pub const NSTCS2_DEFAULT: NSTCSTYLE2 = 0;
pub const NSTCS2_DISPLAYPADDING: NSTCSTYLE2 = 4;
pub const NSTCS2_DISPLAYPINNEDONLY: NSTCSTYLE2 = 8;
pub const NSTCS2_INTERRUPTNOTIFICATIONS: NSTCSTYLE2 = 1;
pub const NSTCS2_SHOWNULLSPACEMENU: NSTCSTYLE2 = 2;
pub type NSTCSTYLE2 = u32;
pub const NTSCS2_NEVERINSERTNONENUMERATED: NSTCSTYLE2 = 32;
pub const NTSCS2_NOSINGLETONAUTOEXPAND: NSTCSTYLE2 = 16;
pub const NamespaceTreeControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae054212_3535_4430_83ed_d501aa6680e6);
pub const PROPSTR_EXTENSIONCOMPLETIONSTATE: windows_sys::core::PCWSTR = windows_sys::core::w!("ExtensionCompletionState");
pub const PreviousVersions: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x596ab062_b4d2_4215_9f74_e9109b0a8153);
pub const PublishDropTarget: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc6eeffb_43f6_46c5_9619_51d571967f7d);
pub const PublishingWizard: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6b33163c_76a5_4b6c_bf21_45de9cd503a1);
pub const QueryCancelAutoPlay: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x331f1768_05a9_4ddd_b86e_dae34ddc998a);
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = 511;
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = 4;
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = 0;
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = 32;
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = 16;
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = 8;
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = 64;
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = 256;
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = 2;
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = 1;
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = 128;
pub const SHPWHF_ANYLOCATION: u32 = 256;
pub const SHPWHF_NOFILESELECTOR: u32 = 4;
pub const SHPWHF_NONETPLACECREATE: u32 = 2;
pub const SHPWHF_NORECOMPRESS: u32 = 1;
pub const SHPWHF_USEMRU: u32 = 8;
pub const SHPWHF_VALIDATEVIAWEBFOLDERS: u32 = 65536;
pub const SID_SCommandBarState: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb99eaa5c_3850_4400_bc33_2ce534048bf8);
pub const SV3CVW3_DEFAULT: _SV3CVW3_FLAGS = 0;
pub type SV3CVW3_FLAGS = u32;
pub const SV3CVW3_FORCEFOLDERFLAGS: _SV3CVW3_FLAGS = 4;
pub const SV3CVW3_FORCEVIEWMODE: _SV3CVW3_FLAGS = 2;
pub const SV3CVW3_NONINTERACTIVE: _SV3CVW3_FLAGS = 1;
pub type SYNC_ENGINE_STATE_FLAGS = u32;
pub const StartMenuPin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa2a9545d_a0c2_42b4_9708_a0b2badd77c8);
pub const StorageProviderBanners: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7ccdf9f4_e576_455a_8bc7_f6ec68d6f063);
pub const TimeCategorizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3bb4118f_ddfd_4d30_a348_9fb5d6bf1afe);
pub const TrayBandSiteService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf60ad0a0_e5e1_45cb_b51a_e15b9f8b2934);
pub const TrayDeskBand: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6442437_6c68_4f52_94dd_2cfed267efb9);
pub type UNDOCK_REASON = i32;
pub const UR_MONITOR_DISCONNECT: UNDOCK_REASON = 1;
pub const UR_RESOLUTION_CHANGE: UNDOCK_REASON = 0;
pub const VPCF_BACKGROUND: VPCOLORFLAGS = 2;
pub const VPCF_SORTCOLUMN: VPCOLORFLAGS = 3;
pub const VPCF_SUBTEXT: VPCOLORFLAGS = 4;
pub const VPCF_TEXT: VPCOLORFLAGS = 1;
pub const VPCF_TEXTBACKGROUND: VPCOLORFLAGS = 5;
pub type VPCOLORFLAGS = i32;
pub type VPWATERMARKFLAGS = u32;
pub const VPWF_ALPHABLEND: VPWATERMARKFLAGS = 1;
pub const VPWF_DEFAULT: VPWATERMARKFLAGS = 0;
pub const VirtualDesktopManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa509086_5ca9_4c25_8f95_589d3c07b48a);
pub const WebWizardHost: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc827f149_55c1_4d28_935e_57e47caed973);
pub type _CDBE_ACTIONS = i32;
pub type _NSTCECLICKTYPE = i32;
pub type _NSTCEHITTEST = i32;
pub type _SV3CVW3_FLAGS = i32;
pub type tagCDBURNINGEXTENSIONRET = i32;
