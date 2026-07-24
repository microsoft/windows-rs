pub const TF_DTLBI_USEPROFILEICON: i32 = 1;
pub const TF_FLOATINGLANGBAR_WNDTITLEA: windows_sys::core::PCSTR = windows_sys::core::s!("TF_FloatingLangBar_WndTitle");
pub const TF_FLOATINGLANGBAR_WNDTITLEW: windows_sys::core::PCWSTR = windows_sys::core::w!("TF_FloatingLangBar_WndTitle");
pub const TF_INVALIDMENUITEM: u32 = 4294967295;
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
pub const TF_LBI_BALLOON: i32 = 16;
pub const TF_LBI_BITMAP: i32 = 8;
pub const TF_LBI_BMPALL: i32 = 12;
pub const TF_LBI_BMPBTNALL: i32 = 14;
pub const TF_LBI_BMPF_VERTICAL: i32 = 1;
pub const TF_LBI_BTNALL: i32 = 7;
pub const TF_LBI_CLK_LEFT: TfLBIClick = 2;
pub const TF_LBI_CLK_RIGHT: TfLBIClick = 1;
pub const TF_LBI_CUSTOMUI: i32 = 32;
pub const TF_LBI_DESC_MAXLEN: i32 = 32;
pub const TF_LBI_ICON: i32 = 1;
pub const TF_LBI_STATUS: i32 = 65536;
pub const TF_LBI_STATUS_BTN_TOGGLED: i32 = 65536;
pub const TF_LBI_STATUS_DISABLED: i32 = 2;
pub const TF_LBI_STATUS_HIDDEN: i32 = 1;
pub const TF_LBI_STYLE_BTN_BUTTON: i32 = 65536;
pub const TF_LBI_STYLE_BTN_MENU: i32 = 131072;
pub const TF_LBI_STYLE_BTN_TOGGLE: i32 = 262144;
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: i32 = 16;
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: i32 = 1;
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: i32 = 4;
pub const TF_LBI_STYLE_SHOWNINTRAY: i32 = 2;
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: i32 = 8;
pub const TF_LBI_STYLE_TEXTCOLORICON: i32 = 32;
pub const TF_LBI_TEXT: i32 = 2;
pub const TF_LBI_TOOLTIP: i32 = 4;
pub const TF_LBMENUF_CHECKED: i32 = 1;
pub const TF_LBMENUF_GRAYED: i32 = 16;
pub const TF_LBMENUF_RADIOCHECKED: i32 = 8;
pub const TF_LBMENUF_SEPARATOR: i32 = 4;
pub const TF_LBMENUF_SUBMENU: i32 = 2;
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = 2;
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = 0;
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = 1;
pub const TF_SFT_DESKBAND: i32 = 2048;
pub const TF_SFT_DOCK: i32 = 2;
pub const TF_SFT_EXTRAICONSONMINIMIZED: i32 = 512;
pub const TF_SFT_HIDDEN: i32 = 8;
pub const TF_SFT_HIGHTRANSPARENCY: i32 = 64;
pub const TF_SFT_LABELS: i32 = 128;
pub const TF_SFT_LOWTRANSPARENCY: i32 = 32;
pub const TF_SFT_MINIMIZED: i32 = 4;
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: i32 = 1024;
pub const TF_SFT_NOLABELS: i32 = 256;
pub const TF_SFT_NOTRANSPARENCY: i32 = 16;
pub const TF_SFT_SHOWNORMAL: i32 = 1;
pub type TfLBBalloonStyle = i32;
pub type TfLBIClick = i32;
