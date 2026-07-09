pub const TF_DTLBI_USEPROFILEICON: u32 = 1;
pub const TF_FLOATINGLANGBAR_WNDTITLEA: windows_sys::core::PCSTR = windows_sys::core::s!("TF_FloatingLangBar_WndTitle");
pub const TF_FLOATINGLANGBAR_WNDTITLEW: windows_sys::core::PCWSTR = windows_sys::core::w!("TF_FloatingLangBar_WndTitle");
pub const TF_INVALIDMENUITEM: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TF_LANGBARITEMINFO {
    pub clsidService: windows_sys::core::GUID,
    pub guidItem: windows_sys::core::GUID,
    pub dwStyle: u32,
    pub ulSort: u32,
    pub szDescription: [u16; 32],
}
impl Default for TF_LANGBARITEMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TF_LBBALLOONINFO {
    pub style: TfLBBalloonStyle,
    pub bstrText: windows_sys::core::BSTR,
}
impl Default for TF_LBBALLOONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_LBI_BALLOON: u32 = 16;
pub const TF_LBI_BITMAP: u32 = 8;
pub const TF_LBI_BMPALL: u32 = 12;
pub const TF_LBI_BMPBTNALL: u32 = 14;
pub const TF_LBI_BMPF_VERTICAL: u32 = 1;
pub const TF_LBI_BTNALL: u32 = 7;
pub const TF_LBI_CLK_LEFT: TfLBIClick = 2;
pub const TF_LBI_CLK_RIGHT: TfLBIClick = 1;
pub const TF_LBI_CUSTOMUI: u32 = 32;
pub const TF_LBI_DESC_MAXLEN: u32 = 32;
pub const TF_LBI_ICON: u32 = 1;
pub const TF_LBI_STATUS: u32 = 65536;
pub const TF_LBI_STATUS_BTN_TOGGLED: u32 = 65536;
pub const TF_LBI_STATUS_DISABLED: u32 = 2;
pub const TF_LBI_STATUS_HIDDEN: u32 = 1;
pub const TF_LBI_STYLE_BTN_BUTTON: u32 = 65536;
pub const TF_LBI_STYLE_BTN_MENU: u32 = 131072;
pub const TF_LBI_STYLE_BTN_TOGGLE: u32 = 262144;
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: u32 = 16;
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: u32 = 1;
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: u32 = 4;
pub const TF_LBI_STYLE_SHOWNINTRAY: u32 = 2;
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: u32 = 8;
pub const TF_LBI_STYLE_TEXTCOLORICON: u32 = 32;
pub const TF_LBI_TEXT: u32 = 2;
pub const TF_LBI_TOOLTIP: u32 = 4;
pub const TF_LBMENUF_CHECKED: u32 = 1;
pub const TF_LBMENUF_GRAYED: u32 = 16;
pub const TF_LBMENUF_RADIOCHECKED: u32 = 8;
pub const TF_LBMENUF_SEPARATOR: u32 = 4;
pub const TF_LBMENUF_SUBMENU: u32 = 2;
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = 2;
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = 0;
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = 1;
pub const TF_SFT_DESKBAND: u32 = 2048;
pub const TF_SFT_DOCK: u32 = 2;
pub const TF_SFT_EXTRAICONSONMINIMIZED: u32 = 512;
pub const TF_SFT_HIDDEN: u32 = 8;
pub const TF_SFT_HIGHTRANSPARENCY: u32 = 64;
pub const TF_SFT_LABELS: u32 = 128;
pub const TF_SFT_LOWTRANSPARENCY: u32 = 32;
pub const TF_SFT_MINIMIZED: u32 = 4;
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: u32 = 1024;
pub const TF_SFT_NOLABELS: u32 = 256;
pub const TF_SFT_NOTRANSPARENCY: u32 = 16;
pub const TF_SFT_SHOWNORMAL: u32 = 1;
pub type TfLBBalloonStyle = i32;
pub type TfLBIClick = i32;
