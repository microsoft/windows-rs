#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct ACCEL {
    pub fVirt: u8,
    pub key: u16,
    pub cmd: u16,
}
impl ::core::marker::Copy for ACCEL {}
impl ::core::clone::Clone for ACCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCEL").field("fVirt", &self.fVirt).field("key", &self.key).field("cmd", &self.cmd).finish()
    }
}
unsafe impl ::windows::core::Abi for ACCEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACCEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACCEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACCEL {}
impl ::core::default::Default for ACCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ALTTABINFO {
    pub cbSize: u32,
    pub cItems: i32,
    pub cColumns: i32,
    pub cRows: i32,
    pub iColFocus: i32,
    pub iRowFocus: i32,
    pub cxItem: i32,
    pub cyItem: i32,
    pub ptStart: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ALTTABINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ALTTABINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ALTTABINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ALTTABINFO").field("cbSize", &self.cbSize).field("cItems", &self.cItems).field("cColumns", &self.cColumns).field("cRows", &self.cRows).field("iColFocus", &self.iColFocus).field("iRowFocus", &self.iRowFocus).field("cxItem", &self.cxItem).field("cyItem", &self.cyItem).field("ptStart", &self.ptStart).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ALTTABINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ALTTABINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ALTTABINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ALTTABINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ALTTABINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ANIMATE_WINDOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_ACTIVATE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_BLEND: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_CENTER: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_HIDE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_HOR_POSITIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_HOR_NEGATIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_SLIDE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_VER_POSITIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const AW_VER_NEGATIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(8u32);
impl ::core::marker::Copy for ANIMATE_WINDOW_FLAGS {}
impl ::core::clone::Clone for ANIMATE_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ANIMATE_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ANIMATE_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ANIMATE_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ANIMATE_WINDOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ANIMATE_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ANIMATE_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct ANIMATIONINFO {
    pub cbSize: u32,
    pub iMinAnimate: i32,
}
impl ::core::marker::Copy for ANIMATIONINFO {}
impl ::core::clone::Clone for ANIMATIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ANIMATIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ANIMATIONINFO").field("cbSize", &self.cbSize).field("iMinAnimate", &self.iMinAnimate).finish()
    }
}
unsafe impl ::windows::core::Abi for ANIMATIONINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ANIMATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ANIMATIONINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ANIMATIONINFO {}
impl ::core::default::Default for ANIMATIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_DOWN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_HIDE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_LEFT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_RIGHT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_STARTMASK: i32 = 3i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_STARTRIGHT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_STARTTOP: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_UP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ASFW_ANY: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIODESCRIPTION {
    pub cbSize: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub Locale: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIODESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIODESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIODESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIODESCRIPTION").field("cbSize", &self.cbSize).field("Enabled", &self.Enabled).field("Locale", &self.Locale).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIODESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIODESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIODESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIODESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIODESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdjustWindowRect<'a, P0>(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AdjustWindowRect(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    AdjustWindowRect(::core::mem::transmute(lprect), dwstyle, bmenu.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdjustWindowRectEx<'a, P0>(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: P0, dwexstyle: WINDOW_EX_STYLE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AdjustWindowRectEx(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: super::super::Foundation::BOOL, dwexstyle: WINDOW_EX_STYLE) -> super::super::Foundation::BOOL;
    }
    AdjustWindowRectEx(::core::mem::transmute(lprect), dwstyle, bmenu.into(), dwexstyle)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllowSetForegroundWindow(dwprocessid: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AllowSetForegroundWindow(dwprocessid: u32) -> super::super::Foundation::BOOL;
    }
    AllowSetForegroundWindow(dwprocessid)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AnimateWindow<'a, P0>(hwnd: P0, dwtime: u32, dwflags: ANIMATE_WINDOW_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AnimateWindow(hwnd: super::super::Foundation::HWND, dwtime: u32, dwflags: ANIMATE_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    }
    AnimateWindow(hwnd.into(), dwtime, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AnyPopup() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AnyPopup() -> super::super::Foundation::BOOL;
    }
    AnyPopup()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppendMenuA<'a, P0, P1>(hmenu: P0, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AppendMenuA(hmenu: HMENU, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    AppendMenuA(hmenu.into(), uflags, uidnewitem, lpnewitem.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppendMenuW<'a, P0, P1>(hmenu: P0, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AppendMenuW(hmenu: HMENU, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    AppendMenuW(hmenu.into(), uflags, uidnewitem, lpnewitem.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ArrangeIconicWindows<'a, P0>(hwnd: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ArrangeIconicWindows(hwnd: super::super::Foundation::HWND) -> u32;
    }
    ArrangeIconicWindows(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_CLICK: u32 = 245u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_GETCHECK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_GETIMAGE: u32 = 246u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_GETSTATE: u32 = 242u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_SETCHECK: u32 = 241u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_SETDONTCLICK: u32 = 248u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_SETIMAGE: u32 = 247u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_SETSTATE: u32 = 243u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BM_SETSTYLE: u32 = 244u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_CLICKED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_DBLCLK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_DISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_DOUBLECLICKED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_HILITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_KILLFOCUS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_PAINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_PUSHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_SETFOCUS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_UNHILITE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BN_UNPUSHED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BROADCAST_QUERY_DENY: u32 = 1112363332u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BSM_INSTALLABLEDRIVERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BSM_NETDRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BSM_VXDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BST_FOCUS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BST_PUSHED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_3STATE: i32 = 5i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_AUTO3STATE: i32 = 6i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_AUTOCHECKBOX: i32 = 3i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_AUTORADIOBUTTON: i32 = 9i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_BITMAP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_BOTTOM: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_CENTER: i32 = 768i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_CHECKBOX: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_DEFPUSHBUTTON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_FLAT: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_GROUPBOX: i32 = 7i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_ICON: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_LEFT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_LEFTTEXT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_MULTILINE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_NOTIFY: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_OWNERDRAW: i32 = 11i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_PUSHBOX: i32 = 10i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_PUSHBUTTON: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_PUSHLIKE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_RADIOBUTTON: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_RIGHT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_RIGHTBUTTON: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_TEXT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_TOP: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_TYPEMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_USERBUTTON: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const BS_VCENTER: i32 = 3072i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn BeginDeferWindowPos(nnumwindows: i32) -> isize {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BeginDeferWindowPos(nnumwindows: i32) -> isize;
    }
    BeginDeferWindowPos(nnumwindows)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BringWindowToTop<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BringWindowToTop(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    BringWindowToTop(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CALERT_SYSTEM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CASCADE_WINDOWS_HOW(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MDITILE_SKIPDISABLED: CASCADE_WINDOWS_HOW = CASCADE_WINDOWS_HOW(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MDITILE_ZORDER: CASCADE_WINDOWS_HOW = CASCADE_WINDOWS_HOW(4u32);
impl ::core::marker::Copy for CASCADE_WINDOWS_HOW {}
impl ::core::clone::Clone for CASCADE_WINDOWS_HOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASCADE_WINDOWS_HOW {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CASCADE_WINDOWS_HOW {
    type Abi = Self;
}
impl ::core::fmt::Debug for CASCADE_WINDOWS_HOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASCADE_WINDOWS_HOW").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CASCADE_WINDOWS_HOW {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CASCADE_WINDOWS_HOW {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_CLOSEUP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_DBLCLK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_DROPDOWN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_EDITCHANGE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_EDITUPDATE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_ERRSPACE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_KILLFOCUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_SELCHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_SELENDCANCEL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_SELENDOK: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBN_SETFOCUS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_AUTOHSCROLL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_DISABLENOSCROLL: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_DROPDOWN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_DROPDOWNLIST: i32 = 3i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_HASSTRINGS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_LOWERCASE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_NOINTEGRALHEIGHT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_OEMCONVERT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_OWNERDRAWFIXED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_OWNERDRAWVARIABLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_SIMPLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_SORT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CBS_UPPERCASE: i32 = 8192i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CBTACTIVATESTRUCT {
    pub fMouse: super::super::Foundation::BOOL,
    pub hWndActive: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CBTACTIVATESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CBTACTIVATESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CBTACTIVATESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBTACTIVATESTRUCT").field("fMouse", &self.fMouse).field("hWndActive", &self.hWndActive).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CBTACTIVATESTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CBTACTIVATESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CBTACTIVATESTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CBTACTIVATESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CBTACTIVATESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CBT_CREATEWNDA {
    pub lpcs: *mut CREATESTRUCTA,
    pub hwndInsertAfter: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CBT_CREATEWNDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CBT_CREATEWNDA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CBT_CREATEWNDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBT_CREATEWNDA").field("lpcs", &self.lpcs).field("hwndInsertAfter", &self.hwndInsertAfter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CBT_CREATEWNDA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CBT_CREATEWNDA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CBT_CREATEWNDA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CBT_CREATEWNDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CBT_CREATEWNDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CBT_CREATEWNDW {
    pub lpcs: *mut CREATESTRUCTW,
    pub hwndInsertAfter: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CBT_CREATEWNDW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CBT_CREATEWNDW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CBT_CREATEWNDW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBT_CREATEWNDW").field("lpcs", &self.lpcs).field("hwndInsertAfter", &self.hwndInsertAfter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CBT_CREATEWNDW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CBT_CREATEWNDW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CBT_CREATEWNDW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CBT_CREATEWNDW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CBT_CREATEWNDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_ADDSTRING: u32 = 323u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_DELETESTRING: u32 = 324u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_DIR: u32 = 325u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_ERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_ERRSPACE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_FINDSTRING: u32 = 332u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_FINDSTRINGEXACT: u32 = 344u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETCOMBOBOXINFO: u32 = 356u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETCOUNT: u32 = 326u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETCURSEL: u32 = 327u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETDROPPEDCONTROLRECT: u32 = 338u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETDROPPEDSTATE: u32 = 343u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETDROPPEDWIDTH: u32 = 351u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETEDITSEL: u32 = 320u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETEXTENDEDUI: u32 = 342u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETHORIZONTALEXTENT: u32 = 349u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETITEMDATA: u32 = 336u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETITEMHEIGHT: u32 = 340u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETLBTEXT: u32 = 328u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETLBTEXTLEN: u32 = 329u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETLOCALE: u32 = 346u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_GETTOPINDEX: u32 = 347u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_INITSTORAGE: u32 = 353u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_INSERTSTRING: u32 = 330u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_LIMITTEXT: u32 = 321u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_MSGMAX: u32 = 357u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_MULTIPLEADDSTRING: u32 = 355u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_OKAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_RESETCONTENT: u32 = 331u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SELECTSTRING: u32 = 333u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETCURSEL: u32 = 334u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETDROPPEDWIDTH: u32 = 352u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETEDITSEL: u32 = 322u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETEXTENDEDUI: u32 = 341u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETHORIZONTALEXTENT: u32 = 350u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETITEMDATA: u32 = 337u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETITEMHEIGHT: u32 = 339u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETLOCALE: u32 = 345u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SETTOPINDEX: u32 = 348u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CB_SHOWDROPDOWN: u32 = 335u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CCHILDREN_SCROLLBAR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CCHILDREN_TITLEBAR: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct CHANGEFILTERSTRUCT {
    pub cbSize: u32,
    pub ExtStatus: MSGFLTINFO_STATUS,
}
impl ::core::marker::Copy for CHANGEFILTERSTRUCT {}
impl ::core::clone::Clone for CHANGEFILTERSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANGEFILTERSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGEFILTERSTRUCT").field("cbSize", &self.cbSize).field("ExtStatus", &self.ExtStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGEFILTERSTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGEFILTERSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGEFILTERSTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGEFILTERSTRUCT {}
impl ::core::default::Default for CHANGEFILTERSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLT_ADD: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS = CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLT_REMOVE: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS = CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(2u32);
impl ::core::marker::Copy for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {}
impl ::core::clone::Clone for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGE_WINDOW_MESSAGE_FILTER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CHILDID_SELF: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIENTCREATESTRUCT {
    pub hWindowMenu: super::super::Foundation::HANDLE,
    pub idFirstChild: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIENTCREATESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIENTCREATESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLIENTCREATESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENTCREATESTRUCT").field("hWindowMenu", &self.hWindowMenu).field("idFirstChild", &self.idFirstChild).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLIENTCREATESTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIENTCREATESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLIENTCREATESTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIENTCREATESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIENTCREATESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_BTNHIGHLIGHT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_BTNHILIGHT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CONSOLE_APPLICATION_16BIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CONSOLE_CARET_SELECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CONSOLE_CARET_VISIBLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CONTACTVISUALIZATION_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CONTACTVISUALIZATION_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CONTACTVISUALIZATION_PRESENTATIONMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CREATEPROCESS_MANIFEST_RESOURCE_ID: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATESTRUCTA {
    pub lpCreateParams: *mut ::core::ffi::c_void,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: super::super::Foundation::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: ::windows::core::PCSTR,
    pub lpszClass: ::windows::core::PCSTR,
    pub dwExStyle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATESTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATESTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATESTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATESTRUCTA").field("lpCreateParams", &self.lpCreateParams).field("hInstance", &self.hInstance).field("hMenu", &self.hMenu).field("hwndParent", &self.hwndParent).field("cy", &self.cy).field("cx", &self.cx).field("y", &self.y).field("x", &self.x).field("style", &self.style).field("lpszName", &self.lpszName).field("lpszClass", &self.lpszClass).field("dwExStyle", &self.dwExStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREATESTRUCTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATESTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATESTRUCTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATESTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATESTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATESTRUCTW {
    pub lpCreateParams: *mut ::core::ffi::c_void,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: super::super::Foundation::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: ::windows::core::PCWSTR,
    pub lpszClass: ::windows::core::PCWSTR,
    pub dwExStyle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATESTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATESTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATESTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATESTRUCTW").field("lpCreateParams", &self.lpCreateParams).field("hInstance", &self.hInstance).field("hMenu", &self.hMenu).field("hwndParent", &self.hwndParent).field("cy", &self.cy).field("cx", &self.cx).field("y", &self.y).field("x", &self.x).field("style", &self.style).field("lpszName", &self.lpszName).field("lpszClass", &self.lpszClass).field("dwExStyle", &self.dwExStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREATESTRUCTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATESTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATESTRUCTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATESTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATESTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CSOUND_SYSTEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_BTN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_DLG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_EDIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_LISTBOX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_MAX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_MSGBOX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_SCROLLBAR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CTLCOLOR_STATIC: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CURSORINFO {
    pub cbSize: u32,
    pub flags: CURSORINFO_FLAGS,
    pub hCursor: HCURSOR,
    pub ptScreenPos: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CURSORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CURSORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CURSORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURSORINFO").field("cbSize", &self.cbSize).field("flags", &self.flags).field("hCursor", &self.hCursor).field("ptScreenPos", &self.ptScreenPos).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CURSORINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CURSORINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CURSORINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CURSORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CURSORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CURSORINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CURSOR_SHOWING: CURSORINFO_FLAGS = CURSORINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CURSOR_SUPPRESSED: CURSORINFO_FLAGS = CURSORINFO_FLAGS(2u32);
impl ::core::marker::Copy for CURSORINFO_FLAGS {}
impl ::core::clone::Clone for CURSORINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CURSORINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CURSORINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CURSORINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CURSORINFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct CURSORSHAPE {
    pub xHotSpot: i32,
    pub yHotSpot: i32,
    pub cx: i32,
    pub cy: i32,
    pub cbWidth: i32,
    pub Planes: u8,
    pub BitsPixel: u8,
}
impl ::core::marker::Copy for CURSORSHAPE {}
impl ::core::clone::Clone for CURSORSHAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CURSORSHAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURSORSHAPE").field("xHotSpot", &self.xHotSpot).field("yHotSpot", &self.yHotSpot).field("cx", &self.cx).field("cy", &self.cy).field("cbWidth", &self.cbWidth).field("Planes", &self.Planes).field("BitsPixel", &self.BitsPixel).finish()
    }
}
unsafe impl ::windows::core::Abi for CURSORSHAPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CURSORSHAPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CURSORSHAPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CURSORSHAPE {}
impl ::core::default::Default for CURSORSHAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CURSOR_CREATION_SCALING_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CURSOR_CREATION_SCALING_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CWF_CREATE_ONLY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CWPRETSTRUCT {
    pub lResult: super::super::Foundation::LRESULT,
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub message: u32,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CWPRETSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CWPRETSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CWPRETSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CWPRETSTRUCT").field("lResult", &self.lResult).field("lParam", &self.lParam).field("wParam", &self.wParam).field("message", &self.message).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CWPRETSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CWPRETSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CWPRETSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CWPRETSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CWPRETSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CWPSTRUCT {
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub message: u32,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CWPSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CWPSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CWPSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CWPSTRUCT").field("lParam", &self.lParam).field("wParam", &self.wParam).field("message", &self.message).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CWPSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CWPSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CWPSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CWPSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CWPSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CWP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CWP_ALL: CWP_FLAGS = CWP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CWP_SKIPINVISIBLE: CWP_FLAGS = CWP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CWP_SKIPDISABLED: CWP_FLAGS = CWP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CWP_SKIPTRANSPARENT: CWP_FLAGS = CWP_FLAGS(4u32);
impl ::core::marker::Copy for CWP_FLAGS {}
impl ::core::clone::Clone for CWP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CWP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CWP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CWP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CWP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CWP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CWP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CWP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CWP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CWP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CW_USEDEFAULT: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CalculatePopupWindowPosition(anchorpoint: *const super::super::Foundation::POINT, windowsize: *const super::super::Foundation::SIZE, flags: u32, excluderect: *const super::super::Foundation::RECT, popupwindowposition: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CalculatePopupWindowPosition(anchorpoint: *const super::super::Foundation::POINT, windowsize: *const super::super::Foundation::SIZE, flags: u32, excluderect: *const super::super::Foundation::RECT, popupwindowposition: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    CalculatePopupWindowPosition(::core::mem::transmute(anchorpoint), ::core::mem::transmute(windowsize), flags, ::core::mem::transmute(excluderect), ::core::mem::transmute(popupwindowposition))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallMsgFilterA(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CallMsgFilterA(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL;
    }
    CallMsgFilterA(::core::mem::transmute(lpmsg), ncode)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallMsgFilterW(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CallMsgFilterW(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL;
    }
    CallMsgFilterW(::core::mem::transmute(lpmsg), ncode)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallNextHookEx<'a, P0, P1, P2>(hhk: P0, ncode: i32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<HHOOK>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CallNextHookEx(hhk: HHOOK, ncode: i32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    CallNextHookEx(hhk.into(), ncode, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallWindowProcA<'a, P0, P1, P2>(lpprevwndfunc: WNDPROC, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CallWindowProcA(lpprevwndfunc: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    CallWindowProcA(::core::mem::transmute(lpprevwndfunc), hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallWindowProcW<'a, P0, P1, P2>(lpprevwndfunc: WNDPROC, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CallWindowProcW(lpprevwndfunc: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    CallWindowProcW(::core::mem::transmute(lpprevwndfunc), hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelShutdown() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CancelShutdown() -> super::super::Foundation::BOOL;
    }
    CancelShutdown()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CascadeWindows<'a, P0>(hwndparent: P0, whow: CASCADE_WINDOWS_HOW, lprect: *const super::super::Foundation::RECT, lpkids: &[super::super::Foundation::HWND]) -> u16
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CascadeWindows(hwndparent: super::super::Foundation::HWND, whow: CASCADE_WINDOWS_HOW, lprect: *const super::super::Foundation::RECT, ckids: u32, lpkids: *const super::super::Foundation::HWND) -> u16;
    }
    CascadeWindows(hwndparent.into(), whow, ::core::mem::transmute(lprect), lpkids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lpkids)))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeMenuA<'a, P0, P1>(hmenu: P0, cmd: u32, lpsznewitem: P1, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeMenuA(hmenu: HMENU, cmd: u32, lpsznewitem: ::windows::core::PCSTR, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL;
    }
    ChangeMenuA(hmenu.into(), cmd, lpsznewitem.into(), cmdinsert, flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeMenuW<'a, P0, P1>(hmenu: P0, cmd: u32, lpsznewitem: P1, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeMenuW(hmenu: HMENU, cmd: u32, lpsznewitem: ::windows::core::PCWSTR, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL;
    }
    ChangeMenuW(hmenu.into(), cmd, lpsznewitem.into(), cmdinsert, flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeWindowMessageFilter(message: u32, dwflag: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeWindowMessageFilter(message: u32, dwflag: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS) -> super::super::Foundation::BOOL;
    }
    ChangeWindowMessageFilter(message, dwflag)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeWindowMessageFilterEx<'a, P0>(hwnd: P0, message: u32, action: WINDOW_MESSAGE_FILTER_ACTION, pchangefilterstruct: *mut CHANGEFILTERSTRUCT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeWindowMessageFilterEx(hwnd: super::super::Foundation::HWND, message: u32, action: WINDOW_MESSAGE_FILTER_ACTION, pchangefilterstruct: *mut CHANGEFILTERSTRUCT) -> super::super::Foundation::BOOL;
    }
    ChangeWindowMessageFilterEx(hwnd.into(), message, action, ::core::mem::transmute(pchangefilterstruct))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharLowerA(lpsz: ::windows::core::PSTR) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharLowerA(lpsz: ::windows::core::PSTR) -> ::windows::core::PSTR;
    }
    CharLowerA(::core::mem::transmute(lpsz))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharLowerBuffA(lpsz: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharLowerBuffA(lpsz: ::windows::core::PSTR, cchlength: u32) -> u32;
    }
    CharLowerBuffA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpsz)), lpsz.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharLowerBuffW(lpsz: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharLowerBuffW(lpsz: ::windows::core::PWSTR, cchlength: u32) -> u32;
    }
    CharLowerBuffW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpsz)), lpsz.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharLowerW(lpsz: ::windows::core::PWSTR) -> ::windows::core::PWSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharLowerW(lpsz: ::windows::core::PWSTR) -> ::windows::core::PWSTR;
    }
    CharLowerW(::core::mem::transmute(lpsz))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharNextA<'a, P0>(lpsz: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharNextA(lpsz: ::windows::core::PCSTR) -> ::windows::core::PSTR;
    }
    CharNextA(lpsz.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharNextExA<'a, P0>(codepage: u16, lpcurrentchar: P0, dwflags: u32) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharNextExA(codepage: u16, lpcurrentchar: ::windows::core::PCSTR, dwflags: u32) -> ::windows::core::PSTR;
    }
    CharNextExA(codepage, lpcurrentchar.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharNextW<'a, P0>(lpsz: P0) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharNextW(lpsz: ::windows::core::PCWSTR) -> ::windows::core::PWSTR;
    }
    CharNextW(lpsz.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharPrevA<'a, P0, P1>(lpszstart: P0, lpszcurrent: P1) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharPrevA(lpszstart: ::windows::core::PCSTR, lpszcurrent: ::windows::core::PCSTR) -> ::windows::core::PSTR;
    }
    CharPrevA(lpszstart.into(), lpszcurrent.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharPrevExA<'a, P0, P1>(codepage: u16, lpstart: P0, lpcurrentchar: P1, dwflags: u32) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharPrevExA(codepage: u16, lpstart: ::windows::core::PCSTR, lpcurrentchar: ::windows::core::PCSTR, dwflags: u32) -> ::windows::core::PSTR;
    }
    CharPrevExA(codepage, lpstart.into(), lpcurrentchar.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharPrevW<'a, P0, P1>(lpszstart: P0, lpszcurrent: P1) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharPrevW(lpszstart: ::windows::core::PCWSTR, lpszcurrent: ::windows::core::PCWSTR) -> ::windows::core::PWSTR;
    }
    CharPrevW(lpszstart.into(), lpszcurrent.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CharToOemA<'a, P0>(psrc: P0, pdst: ::windows::core::PSTR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharToOemA(psrc: ::windows::core::PCSTR, pdst: ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    CharToOemA(psrc.into(), ::core::mem::transmute(pdst))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CharToOemBuffA<'a, P0>(lpszsrc: P0, lpszdst: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharToOemBuffA(lpszsrc: ::windows::core::PCSTR, lpszdst: ::windows::core::PSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    }
    CharToOemBuffA(lpszsrc.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszdst)), lpszdst.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CharToOemBuffW<'a, P0>(lpszsrc: P0, lpszdst: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharToOemBuffW(lpszsrc: ::windows::core::PCWSTR, lpszdst: ::windows::core::PSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    }
    CharToOemBuffW(lpszsrc.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszdst)), lpszdst.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CharToOemW<'a, P0>(psrc: P0, pdst: ::windows::core::PSTR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharToOemW(psrc: ::windows::core::PCWSTR, pdst: ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    CharToOemW(psrc.into(), ::core::mem::transmute(pdst))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharUpperA(lpsz: ::windows::core::PSTR) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharUpperA(lpsz: ::windows::core::PSTR) -> ::windows::core::PSTR;
    }
    CharUpperA(::core::mem::transmute(lpsz))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharUpperBuffA(lpsz: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharUpperBuffA(lpsz: ::windows::core::PSTR, cchlength: u32) -> u32;
    }
    CharUpperBuffA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpsz)), lpsz.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharUpperBuffW(lpsz: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharUpperBuffW(lpsz: ::windows::core::PWSTR, cchlength: u32) -> u32;
    }
    CharUpperBuffW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpsz)), lpsz.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CharUpperW(lpsz: ::windows::core::PWSTR) -> ::windows::core::PWSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CharUpperW(lpsz: ::windows::core::PWSTR) -> ::windows::core::PWSTR;
    }
    CharUpperW(::core::mem::transmute(lpsz))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CheckMenuItem<'a, P0>(hmenu: P0, uidcheckitem: u32, ucheck: u32) -> u32
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CheckMenuItem(hmenu: HMENU, uidcheckitem: u32, ucheck: u32) -> u32;
    }
    CheckMenuItem(hmenu.into(), uidcheckitem, ucheck)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckMenuRadioItem<'a, P0>(hmenu: P0, first: u32, last: u32, check: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CheckMenuRadioItem(hmenu: HMENU, first: u32, last: u32, check: u32, flags: u32) -> super::super::Foundation::BOOL;
    }
    CheckMenuRadioItem(hmenu.into(), first, last, check, flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChildWindowFromPoint<'a, P0>(hwndparent: P0, point: super::super::Foundation::POINT) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChildWindowFromPoint(hwndparent: super::super::Foundation::HWND, point: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    }
    ChildWindowFromPoint(hwndparent.into(), ::core::mem::transmute(point))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChildWindowFromPointEx<'a, P0>(hwnd: P0, pt: super::super::Foundation::POINT, flags: CWP_FLAGS) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChildWindowFromPointEx(hwnd: super::super::Foundation::HWND, pt: super::super::Foundation::POINT, flags: CWP_FLAGS) -> super::super::Foundation::HWND;
    }
    ChildWindowFromPointEx(hwnd.into(), ::core::mem::transmute(pt), flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClipCursor(lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ClipCursor(lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    ClipCursor(::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    CloseWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CopyAcceleratorTableA<'a, P0>(haccelsrc: P0, lpacceldst: &mut [ACCEL]) -> i32
where
    P0: ::std::convert::Into<HACCEL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyAcceleratorTableA(haccelsrc: HACCEL, lpacceldst: *mut ACCEL, caccelentries: i32) -> i32;
    }
    CopyAcceleratorTableA(haccelsrc.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpacceldst)), lpacceldst.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CopyAcceleratorTableW<'a, P0>(haccelsrc: P0, lpacceldst: &mut [ACCEL]) -> i32
where
    P0: ::std::convert::Into<HACCEL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyAcceleratorTableW(haccelsrc: HACCEL, lpacceldst: *mut ACCEL, caccelentries: i32) -> i32;
    }
    CopyAcceleratorTableW(haccelsrc.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpacceldst)), lpacceldst.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CopyIcon<'a, P0>(hicon: P0) -> ::windows::core::Result<HICON>
where
    P0: ::std::convert::Into<HICON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyIcon(hicon: HICON) -> HICON;
    }
    let result__ = CopyIcon(hicon.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyImage<'a, P0>(h: P0, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, flags: IMAGE_FLAGS) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyImage(h: super::super::Foundation::HANDLE, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, flags: IMAGE_FLAGS) -> super::super::Foundation::HANDLE;
    }
    let result__ = CopyImage(h.into(), r#type, cx, cy, flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CreateAcceleratorTableA(paccel: &[ACCEL]) -> ::windows::core::Result<HACCEL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateAcceleratorTableA(paccel: *const ACCEL, caccel: i32) -> HACCEL;
    }
    let result__ = CreateAcceleratorTableA(::core::mem::transmute(::windows::core::as_ptr_or_null(paccel)), paccel.len() as _);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CreateAcceleratorTableW(paccel: &[ACCEL]) -> ::windows::core::Result<HACCEL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateAcceleratorTableW(paccel: *const ACCEL, caccel: i32) -> HACCEL;
    }
    let result__ = CreateAcceleratorTableW(::core::mem::transmute(::windows::core::as_ptr_or_null(paccel)), paccel.len() as _);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateCaret<'a, P0, P1>(hwnd: P0, hbitmap: P1, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateCaret(hwnd: super::super::Foundation::HWND, hbitmap: super::super::Graphics::Gdi::HBITMAP, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    }
    CreateCaret(hwnd.into(), hbitmap.into(), nwidth, nheight)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateCursor<'a, P0>(hinst: P0, xhotspot: i32, yhotspot: i32, nwidth: i32, nheight: i32, pvandplane: *const ::core::ffi::c_void, pvxorplane: *const ::core::ffi::c_void) -> ::windows::core::Result<HCURSOR>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateCursor(hinst: super::super::Foundation::HINSTANCE, xhotspot: i32, yhotspot: i32, nwidth: i32, nheight: i32, pvandplane: *const ::core::ffi::c_void, pvxorplane: *const ::core::ffi::c_void) -> HCURSOR;
    }
    let result__ = CreateCursor(hinst.into(), xhotspot, yhotspot, nwidth, nheight, ::core::mem::transmute(pvandplane), ::core::mem::transmute(pvxorplane));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDialogIndirectParamA<'a, P0, P1, P2>(hinstance: P0, lptemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDialogIndirectParamA(hinstance: super::super::Foundation::HINSTANCE, lptemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    }
    CreateDialogIndirectParamA(hinstance.into(), ::core::mem::transmute(lptemplate), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDialogIndirectParamW<'a, P0, P1, P2>(hinstance: P0, lptemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDialogIndirectParamW(hinstance: super::super::Foundation::HINSTANCE, lptemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    }
    CreateDialogIndirectParamW(hinstance.into(), ::core::mem::transmute(lptemplate), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDialogParamA<'a, P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDialogParamA(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: ::windows::core::PCSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    }
    CreateDialogParamA(hinstance.into(), lptemplatename.into(), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDialogParamW<'a, P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDialogParamW(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: ::windows::core::PCWSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    }
    CreateDialogParamW(hinstance.into(), lptemplatename.into(), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateIcon<'a, P0>(hinstance: P0, nwidth: i32, nheight: i32, cplanes: u8, cbitspixel: u8, lpbandbits: *const u8, lpbxorbits: *const u8) -> ::windows::core::Result<HICON>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateIcon(hinstance: super::super::Foundation::HINSTANCE, nwidth: i32, nheight: i32, cplanes: u8, cbitspixel: u8, lpbandbits: *const u8, lpbxorbits: *const u8) -> HICON;
    }
    let result__ = CreateIcon(hinstance.into(), nwidth, nheight, cplanes, cbitspixel, ::core::mem::transmute(lpbandbits), ::core::mem::transmute(lpbxorbits));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateIconFromResource<'a, P0>(presbits: *const u8, dwressize: u32, ficon: P0, dwver: u32) -> ::windows::core::Result<HICON>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateIconFromResource(presbits: *const u8, dwressize: u32, ficon: super::super::Foundation::BOOL, dwver: u32) -> HICON;
    }
    let result__ = CreateIconFromResource(::core::mem::transmute(presbits), dwressize, ficon.into(), dwver);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateIconFromResourceEx<'a, P0>(presbits: *const u8, dwressize: u32, ficon: P0, dwver: u32, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> ::windows::core::Result<HICON>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateIconFromResourceEx(presbits: *const u8, dwressize: u32, ficon: super::super::Foundation::BOOL, dwver: u32, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> HICON;
    }
    let result__ = CreateIconFromResourceEx(::core::mem::transmute(presbits), dwressize, ficon.into(), dwver, cxdesired, cydesired, flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateIconIndirect(piconinfo: *const ICONINFO) -> ::windows::core::Result<HICON> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateIconIndirect(piconinfo: *const ICONINFO) -> HICON;
    }
    let result__ = CreateIconIndirect(::core::mem::transmute(piconinfo));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateMDIWindowA<'a, P0, P1, P2, P3, P4>(lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hinstance: P3, lparam: P4) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P4: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateMDIWindowA(lpclassname: ::windows::core::PCSTR, lpwindowname: ::windows::core::PCSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    }
    CreateMDIWindowA(lpclassname.into(), lpwindowname.into(), dwstyle, x, y, nwidth, nheight, hwndparent.into(), hinstance.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateMDIWindowW<'a, P0, P1, P2, P3, P4>(lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hinstance: P3, lparam: P4) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P4: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateMDIWindowW(lpclassname: ::windows::core::PCWSTR, lpwindowname: ::windows::core::PCWSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    }
    CreateMDIWindowW(lpclassname.into(), lpwindowname.into(), dwstyle, x, y, nwidth, nheight, hwndparent.into(), hinstance.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CreateMenu() -> ::windows::core::Result<HMENU> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateMenu() -> HMENU;
    }
    let result__ = CreateMenu();
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CreatePopupMenu() -> ::windows::core::Result<HMENU> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePopupMenu() -> HMENU;
    }
    let result__ = CreatePopupMenu();
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn CreateResourceIndexer<'a, P0, P1>(projectroot: P0, extensiondllpath: P1, ppresourceindexer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateResourceIndexer(projectroot: ::windows::core::PCWSTR, extensiondllpath: ::windows::core::PCWSTR, ppresourceindexer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    CreateResourceIndexer(projectroot.into(), extensiondllpath.into(), ::core::mem::transmute(ppresourceindexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateWindowExA<'a, P0, P1, P2, P3, P4>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hmenu: P3, hinstance: P4, lpparam: *const ::core::ffi::c_void) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<HMENU>,
    P4: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateWindowExA(dwexstyle: WINDOW_EX_STYLE, lpclassname: ::windows::core::PCSTR, lpwindowname: ::windows::core::PCSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hmenu: HMENU, hinstance: super::super::Foundation::HINSTANCE, lpparam: *const ::core::ffi::c_void) -> super::super::Foundation::HWND;
    }
    CreateWindowExA(dwexstyle, lpclassname.into(), lpwindowname.into(), dwstyle, x, y, nwidth, nheight, hwndparent.into(), hmenu.into(), hinstance.into(), ::core::mem::transmute(lpparam))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateWindowExW<'a, P0, P1, P2, P3, P4>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hmenu: P3, hinstance: P4, lpparam: *const ::core::ffi::c_void) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<HMENU>,
    P4: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateWindowExW(dwexstyle: WINDOW_EX_STYLE, lpclassname: ::windows::core::PCWSTR, lpwindowname: ::windows::core::PCWSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hmenu: HMENU, hinstance: super::super::Foundation::HINSTANCE, lpparam: *const ::core::ffi::c_void) -> super::super::Foundation::HWND;
    }
    CreateWindowExW(dwexstyle, lpclassname.into(), lpwindowname.into(), dwstyle, x, y, nwidth, nheight, hwndparent.into(), hmenu.into(), hinstance.into(), ::core::mem::transmute(lpparam))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DCX_EXCLUDEUPDATE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DC_HASDEFID: u32 = 21323u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEBUGHOOKINFO {
    pub idThread: u32,
    pub idThreadInstaller: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub code: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEBUGHOOKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEBUGHOOKINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUGHOOKINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUGHOOKINFO").field("idThread", &self.idThread).field("idThreadInstaller", &self.idThreadInstaller).field("lParam", &self.lParam).field("wParam", &self.wParam).field("code", &self.code).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEBUGHOOKINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUGHOOKINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEBUGHOOKINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUGHOOKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUGHOOKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_CREATEMENU: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_CREATEWINDOW: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_ENUMERATE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_HOOKCONTROL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_JOURNALPLAYBACK: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_JOURNALRECORD: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_READOBJECTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_SWITCHDESKTOP: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DESKTOP_WRITEOBJECTS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DF_ALLOWOTHERACCOUNTHOOK: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DIFFERENCE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DI_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DI_MASK: DI_FLAGS = DI_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DI_IMAGE: DI_FLAGS = DI_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DI_NORMAL: DI_FLAGS = DI_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DI_COMPAT: DI_FLAGS = DI_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DI_DEFAULTSIZE: DI_FLAGS = DI_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DI_NOMIRROR: DI_FLAGS = DI_FLAGS(16u32);
impl ::core::marker::Copy for DI_FLAGS {}
impl ::core::clone::Clone for DI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DI_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_BUTTON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_DEFPUSHBUTTON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_HASSETSEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_RADIOBUTTON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_STATIC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_UNDEFPUSHBUTTON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_WANTALLKEYS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_WANTARROWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_WANTCHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_WANTMESSAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGC_WANTTAB: u32 = 2u32;
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct DLGITEMTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub id: u16,
}
impl ::core::marker::Copy for DLGITEMTEMPLATE {}
impl ::core::clone::Clone for DLGITEMTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DLGITEMTEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DLGITEMTEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DLGITEMTEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DLGITEMTEMPLATE {}
impl ::core::default::Default for DLGITEMTEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DLGPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> isize>;
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct DLGTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub cdit: u16,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
}
impl ::core::marker::Copy for DLGTEMPLATE {}
impl ::core::clone::Clone for DLGTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DLGTEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DLGTEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DLGTEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DLGTEMPLATE {}
impl ::core::default::Default for DLGTEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DLGWINDOWEXTRA: u32 = 30u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DM_GETDEFID: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DM_POINTERHITTEST: u32 = 592u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DM_REPOSITION: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DM_SETDEFID: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DOF_DIRECTORY: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DOF_DOCUMENT: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DOF_EXECUTABLE: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DOF_MULTIPLE: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DOF_PROGMAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DOF_SHELLDATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DO_DROPFILE: i32 = 1162627398i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DO_PRINTFILE: i32 = 1414419024i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DROPSTRUCT {
    pub hwndSource: super::super::Foundation::HWND,
    pub hwndSink: super::super::Foundation::HWND,
    pub wFmt: u32,
    pub dwData: usize,
    pub ptDrop: super::super::Foundation::POINT,
    pub dwControlData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DROPSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DROPSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DROPSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DROPSTRUCT").field("hwndSource", &self.hwndSource).field("hwndSink", &self.hwndSink).field("wFmt", &self.wFmt).field("dwData", &self.dwData).field("ptDrop", &self.ptDrop).field("dwControlData", &self.dwControlData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DROPSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DROPSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DROPSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DROPSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DROPSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_3DLOOK: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_ABSALIGN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_CENTER: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_CENTERMOUSE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_CONTEXTHELP: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_CONTROL: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_FIXEDSYS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_LOCALEDIT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_MODALFRAME: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_NOFAILCREATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_NOIDLEMSG: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_SETFONT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_SETFOREGROUND: i32 = 512i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_SYSMODAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DS_USEPIXELS: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DWLP_MSGRESULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DWL_DLGPROC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DWL_MSGRESULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const DWL_USER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefDlgProcA<'a, P0, P1, P2>(hdlg: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefDlgProcA(hdlg: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefDlgProcA(hdlg.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefDlgProcW<'a, P0, P1, P2>(hdlg: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefDlgProcW(hdlg: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefDlgProcW(hdlg.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefFrameProcA<'a, P0, P1, P2, P3>(hwnd: P0, hwndmdiclient: P1, umsg: u32, wparam: P2, lparam: P3) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefFrameProcA(hwnd: super::super::Foundation::HWND, hwndmdiclient: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefFrameProcA(hwnd.into(), hwndmdiclient.into(), umsg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefFrameProcW<'a, P0, P1, P2, P3>(hwnd: P0, hwndmdiclient: P1, umsg: u32, wparam: P2, lparam: P3) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefFrameProcW(hwnd: super::super::Foundation::HWND, hwndmdiclient: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefFrameProcW(hwnd.into(), hwndmdiclient.into(), umsg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefMDIChildProcA<'a, P0, P1, P2>(hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefMDIChildProcA(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefMDIChildProcA(hwnd.into(), umsg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefMDIChildProcW<'a, P0, P1, P2>(hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefMDIChildProcW(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefMDIChildProcW(hwnd.into(), umsg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefWindowProcA<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefWindowProcA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefWindowProcA(hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefWindowProcW<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DefWindowProcW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    DefWindowProcW(hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeferWindowPos<'a, P0, P1>(hwinposinfo: isize, hwnd: P0, hwndinsertafter: P1, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeferWindowPos(hwinposinfo: isize, hwnd: super::super::Foundation::HWND, hwndinsertafter: super::super::Foundation::HWND, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> isize;
    }
    DeferWindowPos(hwinposinfo, hwnd.into(), hwndinsertafter.into(), x, y, cx, cy, uflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteMenu<'a, P0>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteMenu(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL;
    }
    DeleteMenu(hmenu.into(), uposition, uflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeregisterShellHookWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeregisterShellHookWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    DeregisterShellHookWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyAcceleratorTable<'a, P0>(haccel: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HACCEL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyAcceleratorTable(haccel: HACCEL) -> super::super::Foundation::BOOL;
    }
    DestroyAcceleratorTable(haccel.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyCaret() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyCaret() -> super::super::Foundation::BOOL;
    }
    DestroyCaret()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyCursor<'a, P0>(hcursor: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCURSOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyCursor(hcursor: HCURSOR) -> super::super::Foundation::BOOL;
    }
    DestroyCursor(hcursor.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyIcon<'a, P0>(hicon: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HICON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyIcon(hicon: HICON) -> super::super::Foundation::BOOL;
    }
    DestroyIcon(hicon.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn DestroyIndexedResults<'a, P0>(resourceuri: P0, qualifiers: &[IndexedResourceQualifier])
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyIndexedResults(resourceuri: ::windows::core::PCWSTR, qualifiercount: u32, qualifiers: *const IndexedResourceQualifier);
    }
    DestroyIndexedResults(resourceuri.into(), qualifiers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(qualifiers)))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyMenu<'a, P0>(hmenu: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyMenu(hmenu: HMENU) -> super::super::Foundation::BOOL;
    }
    DestroyMenu(hmenu.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn DestroyResourceIndexer(resourceindexer: *const ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyResourceIndexer(resourceindexer: *const ::core::ffi::c_void);
    }
    DestroyResourceIndexer(::core::mem::transmute(resourceindexer))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DestroyWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    DestroyWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DialogBoxIndirectParamA<'a, P0, P1, P2>(hinstance: P0, hdialogtemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DialogBoxIndirectParamA(hinstance: super::super::Foundation::HINSTANCE, hdialogtemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    }
    DialogBoxIndirectParamA(hinstance.into(), ::core::mem::transmute(hdialogtemplate), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DialogBoxIndirectParamW<'a, P0, P1, P2>(hinstance: P0, hdialogtemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DialogBoxIndirectParamW(hinstance: super::super::Foundation::HINSTANCE, hdialogtemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    }
    DialogBoxIndirectParamW(hinstance.into(), ::core::mem::transmute(hdialogtemplate), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DialogBoxParamA<'a, P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DialogBoxParamA(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: ::windows::core::PCSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    }
    DialogBoxParamA(hinstance.into(), lptemplatename.into(), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DialogBoxParamW<'a, P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
    P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DialogBoxParamW(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: ::windows::core::PCWSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: *mut ::core::ffi::c_void, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    }
    DialogBoxParamW(hinstance.into(), lptemplatename.into(), hwndparent.into(), ::core::mem::transmute(lpdialogfunc), dwinitparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn DisableProcessWindowsGhosting() {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DisableProcessWindowsGhosting();
    }
    DisableProcessWindowsGhosting()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DispatchMessageA(lpmsg: *const MSG) -> super::super::Foundation::LRESULT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DispatchMessageA(lpmsg: *const MSG) -> super::super::Foundation::LRESULT;
    }
    DispatchMessageA(::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DispatchMessageW(lpmsg: *const MSG) -> super::super::Foundation::LRESULT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DispatchMessageW(lpmsg: *const MSG) -> super::super::Foundation::LRESULT;
    }
    DispatchMessageW(::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DragObject<'a, P0, P1, P2>(hwndparent: P0, hwndfrom: P1, fmt: u32, data: usize, hcur: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<HCURSOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DragObject(hwndparent: super::super::Foundation::HWND, hwndfrom: super::super::Foundation::HWND, fmt: u32, data: usize, hcur: HCURSOR) -> u32;
    }
    DragObject(hwndparent.into(), hwndfrom.into(), fmt, data, hcur.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawIcon<'a, P0, P1>(hdc: P0, x: i32, y: i32, hicon: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P1: ::std::convert::Into<HICON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawIcon(hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, hicon: HICON) -> super::super::Foundation::BOOL;
    }
    DrawIcon(hdc.into(), x, y, hicon.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawIconEx<'a, P0, P1, P2>(hdc: P0, xleft: i32, ytop: i32, hicon: P1, cxwidth: i32, cywidth: i32, istepifanicur: u32, hbrflickerfreedraw: P2, diflags: DI_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P1: ::std::convert::Into<HICON>,
    P2: ::std::convert::Into<super::super::Graphics::Gdi::HBRUSH>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawIconEx(hdc: super::super::Graphics::Gdi::HDC, xleft: i32, ytop: i32, hicon: HICON, cxwidth: i32, cywidth: i32, istepifanicur: u32, hbrflickerfreedraw: super::super::Graphics::Gdi::HBRUSH, diflags: DI_FLAGS) -> super::super::Foundation::BOOL;
    }
    DrawIconEx(hdc.into(), xleft, ytop, hicon.into(), cxwidth, cywidth, istepifanicur, hbrflickerfreedraw.into(), diflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawMenuBar<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawMenuBar(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    DrawMenuBar(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EC_LEFTMARGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EC_RIGHTMARGIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EC_USEFONTINFO: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EDD_GET_DEVICE_INTERFACE_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDIT_CONTROL_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT: EDIT_CONTROL_FEATURE = EDIT_CONTROL_FEATURE(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS: EDIT_CONTROL_FEATURE = EDIT_CONTROL_FEATURE(1i32);
impl ::core::marker::Copy for EDIT_CONTROL_FEATURE {}
impl ::core::clone::Clone for EDIT_CONTROL_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDIT_CONTROL_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EDIT_CONTROL_FEATURE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EDIT_CONTROL_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDIT_CONTROL_FEATURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EDS_RAWMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EDS_ROTATEDMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EIMES_CANCELCOMPSTRINFOCUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EIMES_COMPLETECOMPSTRKILLFOCUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EIMES_GETCOMPSTRATONCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EMSIS_COMPOSITIONSTRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ENDSESSION_CLOSEAPP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ENDSESSION_CRITICAL: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ENDSESSION_LOGOFF: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_AFTER_PASTE: u32 = 2049u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_ALIGN_LTR_EC: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_ALIGN_RTL_EC: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_BEFORE_PASTE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_CHANGE: u32 = 768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_ERRSPACE: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_HSCROLL: u32 = 1537u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_KILLFOCUS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_MAXTEXT: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_SETFOCUS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_UPDATE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EN_VSCROLL: u32 = 1538u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_AUTOHSCROLL: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_AUTOVSCROLL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_CENTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_LEFT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_LOWERCASE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_MULTILINE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_NOHIDESEL: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_NUMBER: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_OEMCONVERT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_PASSWORD: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_READONLY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_UPPERCASE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ES_WANTRETURN: i32 = 4096i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENTMSG {
    pub message: u32,
    pub paramL: u32,
    pub paramH: u32,
    pub time: u32,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENTMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENTMSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENTMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTMSG").field("message", &self.message).field("paramL", &self.paramL).field("paramH", &self.paramH).field("time", &self.time).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENTMSG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENTMSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENTMSG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENTMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENTMSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_AIA_END: u32 = 45055u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_AIA_START: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_CARET: u32 = 16385u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_END: u32 = 16639u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_END_APPLICATION: u32 = 16391u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_LAYOUT: u32 = 16389u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_START_APPLICATION: u32 = 16390u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_UPDATE_REGION: u32 = 16386u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_UPDATE_SCROLL: u32 = 16388u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_CONSOLE_UPDATE_SIMPLE: u32 = 16387u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_MAX: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_ACCELERATORCHANGE: u32 = 32786u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_CLOAKED: u32 = 32791u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_CONTENTSCROLLED: u32 = 32789u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_CREATE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DEFACTIONCHANGE: u32 = 32785u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DESCRIPTIONCHANGE: u32 = 32781u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DESTROY: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DRAGCANCEL: u32 = 32802u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DRAGCOMPLETE: u32 = 32803u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DRAGDROPPED: u32 = 32806u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DRAGENTER: u32 = 32804u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DRAGLEAVE: u32 = 32805u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_DRAGSTART: u32 = 32801u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_END: u32 = 33023u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_FOCUS: u32 = 32773u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_HELPCHANGE: u32 = 32784u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_HIDE: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED: u32 = 32800u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_IME_CHANGE: u32 = 32809u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_IME_HIDE: u32 = 32808u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_IME_SHOW: u32 = 32807u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_INVOKED: u32 = 32787u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_LIVEREGIONCHANGED: u32 = 32793u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_LOCATIONCHANGE: u32 = 32779u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_NAMECHANGE: u32 = 32780u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_PARENTCHANGE: u32 = 32783u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_REORDER: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_SELECTION: u32 = 32774u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_SELECTIONADD: u32 = 32775u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_SELECTIONREMOVE: u32 = 32776u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_SELECTIONWITHIN: u32 = 32777u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_SHOW: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_STATECHANGE: u32 = 32778u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: u32 = 32816u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_TEXTSELECTIONCHANGED: u32 = 32788u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_UNCLOAKED: u32 = 32792u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OBJECT_VALUECHANGE: u32 = 32782u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OEM_DEFINED_END: u32 = 511u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_OEM_DEFINED_START: u32 = 257u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_ALERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_ARRANGMENTPREVIEW: u32 = 32790u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_CAPTUREEND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_CAPTURESTART: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_CONTEXTHELPEND: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_CONTEXTHELPSTART: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_DESKTOPSWITCH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_DIALOGEND: u32 = 17u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_DIALOGSTART: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_DRAGDROPEND: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_DRAGDROPSTART: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_END: u32 = 255u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_FOREGROUND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_IME_KEY_NOTIFICATION: u32 = 41u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MENUEND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MENUPOPUPEND: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MENUPOPUPSTART: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MENUSTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MINIMIZEEND: u32 = 23u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MINIMIZESTART: u32 = 22u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MOVESIZEEND: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_MOVESIZESTART: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SCROLLINGEND: u32 = 19u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SCROLLINGSTART: u32 = 18u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SWITCHEND: u32 = 21u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SWITCHER_APPDROPPED: u32 = 38u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SWITCHER_APPGRABBED: u32 = 36u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SWITCHER_APPOVERTARGET: u32 = 37u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SWITCHER_CANCELLED: u32 = 39u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_SYSTEM_SWITCHSTART: u32 = 20u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_UIA_EVENTID_END: u32 = 20223u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_UIA_EVENTID_START: u32 = 19968u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_UIA_PROPID_END: u32 = 30207u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EVENT_UIA_PROPID_START: u32 = 29952u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EWX_ARSO: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EWX_BOOTOPTIONS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EWX_CHECK_SAFE_FOR_SERVER: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EWX_FORCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EWX_FORCEIFHUNG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EWX_QUICKRESOLVE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const EWX_SYSTEM_INITIATED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableMenuItem<'a, P0>(hmenu: P0, uidenableitem: u32, uenable: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnableMenuItem(hmenu: HMENU, uidenableitem: u32, uenable: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL;
    }
    EnableMenuItem(hmenu.into(), uidenableitem, uenable)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndDeferWindowPos(hwinposinfo: isize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EndDeferWindowPos(hwinposinfo: isize) -> super::super::Foundation::BOOL;
    }
    EndDeferWindowPos(hwinposinfo)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndDialog<'a, P0>(hdlg: P0, nresult: isize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EndDialog(hdlg: super::super::Foundation::HWND, nresult: isize) -> super::super::Foundation::BOOL;
    }
    EndDialog(hdlg.into(), nresult)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndMenu() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EndMenu() -> super::super::Foundation::BOOL;
    }
    EndMenu()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumChildWindows<'a, P0, P1>(hwndparent: P0, lpenumfunc: WNDENUMPROC, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumChildWindows(hwndparent: super::super::Foundation::HWND, lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumChildWindows(hwndparent.into(), ::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPropsA<'a, P0>(hwnd: P0, lpenumfunc: PROPENUMPROCA) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumPropsA(hwnd: super::super::Foundation::HWND, lpenumfunc: *mut ::core::ffi::c_void) -> i32;
    }
    EnumPropsA(hwnd.into(), ::core::mem::transmute(lpenumfunc))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPropsExA<'a, P0, P1>(hwnd: P0, lpenumfunc: PROPENUMPROCEXA, lparam: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumPropsExA(hwnd: super::super::Foundation::HWND, lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    EnumPropsExA(hwnd.into(), ::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPropsExW<'a, P0, P1>(hwnd: P0, lpenumfunc: PROPENUMPROCEXW, lparam: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumPropsExW(hwnd: super::super::Foundation::HWND, lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    EnumPropsExW(hwnd.into(), ::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPropsW<'a, P0>(hwnd: P0, lpenumfunc: PROPENUMPROCW) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumPropsW(hwnd: super::super::Foundation::HWND, lpenumfunc: *mut ::core::ffi::c_void) -> i32;
    }
    EnumPropsW(hwnd.into(), ::core::mem::transmute(lpenumfunc))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumThreadWindows<'a, P0>(dwthreadid: u32, lpfn: WNDENUMPROC, lparam: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumThreadWindows(dwthreadid: u32, lpfn: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumThreadWindows(dwthreadid, ::core::mem::transmute(lpfn), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumWindows<'a, P0>(lpenumfunc: WNDENUMPROC, lparam: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumWindows(lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumWindows(::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FALT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FAPPCOMMAND_KEY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FAPPCOMMAND_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FAPPCOMMAND_MOUSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FAPPCOMMAND_OEM: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FCONTROL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FE_FONTSMOOTHINGCLEARTYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FE_FONTSMOOTHINGORIENTATIONBGR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FE_FONTSMOOTHINGORIENTATIONRGB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FE_FONTSMOOTHINGSTANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FKF_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FKF_CLICKON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FKF_CONFIRMHOTKEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FKF_FILTERKEYSON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FKF_HOTKEYACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FKF_HOTKEYSOUND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FKF_INDICATOR: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FLASHWINFO {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub dwFlags: FLASHWINFO_FLAGS,
    pub uCount: u32,
    pub dwTimeout: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FLASHWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FLASHWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FLASHWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLASHWINFO").field("cbSize", &self.cbSize).field("hwnd", &self.hwnd).field("dwFlags", &self.dwFlags).field("uCount", &self.uCount).field("dwTimeout", &self.dwTimeout).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FLASHWINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FLASHWINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLASHWINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FLASHWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FLASHWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FLASHWINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FLASHW_ALL: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FLASHW_CAPTION: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FLASHW_STOP: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FLASHW_TIMER: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FLASHW_TIMERNOFG: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(12u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FLASHW_TRAY: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(2u32);
impl ::core::marker::Copy for FLASHWINFO_FLAGS {}
impl ::core::clone::Clone for FLASHWINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FLASHWINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FLASHWINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FLASHWINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLASHWINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FLASHWINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FLASHWINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FLASHWINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FLASHWINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FLASHWINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FNOINVERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FOREGROUND_WINDOW_LOCK_CODE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LSFW_LOCK: FOREGROUND_WINDOW_LOCK_CODE = FOREGROUND_WINDOW_LOCK_CODE(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LSFW_UNLOCK: FOREGROUND_WINDOW_LOCK_CODE = FOREGROUND_WINDOW_LOCK_CODE(2u32);
impl ::core::marker::Copy for FOREGROUND_WINDOW_LOCK_CODE {}
impl ::core::clone::Clone for FOREGROUND_WINDOW_LOCK_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FOREGROUND_WINDOW_LOCK_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FOREGROUND_WINDOW_LOCK_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FOREGROUND_WINDOW_LOCK_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOREGROUND_WINDOW_LOCK_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FSHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const FVIRTKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindWindowA<'a, P0, P1>(lpclassname: P0, lpwindowname: P1) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindWindowA(lpclassname: ::windows::core::PCSTR, lpwindowname: ::windows::core::PCSTR) -> super::super::Foundation::HWND;
    }
    FindWindowA(lpclassname.into(), lpwindowname.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindWindowExA<'a, P0, P1, P2, P3>(hwndparent: P0, hwndchildafter: P1, lpszclass: P2, lpszwindow: P3) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindWindowExA(hwndparent: super::super::Foundation::HWND, hwndchildafter: super::super::Foundation::HWND, lpszclass: ::windows::core::PCSTR, lpszwindow: ::windows::core::PCSTR) -> super::super::Foundation::HWND;
    }
    FindWindowExA(hwndparent.into(), hwndchildafter.into(), lpszclass.into(), lpszwindow.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindWindowExW<'a, P0, P1, P2, P3>(hwndparent: P0, hwndchildafter: P1, lpszclass: P2, lpszwindow: P3) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindWindowExW(hwndparent: super::super::Foundation::HWND, hwndchildafter: super::super::Foundation::HWND, lpszclass: ::windows::core::PCWSTR, lpszwindow: ::windows::core::PCWSTR) -> super::super::Foundation::HWND;
    }
    FindWindowExW(hwndparent.into(), hwndchildafter.into(), lpszclass.into(), lpszwindow.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindWindowW<'a, P0, P1>(lpclassname: P0, lpwindowname: P1) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindWindowW(lpclassname: ::windows::core::PCWSTR, lpwindowname: ::windows::core::PCWSTR) -> super::super::Foundation::HWND;
    }
    FindWindowW(lpclassname.into(), lpwindowname.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlashWindow<'a, P0, P1>(hwnd: P0, binvert: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FlashWindow(hwnd: super::super::Foundation::HWND, binvert: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    FlashWindow(hwnd.into(), binvert.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlashWindowEx(pfwi: *const FLASHWINFO) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FlashWindowEx(pfwi: *const FLASHWINFO) -> super::super::Foundation::BOOL;
    }
    FlashWindowEx(::core::mem::transmute(pfwi))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCF_INCLUDE_ANCESTORS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GDI_IMAGE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IMAGE_BITMAP: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IMAGE_CURSOR: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IMAGE_ICON: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(1u32);
impl ::core::marker::Copy for GDI_IMAGE_TYPE {}
impl ::core::clone::Clone for GDI_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GDI_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GDI_IMAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GDI_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GDI_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTURECONFIGMAXCOUNT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTUREVISUALIZATION_DOUBLETAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTUREVISUALIZATION_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTUREVISUALIZATION_ON: u32 = 31u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTUREVISUALIZATION_PRESSANDHOLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTUREVISUALIZATION_PRESSANDTAP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTUREVISUALIZATION_RIGHTTAP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GESTUREVISUALIZATION_TAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_ANCESTOR_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GA_PARENT: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GA_ROOT: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GA_ROOTOWNER: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(3u32);
impl ::core::marker::Copy for GET_ANCESTOR_FLAGS {}
impl ::core::clone::Clone for GET_ANCESTOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_ANCESTOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_ANCESTOR_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_ANCESTOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_ANCESTOR_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_CLASS_LONG_INDEX(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCW_ATOM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-32i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_CBCLSEXTRA: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-20i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_CBWNDEXTRA: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-18i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_HBRBACKGROUND: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-10i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_HCURSOR: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-12i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_HICON: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-14i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_HICONSM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-34i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_HMODULE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-16i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_MENUNAME: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-8i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_STYLE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-26i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCL_WNDPROC: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-24i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCLP_HBRBACKGROUND: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-10i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCLP_HCURSOR: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-12i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCLP_HICON: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-14i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCLP_HICONSM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-34i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCLP_HMODULE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-16i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCLP_MENUNAME: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-8i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GCLP_WNDPROC: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-24i32);
impl ::core::marker::Copy for GET_CLASS_LONG_INDEX {}
impl ::core::clone::Clone for GET_CLASS_LONG_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_CLASS_LONG_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_CLASS_LONG_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_CLASS_LONG_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CLASS_LONG_INDEX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_MENU_DEFAULT_ITEM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GMDI_GOINTOPOPUPS: GET_MENU_DEFAULT_ITEM_FLAGS = GET_MENU_DEFAULT_ITEM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GMDI_USEDISABLED: GET_MENU_DEFAULT_ITEM_FLAGS = GET_MENU_DEFAULT_ITEM_FLAGS(1u32);
impl ::core::marker::Copy for GET_MENU_DEFAULT_ITEM_FLAGS {}
impl ::core::clone::Clone for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_MENU_DEFAULT_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_WINDOW_CMD(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_CHILD: GET_WINDOW_CMD = GET_WINDOW_CMD(5u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_ENABLEDPOPUP: GET_WINDOW_CMD = GET_WINDOW_CMD(6u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_HWNDFIRST: GET_WINDOW_CMD = GET_WINDOW_CMD(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_HWNDLAST: GET_WINDOW_CMD = GET_WINDOW_CMD(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_HWNDNEXT: GET_WINDOW_CMD = GET_WINDOW_CMD(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_HWNDPREV: GET_WINDOW_CMD = GET_WINDOW_CMD(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_OWNER: GET_WINDOW_CMD = GET_WINDOW_CMD(4u32);
impl ::core::marker::Copy for GET_WINDOW_CMD {}
impl ::core::clone::Clone for GET_WINDOW_CMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_WINDOW_CMD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_WINDOW_CMD {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_WINDOW_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_WINDOW_CMD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GF_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GF_END: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GF_INERTIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GIDC_ARRIVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GIDC_REMOVAL: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GUITHREADINFO {
    pub cbSize: u32,
    pub flags: GUITHREADINFO_FLAGS,
    pub hwndActive: super::super::Foundation::HWND,
    pub hwndFocus: super::super::Foundation::HWND,
    pub hwndCapture: super::super::Foundation::HWND,
    pub hwndMenuOwner: super::super::Foundation::HWND,
    pub hwndMoveSize: super::super::Foundation::HWND,
    pub hwndCaret: super::super::Foundation::HWND,
    pub rcCaret: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GUITHREADINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GUITHREADINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GUITHREADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GUITHREADINFO").field("cbSize", &self.cbSize).field("flags", &self.flags).field("hwndActive", &self.hwndActive).field("hwndFocus", &self.hwndFocus).field("hwndCapture", &self.hwndCapture).field("hwndMenuOwner", &self.hwndMenuOwner).field("hwndMoveSize", &self.hwndMoveSize).field("hwndCaret", &self.hwndCaret).field("rcCaret", &self.rcCaret).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GUITHREADINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GUITHREADINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GUITHREADINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GUITHREADINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GUITHREADINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GUITHREADINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GUI_CARETBLINKING: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GUI_INMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GUI_INMOVESIZE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GUI_POPUPMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GUI_SYSTEMMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(8u32);
impl ::core::marker::Copy for GUITHREADINFO_FLAGS {}
impl ::core::clone::Clone for GUITHREADINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GUITHREADINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GUITHREADINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GUITHREADINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GUITHREADINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GUITHREADINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GUITHREADINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GUI_16BITTASK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWFS_INCLUDE_ANCESTORS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GW_MAX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAltTabInfoA<'a, P0>(hwnd: P0, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAltTabInfoA(hwnd: super::super::Foundation::HWND, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: ::windows::core::PSTR, cchitemtext: u32) -> super::super::Foundation::BOOL;
    }
    GetAltTabInfoA(hwnd.into(), iitem, ::core::mem::transmute(pati), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszitemtext)), pszitemtext.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAltTabInfoW<'a, P0>(hwnd: P0, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: &mut [u16]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAltTabInfoW(hwnd: super::super::Foundation::HWND, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: ::windows::core::PWSTR, cchitemtext: u32) -> super::super::Foundation::BOOL;
    }
    GetAltTabInfoW(hwnd.into(), iitem, ::core::mem::transmute(pati), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszitemtext)), pszitemtext.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAncestor<'a, P0>(hwnd: P0, gaflags: GET_ANCESTOR_FLAGS) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAncestor(hwnd: super::super::Foundation::HWND, gaflags: GET_ANCESTOR_FLAGS) -> super::super::Foundation::HWND;
    }
    GetAncestor(hwnd.into(), gaflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetCaretBlinkTime() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCaretBlinkTime() -> u32;
    }
    GetCaretBlinkTime()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCaretPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCaretPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetCaretPos(::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetClassInfoA<'a, P0, P1>(hinstance: P0, lpclassname: P1, lpwndclass: *mut WNDCLASSA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassInfoA(hinstance: super::super::Foundation::HINSTANCE, lpclassname: ::windows::core::PCSTR, lpwndclass: *mut WNDCLASSA) -> super::super::Foundation::BOOL;
    }
    GetClassInfoA(hinstance.into(), lpclassname.into(), ::core::mem::transmute(lpwndclass))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetClassInfoExA<'a, P0, P1>(hinstance: P0, lpszclass: P1, lpwcx: *mut WNDCLASSEXA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassInfoExA(hinstance: super::super::Foundation::HINSTANCE, lpszclass: ::windows::core::PCSTR, lpwcx: *mut WNDCLASSEXA) -> super::super::Foundation::BOOL;
    }
    GetClassInfoExA(hinstance.into(), lpszclass.into(), ::core::mem::transmute(lpwcx))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetClassInfoExW<'a, P0, P1>(hinstance: P0, lpszclass: P1, lpwcx: *mut WNDCLASSEXW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassInfoExW(hinstance: super::super::Foundation::HINSTANCE, lpszclass: ::windows::core::PCWSTR, lpwcx: *mut WNDCLASSEXW) -> super::super::Foundation::BOOL;
    }
    GetClassInfoExW(hinstance.into(), lpszclass.into(), ::core::mem::transmute(lpwcx))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetClassInfoW<'a, P0, P1>(hinstance: P0, lpclassname: P1, lpwndclass: *mut WNDCLASSW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassInfoW(hinstance: super::super::Foundation::HINSTANCE, lpclassname: ::windows::core::PCWSTR, lpwndclass: *mut WNDCLASSW) -> super::super::Foundation::BOOL;
    }
    GetClassInfoW(hinstance.into(), lpclassname.into(), ::core::mem::transmute(lpwndclass))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassLongA<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassLongA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> u32;
    }
    GetClassLongA(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassLongPtrA<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> usize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassLongPtrA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> usize;
    }
    GetClassLongPtrA(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassLongPtrW<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> usize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassLongPtrW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> usize;
    }
    GetClassLongPtrW(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassLongW<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassLongW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> u32;
    }
    GetClassLongW(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassNameA<'a, P0>(hwnd: P0, lpclassname: &mut [u8]) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassNameA(hwnd: super::super::Foundation::HWND, lpclassname: ::windows::core::PSTR, nmaxcount: i32) -> i32;
    }
    GetClassNameA(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpclassname)), lpclassname.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassNameW<'a, P0>(hwnd: P0, lpclassname: &mut [u16]) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassNameW(hwnd: super::super::Foundation::HWND, lpclassname: ::windows::core::PWSTR, nmaxcount: i32) -> i32;
    }
    GetClassNameW(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpclassname)), lpclassname.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassWord<'a, P0>(hwnd: P0, nindex: i32) -> u16
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClassWord(hwnd: super::super::Foundation::HWND, nindex: i32) -> u16;
    }
    GetClassWord(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClientRect<'a, P0>(hwnd: P0, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClientRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    GetClientRect(hwnd.into(), ::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipCursor(lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipCursor(lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    GetClipCursor(::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetCursor() -> HCURSOR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCursor() -> HCURSOR;
    }
    GetCursor()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCursorInfo(pci: *mut CURSORINFO) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCursorInfo(pci: *mut CURSORINFO) -> super::super::Foundation::BOOL;
    }
    GetCursorInfo(::core::mem::transmute(pci))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCursorPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCursorPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetCursorPos(::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDesktopWindow() -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDesktopWindow() -> super::super::Foundation::HWND;
    }
    GetDesktopWindow()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetDialogBaseUnits() -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDialogBaseUnits() -> i32;
    }
    GetDialogBaseUnits()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDlgCtrlID<'a, P0>(hwnd: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDlgCtrlID(hwnd: super::super::Foundation::HWND) -> i32;
    }
    GetDlgCtrlID(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDlgItem<'a, P0>(hdlg: P0, niddlgitem: i32) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDlgItem(hdlg: super::super::Foundation::HWND, niddlgitem: i32) -> super::super::Foundation::HWND;
    }
    GetDlgItem(hdlg.into(), niddlgitem)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDlgItemInt<'a, P0, P1>(hdlg: P0, niddlgitem: i32, lptranslated: *mut super::super::Foundation::BOOL, bsigned: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDlgItemInt(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lptranslated: *mut super::super::Foundation::BOOL, bsigned: super::super::Foundation::BOOL) -> u32;
    }
    GetDlgItemInt(hdlg.into(), niddlgitem, ::core::mem::transmute(lptranslated), bsigned.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDlgItemTextA<'a, P0>(hdlg: P0, niddlgitem: i32, lpstring: &mut [u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDlgItemTextA(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: ::windows::core::PSTR, cchmax: i32) -> u32;
    }
    GetDlgItemTextA(hdlg.into(), niddlgitem, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDlgItemTextW<'a, P0>(hdlg: P0, niddlgitem: i32, lpstring: &mut [u16]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDlgItemTextW(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: ::windows::core::PWSTR, cchmax: i32) -> u32;
    }
    GetDlgItemTextW(hdlg.into(), niddlgitem, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetForegroundWindow() -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetForegroundWindow() -> super::super::Foundation::HWND;
    }
    GetForegroundWindow()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGUIThreadInfo(idthread: u32, pgui: *mut GUITHREADINFO) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGUIThreadInfo(idthread: u32, pgui: *mut GUITHREADINFO) -> super::super::Foundation::BOOL;
    }
    GetGUIThreadInfo(idthread, ::core::mem::transmute(pgui))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetIconInfo<'a, P0>(hicon: P0, piconinfo: *mut ICONINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HICON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetIconInfo(hicon: HICON, piconinfo: *mut ICONINFO) -> super::super::Foundation::BOOL;
    }
    GetIconInfo(hicon.into(), ::core::mem::transmute(piconinfo))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetIconInfoExA<'a, P0>(hicon: P0, piconinfo: *mut ICONINFOEXA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HICON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetIconInfoExA(hicon: HICON, piconinfo: *mut ICONINFOEXA) -> super::super::Foundation::BOOL;
    }
    GetIconInfoExA(hicon.into(), ::core::mem::transmute(piconinfo))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetIconInfoExW<'a, P0>(hicon: P0, piconinfo: *mut ICONINFOEXW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HICON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetIconInfoExW(hicon: HICON, piconinfo: *mut ICONINFOEXW) -> super::super::Foundation::BOOL;
    }
    GetIconInfoExW(hicon.into(), ::core::mem::transmute(piconinfo))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInputState() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetInputState() -> super::super::Foundation::BOOL;
    }
    GetInputState()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLastActivePopup<'a, P0>(hwnd: P0) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetLastActivePopup(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    }
    GetLastActivePopup(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLayeredWindowAttributes<'a, P0>(hwnd: P0, pcrkey: *mut u32, pbalpha: *mut u8, pdwflags: *mut LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetLayeredWindowAttributes(hwnd: super::super::Foundation::HWND, pcrkey: *mut u32, pbalpha: *mut u8, pdwflags: *mut LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation::BOOL;
    }
    GetLayeredWindowAttributes(hwnd.into(), ::core::mem::transmute(pcrkey), ::core::mem::transmute(pbalpha), ::core::mem::transmute(pdwflags))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMenu<'a, P0>(hwnd: P0) -> HMENU
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenu(hwnd: super::super::Foundation::HWND) -> HMENU;
    }
    GetMenu(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMenuBarInfo<'a, P0>(hwnd: P0, idobject: OBJECT_IDENTIFIER, iditem: i32, pmbi: *mut MENUBARINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuBarInfo(hwnd: super::super::Foundation::HWND, idobject: OBJECT_IDENTIFIER, iditem: i32, pmbi: *mut MENUBARINFO) -> super::super::Foundation::BOOL;
    }
    GetMenuBarInfo(hwnd.into(), idobject, iditem, ::core::mem::transmute(pmbi))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMenuCheckMarkDimensions() -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuCheckMarkDimensions() -> i32;
    }
    GetMenuCheckMarkDimensions()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMenuDefaultItem<'a, P0>(hmenu: P0, fbypos: u32, gmdiflags: GET_MENU_DEFAULT_ITEM_FLAGS) -> u32
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuDefaultItem(hmenu: HMENU, fbypos: u32, gmdiflags: GET_MENU_DEFAULT_ITEM_FLAGS) -> u32;
    }
    GetMenuDefaultItem(hmenu.into(), fbypos, gmdiflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetMenuInfo<'a, P0>(param0: P0, param1: *mut MENUINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuInfo(param0: HMENU, param1: *mut MENUINFO) -> super::super::Foundation::BOOL;
    }
    GetMenuInfo(param0.into(), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMenuItemCount<'a, P0>(hmenu: P0) -> i32
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuItemCount(hmenu: HMENU) -> i32;
    }
    GetMenuItemCount(hmenu.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMenuItemID<'a, P0>(hmenu: P0, npos: i32) -> u32
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuItemID(hmenu: HMENU, npos: i32) -> u32;
    }
    GetMenuItemID(hmenu.into(), npos)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetMenuItemInfoA<'a, P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmii: *mut MENUITEMINFOA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuItemInfoA(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmii: *mut MENUITEMINFOA) -> super::super::Foundation::BOOL;
    }
    GetMenuItemInfoA(hmenu.into(), item, fbyposition.into(), ::core::mem::transmute(lpmii))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetMenuItemInfoW<'a, P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmii: *mut MENUITEMINFOW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuItemInfoW(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmii: *mut MENUITEMINFOW) -> super::super::Foundation::BOOL;
    }
    GetMenuItemInfoW(hmenu.into(), item, fbyposition.into(), ::core::mem::transmute(lpmii))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMenuItemRect<'a, P0, P1>(hwnd: P0, hmenu: P1, uitem: u32, lprcitem: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuItemRect(hwnd: super::super::Foundation::HWND, hmenu: HMENU, uitem: u32, lprcitem: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    GetMenuItemRect(hwnd.into(), hmenu.into(), uitem, ::core::mem::transmute(lprcitem))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMenuState<'a, P0>(hmenu: P0, uid: u32, uflags: MENU_ITEM_FLAGS) -> u32
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuState(hmenu: HMENU, uid: u32, uflags: MENU_ITEM_FLAGS) -> u32;
    }
    GetMenuState(hmenu.into(), uid, uflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMenuStringA<'a, P0>(hmenu: P0, uiditem: u32, lpstring: &mut [u8], flags: MENU_ITEM_FLAGS) -> i32
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuStringA(hmenu: HMENU, uiditem: u32, lpstring: ::windows::core::PSTR, cchmax: i32, flags: MENU_ITEM_FLAGS) -> i32;
    }
    GetMenuStringA(hmenu.into(), uiditem, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMenuStringW<'a, P0>(hmenu: P0, uiditem: u32, lpstring: &mut [u16], flags: MENU_ITEM_FLAGS) -> i32
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMenuStringW(hmenu: HMENU, uiditem: u32, lpstring: ::windows::core::PWSTR, cchmax: i32, flags: MENU_ITEM_FLAGS) -> i32;
    }
    GetMenuStringW(hmenu.into(), uiditem, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMessageA<'a, P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMessageA(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL;
    }
    GetMessageA(::core::mem::transmute(lpmsg), hwnd.into(), wmsgfiltermin, wmsgfiltermax)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMessageExtraInfo() -> super::super::Foundation::LPARAM {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMessageExtraInfo() -> super::super::Foundation::LPARAM;
    }
    GetMessageExtraInfo()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMessagePos() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMessagePos() -> u32;
    }
    GetMessagePos()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetMessageTime() -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMessageTime() -> i32;
    }
    GetMessageTime()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMessageW<'a, P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMessageW(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL;
    }
    GetMessageW(::core::mem::transmute(lpmsg), hwnd.into(), wmsgfiltermin, wmsgfiltermax)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNextDlgGroupItem<'a, P0, P1, P2>(hdlg: P0, hctl: P1, bprevious: P2) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetNextDlgGroupItem(hdlg: super::super::Foundation::HWND, hctl: super::super::Foundation::HWND, bprevious: super::super::Foundation::BOOL) -> super::super::Foundation::HWND;
    }
    GetNextDlgGroupItem(hdlg.into(), hctl.into(), bprevious.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNextDlgTabItem<'a, P0, P1, P2>(hdlg: P0, hctl: P1, bprevious: P2) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetNextDlgTabItem(hdlg: super::super::Foundation::HWND, hctl: super::super::Foundation::HWND, bprevious: super::super::Foundation::BOOL) -> super::super::Foundation::HWND;
    }
    GetNextDlgTabItem(hdlg.into(), hctl.into(), bprevious.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetParent<'a, P0>(hwnd: P0) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetParent(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    }
    GetParent(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPhysicalCursorPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPhysicalCursorPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetPhysicalCursorPos(::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDefaultLayout(pdwdefaultlayout: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetProcessDefaultLayout(pdwdefaultlayout: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetProcessDefaultLayout(::core::mem::transmute(pdwdefaultlayout))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPropA<'a, P0, P1>(hwnd: P0, lpstring: P1) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPropA(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
    }
    GetPropA(hwnd.into(), lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPropW<'a, P0, P1>(hwnd: P0, lpstring: P1) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPropW(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
    }
    GetPropW(hwnd.into(), lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetQueueStatus(flags: QUEUE_STATUS_FLAGS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetQueueStatus(flags: QUEUE_STATUS_FLAGS) -> u32;
    }
    GetQueueStatus(flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetScrollBarInfo<'a, P0>(hwnd: P0, idobject: OBJECT_IDENTIFIER, psbi: *mut SCROLLBARINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetScrollBarInfo(hwnd: super::super::Foundation::HWND, idobject: OBJECT_IDENTIFIER, psbi: *mut SCROLLBARINFO) -> super::super::Foundation::BOOL;
    }
    GetScrollBarInfo(hwnd.into(), idobject, ::core::mem::transmute(psbi))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetScrollInfo<'a, P0>(hwnd: P0, nbar: SCROLLBAR_CONSTANTS, lpsi: *mut SCROLLINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetScrollInfo(hwnd: super::super::Foundation::HWND, nbar: SCROLLBAR_CONSTANTS, lpsi: *mut SCROLLINFO) -> super::super::Foundation::BOOL;
    }
    GetScrollInfo(hwnd.into(), nbar, ::core::mem::transmute(lpsi))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetScrollPos<'a, P0>(hwnd: P0, nbar: SCROLLBAR_CONSTANTS) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetScrollPos(hwnd: super::super::Foundation::HWND, nbar: SCROLLBAR_CONSTANTS) -> i32;
    }
    GetScrollPos(hwnd.into(), nbar)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetScrollRange<'a, P0>(hwnd: P0, nbar: SCROLLBAR_CONSTANTS, lpminpos: *mut i32, lpmaxpos: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetScrollRange(hwnd: super::super::Foundation::HWND, nbar: SCROLLBAR_CONSTANTS, lpminpos: *mut i32, lpmaxpos: *mut i32) -> super::super::Foundation::BOOL;
    }
    GetScrollRange(hwnd.into(), nbar, ::core::mem::transmute(lpminpos), ::core::mem::transmute(lpmaxpos))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetShellWindow() -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetShellWindow() -> super::super::Foundation::HWND;
    }
    GetShellWindow()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetSubMenu<'a, P0>(hmenu: P0, npos: i32) -> HMENU
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSubMenu(hmenu: HMENU, npos: i32) -> HMENU;
    }
    GetSubMenu(hmenu.into(), npos)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetSysColor(nindex: SYS_COLOR_INDEX) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSysColor(nindex: SYS_COLOR_INDEX) -> u32;
    }
    GetSysColor(nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemMenu<'a, P0, P1>(hwnd: P0, brevert: P1) -> HMENU
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSystemMenu(hwnd: super::super::Foundation::HWND, brevert: super::super::Foundation::BOOL) -> HMENU;
    }
    GetSystemMenu(hwnd.into(), brevert.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn GetSystemMetrics(nindex: SYSTEM_METRICS_INDEX) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSystemMetrics(nindex: SYSTEM_METRICS_INDEX) -> i32;
    }
    GetSystemMetrics(nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTitleBarInfo<'a, P0>(hwnd: P0, pti: *mut TITLEBARINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTitleBarInfo(hwnd: super::super::Foundation::HWND, pti: *mut TITLEBARINFO) -> super::super::Foundation::BOOL;
    }
    GetTitleBarInfo(hwnd.into(), ::core::mem::transmute(pti))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTopWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTopWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    }
    GetTopWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindow<'a, P0>(hwnd: P0, ucmd: GET_WINDOW_CMD) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindow(hwnd: super::super::Foundation::HWND, ucmd: GET_WINDOW_CMD) -> super::super::Foundation::HWND;
    }
    GetWindow(hwnd.into(), ucmd)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDisplayAffinity<'a, P0>(hwnd: P0, pdwaffinity: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowDisplayAffinity(hwnd: super::super::Foundation::HWND, pdwaffinity: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetWindowDisplayAffinity(hwnd.into(), ::core::mem::transmute(pdwaffinity))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowInfo<'a, P0>(hwnd: P0, pwi: *mut WINDOWINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowInfo(hwnd: super::super::Foundation::HWND, pwi: *mut WINDOWINFO) -> super::super::Foundation::BOOL;
    }
    GetWindowInfo(hwnd.into(), ::core::mem::transmute(pwi))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowLongA<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowLongA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> i32;
    }
    GetWindowLongA(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowLongPtrA<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowLongPtrA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize;
    }
    GetWindowLongPtrA(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowLongPtrW<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowLongPtrW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize;
    }
    GetWindowLongPtrW(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowLongW<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowLongW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> i32;
    }
    GetWindowLongW(hwnd.into(), nindex)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowModuleFileNameA<'a, P0>(hwnd: P0, pszfilename: &mut [u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowModuleFileNameA(hwnd: super::super::Foundation::HWND, pszfilename: ::windows::core::PSTR, cchfilenamemax: u32) -> u32;
    }
    GetWindowModuleFileNameA(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszfilename)), pszfilename.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowModuleFileNameW<'a, P0>(hwnd: P0, pszfilename: &mut [u16]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowModuleFileNameW(hwnd: super::super::Foundation::HWND, pszfilename: ::windows::core::PWSTR, cchfilenamemax: u32) -> u32;
    }
    GetWindowModuleFileNameW(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszfilename)), pszfilename.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowPlacement<'a, P0>(hwnd: P0, lpwndpl: *mut WINDOWPLACEMENT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowPlacement(hwnd: super::super::Foundation::HWND, lpwndpl: *mut WINDOWPLACEMENT) -> super::super::Foundation::BOOL;
    }
    GetWindowPlacement(hwnd.into(), ::core::mem::transmute(lpwndpl))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowRect<'a, P0>(hwnd: P0, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    GetWindowRect(hwnd.into(), ::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowTextA<'a, P0>(hwnd: P0, lpstring: &mut [u8]) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowTextA(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PSTR, nmaxcount: i32) -> i32;
    }
    GetWindowTextA(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowTextLengthA<'a, P0>(hwnd: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowTextLengthA(hwnd: super::super::Foundation::HWND) -> i32;
    }
    GetWindowTextLengthA(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowTextLengthW<'a, P0>(hwnd: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowTextLengthW(hwnd: super::super::Foundation::HWND) -> i32;
    }
    GetWindowTextLengthW(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowTextW<'a, P0>(hwnd: P0, lpstring: &mut [u16]) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowTextW(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PWSTR, nmaxcount: i32) -> i32;
    }
    GetWindowTextW(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowThreadProcessId<'a, P0>(hwnd: P0, lpdwprocessid: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowThreadProcessId(hwnd: super::super::Foundation::HWND, lpdwprocessid: *mut u32) -> u32;
    }
    GetWindowThreadProcessId(hwnd.into(), ::core::mem::transmute(lpdwprocessid))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowWord<'a, P0>(hwnd: P0, nindex: i32) -> u16
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowWord(hwnd: super::super::Foundation::HWND, nindex: i32) -> u16;
    }
    GetWindowWord(hwnd.into(), nindex)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HACCEL(pub isize);
impl HACCEL {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HACCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACCEL {}
impl ::core::fmt::Debug for HACCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACCEL").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HACCEL>> for HACCEL {
    fn from(optional: ::core::option::Option<HACCEL>) -> HACCEL {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HACCEL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HANDEDNESS(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HANDEDNESS_LEFT: HANDEDNESS = HANDEDNESS(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HANDEDNESS_RIGHT: HANDEDNESS = HANDEDNESS(1i32);
impl ::core::marker::Copy for HANDEDNESS {}
impl ::core::clone::Clone for HANDEDNESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HANDEDNESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HANDEDNESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HANDEDNESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDEDNESS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HARDWAREHOOKSTRUCT {
    pub hwnd: super::super::Foundation::HWND,
    pub message: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HARDWAREHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HARDWAREHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HARDWAREHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWAREHOOKSTRUCT").field("hwnd", &self.hwnd).field("message", &self.message).field("wParam", &self.wParam).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HARDWAREHOOKSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HARDWAREHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HARDWAREHOOKSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HARDWAREHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HARDWAREHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_CALLBACK: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(-1i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_CLOSE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(5i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_CLOSE_D: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(6i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_MINIMIZE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(3i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_MINIMIZE_D: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(7i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_RESTORE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(2i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_CLOSE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(8i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_MAXIMIZE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(10i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_MINIMIZE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(11i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_RESTORE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(9i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_SYSTEM: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(1i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_ACTIVATE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_CLICKSKIPPED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_CREATEWND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_DESTROYWND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_KEYSKIPPED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_MINMAX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_MOVESIZE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_QS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_SETFOCUS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCBT_SYSCOMMAND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCF_DEFAULTDESKTOP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HCF_LOGONDESKTOP: u32 = 256u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCURSOR(pub isize);
impl HCURSOR {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCURSOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCURSOR {}
impl ::core::fmt::Debug for HCURSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCURSOR").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HCURSOR>> for HCURSOR {
    fn from(optional: ::core::option::Option<HCURSOR>) -> HCURSOR {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HCURSOR {
    type Abi = Self;
}
impl ::core::convert::From<HCURSOR> for HICON {
    fn from(item: HCURSOR) -> HICON {
        HICON(item.0)
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HC_ACTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HC_GETNEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HC_NOREM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HC_NOREMOVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HC_SKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HC_SYSMODALOFF: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HC_SYSMODALON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELPINFO_MENUITEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELPINFO_WINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_COMMAND: i32 = 258i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_CONTENTS: i32 = 3i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_CONTEXT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_CONTEXTMENU: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_CONTEXTPOPUP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_FINDER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_FORCEFILE: i32 = 9i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_HELPONHELP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_INDEX: i32 = 3i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_KEY: i32 = 257i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_MULTIKEY: i32 = 513i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_PARTIALKEY: i32 = 261i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_QUIT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_SETCONTENTS: i32 = 5i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_SETINDEX: i32 = 5i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_SETPOPUP_POS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_SETWINPOS: i32 = 515i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_TCARD: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_TCARD_DATA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_TCARD_OTHER_CALLER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HELP_WM_HELP: u32 = 12u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HHOOK(pub isize);
impl HHOOK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HHOOK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HHOOK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HHOOK {}
impl ::core::fmt::Debug for HHOOK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HHOOK").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HHOOK>> for HHOOK {
    fn from(optional: ::core::option::Option<HHOOK>) -> HHOOK {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HHOOK {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HICON(pub isize);
impl HICON {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HICON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HICON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HICON {}
impl ::core::fmt::Debug for HICON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HICON").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HICON>> for HICON {
    fn from(optional: ::core::option::Option<HICON>) -> HICON {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HICON {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HIDE_WINDOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HKL_NEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HKL_PREV: u32 = 0u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMENU(pub isize);
impl HMENU {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMENU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMENU {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMENU {}
impl ::core::fmt::Debug for HMENU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMENU").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HMENU>> for HMENU {
    fn from(optional: ::core::option::Option<HMENU>) -> HMENU {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HMENU {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type HOOKPROC = ::core::option::Option<unsafe extern "system" fn(code: i32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_ACCESSIBILITYSTATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_ACTIVATESHELLWINDOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_APPCOMMAND: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_ENDTASK: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_GETMINRECT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_HIGHBIT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_LANGUAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_MONITORCHANGED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_REDRAW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_SYSMENU: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_TASKMAN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_WINDOWACTIVATED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_WINDOWCREATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_WINDOWDESTROYED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_WINDOWREPLACED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HSHELL_WINDOWREPLACING: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTBORDER: u32 = 18u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTBOTTOM: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTBOTTOMLEFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTBOTTOMRIGHT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTCAPTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTCLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTCLOSE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTERROR: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTGROWBOX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTHELP: u32 = 21u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTHSCROLL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTLEFT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTMAXBUTTON: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTMENU: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTMINBUTTON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTNOWHERE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTOBJECT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTREDUCE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTRIGHT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTSIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTSIZEFIRST: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTSIZELAST: u32 = 17u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTSYSMENU: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTTOP: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTTOPLEFT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTTOPRIGHT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTTRANSPARENT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTVSCROLL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const HTZOOM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const HWND_BOTTOM: super::super::Foundation::HWND = super::super::Foundation::HWND(1i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const HWND_DESKTOP: super::super::Foundation::HWND = super::super::Foundation::HWND(0i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const HWND_MESSAGE: super::super::Foundation::HWND = super::super::Foundation::HWND(-3i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const HWND_NOTOPMOST: super::super::Foundation::HWND = super::super::Foundation::HWND(-2i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const HWND_TOP: super::super::Foundation::HWND = super::super::Foundation::HWND(0i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const HWND_TOPMOST: super::super::Foundation::HWND = super::super::Foundation::HWND(-1i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HideCaret<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HideCaret(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    HideCaret(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HiliteMenuItem<'a, P0, P1>(hwnd: P0, hmenu: P1, uidhiliteitem: u32, uhilite: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HiliteMenuItem(hwnd: super::super::Foundation::HWND, hmenu: HMENU, uidhiliteitem: u32, uhilite: u32) -> super::super::Foundation::BOOL;
    }
    HiliteMenuItem(hwnd.into(), hmenu.into(), uidhiliteitem, uhilite)
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICONINFO {
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ICONINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ICONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFO").field("fIcon", &self.fIcon).field("xHotspot", &self.xHotspot).field("yHotspot", &self.yHotspot).field("hbmMask", &self.hbmMask).field("hbmColor", &self.hbmColor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for ICONINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICONINFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICONINFOEXA {
    pub cbSize: u32,
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
    pub wResID: u16,
    pub szModName: [super::super::Foundation::CHAR; 260],
    pub szResName: [super::super::Foundation::CHAR; 260],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ICONINFOEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ICONINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFOEXA").field("cbSize", &self.cbSize).field("fIcon", &self.fIcon).field("xHotspot", &self.xHotspot).field("yHotspot", &self.yHotspot).field("hbmMask", &self.hbmMask).field("hbmColor", &self.hbmColor).field("wResID", &self.wResID).field("szModName", &self.szModName).field("szResName", &self.szResName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for ICONINFOEXA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICONINFOEXA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONINFOEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICONINFOEXW {
    pub cbSize: u32,
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
    pub wResID: u16,
    pub szModName: [u16; 260],
    pub szResName: [u16; 260],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ICONINFOEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ICONINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFOEXW").field("cbSize", &self.cbSize).field("fIcon", &self.fIcon).field("xHotspot", &self.xHotspot).field("yHotspot", &self.yHotspot).field("hbmMask", &self.hbmMask).field("hbmColor", &self.hbmColor).field("wResID", &self.wResID).field("szModName", &self.szModName).field("szResName", &self.szResName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for ICONINFOEXW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICONINFOEXW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONINFOEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICONMETRICSA {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::super::Graphics::Gdi::LOGFONTA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ICONMETRICSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ICONMETRICSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONMETRICSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONMETRICSA").field("cbSize", &self.cbSize).field("iHorzSpacing", &self.iHorzSpacing).field("iVertSpacing", &self.iVertSpacing).field("iTitleWrap", &self.iTitleWrap).field("lfFont", &self.lfFont).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for ICONMETRICSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONMETRICSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICONMETRICSA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONMETRICSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONMETRICSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICONMETRICSW {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::super::Graphics::Gdi::LOGFONTW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for ICONMETRICSW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for ICONMETRICSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICONMETRICSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONMETRICSW").field("cbSize", &self.cbSize).field("iHorzSpacing", &self.iHorzSpacing).field("iVertSpacing", &self.iVertSpacing).field("iTitleWrap", &self.iTitleWrap).field("lfFont", &self.lfFont).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for ICONMETRICSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICONMETRICSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICONMETRICSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICONMETRICSW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICONMETRICSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ICON_BIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ICON_SMALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ICON_SMALL2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDANI_CAPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDANI_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_APPSTARTING: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32650i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_ARROW: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32512i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_CROSS: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32515i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_HAND: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32649i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_HELP: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32651i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_IBEAM: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32513i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_ICON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32641i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_NO: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32648i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_PERSON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32672i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_PIN: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32671i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_SIZE: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32640i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_SIZEALL: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32646i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_SIZENESW: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32643i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_SIZENS: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32645i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_SIZENWSE: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32642i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_SIZEWE: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32644i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_UPARROW: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32516i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDC_WAIT: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32514i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDHOT_SNAPDESKTOP: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDHOT_SNAPWINDOW: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDH_CANCEL: u32 = 28444u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDH_GENERIC_HELP_BUTTON: u32 = 28442u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDH_HELP: u32 = 28445u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDH_MISSING_CONTEXT: u32 = 28441u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDH_NO_HELP: u32 = 28440u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDH_OK: u32 = 28443u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_APPLICATION: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32512u32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_ASTERISK: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32516u32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_ERROR: u32 = 32513u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_EXCLAMATION: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32515u32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_HAND: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32513u32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_INFORMATION: u32 = 32516u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_QUESTION: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32514u32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_SHIELD: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32518u32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_WARNING: u32 = 32515u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDI_WINLOGO: ::windows::core::PCWSTR = ::windows::core::PCWSTR(32517u32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IMAGE_ENHMETAFILE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_CREATEDIBSECTION: IMAGE_FLAGS = IMAGE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_DEFAULTCOLOR: IMAGE_FLAGS = IMAGE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_DEFAULTSIZE: IMAGE_FLAGS = IMAGE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_LOADFROMFILE: IMAGE_FLAGS = IMAGE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_LOADMAP3DCOLORS: IMAGE_FLAGS = IMAGE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_LOADTRANSPARENT: IMAGE_FLAGS = IMAGE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_MONOCHROME: IMAGE_FLAGS = IMAGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_SHARED: IMAGE_FLAGS = IMAGE_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_VGACOLOR: IMAGE_FLAGS = IMAGE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_COPYDELETEORG: IMAGE_FLAGS = IMAGE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_COPYFROMRESOURCE: IMAGE_FLAGS = IMAGE_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_COPYRETURNORG: IMAGE_FLAGS = IMAGE_FLAGS(4u32);
impl ::core::marker::Copy for IMAGE_FLAGS {}
impl ::core::clone::Clone for IMAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const INDEXID_CONTAINER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const INDEXID_OBJECT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const INPUTLANGCHANGE_BACKWARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const INPUTLANGCHANGE_FORWARD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const INPUTLANGCHANGE_SYSCHARSET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISMEX_CALLBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISMEX_NOSEND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISMEX_NOTIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISMEX_REPLIED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISMEX_SEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISOLATIONAWARE_MANIFEST_RESOURCE_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISOLATIONAWARE_NOSTATICIMPORT_MANIFEST_RESOURCE_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISOLATIONPOLICY_BROWSER_MANIFEST_RESOURCE_ID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ISOLATIONPOLICY_MANIFEST_RESOURCE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InSendMessage() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InSendMessage() -> super::super::Foundation::BOOL;
    }
    InSendMessage()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn InSendMessageEx(lpreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InSendMessageEx(lpreserved: *mut ::core::ffi::c_void) -> u32;
    }
    InSendMessageEx(::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn IndexFilePath<'a, P0>(resourceindexer: *const ::core::ffi::c_void, filepath: P0, ppresourceuri: *mut ::windows::core::PWSTR, pqualifiercount: *mut u32, ppqualifiers: *mut *mut IndexedResourceQualifier) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IndexFilePath(resourceindexer: *const ::core::ffi::c_void, filepath: ::windows::core::PCWSTR, ppresourceuri: *mut ::windows::core::PWSTR, pqualifiercount: *mut u32, ppqualifiers: *mut *mut IndexedResourceQualifier) -> ::windows::core::HRESULT;
    }
    IndexFilePath(::core::mem::transmute(resourceindexer), filepath.into(), ::core::mem::transmute(ppresourceuri), ::core::mem::transmute(pqualifiercount), ::core::mem::transmute(ppqualifiers)).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct IndexedResourceQualifier {
    pub name: ::windows::core::PWSTR,
    pub value: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for IndexedResourceQualifier {}
impl ::core::clone::Clone for IndexedResourceQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IndexedResourceQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IndexedResourceQualifier").field("name", &self.name).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for IndexedResourceQualifier {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IndexedResourceQualifier {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IndexedResourceQualifier>()) == 0 }
    }
}
impl ::core::cmp::Eq for IndexedResourceQualifier {}
impl ::core::default::Default for IndexedResourceQualifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InheritWindowMonitor<'a, P0, P1>(hwnd: P0, hwndinherit: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InheritWindowMonitor(hwnd: super::super::Foundation::HWND, hwndinherit: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    InheritWindowMonitor(hwnd.into(), hwndinherit.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InsertMenuA<'a, P0, P1>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InsertMenuA(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    InsertMenuA(hmenu.into(), uposition, uflags, uidnewitem, lpnewitem.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn InsertMenuItemA<'a, P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmi: *const MENUITEMINFOA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InsertMenuItemA(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmi: *const MENUITEMINFOA) -> super::super::Foundation::BOOL;
    }
    InsertMenuItemA(hmenu.into(), item, fbyposition.into(), ::core::mem::transmute(lpmi))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn InsertMenuItemW<'a, P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmi: *const MENUITEMINFOW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InsertMenuItemW(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmi: *const MENUITEMINFOW) -> super::super::Foundation::BOOL;
    }
    InsertMenuItemW(hmenu.into(), item, fbyposition.into(), ::core::mem::transmute(lpmi))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InsertMenuW<'a, P0, P1>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InsertMenuW(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    InsertMenuW(hmenu.into(), uposition, uflags, uidnewitem, lpnewitem.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternalGetWindowText<'a, P0>(hwnd: P0, pstring: &mut [u16]) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InternalGetWindowText(hwnd: super::super::Foundation::HWND, pstring: ::windows::core::PWSTR, cchmaxcount: i32) -> i32;
    }
    InternalGetWindowText(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pstring)), pstring.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharAlphaA<'a, P0>(ch: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsCharAlphaA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    }
    IsCharAlphaA(ch.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharAlphaNumericA<'a, P0>(ch: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsCharAlphaNumericA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    }
    IsCharAlphaNumericA(ch.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharAlphaNumericW(ch: u16) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsCharAlphaNumericW(ch: u16) -> super::super::Foundation::BOOL;
    }
    IsCharAlphaNumericW(ch)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharAlphaW(ch: u16) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsCharAlphaW(ch: u16) -> super::super::Foundation::BOOL;
    }
    IsCharAlphaW(ch)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharLowerA<'a, P0>(ch: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsCharLowerA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    }
    IsCharLowerA(ch.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharUpperA<'a, P0>(ch: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsCharUpperA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    }
    IsCharUpperA(ch.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharUpperW(ch: u16) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsCharUpperW(ch: u16) -> super::super::Foundation::BOOL;
    }
    IsCharUpperW(ch)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsChild<'a, P0, P1>(hwndparent: P0, hwnd: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsChild(hwndparent: super::super::Foundation::HWND, hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    IsChild(hwndparent.into(), hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDialogMessageA<'a, P0>(hdlg: P0, lpmsg: *const MSG) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsDialogMessageA(hdlg: super::super::Foundation::HWND, lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    }
    IsDialogMessageA(hdlg.into(), ::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDialogMessageW<'a, P0>(hdlg: P0, lpmsg: *const MSG) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsDialogMessageW(hdlg: super::super::Foundation::HWND, lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    }
    IsDialogMessageW(hdlg.into(), ::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsGUIThread<'a, P0>(bconvert: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsGUIThread(bconvert: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    IsGUIThread(bconvert.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsHungAppWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsHungAppWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    IsHungAppWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsIconic<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsIconic(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    IsIconic(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsMenu<'a, P0>(hmenu: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsMenu(hmenu: HMENU) -> super::super::Foundation::BOOL;
    }
    IsMenu(hmenu.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessDPIAware() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsProcessDPIAware() -> super::super::Foundation::BOOL;
    }
    IsProcessDPIAware()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    IsWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWindowUnicode<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsWindowUnicode(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    IsWindowUnicode(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWindowVisible<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsWindowVisible(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    IsWindowVisible(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWow64Message() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsWow64Message() -> super::super::Foundation::BOOL;
    }
    IsWow64Message()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsZoomed<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsZoomed(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    IsZoomed(hwnd.into())
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct KBDLLHOOKSTRUCT {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: KBDLLHOOKSTRUCT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl ::core::marker::Copy for KBDLLHOOKSTRUCT {}
impl ::core::clone::Clone for KBDLLHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KBDLLHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDLLHOOKSTRUCT").field("vkCode", &self.vkCode).field("scanCode", &self.scanCode).field("flags", &self.flags).field("time", &self.time).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for KBDLLHOOKSTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KBDLLHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KBDLLHOOKSTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for KBDLLHOOKSTRUCT {}
impl ::core::default::Default for KBDLLHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KBDLLHOOKSTRUCT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LLKHF_EXTENDED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LLKHF_ALTDOWN: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LLKHF_UP: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LLKHF_INJECTED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LLKHF_LOWER_IL_INJECTED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(2u32);
impl ::core::marker::Copy for KBDLLHOOKSTRUCT_FLAGS {}
impl ::core::clone::Clone for KBDLLHOOKSTRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KBDLLHOOKSTRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KBDLLHOOKSTRUCT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KBDLLHOOKSTRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KBDLLHOOKSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KBDLLHOOKSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KBDLLHOOKSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const KF_ALTDOWN: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const KF_DLGMODE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const KF_EXTENDED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const KF_MENUMODE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const KF_REPEAT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const KF_UP: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const KL_NAMELENGTH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KillTimer<'a, P0>(hwnd: P0, uidevent: usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn KillTimer(hwnd: super::super::Foundation::HWND, uidevent: usize) -> super::super::Foundation::BOOL;
    }
    KillTimer(hwnd.into(), uidevent)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LAYERED_WINDOW_ATTRIBUTES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LWA_ALPHA: LAYERED_WINDOW_ATTRIBUTES_FLAGS = LAYERED_WINDOW_ATTRIBUTES_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LWA_COLORKEY: LAYERED_WINDOW_ATTRIBUTES_FLAGS = LAYERED_WINDOW_ATTRIBUTES_FLAGS(1u32);
impl ::core::marker::Copy for LAYERED_WINDOW_ATTRIBUTES_FLAGS {}
impl ::core::clone::Clone for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LAYERED_WINDOW_ATTRIBUTES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBN_DBLCLK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBN_ERRSPACE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBN_KILLFOCUS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBN_SELCANCEL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBN_SELCHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBN_SETFOCUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_COMBOBOX: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_DISABLENOSCROLL: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_EXTENDEDSEL: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_HASSTRINGS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_MULTICOLUMN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_MULTIPLESEL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_NODATA: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_NOINTEGRALHEIGHT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_NOREDRAW: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_NOSEL: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_NOTIFY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_OWNERDRAWFIXED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_OWNERDRAWVARIABLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_SORT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_STANDARD: i32 = 10485763i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_USETABSTOPS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LBS_WANTKEYBOARDINPUT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_ADDFILE: u32 = 406u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_ADDSTRING: u32 = 384u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_CTLCODE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_DELETESTRING: u32 = 386u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_DIR: u32 = 397u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_ERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_ERRSPACE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_FINDSTRING: u32 = 399u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_FINDSTRINGEXACT: u32 = 418u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETANCHORINDEX: u32 = 413u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETCARETINDEX: u32 = 415u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETCOUNT: u32 = 395u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETCURSEL: u32 = 392u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETHORIZONTALEXTENT: u32 = 403u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETITEMDATA: u32 = 409u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETITEMHEIGHT: u32 = 417u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETITEMRECT: u32 = 408u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETLISTBOXINFO: u32 = 434u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETLOCALE: u32 = 422u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETSEL: u32 = 391u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETSELCOUNT: u32 = 400u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETSELITEMS: u32 = 401u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETTEXT: u32 = 393u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETTEXTLEN: u32 = 394u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_GETTOPINDEX: u32 = 398u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_INITSTORAGE: u32 = 424u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_INSERTSTRING: u32 = 385u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_ITEMFROMPOINT: u32 = 425u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_MSGMAX: u32 = 435u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_MULTIPLEADDSTRING: u32 = 433u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_OKAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_RESETCONTENT: u32 = 388u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SELECTSTRING: u32 = 396u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SELITEMRANGE: u32 = 411u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SELITEMRANGEEX: u32 = 387u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETANCHORINDEX: u32 = 412u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETCARETINDEX: u32 = 414u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETCOLUMNWIDTH: u32 = 405u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETCOUNT: u32 = 423u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETCURSEL: u32 = 390u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETHORIZONTALEXTENT: u32 = 404u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETITEMDATA: u32 = 410u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETITEMHEIGHT: u32 = 416u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETLOCALE: u32 = 421u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETSEL: u32 = 389u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETTABSTOPS: u32 = 402u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LB_SETTOPINDEX: u32 = 407u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LLMHF_INJECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LLMHF_LOWER_IL_INJECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const LR_COLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadAcceleratorsA<'a, P0, P1>(hinstance: P0, lptablename: P1) -> ::windows::core::Result<HACCEL>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadAcceleratorsA(hinstance: super::super::Foundation::HINSTANCE, lptablename: ::windows::core::PCSTR) -> HACCEL;
    }
    let result__ = LoadAcceleratorsA(hinstance.into(), lptablename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadAcceleratorsW<'a, P0, P1>(hinstance: P0, lptablename: P1) -> ::windows::core::Result<HACCEL>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadAcceleratorsW(hinstance: super::super::Foundation::HINSTANCE, lptablename: ::windows::core::PCWSTR) -> HACCEL;
    }
    let result__ = LoadAcceleratorsW(hinstance.into(), lptablename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadCursorA<'a, P0, P1>(hinstance: P0, lpcursorname: P1) -> ::windows::core::Result<HCURSOR>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadCursorA(hinstance: super::super::Foundation::HINSTANCE, lpcursorname: ::windows::core::PCSTR) -> HCURSOR;
    }
    let result__ = LoadCursorA(hinstance.into(), lpcursorname.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn LoadCursorFromFileA<'a, P0>(lpfilename: P0) -> ::windows::core::Result<HCURSOR>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadCursorFromFileA(lpfilename: ::windows::core::PCSTR) -> HCURSOR;
    }
    let result__ = LoadCursorFromFileA(lpfilename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn LoadCursorFromFileW<'a, P0>(lpfilename: P0) -> ::windows::core::Result<HCURSOR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadCursorFromFileW(lpfilename: ::windows::core::PCWSTR) -> HCURSOR;
    }
    let result__ = LoadCursorFromFileW(lpfilename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadCursorW<'a, P0, P1>(hinstance: P0, lpcursorname: P1) -> ::windows::core::Result<HCURSOR>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadCursorW(hinstance: super::super::Foundation::HINSTANCE, lpcursorname: ::windows::core::PCWSTR) -> HCURSOR;
    }
    let result__ = LoadCursorW(hinstance.into(), lpcursorname.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadIconA<'a, P0, P1>(hinstance: P0, lpiconname: P1) -> ::windows::core::Result<HICON>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadIconA(hinstance: super::super::Foundation::HINSTANCE, lpiconname: ::windows::core::PCSTR) -> HICON;
    }
    let result__ = LoadIconA(hinstance.into(), lpiconname.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadIconW<'a, P0, P1>(hinstance: P0, lpiconname: P1) -> ::windows::core::Result<HICON>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadIconW(hinstance: super::super::Foundation::HINSTANCE, lpiconname: ::windows::core::PCWSTR) -> HICON;
    }
    let result__ = LoadIconW(hinstance.into(), lpiconname.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadImageA<'a, P0, P1>(hinst: P0, name: P1, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadImageA(hinst: super::super::Foundation::HINSTANCE, name: ::windows::core::PCSTR, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> super::super::Foundation::HANDLE;
    }
    let result__ = LoadImageA(hinst.into(), name.into(), r#type, cx, cy, fuload);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadImageW<'a, P0, P1>(hinst: P0, name: P1, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadImageW(hinst: super::super::Foundation::HINSTANCE, name: ::windows::core::PCWSTR, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> super::super::Foundation::HANDLE;
    }
    let result__ = LoadImageW(hinst.into(), name.into(), r#type, cx, cy, fuload);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadMenuA<'a, P0, P1>(hinstance: P0, lpmenuname: P1) -> ::windows::core::Result<HMENU>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadMenuA(hinstance: super::super::Foundation::HINSTANCE, lpmenuname: ::windows::core::PCSTR) -> HMENU;
    }
    let result__ = LoadMenuA(hinstance.into(), lpmenuname.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn LoadMenuIndirectA(lpmenutemplate: *const ::core::ffi::c_void) -> ::windows::core::Result<HMENU> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadMenuIndirectA(lpmenutemplate: *const ::core::ffi::c_void) -> HMENU;
    }
    let result__ = LoadMenuIndirectA(::core::mem::transmute(lpmenutemplate));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn LoadMenuIndirectW(lpmenutemplate: *const ::core::ffi::c_void) -> ::windows::core::Result<HMENU> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadMenuIndirectW(lpmenutemplate: *const ::core::ffi::c_void) -> HMENU;
    }
    let result__ = LoadMenuIndirectW(::core::mem::transmute(lpmenutemplate));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadMenuW<'a, P0, P1>(hinstance: P0, lpmenuname: P1) -> ::windows::core::Result<HMENU>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadMenuW(hinstance: super::super::Foundation::HINSTANCE, lpmenuname: ::windows::core::PCWSTR) -> HMENU;
    }
    let result__ = LoadMenuW(hinstance.into(), lpmenuname.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadStringA<'a, P0>(hinstance: P0, uid: u32, lpbuffer: ::windows::core::PSTR, cchbuffermax: i32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadStringA(hinstance: super::super::Foundation::HINSTANCE, uid: u32, lpbuffer: ::windows::core::PSTR, cchbuffermax: i32) -> i32;
    }
    LoadStringA(hinstance.into(), uid, ::core::mem::transmute(lpbuffer), cchbuffermax)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadStringW<'a, P0>(hinstance: P0, uid: u32, lpbuffer: ::windows::core::PWSTR, cchbuffermax: i32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadStringW(hinstance: super::super::Foundation::HINSTANCE, uid: u32, lpbuffer: ::windows::core::PWSTR, cchbuffermax: i32) -> i32;
    }
    LoadStringW(hinstance.into(), uid, ::core::mem::transmute(lpbuffer), cchbuffermax)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LockSetForegroundWindow(ulockcode: FOREGROUND_WINDOW_LOCK_CODE) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LockSetForegroundWindow(ulockcode: FOREGROUND_WINDOW_LOCK_CODE) -> super::super::Foundation::BOOL;
    }
    LockSetForegroundWindow(ulockcode)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogicalToPhysicalPoint<'a, P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LogicalToPhysicalPoint(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    LogicalToPhysicalPoint(hwnd.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupIconIdFromDirectory<'a, P0>(presbits: *const u8, ficon: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LookupIconIdFromDirectory(presbits: *const u8, ficon: super::super::Foundation::BOOL) -> i32;
    }
    LookupIconIdFromDirectory(::core::mem::transmute(presbits), ficon.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupIconIdFromDirectoryEx<'a, P0>(presbits: *const u8, ficon: P0, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LookupIconIdFromDirectoryEx(presbits: *const u8, ficon: super::super::Foundation::BOOL, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> i32;
    }
    LookupIconIdFromDirectoryEx(::core::mem::transmute(presbits), ficon.into(), cxdesired, cydesired, flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAPVK_VK_TO_CHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAPVK_VK_TO_VSC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAPVK_VK_TO_VSC_EX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAPVK_VSC_TO_VK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAPVK_VSC_TO_VK_EX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAXIMUM_RESERVED_MANIFEST_RESOURCE_ID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAX_LOGICALDPIOVERRIDE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAX_STR_BLOCKREASON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAX_TOUCH_COUNT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MAX_TOUCH_PREDICTION_FILTER_TAPS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MA_ACTIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MA_ACTIVATEANDEAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MA_NOACTIVATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MA_NOACTIVATEANDEAT: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MDICREATESTRUCTA {
    pub szClass: ::windows::core::PCSTR,
    pub szTitle: ::windows::core::PCSTR,
    pub hOwner: super::super::Foundation::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: WINDOW_STYLE,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MDICREATESTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MDICREATESTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MDICREATESTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDICREATESTRUCTA").field("szClass", &self.szClass).field("szTitle", &self.szTitle).field("hOwner", &self.hOwner).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("style", &self.style).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MDICREATESTRUCTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MDICREATESTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MDICREATESTRUCTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MDICREATESTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MDICREATESTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MDICREATESTRUCTW {
    pub szClass: ::windows::core::PCWSTR,
    pub szTitle: ::windows::core::PCWSTR,
    pub hOwner: super::super::Foundation::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: WINDOW_STYLE,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MDICREATESTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MDICREATESTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MDICREATESTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDICREATESTRUCTW").field("szClass", &self.szClass).field("szTitle", &self.szTitle).field("hOwner", &self.hOwner).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("style", &self.style).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MDICREATESTRUCTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MDICREATESTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MDICREATESTRUCTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MDICREATESTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MDICREATESTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MDINEXTMENU {
    pub hmenuIn: HMENU,
    pub hmenuNext: HMENU,
    pub hwndNext: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MDINEXTMENU {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MDINEXTMENU {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MDINEXTMENU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDINEXTMENU").field("hmenuIn", &self.hmenuIn).field("hmenuNext", &self.hmenuNext).field("hwndNext", &self.hwndNext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MDINEXTMENU {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MDINEXTMENU {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MDINEXTMENU>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MDINEXTMENU {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MDINEXTMENU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MDIS_ALLCHILDSTYLES: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MENUBARINFO {
    pub cbSize: u32,
    pub rcBar: super::super::Foundation::RECT,
    pub hMenu: HMENU,
    pub hwndMenu: super::super::Foundation::HWND,
    pub _bitfield: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MENUBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MENUBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MENUBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUBARINFO").field("cbSize", &self.cbSize).field("rcBar", &self.rcBar).field("hMenu", &self.hMenu).field("hwndMenu", &self.hwndMenu).field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MENUBARINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MENUBARINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUBARINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MENUBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MENUBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MENUGETOBJECTINFO {
    pub dwFlags: MENUGETOBJECTINFO_FLAGS,
    pub uPos: u32,
    pub hmenu: HMENU,
    pub riid: *mut ::core::ffi::c_void,
    pub pvObj: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MENUGETOBJECTINFO {}
impl ::core::clone::Clone for MENUGETOBJECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUGETOBJECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUGETOBJECTINFO").field("dwFlags", &self.dwFlags).field("uPos", &self.uPos).field("hmenu", &self.hmenu).field("riid", &self.riid).field("pvObj", &self.pvObj).finish()
    }
}
unsafe impl ::windows::core::Abi for MENUGETOBJECTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MENUGETOBJECTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUGETOBJECTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MENUGETOBJECTINFO {}
impl ::core::default::Default for MENUGETOBJECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENUGETOBJECTINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNGOF_BOTTOMGAP: MENUGETOBJECTINFO_FLAGS = MENUGETOBJECTINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNGOF_TOPGAP: MENUGETOBJECTINFO_FLAGS = MENUGETOBJECTINFO_FLAGS(1u32);
impl ::core::marker::Copy for MENUGETOBJECTINFO_FLAGS {}
impl ::core::clone::Clone for MENUGETOBJECTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUGETOBJECTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MENUGETOBJECTINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENUGETOBJECTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUGETOBJECTINFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct MENUINFO {
    pub cbSize: u32,
    pub fMask: MENUINFO_MASK,
    pub dwStyle: MENUINFO_STYLE,
    pub cyMax: u32,
    pub hbrBack: super::super::Graphics::Gdi::HBRUSH,
    pub dwContextHelpID: u32,
    pub dwMenuData: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for MENUINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for MENUINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for MENUINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("dwStyle", &self.dwStyle).field("cyMax", &self.cyMax).field("hbrBack", &self.hbrBack).field("dwContextHelpID", &self.dwContextHelpID).field("dwMenuData", &self.dwMenuData).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for MENUINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for MENUINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for MENUINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for MENUINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENUINFO_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIM_APPLYTOSUBMENUS: MENUINFO_MASK = MENUINFO_MASK(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIM_BACKGROUND: MENUINFO_MASK = MENUINFO_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIM_HELPID: MENUINFO_MASK = MENUINFO_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIM_MAXHEIGHT: MENUINFO_MASK = MENUINFO_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIM_MENUDATA: MENUINFO_MASK = MENUINFO_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIM_STYLE: MENUINFO_MASK = MENUINFO_MASK(16u32);
impl ::core::marker::Copy for MENUINFO_MASK {}
impl ::core::clone::Clone for MENUINFO_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUINFO_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MENUINFO_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENUINFO_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUINFO_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENUINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENUINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENUINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENUINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENUINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENUINFO_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNS_AUTODISMISS: MENUINFO_STYLE = MENUINFO_STYLE(268435456u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNS_CHECKORBMP: MENUINFO_STYLE = MENUINFO_STYLE(67108864u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNS_DRAGDROP: MENUINFO_STYLE = MENUINFO_STYLE(536870912u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNS_MODELESS: MENUINFO_STYLE = MENUINFO_STYLE(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNS_NOCHECK: MENUINFO_STYLE = MENUINFO_STYLE(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNS_NOTIFYBYPOS: MENUINFO_STYLE = MENUINFO_STYLE(134217728u32);
impl ::core::marker::Copy for MENUINFO_STYLE {}
impl ::core::clone::Clone for MENUINFO_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUINFO_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MENUINFO_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENUINFO_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUINFO_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENUINFO_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENUINFO_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENUINFO_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENUINFO_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENUINFO_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct MENUITEMINFOA {
    pub cbSize: u32,
    pub fMask: MENU_ITEM_MASK,
    pub fType: MENU_ITEM_TYPE,
    pub fState: MENU_ITEM_STATE,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: ::windows::core::PSTR,
    pub cch: u32,
    pub hbmpItem: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for MENUITEMINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for MENUITEMINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for MENUITEMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMINFOA").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hSubMenu", &self.hSubMenu).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("dwTypeData", &self.dwTypeData).field("cch", &self.cch).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for MENUITEMINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for MENUITEMINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUITEMINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for MENUITEMINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for MENUITEMINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct MENUITEMINFOW {
    pub cbSize: u32,
    pub fMask: MENU_ITEM_MASK,
    pub fType: MENU_ITEM_TYPE,
    pub fState: MENU_ITEM_STATE,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: ::windows::core::PWSTR,
    pub cch: u32,
    pub hbmpItem: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for MENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for MENUITEMINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for MENUITEMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMINFOW").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hSubMenu", &self.hSubMenu).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("dwTypeData", &self.dwTypeData).field("cch", &self.cch).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for MENUITEMINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for MENUITEMINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUITEMINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for MENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for MENUITEMINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MENUITEMTEMPLATE {
    pub mtOption: u16,
    pub mtID: u16,
    pub mtString: [u16; 1],
}
impl ::core::marker::Copy for MENUITEMTEMPLATE {}
impl ::core::clone::Clone for MENUITEMTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUITEMTEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMTEMPLATE").field("mtOption", &self.mtOption).field("mtID", &self.mtID).field("mtString", &self.mtString).finish()
    }
}
unsafe impl ::windows::core::Abi for MENUITEMTEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MENUITEMTEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUITEMTEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MENUITEMTEMPLATE {}
impl ::core::default::Default for MENUITEMTEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MENUITEMTEMPLATEHEADER {
    pub versionNumber: u16,
    pub offset: u16,
}
impl ::core::marker::Copy for MENUITEMTEMPLATEHEADER {}
impl ::core::clone::Clone for MENUITEMTEMPLATEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUITEMTEMPLATEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMTEMPLATEHEADER").field("versionNumber", &self.versionNumber).field("offset", &self.offset).finish()
    }
}
unsafe impl ::windows::core::Abi for MENUITEMTEMPLATEHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MENUITEMTEMPLATEHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENUITEMTEMPLATEHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MENUITEMTEMPLATEHEADER {}
impl ::core::default::Default for MENUITEMTEMPLATEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENU_ITEM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_BYCOMMAND: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_BYPOSITION: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_BITMAP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_CHECKED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_DISABLED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_ENABLED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_GRAYED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_MENUBARBREAK: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_MENUBREAK: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_OWNERDRAW: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_POPUP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_SEPARATOR: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_STRING: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_UNCHECKED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_INSERT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_CHANGE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_APPEND: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_DELETE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_REMOVE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_USECHECKBITMAPS: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_UNHILITE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_HILITE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_DEFAULT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_SYSMENU: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_HELP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_RIGHTJUSTIFY: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_MOUSESELECT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MF_END: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
impl ::core::marker::Copy for MENU_ITEM_FLAGS {}
impl ::core::clone::Clone for MENU_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MENU_ITEM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENU_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENU_ITEM_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_BITMAP: MENU_ITEM_MASK = MENU_ITEM_MASK(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_CHECKMARKS: MENU_ITEM_MASK = MENU_ITEM_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_DATA: MENU_ITEM_MASK = MENU_ITEM_MASK(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_FTYPE: MENU_ITEM_MASK = MENU_ITEM_MASK(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_ID: MENU_ITEM_MASK = MENU_ITEM_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_STATE: MENU_ITEM_MASK = MENU_ITEM_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_STRING: MENU_ITEM_MASK = MENU_ITEM_MASK(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_SUBMENU: MENU_ITEM_MASK = MENU_ITEM_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIIM_TYPE: MENU_ITEM_MASK = MENU_ITEM_MASK(16u32);
impl ::core::marker::Copy for MENU_ITEM_MASK {}
impl ::core::clone::Clone for MENU_ITEM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MENU_ITEM_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENU_ITEM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENU_ITEM_STATE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_GRAYED: MENU_ITEM_STATE = MENU_ITEM_STATE(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_DISABLED: MENU_ITEM_STATE = MENU_ITEM_STATE(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_CHECKED: MENU_ITEM_STATE = MENU_ITEM_STATE(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_HILITE: MENU_ITEM_STATE = MENU_ITEM_STATE(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_ENABLED: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_UNCHECKED: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_UNHILITE: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFS_DEFAULT: MENU_ITEM_STATE = MENU_ITEM_STATE(4096u32);
impl ::core::marker::Copy for MENU_ITEM_STATE {}
impl ::core::clone::Clone for MENU_ITEM_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MENU_ITEM_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENU_ITEM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENU_ITEM_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_BITMAP: MENU_ITEM_TYPE = MENU_ITEM_TYPE(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_MENUBARBREAK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_MENUBREAK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_OWNERDRAW: MENU_ITEM_TYPE = MENU_ITEM_TYPE(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_RADIOCHECK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_RIGHTJUSTIFY: MENU_ITEM_TYPE = MENU_ITEM_TYPE(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_RIGHTORDER: MENU_ITEM_TYPE = MENU_ITEM_TYPE(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_SEPARATOR: MENU_ITEM_TYPE = MENU_ITEM_TYPE(2048u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MFT_STRING: MENU_ITEM_TYPE = MENU_ITEM_TYPE(0u32);
impl ::core::marker::Copy for MENU_ITEM_TYPE {}
impl ::core::clone::Clone for MENU_ITEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MENU_ITEM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENU_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MESSAGEBOX_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDOK: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDCANCEL: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(2i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDABORT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(3i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDRETRY: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(4i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDIGNORE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(5i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDYES: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(6i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDNO: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(7i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDCLOSE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(8i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDHELP: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(9i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDTRYAGAIN: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(10i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDCONTINUE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(11i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDASYNC: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32001i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const IDTIMEOUT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32000i32);
impl ::core::marker::Copy for MESSAGEBOX_RESULT {}
impl ::core::clone::Clone for MESSAGEBOX_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MESSAGEBOX_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MESSAGEBOX_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MESSAGEBOX_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MESSAGEBOX_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MESSAGEBOX_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ABORTRETRYIGNORE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_CANCELTRYCONTINUE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(6u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_HELP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_OK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_OKCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_RETRYCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(5u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_YESNO: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_YESNOCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONHAND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONQUESTION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONEXCLAMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONASTERISK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_USERICON: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONWARNING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONERROR: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONINFORMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONSTOP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_DEFBUTTON1: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_DEFBUTTON2: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_DEFBUTTON3: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_DEFBUTTON4: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(768u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_APPLMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_SYSTEMMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_TASKMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_NOFOCUS: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32768u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_SETFOREGROUND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(65536u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_DEFAULT_DESKTOP_ONLY: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(131072u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_TOPMOST: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_RIGHT: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(524288u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_RTLREADING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1048576u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_SERVICE_NOTIFICATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2097152u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_SERVICE_NOTIFICATION_NT3X: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_TYPEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(15u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_ICONMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(240u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_DEFMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3840u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_MODEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(12288u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MB_MISCMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(49152u32);
impl ::core::marker::Copy for MESSAGEBOX_STYLE {}
impl ::core::clone::Clone for MESSAGEBOX_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MESSAGEBOX_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MESSAGEBOX_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MESSAGEBOX_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MESSAGEBOX_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MESSAGEBOX_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MESSAGEBOX_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MESSAGEBOX_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MESSAGEBOX_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MESSAGEBOX_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MESSAGE_RESOURCE_BLOCK {
    pub LowId: u32,
    pub HighId: u32,
    pub OffsetToEntries: u32,
}
impl ::core::marker::Copy for MESSAGE_RESOURCE_BLOCK {}
impl ::core::clone::Clone for MESSAGE_RESOURCE_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_BLOCK").field("LowId", &self.LowId).field("HighId", &self.HighId).field("OffsetToEntries", &self.OffsetToEntries).finish()
    }
}
unsafe impl ::windows::core::Abi for MESSAGE_RESOURCE_BLOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MESSAGE_RESOURCE_BLOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_BLOCK {}
impl ::core::default::Default for MESSAGE_RESOURCE_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MESSAGE_RESOURCE_DATA {
    pub NumberOfBlocks: u32,
    pub Blocks: [MESSAGE_RESOURCE_BLOCK; 1],
}
impl ::core::marker::Copy for MESSAGE_RESOURCE_DATA {}
impl ::core::clone::Clone for MESSAGE_RESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_DATA").field("NumberOfBlocks", &self.NumberOfBlocks).field("Blocks", &self.Blocks).finish()
    }
}
unsafe impl ::windows::core::Abi for MESSAGE_RESOURCE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MESSAGE_RESOURCE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_DATA {}
impl ::core::default::Default for MESSAGE_RESOURCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MESSAGE_RESOURCE_ENTRY {
    pub Length: u16,
    pub Flags: u16,
    pub Text: [u8; 1],
}
impl ::core::marker::Copy for MESSAGE_RESOURCE_ENTRY {}
impl ::core::clone::Clone for MESSAGE_RESOURCE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_ENTRY").field("Length", &self.Length).field("Flags", &self.Flags).field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows::core::Abi for MESSAGE_RESOURCE_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MESSAGE_RESOURCE_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_ENTRY {}
impl ::core::default::Default for MESSAGE_RESOURCE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const METRICS_USEDEFAULT: i32 = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MINIMIZEDMETRICS {
    pub cbSize: u32,
    pub iWidth: i32,
    pub iHorzGap: i32,
    pub iVertGap: i32,
    pub iArrange: MINIMIZEDMETRICS_ARRANGE,
}
impl ::core::marker::Copy for MINIMIZEDMETRICS {}
impl ::core::clone::Clone for MINIMIZEDMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MINIMIZEDMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIMIZEDMETRICS").field("cbSize", &self.cbSize).field("iWidth", &self.iWidth).field("iHorzGap", &self.iHorzGap).field("iVertGap", &self.iVertGap).field("iArrange", &self.iArrange).finish()
    }
}
unsafe impl ::windows::core::Abi for MINIMIZEDMETRICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MINIMIZEDMETRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MINIMIZEDMETRICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MINIMIZEDMETRICS {}
impl ::core::default::Default for MINIMIZEDMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIMIZEDMETRICS_ARRANGE(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_BOTTOMLEFT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_BOTTOMRIGHT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_TOPLEFT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(2i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ARW_TOPRIGHT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(3i32);
impl ::core::marker::Copy for MINIMIZEDMETRICS_ARRANGE {}
impl ::core::clone::Clone for MINIMIZEDMETRICS_ARRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIMIZEDMETRICS_ARRANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MINIMIZEDMETRICS_ARRANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MINIMIZEDMETRICS_ARRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIMIZEDMETRICS_ARRANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MINIMUM_RESERVED_MANIFEST_RESOURCE_ID: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MINMAXINFO {
    pub ptReserved: super::super::Foundation::POINT,
    pub ptMaxSize: super::super::Foundation::POINT,
    pub ptMaxPosition: super::super::Foundation::POINT,
    pub ptMinTrackSize: super::super::Foundation::POINT,
    pub ptMaxTrackSize: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MINMAXINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MINMAXINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MINMAXINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINMAXINFO").field("ptReserved", &self.ptReserved).field("ptMaxSize", &self.ptMaxSize).field("ptMaxPosition", &self.ptMaxPosition).field("ptMinTrackSize", &self.ptMinTrackSize).field("ptMaxTrackSize", &self.ptMaxTrackSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MINMAXINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MINMAXINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MINMAXINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MINMAXINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINMAXINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MIN_LOGICALDPIOVERRIDE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_CONFIRMHOTKEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_HOTKEYACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_HOTKEYSOUND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_INDICATOR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_LEFTBUTTONDOWN: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_LEFTBUTTONSEL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_MODIFIERS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_MOUSEKEYSON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_MOUSEMODE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_REPLACENUMBERS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_RIGHTBUTTONDOWN: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MKF_RIGHTBUTTONSEL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MK_CONTROL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MK_LBUTTON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MK_MBUTTON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MK_RBUTTON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MK_SHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MK_XBUTTON1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MK_XBUTTON2: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNC_CLOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNC_EXECUTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNC_IGNORE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNC_SELECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MND_CONTINUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MND_ENDMENU: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNGO_NOERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MNGO_NOINTERFACE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MN_GETHMENU: u32 = 481u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MONITORINFOF_PRIMARY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOUSEHOOKSTRUCT {
    pub pt: super::super::Foundation::POINT,
    pub hwnd: super::super::Foundation::HWND,
    pub wHitTestCode: u32,
    pub dwExtraInfo: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOUSEHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOUSEHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOUSEHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEHOOKSTRUCT").field("pt", &self.pt).field("hwnd", &self.hwnd).field("wHitTestCode", &self.wHitTestCode).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MOUSEHOOKSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOUSEHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOUSEHOOKSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOUSEHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOUSEHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOUSEHOOKSTRUCTEX {
    pub __AnonymousBase_winuser_L1166_C46: MOUSEHOOKSTRUCT,
    pub mouseData: MOUSEHOOKSTRUCTEX_MOUSE_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOUSEHOOKSTRUCTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOUSEHOOKSTRUCTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOUSEHOOKSTRUCTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEHOOKSTRUCTEX").field("__AnonymousBase_winuser_L1166_C46", &self.__AnonymousBase_winuser_L1166_C46).field("mouseData", &self.mouseData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MOUSEHOOKSTRUCTEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOUSEHOOKSTRUCTEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOUSEHOOKSTRUCTEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOUSEHOOKSTRUCTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOUSEHOOKSTRUCTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MOUSEHOOKSTRUCTEX_MOUSE_DATA(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const XBUTTON1: MOUSEHOOKSTRUCTEX_MOUSE_DATA = MOUSEHOOKSTRUCTEX_MOUSE_DATA(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const XBUTTON2: MOUSEHOOKSTRUCTEX_MOUSE_DATA = MOUSEHOOKSTRUCTEX_MOUSE_DATA(2u32);
impl ::core::marker::Copy for MOUSEHOOKSTRUCTEX_MOUSE_DATA {}
impl ::core::clone::Clone for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    type Abi = Self;
}
impl ::core::fmt::Debug for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOUSEHOOKSTRUCTEX_MOUSE_DATA").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MOUSEWHEEL_ROUTING_FOCUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MOUSEWHEEL_ROUTING_HYBRID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MOUSEWHEEL_ROUTING_MOUSE_POS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSG {
    pub hwnd: super::super::Foundation::HWND,
    pub message: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub time: u32,
    pub pt: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSG").field("hwnd", &self.hwnd).field("message", &self.message).field("wParam", &self.wParam).field("lParam", &self.lParam).field("time", &self.time).field("pt", &self.pt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub type MSGBOXCALLBACK = ::core::option::Option<unsafe extern "system" fn(lphelpinfo: *mut super::Shell::HELPINFO)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct MSGBOXPARAMSA {
    pub cbSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszText: ::windows::core::PCSTR,
    pub lpszCaption: ::windows::core::PCSTR,
    pub dwStyle: MESSAGEBOX_STYLE,
    pub lpszIcon: ::windows::core::PCSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::marker::Copy for MSGBOXPARAMSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::clone::Clone for MSGBOXPARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::fmt::Debug for MSGBOXPARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSGBOXPARAMSA").field("cbSize", &self.cbSize).field("hwndOwner", &self.hwndOwner).field("hInstance", &self.hInstance).field("lpszText", &self.lpszText).field("lpszCaption", &self.lpszCaption).field("dwStyle", &self.dwStyle).field("lpszIcon", &self.lpszIcon).field("dwContextHelpId", &self.dwContextHelpId).field("lpfnMsgBoxCallback", &self.lpfnMsgBoxCallback.map(|f| f as usize)).field("dwLanguageId", &self.dwLanguageId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
unsafe impl ::windows::core::Abi for MSGBOXPARAMSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::cmp::PartialEq for MSGBOXPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSGBOXPARAMSA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::cmp::Eq for MSGBOXPARAMSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for MSGBOXPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct MSGBOXPARAMSW {
    pub cbSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszText: ::windows::core::PCWSTR,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub dwStyle: MESSAGEBOX_STYLE,
    pub lpszIcon: ::windows::core::PCWSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::marker::Copy for MSGBOXPARAMSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::clone::Clone for MSGBOXPARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::fmt::Debug for MSGBOXPARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSGBOXPARAMSW").field("cbSize", &self.cbSize).field("hwndOwner", &self.hwndOwner).field("hInstance", &self.hInstance).field("lpszText", &self.lpszText).field("lpszCaption", &self.lpszCaption).field("dwStyle", &self.dwStyle).field("lpszIcon", &self.lpszIcon).field("dwContextHelpId", &self.dwContextHelpId).field("lpfnMsgBoxCallback", &self.lpfnMsgBoxCallback.map(|f| f as usize)).field("dwLanguageId", &self.dwLanguageId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
unsafe impl ::windows::core::Abi for MSGBOXPARAMSW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::cmp::PartialEq for MSGBOXPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSGBOXPARAMSW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::cmp::Eq for MSGBOXPARAMSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for MSGBOXPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSGFLTINFO_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLTINFO_NONE: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLTINFO_ALLOWED_HIGHER: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLTINFO_ALREADYALLOWED_FORWND: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLTINFO_ALREADYDISALLOWED_FORWND: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(2u32);
impl ::core::marker::Copy for MSGFLTINFO_STATUS {}
impl ::core::clone::Clone for MSGFLTINFO_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSGFLTINFO_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSGFLTINFO_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSGFLTINFO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSGFLTINFO_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGF_DIALOGBOX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGF_MAX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGF_MENU: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGF_MESSAGEBOX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGF_NEXTWINDOW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGF_SCROLLBAR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGF_USER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MWMO_NONE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MWMO_ALERTABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MWMO_INPUTAVAILABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MWMO_WAITALL: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(1u32);
impl ::core::marker::Copy for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {}
impl ::core::clone::Clone for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSLLHOOKSTRUCT {
    pub pt: super::super::Foundation::POINT,
    pub mouseData: MOUSEHOOKSTRUCTEX_MOUSE_DATA,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSLLHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSLLHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSLLHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSLLHOOKSTRUCT").field("pt", &self.pt).field("mouseData", &self.mouseData).field("flags", &self.flags).field("time", &self.time).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSLLHOOKSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSLLHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSLLHOOKSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSLLHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSLLHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapDialogRect<'a, P0>(hdlg: P0, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MapDialogRect(hdlg: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    MapDialogRect(hdlg.into(), ::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MenuItemFromPoint<'a, P0, P1>(hwnd: P0, hmenu: P1, ptscreen: super::super::Foundation::POINT) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MenuItemFromPoint(hwnd: super::super::Foundation::HWND, hmenu: HMENU, ptscreen: super::super::Foundation::POINT) -> i32;
    }
    MenuItemFromPoint(hwnd.into(), hmenu.into(), ::core::mem::transmute(ptscreen))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MessageBoxA<'a, P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MessageBoxA(hwnd: super::super::Foundation::HWND, lptext: ::windows::core::PCSTR, lpcaption: ::windows::core::PCSTR, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT;
    }
    MessageBoxA(hwnd.into(), lptext.into(), lpcaption.into(), utype)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MessageBoxExA<'a, P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MessageBoxExA(hwnd: super::super::Foundation::HWND, lptext: ::windows::core::PCSTR, lpcaption: ::windows::core::PCSTR, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT;
    }
    MessageBoxExA(hwnd.into(), lptext.into(), lpcaption.into(), utype, wlanguageid)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MessageBoxExW<'a, P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MessageBoxExW(hwnd: super::super::Foundation::HWND, lptext: ::windows::core::PCWSTR, lpcaption: ::windows::core::PCWSTR, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT;
    }
    MessageBoxExW(hwnd.into(), lptext.into(), lpcaption.into(), utype, wlanguageid)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[inline]
pub unsafe fn MessageBoxIndirectA(lpmbp: *const MSGBOXPARAMSA) -> MESSAGEBOX_RESULT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MessageBoxIndirectA(lpmbp: *const MSGBOXPARAMSA) -> MESSAGEBOX_RESULT;
    }
    MessageBoxIndirectA(::core::mem::transmute(lpmbp))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[inline]
pub unsafe fn MessageBoxIndirectW(lpmbp: *const MSGBOXPARAMSW) -> MESSAGEBOX_RESULT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MessageBoxIndirectW(lpmbp: *const MSGBOXPARAMSW) -> MESSAGEBOX_RESULT;
    }
    MessageBoxIndirectW(::core::mem::transmute(lpmbp))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MessageBoxW<'a, P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MessageBoxW(hwnd: super::super::Foundation::HWND, lptext: ::windows::core::PCWSTR, lpcaption: ::windows::core::PCWSTR, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT;
    }
    MessageBoxW(hwnd.into(), lptext.into(), lpcaption.into(), utype)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ModifyMenuA<'a, P0, P1>(hmnu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ModifyMenuA(hmnu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    ModifyMenuA(hmnu.into(), uposition, uflags, uidnewitem, lpnewitem.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ModifyMenuW<'a, P0, P1>(hmnu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ModifyMenuW(hmnu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    ModifyMenuW(hmnu.into(), uposition, uflags, uidnewitem, lpnewitem.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveWindow<'a, P0, P1>(hwnd: P0, x: i32, y: i32, nwidth: i32, nheight: i32, brepaint: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoveWindow(hwnd: super::super::Foundation::HWND, x: i32, y: i32, nwidth: i32, nheight: i32, brepaint: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    MoveWindow(hwnd.into(), x, y, nwidth, nheight, brepaint.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateConfig<'a, P0, P1>(platformversion: MrmPlatformVersion, defaultqualifiers: P0, outputxmlfile: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateConfig(platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, outputxmlfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmCreateConfig(platformversion, defaultqualifiers.into(), outputxmlfile.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateConfigInMemory<'a, P0>(platformversion: MrmPlatformVersion, defaultqualifiers: P0, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateConfigInMemory(platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::core::HRESULT;
    }
    MrmCreateConfigInMemory(platformversion, defaultqualifiers.into(), ::core::mem::transmute(outputxmldata), ::core::mem::transmute(outputxmlsize)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceFile<'a, P0>(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputdirectory: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceFile(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceFile(::core::mem::transmute(indexer), packagingmode, packagingoptions, outputdirectory.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceFileInMemory(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputpridata: *mut *mut u8, outputprisize: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceFileInMemory(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputpridata: *mut *mut u8, outputprisize: *mut u32) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceFileInMemory(::core::mem::transmute(indexer), packagingmode, packagingoptions, ::core::mem::transmute(outputpridata), ::core::mem::transmute(outputprisize)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceFileWithChecksum<'a, P0>(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, checksum: u32, outputdirectory: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceFileWithChecksum(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, checksum: u32, outputdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceFileWithChecksum(::core::mem::transmute(indexer), packagingmode, packagingoptions, checksum, outputdirectory.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceIndexer<'a, P0, P1, P2>(packagefamilyname: P0, projectroot: P1, platformversion: MrmPlatformVersion, defaultqualifiers: P2, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceIndexer(packagefamilyname: ::windows::core::PCWSTR, projectroot: ::windows::core::PCWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceIndexer(packagefamilyname.into(), projectroot.into(), platformversion, defaultqualifiers.into(), ::core::mem::transmute(indexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousPriData<'a, P0, P1>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, pridata: *const u8, prisize: u32, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceIndexerFromPreviousPriData(projectroot: ::windows::core::PCWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, pridata: *const u8, prisize: u32, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceIndexerFromPreviousPriData(projectroot.into(), platformversion, defaultqualifiers.into(), ::core::mem::transmute(pridata), prisize, ::core::mem::transmute(indexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousPriFile<'a, P0, P1, P2>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, prifile: P2, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceIndexerFromPreviousPriFile(projectroot: ::windows::core::PCWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, prifile: ::windows::core::PCWSTR, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceIndexerFromPreviousPriFile(projectroot.into(), platformversion, defaultqualifiers.into(), prifile.into(), ::core::mem::transmute(indexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousSchemaData<'a, P0, P1>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, schemaxmldata: *const u8, schemaxmlsize: u32, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceIndexerFromPreviousSchemaData(projectroot: ::windows::core::PCWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, schemaxmldata: *const u8, schemaxmlsize: u32, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceIndexerFromPreviousSchemaData(projectroot.into(), platformversion, defaultqualifiers.into(), ::core::mem::transmute(schemaxmldata), schemaxmlsize, ::core::mem::transmute(indexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousSchemaFile<'a, P0, P1, P2>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, schemafile: P2, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceIndexerFromPreviousSchemaFile(projectroot: ::windows::core::PCWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, schemafile: ::windows::core::PCWSTR, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceIndexerFromPreviousSchemaFile(projectroot.into(), platformversion, defaultqualifiers.into(), schemafile.into(), ::core::mem::transmute(indexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmCreateResourceIndexerWithFlags<'a, P0, P1, P2>(packagefamilyname: P0, projectroot: P1, platformversion: MrmPlatformVersion, defaultqualifiers: P2, flags: MrmIndexerFlags, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmCreateResourceIndexerWithFlags(packagefamilyname: ::windows::core::PCWSTR, projectroot: ::windows::core::PCWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: ::windows::core::PCWSTR, flags: MrmIndexerFlags, indexer: *mut MrmResourceIndexerHandle) -> ::windows::core::HRESULT;
    }
    MrmCreateResourceIndexerWithFlags(packagefamilyname.into(), projectroot.into(), platformversion, defaultqualifiers.into(), flags, ::core::mem::transmute(indexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmDestroyIndexerAndMessages(indexer: MrmResourceIndexerHandle) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmDestroyIndexerAndMessages(indexer: MrmResourceIndexerHandle) -> ::windows::core::HRESULT;
    }
    MrmDestroyIndexerAndMessages(::core::mem::transmute(indexer)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmDumpPriDataInMemory(inputpridata: *const u8, inputprisize: u32, schemapridata: *const u8, schemaprisize: u32, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmDumpPriDataInMemory(inputpridata: *const u8, inputprisize: u32, schemapridata: *const u8, schemaprisize: u32, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::core::HRESULT;
    }
    MrmDumpPriDataInMemory(::core::mem::transmute(inputpridata), inputprisize, ::core::mem::transmute(schemapridata), schemaprisize, dumptype, ::core::mem::transmute(outputxmldata), ::core::mem::transmute(outputxmlsize)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmDumpPriFile<'a, P0, P1, P2>(indexfilename: P0, schemaprifile: P1, dumptype: MrmDumpType, outputxmlfile: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmDumpPriFile(indexfilename: ::windows::core::PCWSTR, schemaprifile: ::windows::core::PCWSTR, dumptype: MrmDumpType, outputxmlfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmDumpPriFile(indexfilename.into(), schemaprifile.into(), dumptype, outputxmlfile.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmDumpPriFileInMemory<'a, P0, P1>(indexfilename: P0, schemaprifile: P1, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmDumpPriFileInMemory(indexfilename: ::windows::core::PCWSTR, schemaprifile: ::windows::core::PCWSTR, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::core::HRESULT;
    }
    MrmDumpPriFileInMemory(indexfilename.into(), schemaprifile.into(), dumptype, ::core::mem::transmute(outputxmldata), ::core::mem::transmute(outputxmlsize)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MrmDumpType(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmDumpType_Basic: MrmDumpType = MrmDumpType(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmDumpType_Detailed: MrmDumpType = MrmDumpType(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmDumpType_Schema: MrmDumpType = MrmDumpType(2i32);
impl ::core::marker::Copy for MrmDumpType {}
impl ::core::clone::Clone for MrmDumpType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmDumpType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MrmDumpType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MrmDumpType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmDumpType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmFreeMemory(data: *const u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmFreeMemory(data: *const u8) -> ::windows::core::HRESULT;
    }
    MrmFreeMemory(::core::mem::transmute(data)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmGetPriFileContentChecksum<'a, P0>(prifile: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmGetPriFileContentChecksum(prifile: ::windows::core::PCWSTR, checksum: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    MrmGetPriFileContentChecksum(prifile.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmIndexEmbeddedData<'a, P0, P1>(indexer: MrmResourceIndexerHandle, resourceuri: P0, embeddeddata: *const u8, embeddeddatasize: u32, qualifiers: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmIndexEmbeddedData(indexer: MrmResourceIndexerHandle, resourceuri: ::windows::core::PCWSTR, embeddeddata: *const u8, embeddeddatasize: u32, qualifiers: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmIndexEmbeddedData(::core::mem::transmute(indexer), resourceuri.into(), ::core::mem::transmute(embeddeddata), embeddeddatasize, qualifiers.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmIndexFile<'a, P0, P1, P2>(indexer: MrmResourceIndexerHandle, resourceuri: P0, filepath: P1, qualifiers: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmIndexFile(indexer: MrmResourceIndexerHandle, resourceuri: ::windows::core::PCWSTR, filepath: ::windows::core::PCWSTR, qualifiers: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmIndexFile(::core::mem::transmute(indexer), resourceuri.into(), filepath.into(), qualifiers.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmIndexFileAutoQualifiers<'a, P0>(indexer: MrmResourceIndexerHandle, filepath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmIndexFileAutoQualifiers(indexer: MrmResourceIndexerHandle, filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmIndexFileAutoQualifiers(::core::mem::transmute(indexer), filepath.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmIndexResourceContainerAutoQualifiers<'a, P0>(indexer: MrmResourceIndexerHandle, containerpath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmIndexResourceContainerAutoQualifiers(indexer: MrmResourceIndexerHandle, containerpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmIndexResourceContainerAutoQualifiers(::core::mem::transmute(indexer), containerpath.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmIndexString<'a, P0, P1, P2>(indexer: MrmResourceIndexerHandle, resourceuri: P0, resourcestring: P1, qualifiers: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmIndexString(indexer: MrmResourceIndexerHandle, resourceuri: ::windows::core::PCWSTR, resourcestring: ::windows::core::PCWSTR, qualifiers: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    MrmIndexString(::core::mem::transmute(indexer), resourceuri.into(), resourcestring.into(), qualifiers.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MrmIndexerFlags(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmIndexerFlagsNone: MrmIndexerFlags = MrmIndexerFlags(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmIndexerFlagsAutoMerge: MrmIndexerFlags = MrmIndexerFlags(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmIndexerFlagsCreateContentChecksum: MrmIndexerFlags = MrmIndexerFlags(2i32);
impl ::core::marker::Copy for MrmIndexerFlags {}
impl ::core::clone::Clone for MrmIndexerFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmIndexerFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MrmIndexerFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for MrmIndexerFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmIndexerFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MrmPackagingMode(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPackagingModeStandaloneFile: MrmPackagingMode = MrmPackagingMode(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPackagingModeAutoSplit: MrmPackagingMode = MrmPackagingMode(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPackagingModeResourcePack: MrmPackagingMode = MrmPackagingMode(2i32);
impl ::core::marker::Copy for MrmPackagingMode {}
impl ::core::clone::Clone for MrmPackagingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmPackagingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MrmPackagingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MrmPackagingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPackagingMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MrmPackagingOptions(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPackagingOptionsNone: MrmPackagingOptions = MrmPackagingOptions(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPackagingOptionsOmitSchemaFromResourcePacks: MrmPackagingOptions = MrmPackagingOptions(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPackagingOptionsSplitLanguageVariants: MrmPackagingOptions = MrmPackagingOptions(2i32);
impl ::core::marker::Copy for MrmPackagingOptions {}
impl ::core::clone::Clone for MrmPackagingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmPackagingOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MrmPackagingOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MrmPackagingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPackagingOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn MrmPeekResourceIndexerMessages(handle: MrmResourceIndexerHandle, messages: *mut *mut MrmResourceIndexerMessage, nummsgs: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MrmPeekResourceIndexerMessages(handle: MrmResourceIndexerHandle, messages: *mut *mut MrmResourceIndexerMessage, nummsgs: *mut u32) -> ::windows::core::HRESULT;
    }
    MrmPeekResourceIndexerMessages(::core::mem::transmute(handle), ::core::mem::transmute(messages), ::core::mem::transmute(nummsgs)).ok()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MrmPlatformVersion(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPlatformVersion_Default: MrmPlatformVersion = MrmPlatformVersion(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPlatformVersion_Windows10_0_0_0: MrmPlatformVersion = MrmPlatformVersion(17432576i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmPlatformVersion_Windows10_0_0_5: MrmPlatformVersion = MrmPlatformVersion(17432581i32);
impl ::core::marker::Copy for MrmPlatformVersion {}
impl ::core::clone::Clone for MrmPlatformVersion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmPlatformVersion {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MrmPlatformVersion {
    type Abi = Self;
}
impl ::core::fmt::Debug for MrmPlatformVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPlatformVersion").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MrmResourceIndexerHandle {
    pub handle: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MrmResourceIndexerHandle {}
impl ::core::clone::Clone for MrmResourceIndexerHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmResourceIndexerHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceIndexerHandle").field("handle", &self.handle).finish()
    }
}
unsafe impl ::windows::core::Abi for MrmResourceIndexerHandle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MrmResourceIndexerHandle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MrmResourceIndexerHandle>()) == 0 }
    }
}
impl ::core::cmp::Eq for MrmResourceIndexerHandle {}
impl ::core::default::Default for MrmResourceIndexerHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct MrmResourceIndexerMessage {
    pub severity: MrmResourceIndexerMessageSeverity,
    pub id: u32,
    pub text: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for MrmResourceIndexerMessage {}
impl ::core::clone::Clone for MrmResourceIndexerMessage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmResourceIndexerMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceIndexerMessage").field("severity", &self.severity).field("id", &self.id).field("text", &self.text).finish()
    }
}
unsafe impl ::windows::core::Abi for MrmResourceIndexerMessage {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MrmResourceIndexerMessage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MrmResourceIndexerMessage>()) == 0 }
    }
}
impl ::core::cmp::Eq for MrmResourceIndexerMessage {}
impl ::core::default::Default for MrmResourceIndexerMessage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MrmResourceIndexerMessageSeverity(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmResourceIndexerMessageSeverityVerbose: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmResourceIndexerMessageSeverityInfo: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmResourceIndexerMessageSeverityWarning: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(2i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MrmResourceIndexerMessageSeverityError: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(3i32);
impl ::core::marker::Copy for MrmResourceIndexerMessageSeverity {}
impl ::core::clone::Clone for MrmResourceIndexerMessageSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmResourceIndexerMessageSeverity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MrmResourceIndexerMessageSeverity {
    type Abi = Self;
}
impl ::core::fmt::Debug for MrmResourceIndexerMessageSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmResourceIndexerMessageSeverity").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsgWaitForMultipleObjects<'a, P0>(phandles: &[super::super::Foundation::HANDLE], fwaitall: P0, dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsgWaitForMultipleObjects(ncount: u32, phandles: *const super::super::Foundation::HANDLE, fwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS) -> u32;
    }
    MsgWaitForMultipleObjects(phandles.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(phandles)), fwaitall.into(), dwmilliseconds, dwwakemask)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsgWaitForMultipleObjectsEx(phandles: &[super::super::Foundation::HANDLE], dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS, dwflags: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsgWaitForMultipleObjectsEx(ncount: u32, phandles: *const super::super::Foundation::HANDLE, dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS, dwflags: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS) -> u32;
    }
    MsgWaitForMultipleObjectsEx(phandles.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(phandles)), dwmilliseconds, dwwakemask, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type NAMEENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type NAMEENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NCCALCSIZE_PARAMS {
    pub rgrc: [super::super::Foundation::RECT; 3],
    pub lppos: *mut WINDOWPOS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NCCALCSIZE_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NCCALCSIZE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NCCALCSIZE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCCALCSIZE_PARAMS").field("rgrc", &self.rgrc).field("lppos", &self.lppos).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NCCALCSIZE_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NCCALCSIZE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NCCALCSIZE_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NCCALCSIZE_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCCALCSIZE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NFR_ANSI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NFR_UNICODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NF_QUERY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NF_REQUERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NID_EXTERNAL_PEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NID_EXTERNAL_TOUCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NID_INTEGRATED_PEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NID_INTEGRATED_TOUCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NID_MULTI_INPUT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const NID_READY: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NONCLIENTMETRICSA {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::super::Graphics::Gdi::LOGFONTA,
    pub lfStatusFont: super::super::Graphics::Gdi::LOGFONTA,
    pub lfMessageFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iPaddedBorderWidth: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NONCLIENTMETRICSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NONCLIENTMETRICSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NONCLIENTMETRICSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NONCLIENTMETRICSA")
            .field("cbSize", &self.cbSize)
            .field("iBorderWidth", &self.iBorderWidth)
            .field("iScrollWidth", &self.iScrollWidth)
            .field("iScrollHeight", &self.iScrollHeight)
            .field("iCaptionWidth", &self.iCaptionWidth)
            .field("iCaptionHeight", &self.iCaptionHeight)
            .field("lfCaptionFont", &self.lfCaptionFont)
            .field("iSmCaptionWidth", &self.iSmCaptionWidth)
            .field("iSmCaptionHeight", &self.iSmCaptionHeight)
            .field("lfSmCaptionFont", &self.lfSmCaptionFont)
            .field("iMenuWidth", &self.iMenuWidth)
            .field("iMenuHeight", &self.iMenuHeight)
            .field("lfMenuFont", &self.lfMenuFont)
            .field("lfStatusFont", &self.lfStatusFont)
            .field("lfMessageFont", &self.lfMessageFont)
            .field("iPaddedBorderWidth", &self.iPaddedBorderWidth)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NONCLIENTMETRICSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NONCLIENTMETRICSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NONCLIENTMETRICSA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NONCLIENTMETRICSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NONCLIENTMETRICSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NONCLIENTMETRICSW {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfStatusFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfMessageFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iPaddedBorderWidth: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NONCLIENTMETRICSW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NONCLIENTMETRICSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NONCLIENTMETRICSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NONCLIENTMETRICSW")
            .field("cbSize", &self.cbSize)
            .field("iBorderWidth", &self.iBorderWidth)
            .field("iScrollWidth", &self.iScrollWidth)
            .field("iScrollHeight", &self.iScrollHeight)
            .field("iCaptionWidth", &self.iCaptionWidth)
            .field("iCaptionHeight", &self.iCaptionHeight)
            .field("lfCaptionFont", &self.lfCaptionFont)
            .field("iSmCaptionWidth", &self.iSmCaptionWidth)
            .field("iSmCaptionHeight", &self.iSmCaptionHeight)
            .field("lfSmCaptionFont", &self.lfSmCaptionFont)
            .field("iMenuWidth", &self.iMenuWidth)
            .field("iMenuHeight", &self.iMenuHeight)
            .field("lfMenuFont", &self.lfMenuFont)
            .field("lfStatusFont", &self.lfStatusFont)
            .field("lfMessageFont", &self.lfMessageFont)
            .field("iPaddedBorderWidth", &self.iPaddedBorderWidth)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for NONCLIENTMETRICSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NONCLIENTMETRICSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NONCLIENTMETRICSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NONCLIENTMETRICSW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NONCLIENTMETRICSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBJECT_IDENTIFIER(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_WINDOW: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_SYSMENU: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_TITLEBAR: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-2i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_MENU: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-3i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_CLIENT: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-4i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_VSCROLL: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-5i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_HSCROLL: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-6i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_SIZEGRIP: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-7i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_CARET: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-8i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_CURSOR: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-9i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_ALERT: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-10i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_SOUND: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-11i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_QUERYCLASSNAMEIDX: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-12i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBJID_NATIVEOM: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-16i32);
impl ::core::marker::Copy for OBJECT_IDENTIFIER {}
impl ::core::clone::Clone for OBJECT_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_IDENTIFIER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OBJECT_IDENTIFIER {
    type Abi = Self;
}
impl ::core::fmt::Debug for OBJECT_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_IDENTIFIER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_BTNCORNERS: u32 = 32758u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_BTSIZE: u32 = 32761u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_CHECK: u32 = 32760u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_CHECKBOXES: u32 = 32759u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_CLOSE: u32 = 32754u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_COMBO: u32 = 32738u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_DNARROW: u32 = 32752u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_DNARROWD: u32 = 32742u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_DNARROWI: u32 = 32736u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_LFARROW: u32 = 32750u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_LFARROWD: u32 = 32740u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_LFARROWI: u32 = 32734u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_MNARROW: u32 = 32739u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_CLOSE: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_DNARROW: u32 = 32764u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_LFARROW: u32 = 32762u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_REDUCE: u32 = 32757u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_RESTORE: u32 = 32755u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_RGARROW: u32 = 32763u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_UPARROW: u32 = 32765u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_OLD_ZOOM: u32 = 32756u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_REDUCE: u32 = 32749u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_REDUCED: u32 = 32746u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_RESTORE: u32 = 32747u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_RESTORED: u32 = 32744u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_RGARROW: u32 = 32751u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_RGARROWD: u32 = 32741u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_RGARROWI: u32 = 32735u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_SIZE: u32 = 32766u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_UPARROW: u32 = 32753u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_UPARROWD: u32 = 32743u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_UPARROWI: u32 = 32737u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_ZOOM: u32 = 32748u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OBM_ZOOMD: u32 = 32745u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_ICOCUR: u32 = 32647u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_ICON: u32 = 32641u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_SIZE: u32 = 32640u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODA_DRAWENTIRE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODA_FOCUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODA_SELECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_CHECKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_COMBOBOXEDIT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_DEFAULT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_FOCUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_GRAYED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_HOTLIGHT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_INACTIVE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_NOACCEL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_NOFOCUSRECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ODS_SELECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_BANG: u32 = 32515u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_ERROR: u32 = 32513u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_HAND: u32 = 32513u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_INFORMATION: u32 = 32516u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_NOTE: u32 = 32516u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_QUES: u32 = 32514u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_SAMPLE: u32 = 32512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_SHIELD: u32 = 32518u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_WARNING: u32 = 32515u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OIC_WINLOGO: u32 = 32517u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ORD_LANGDRIVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OemToCharA<'a, P0>(psrc: P0, pdst: ::windows::core::PSTR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OemToCharA(psrc: ::windows::core::PCSTR, pdst: ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    OemToCharA(psrc.into(), ::core::mem::transmute(pdst))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OemToCharBuffA<'a, P0>(lpszsrc: P0, lpszdst: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OemToCharBuffA(lpszsrc: ::windows::core::PCSTR, lpszdst: ::windows::core::PSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    }
    OemToCharBuffA(lpszsrc.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszdst)), lpszdst.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OemToCharBuffW<'a, P0>(lpszsrc: P0, lpszdst: &mut [u16]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OemToCharBuffW(lpszsrc: ::windows::core::PCSTR, lpszdst: ::windows::core::PWSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    }
    OemToCharBuffW(lpszsrc.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszdst)), lpszdst.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OemToCharW<'a, P0>(psrc: P0, pdst: ::windows::core::PWSTR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OemToCharW(psrc: ::windows::core::PCSTR, pdst: ::windows::core::PWSTR) -> super::super::Foundation::BOOL;
    }
    OemToCharW(psrc.into(), ::core::mem::transmute(pdst))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenIcon<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenIcon(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    OpenIcon(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PA_ACTIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PA_NOACTIVATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBTF_APMRESUMEFROMFAILURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMBATTERYLOW: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMOEMEVENT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMPOWERSTATUSCHANGE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMQUERYSTANDBY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMQUERYSTANDBYFAILED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMQUERYSUSPEND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMQUERYSUSPENDFAILED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMRESUMEAUTOMATIC: u32 = 18u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMRESUMECRITICAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMRESUMESTANDBY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMRESUMESUSPEND: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMSTANDBY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_APMSUSPEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PBT_POWERSETTINGCHANGE: u32 = 32787u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_ARRIVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_MAPPING_CHANGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_MODE_ASPECTRATIOPRESERVED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_MODE_CENTERED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_MODE_DEFAULT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_ORIENTATION_0: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_ORIENTATION_180: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_ORIENTATION_270: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_ORIENTATION_90: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_ORIGIN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_REMOVAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PDC_RESOLUTION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEEK_MESSAGE_REMOVE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PM_NOREMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PM_REMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PM_NOYIELD: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PM_QS_INPUT: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(67567616u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PM_QS_POSTMESSAGE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(9961472u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PM_QS_PAINT: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(2097152u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PM_QS_SENDMESSAGE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(4194304u32);
impl ::core::marker::Copy for PEEK_MESSAGE_REMOVE_TYPE {}
impl ::core::clone::Clone for PEEK_MESSAGE_REMOVE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEEK_MESSAGE_REMOVE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PEEK_MESSAGE_REMOVE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PEEK_MESSAGE_REMOVE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEEK_MESSAGE_REMOVE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PEEK_MESSAGE_REMOVE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PEEK_MESSAGE_REMOVE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENARBITRATIONTYPE_FIS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENARBITRATIONTYPE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENARBITRATIONTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENARBITRATIONTYPE_SPT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENARBITRATIONTYPE_WIN8: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENVISUALIZATION_CURSOR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENVISUALIZATION_DOUBLETAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENVISUALIZATION_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENVISUALIZATION_ON: u32 = 35u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PENVISUALIZATION_TAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_FLAG_BARREL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_FLAG_ERASER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_FLAG_INVERTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_MASK_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_MASK_PRESSURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_MASK_ROTATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_MASK_TILT_X: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PEN_MASK_TILT_Y: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PMB_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_DEVICE_PRODUCT_STRING_MAX: u32 = 520u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POINTER_INPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PT_POINTER: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PT_TOUCH: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PT_PEN: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PT_MOUSE: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PT_TOUCHPAD: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(5i32);
impl ::core::marker::Copy for POINTER_INPUT_TYPE {}
impl ::core::clone::Clone for POINTER_INPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for POINTER_INPUT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for POINTER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_INPUT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_CANCELED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_CONFIDENCE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_FIFTHBUTTON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_FIRSTBUTTON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_FOURTHBUTTON: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_INCONTACT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_INRANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_PRIMARY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_SECONDBUTTON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MESSAGE_FLAG_THIRDBUTTON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MOD_CTRL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const POINTER_MOD_SHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PREGISTERCLASSNAMEW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PRF_CHECKVISIBLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PRF_CHILDREN: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PRF_CLIENT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PRF_ERASEBKGND: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PRF_NONCLIENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PRF_OWNED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PROPENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: ::windows::core::PCSTR, param2: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PROPENUMPROCEXA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: ::windows::core::PCSTR, param2: super::super::Foundation::HANDLE, param3: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PROPENUMPROCEXW = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: ::windows::core::PCWSTR, param2: super::super::Foundation::HANDLE, param3: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PROPENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: ::windows::core::PCWSTR, param2: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PWR_CRITICALRESUME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PWR_FAIL: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PWR_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PWR_SUSPENDREQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PWR_SUSPENDRESUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const PW_RENDERFULLCONTENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeekMessageA<'a, P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PeekMessageA(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL;
    }
    PeekMessageA(::core::mem::transmute(lpmsg), hwnd.into(), wmsgfiltermin, wmsgfiltermax, wremovemsg)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeekMessageW<'a, P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PeekMessageW(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL;
    }
    PeekMessageW(::core::mem::transmute(lpmsg), hwnd.into(), wmsgfiltermin, wmsgfiltermax, wremovemsg)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PhysicalToLogicalPoint<'a, P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PhysicalToLogicalPoint(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    PhysicalToLogicalPoint(hwnd.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostMessageA<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PostMessageA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    PostMessageA(hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostMessageW<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PostMessageW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    PostMessageW(hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn PostQuitMessage(nexitcode: i32) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PostQuitMessage(nexitcode: i32);
    }
    PostQuitMessage(nexitcode)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostThreadMessageA<'a, P0, P1>(idthread: u32, msg: u32, wparam: P0, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PostThreadMessageA(idthread: u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    PostThreadMessageA(idthread, msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostThreadMessageW<'a, P0, P1>(idthread: u32, msg: u32, wparam: P0, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PostThreadMessageW(idthread: u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    PostThreadMessageW(idthread, msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn PrivateExtractIconsA(szfilename: &[u8; 260], niconindex: i32, cxicon: i32, cyicon: i32, phicon: *mut HICON, piconid: *mut u32, nicons: u32, flags: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrivateExtractIconsA(szfilename: ::windows::core::PCSTR, niconindex: i32, cxicon: i32, cyicon: i32, phicon: *mut HICON, piconid: *mut u32, nicons: u32, flags: u32) -> u32;
    }
    PrivateExtractIconsA(::core::mem::transmute(::windows::core::as_ptr_or_null(szfilename)), niconindex, cxicon, cyicon, ::core::mem::transmute(phicon), ::core::mem::transmute(piconid), ::core::mem::transmute(nicons), flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn PrivateExtractIconsW(szfilename: &[u16; 260], niconindex: i32, cxicon: i32, cyicon: i32, phicon: *mut HICON, piconid: *mut u32, nicons: u32, flags: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrivateExtractIconsW(szfilename: ::windows::core::PCWSTR, niconindex: i32, cxicon: i32, cyicon: i32, phicon: *mut HICON, piconid: *mut u32, nicons: u32, flags: u32) -> u32;
    }
    PrivateExtractIconsW(::core::mem::transmute(::windows::core::as_ptr_or_null(szfilename)), niconindex, cxicon, cyicon, ::core::mem::transmute(phicon), ::core::mem::transmute(piconid), ::core::mem::transmute(nicons), flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_POINTER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_TOUCH: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QUEUE_STATUS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_ALLEVENTS: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1215u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_ALLINPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1279u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_ALLPOSTMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_HOTKEY: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_INPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1031u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_KEY: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_MOUSE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(6u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_MOUSEBUTTON: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_MOUSEMOVE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_PAINT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_POSTMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_RAWINPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_SENDMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const QS_TIMER: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(16u32);
impl ::core::marker::Copy for QUEUE_STATUS_FLAGS {}
impl ::core::clone::Clone for QUEUE_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QUEUE_STATUS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUEUE_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUEUE_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUEUE_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RES_CURSOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RES_ICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RIDEV_EXMODEMASK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RIM_INPUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RIM_INPUTSINK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RIM_TYPEMAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_KEY_BREAK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_KEY_E0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_KEY_E1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_KEY_MAKE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_KEY_TERMSRV_SET_LED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_KEY_TERMSRV_SHADOW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_1_DOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_1_UP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_2_DOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_2_UP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_3_DOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_3_UP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_4_DOWN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_4_UP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_5_DOWN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_BUTTON_5_UP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_HWHEEL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_LEFT_BUTTON_DOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_LEFT_BUTTON_UP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_MIDDLE_BUTTON_UP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_RIGHT_BUTTON_UP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RI_MOUSE_WHEEL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_ACCELERATOR: ::windows::core::PCWSTR = ::windows::core::PCWSTR(9i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_ANICURSOR: ::windows::core::PCWSTR = ::windows::core::PCWSTR(21i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_ANIICON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(22i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_BITMAP: ::windows::core::PCWSTR = ::windows::core::PCWSTR(2i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_CURSOR: ::windows::core::PCWSTR = ::windows::core::PCWSTR(1i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_DIALOG: ::windows::core::PCWSTR = ::windows::core::PCWSTR(5i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_DLGINCLUDE: ::windows::core::PCWSTR = ::windows::core::PCWSTR(17i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_FONT: ::windows::core::PCWSTR = ::windows::core::PCWSTR(8i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_FONTDIR: ::windows::core::PCWSTR = ::windows::core::PCWSTR(7i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_HTML: ::windows::core::PCWSTR = ::windows::core::PCWSTR(23i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_ICON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(3i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_MANIFEST: u32 = 24u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_MENU: ::windows::core::PCWSTR = ::windows::core::PCWSTR(4i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_MESSAGETABLE: ::windows::core::PCWSTR = ::windows::core::PCWSTR(11i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_PLUGPLAY: ::windows::core::PCWSTR = ::windows::core::PCWSTR(19i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_VERSION: ::windows::core::PCWSTR = ::windows::core::PCWSTR(16i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const RT_VXD: ::windows::core::PCWSTR = ::windows::core::PCWSTR(20i32 as _);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RealChildWindowFromPoint<'a, P0>(hwndparent: P0, ptparentclientcoords: super::super::Foundation::POINT) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RealChildWindowFromPoint(hwndparent: super::super::Foundation::HWND, ptparentclientcoords: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    }
    RealChildWindowFromPoint(hwndparent.into(), ::core::mem::transmute(ptparentclientcoords))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RealGetWindowClassA<'a, P0>(hwnd: P0, ptszclassname: &mut [u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RealGetWindowClassA(hwnd: super::super::Foundation::HWND, ptszclassname: ::windows::core::PSTR, cchclassnamemax: u32) -> u32;
    }
    RealGetWindowClassA(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ptszclassname)), ptszclassname.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RealGetWindowClassW<'a, P0>(hwnd: P0, ptszclassname: &mut [u16]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RealGetWindowClassW(hwnd: super::super::Foundation::HWND, ptszclassname: ::windows::core::PWSTR, cchclassnamemax: u32) -> u32;
    }
    RealGetWindowClassW(hwnd.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ptszclassname)), ptszclassname.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16;
    }
    RegisterClassA(::core::mem::transmute(lpwndclass))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn RegisterClassExA(param0: *const WNDCLASSEXA) -> u16 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterClassExA(param0: *const WNDCLASSEXA) -> u16;
    }
    RegisterClassExA(::core::mem::transmute(param0))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn RegisterClassExW(param0: *const WNDCLASSEXW) -> u16 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterClassExW(param0: *const WNDCLASSEXW) -> u16;
    }
    RegisterClassExW(::core::mem::transmute(param0))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn RegisterClassW(lpwndclass: *const WNDCLASSW) -> u16 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterClassW(lpwndclass: *const WNDCLASSW) -> u16;
    }
    RegisterClassW(::core::mem::transmute(lpwndclass))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Power\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn RegisterDeviceNotificationA<'a, P0>(hrecipient: P0, notificationfilter: *const ::core::ffi::c_void, flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterDeviceNotificationA(hrecipient: super::super::Foundation::HANDLE, notificationfilter: *const ::core::ffi::c_void, flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS) -> *mut ::core::ffi::c_void;
    }
    RegisterDeviceNotificationA(hrecipient.into(), ::core::mem::transmute(notificationfilter), flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Power\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn RegisterDeviceNotificationW<'a, P0>(hrecipient: P0, notificationfilter: *const ::core::ffi::c_void, flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterDeviceNotificationW(hrecipient: super::super::Foundation::HANDLE, notificationfilter: *const ::core::ffi::c_void, flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS) -> *mut ::core::ffi::c_void;
    }
    RegisterDeviceNotificationW(hrecipient.into(), ::core::mem::transmute(notificationfilter), flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterShellHookWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterShellHookWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    RegisterShellHookWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn RegisterWindowMessageA<'a, P0>(lpstring: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterWindowMessageA(lpstring: ::windows::core::PCSTR) -> u32;
    }
    RegisterWindowMessageA(lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn RegisterWindowMessageW<'a, P0>(lpstring: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterWindowMessageW(lpstring: ::windows::core::PCWSTR) -> u32;
    }
    RegisterWindowMessageW(lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveMenu<'a, P0>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveMenu(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL;
    }
    RemoveMenu(hmenu.into(), uposition, uflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemovePropA<'a, P0, P1>(hwnd: P0, lpstring: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemovePropA(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
    }
    let result__ = RemovePropA(hwnd.into(), lpstring.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemovePropW<'a, P0, P1>(hwnd: P0, lpstring: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemovePropW(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
    }
    let result__ = RemovePropW(hwnd.into(), lpstring.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplyMessage<'a, P0>(lresult: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LRESULT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReplyMessage(lresult: super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL;
    }
    ReplyMessage(lresult.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_ENABLE_ARROWS: u32 = 228u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_GETPOS: u32 = 225u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_GETRANGE: u32 = 227u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_GETSCROLLBARINFO: u32 = 235u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_GETSCROLLINFO: u32 = 234u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_SETPOS: u32 = 224u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_SETRANGE: u32 = 226u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_SETRANGEREDRAW: u32 = 230u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBM_SETSCROLLINFO: u32 = 233u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_BOTTOMALIGN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_HORZ: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_LEFTALIGN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_RIGHTALIGN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_SIZEBOX: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_SIZEBOXBOTTOMRIGHTALIGN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_SIZEBOXTOPLEFTALIGN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_SIZEGRIP: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_TOPALIGN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SBS_VERT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_BOTTOM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_ENDSCROLL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_LEFT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_LINEDOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_LINELEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_LINERIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_LINEUP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_PAGEDOWN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_PAGELEFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_PAGERIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_PAGEUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_RIGHT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_THUMBPOSITION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_THUMBTRACK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_TOP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SCF_ISSECURE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCROLLBARINFO {
    pub cbSize: u32,
    pub rcScrollBar: super::super::Foundation::RECT,
    pub dxyLineButton: i32,
    pub xyThumbTop: i32,
    pub xyThumbBottom: i32,
    pub reserved: i32,
    pub rgstate: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCROLLBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCROLLBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCROLLBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCROLLBARINFO").field("cbSize", &self.cbSize).field("rcScrollBar", &self.rcScrollBar).field("dxyLineButton", &self.dxyLineButton).field("xyThumbTop", &self.xyThumbTop).field("xyThumbBottom", &self.xyThumbBottom).field("reserved", &self.reserved).field("rgstate", &self.rgstate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCROLLBARINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCROLLBARINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCROLLBARINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCROLLBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCROLLBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCROLLBAR_CONSTANTS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_CTL: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_HORZ: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_VERT: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SB_BOTH: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(3u32);
impl ::core::marker::Copy for SCROLLBAR_CONSTANTS {}
impl ::core::clone::Clone for SCROLLBAR_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCROLLBAR_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCROLLBAR_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCROLLBAR_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLBAR_CONSTANTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SCROLLBAR_CONSTANTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SCROLLBAR_CONSTANTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct SCROLLINFO {
    pub cbSize: u32,
    pub fMask: SCROLLINFO_MASK,
    pub nMin: i32,
    pub nMax: i32,
    pub nPage: u32,
    pub nPos: i32,
    pub nTrackPos: i32,
}
impl ::core::marker::Copy for SCROLLINFO {}
impl ::core::clone::Clone for SCROLLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCROLLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCROLLINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("nMin", &self.nMin).field("nMax", &self.nMax).field("nPage", &self.nPage).field("nPos", &self.nPos).field("nTrackPos", &self.nTrackPos).finish()
    }
}
unsafe impl ::windows::core::Abi for SCROLLINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCROLLINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCROLLINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCROLLINFO {}
impl ::core::default::Default for SCROLLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCROLLINFO_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIF_ALL: SCROLLINFO_MASK = SCROLLINFO_MASK(23u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIF_DISABLENOSCROLL: SCROLLINFO_MASK = SCROLLINFO_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIF_PAGE: SCROLLINFO_MASK = SCROLLINFO_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIF_POS: SCROLLINFO_MASK = SCROLLINFO_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIF_RANGE: SCROLLINFO_MASK = SCROLLINFO_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIF_TRACKPOS: SCROLLINFO_MASK = SCROLLINFO_MASK(16u32);
impl ::core::marker::Copy for SCROLLINFO_MASK {}
impl ::core::clone::Clone for SCROLLINFO_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCROLLINFO_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCROLLINFO_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCROLLINFO_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLINFO_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SCROLLINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SCROLLINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SCROLLINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SCROLLINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SCROLLINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_ARRANGE: u32 = 61712u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_CLOSE: u32 = 61536u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_CONTEXTHELP: u32 = 61824u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_DEFAULT: u32 = 61792u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_HOTKEY: u32 = 61776u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_HSCROLL: u32 = 61568u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_ICON: u32 = 61472u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_KEYMENU: u32 = 61696u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_MAXIMIZE: u32 = 61488u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_MINIMIZE: u32 = 61472u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_MONITORPOWER: u32 = 61808u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_MOUSEMENU: u32 = 61584u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_MOVE: u32 = 61456u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_NEXTWINDOW: u32 = 61504u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_PREVWINDOW: u32 = 61520u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_RESTORE: u32 = 61728u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_SEPARATOR: u32 = 61455u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_SIZE: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_TASKLIST: u32 = 61744u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_VSCROLL: u32 = 61552u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SC_ZOOM: u32 = 61488u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SENDASYNCPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: usize, param3: super::super::Foundation::LRESULT)>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SEND_MESSAGE_TIMEOUT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SMTO_ABORTIFHUNG: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SMTO_BLOCK: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SMTO_NORMAL: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SMTO_NOTIMEOUTIFNOTHUNG: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SMTO_ERRORONEXIT: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(32u32);
impl ::core::marker::Copy for SEND_MESSAGE_TIMEOUT_FLAGS {}
impl ::core::clone::Clone for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEND_MESSAGE_TIMEOUT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SET_WINDOW_POS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_ASYNCWINDOWPOS: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_DEFERERASE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_DRAWFRAME: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_FRAMECHANGED: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_HIDEWINDOW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOACTIVATE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOCOPYBITS: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOMOVE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOOWNERZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOREDRAW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOREPOSITION: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOSENDCHANGING: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOSIZE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_NOZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP_SHOWWINDOW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SWP__NOOWNERZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
impl ::core::marker::Copy for SET_WINDOW_POS_FLAGS {}
impl ::core::clone::Clone for SET_WINDOW_POS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_WINDOW_POS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SET_WINDOW_POS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SET_WINDOW_POS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_WINDOW_POS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SET_WINDOW_POS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SET_WINDOW_POS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SHELLHOOKINFO {
    pub hwnd: super::super::Foundation::HWND,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SHELLHOOKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SHELLHOOKINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SHELLHOOKINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHELLHOOKINFO").field("hwnd", &self.hwnd).field("rc", &self.rc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SHELLHOOKINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SHELLHOOKINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHELLHOOKINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SHELLHOOKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHELLHOOKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SHOW_FULLSCREEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SHOW_ICONWINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SHOW_OPENNOACTIVATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SHOW_OPENWINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHOW_WINDOW_CMD(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_FORCEMINIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(11u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_HIDE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_MAXIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_MINIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(6u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_RESTORE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(9u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOW: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(5u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOWDEFAULT: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(10u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOWMAXIMIZED: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOWMINIMIZED: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOWMINNOACTIVE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(7u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOWNA: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOWNOACTIVATE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SHOWNORMAL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_NORMAL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_MAX: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(11u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_PARENTCLOSING: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_OTHERZOOM: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_PARENTOPENING: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_OTHERUNZOOM: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SCROLLCHILDREN: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_INVALIDATE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_ERASE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SW_SMOOTHSCROLL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(16u32);
impl ::core::marker::Copy for SHOW_WINDOW_CMD {}
impl ::core::clone::Clone for SHOW_WINDOW_CMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHOW_WINDOW_CMD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SHOW_WINDOW_CMD {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHOW_WINDOW_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHOW_WINDOW_CMD").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHOW_WINDOW_CMD {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHOW_WINDOW_CMD {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHOW_WINDOW_CMD {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHOW_WINDOW_CMD {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHOW_WINDOW_CMD {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZEFULLSCREEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZEICONIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZENORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZEZOOMHIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZEZOOMSHOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZE_MAXHIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZE_MAXIMIZED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZE_MAXSHOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZE_MINIMIZED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SIZE_RESTORED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CARETBLINKINGENABLED: u32 = 8194u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CMETRICS: u32 = 76u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_RESERVED1: u32 = 24u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_RESERVED2: u32 = 25u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_RESERVED3: u32 = 26u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_RESERVED4: u32 = 27u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_APPEND: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_APPSTART: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_BEEP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_FAULT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_INFORMATION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_MAXIMIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_MENUCOMMAND: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_MENUPOPUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_MINIMIZE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_QUESTION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_RESTOREDOWN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_RESTOREUP: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_SHUTDOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_STARTUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SOUND_SYSTEM_WARNING: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_BITMAP: i32 = 14i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_BLACKFRAME: i32 = 7i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_BLACKRECT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_CENTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_CENTERIMAGE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_EDITCONTROL: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_ELLIPSISMASK: i32 = 49152i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_ENDELLIPSIS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_ENHMETAFILE: i32 = 15i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_ETCHEDFRAME: i32 = 18i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_ETCHEDHORZ: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_ETCHEDVERT: i32 = 17i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_GRAYFRAME: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_GRAYRECT: i32 = 5i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_ICON: i32 = 3i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_LEFT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_LEFTNOWORDWRAP: i32 = 12i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_NOPREFIX: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_NOTIFY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_OWNERDRAW: i32 = 13i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_PATHELLIPSIS: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_REALSIZECONTROL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_REALSIZEIMAGE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_RIGHTJUST: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_SIMPLE: i32 = 11i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_SUNKEN: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_TYPEMASK: i32 = 31i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_USERITEM: i32 = 10i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_WHITEFRAME: i32 = 9i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_WHITERECT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SS_WORDELLIPSIS: i32 = 49152i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_ALERT_HIGH: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_ALERT_LOW: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_ALERT_MEDIUM: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_ANIMATED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_BUSY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_CHECKED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_COLLAPSED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_DEFAULT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_EXPANDED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_EXTSELECTABLE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_FLOATING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_FOCUSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_HOTTRACKED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_INDETERMINATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_LINKED: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_MARQUEED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_MIXED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_MOVEABLE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_MULTISELECTABLE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_PROTECTED: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_READONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_SELECTABLE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_SELECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_SELFVOICING: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_SIZEABLE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_TRAVERSED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STATE_SYSTEM_VALID: u32 = 1073741823u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STM_GETICON: u32 = 369u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STM_GETIMAGE: u32 = 371u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STM_MSGMAX: u32 = 372u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STM_SETICON: u32 = 368u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STM_SETIMAGE: u32 = 370u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STN_CLICKED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STN_DBLCLK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STN_DISABLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STN_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_E_END_OF_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024858i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_E_INSUFFICIENT_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024774i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_E_INVALID_PARAMETER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024809i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_FILL_BEHIND_NULL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_FILL_ON_FAILURE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_IGNORE_NULLS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_MAX_CCH: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_NO_TRUNCATION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_NULL_ON_FAILURE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const STRSAFE_USE_SECURE_CRT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct STYLESTRUCT {
    pub styleOld: u32,
    pub styleNew: u32,
}
impl ::core::marker::Copy for STYLESTRUCT {}
impl ::core::clone::Clone for STYLESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STYLESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLESTRUCT").field("styleOld", &self.styleOld).field("styleNew", &self.styleNew).finish()
    }
}
unsafe impl ::windows::core::Abi for STYLESTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STYLESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STYLESTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STYLESTRUCT {}
impl ::core::default::Default for STYLESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_CURSOR_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_APPSTARTING: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32650u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_NORMAL: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_CROSS: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32515u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_HAND: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32649u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_HELP: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32651u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_IBEAM: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32513u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_NO: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32648u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_SIZEALL: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32646u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_SIZENESW: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32643u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_SIZENS: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32645u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_SIZENWSE: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32642u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_SIZEWE: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32644u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_UP: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32516u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const OCR_WAIT: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32514u32);
impl ::core::marker::Copy for SYSTEM_CURSOR_ID {}
impl ::core::clone::Clone for SYSTEM_CURSOR_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_CURSOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_CURSOR_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSTEM_CURSOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_CURSOR_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_METRICS_INDEX(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_ARRANGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(56u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CLEANBOOT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(67u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CMONITORS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(80u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CMOUSEBUTTONS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(43u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CONVERTIBLESLATEMODE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8195u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(5u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXCURSOR: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(13u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXDLGFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(7u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXDOUBLECLK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(36u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXDRAG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(68u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXEDGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(45u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXFIXEDFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(7u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXFOCUSBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(83u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXFULLSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXHSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(21u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXHTHUMB: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(10u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(11u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXICONSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(38u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMAXIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(61u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMAXTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(59u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMENUCHECK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(71u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMENUSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(54u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMIN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(28u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMINIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(57u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMINSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(47u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXMINTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(34u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXPADDEDBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(92u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(30u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXSIZEFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXSMICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(49u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXSMSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(52u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(78u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CXVSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(6u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYCAPTION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYCURSOR: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(14u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYDLGFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYDOUBLECLK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(37u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYDRAG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(69u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYEDGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(46u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYFIXEDFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYFOCUSBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(84u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(33u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYFULLSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(17u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYHSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(12u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYICONSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(39u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYKANJIWINDOW: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(18u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMAXIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(62u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMAXTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(60u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMENU: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(15u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMENUCHECK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(72u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMENUSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(55u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMIN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(29u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMINIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(58u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMINSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(48u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYMINTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(35u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(31u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYSIZEFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(33u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYSMCAPTION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(51u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYSMICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(50u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYSMSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(53u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(79u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYVSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(20u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_CYVTHUMB: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(9u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_DBCSENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(42u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_DEBUG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(22u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_DIGITIZER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(94u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_IMMENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(82u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_MAXIMUMTOUCHES: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(95u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_MEDIACENTER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(87u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_MENUDROPALIGNMENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(40u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_MIDEASTENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(74u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_MOUSEPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(19u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_MOUSEHORIZONTALWHEELPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(91u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_MOUSEWHEELPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(75u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_NETWORK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(63u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_PENWINDOWS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(41u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_REMOTECONTROL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8193u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_REMOTESESSION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SAMEDISPLAYFORMAT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(81u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SECURE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(44u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SERVERR2: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(89u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SHOWSOUNDS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(70u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SHUTTINGDOWN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SLOWMACHINE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(73u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_STARTER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(88u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SWAPBUTTON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(23u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_SYSTEMDOCKED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8196u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_TABLETPC: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(86u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_XVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(76u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SM_YVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(77u32);
impl ::core::marker::Copy for SYSTEM_METRICS_INDEX {}
impl ::core::clone::Clone for SYSTEM_METRICS_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_METRICS_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_METRICS_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSTEM_METRICS_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_METRICS_INDEX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_PARAMETERS_INFO_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETBEEP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETBEEP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETBORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(5u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETBORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(6u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETKEYBOARDSPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(10u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETKEYBOARDSPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(11u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_LANGDRIVER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(12u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_ICONHORIZONTALSPACING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(13u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSCREENSAVETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(14u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSCREENSAVETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(15u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSCREENSAVEACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSCREENSAVEACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(17u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETGRIDGRANULARITY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(18u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETGRIDGRANULARITY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(19u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDESKWALLPAPER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(20u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDESKPATTERN: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(21u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETKEYBOARDDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(22u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETKEYBOARDDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(23u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_ICONVERTICALSPACING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(24u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETICONTITLEWRAP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(25u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETICONTITLEWRAP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(26u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMENUDROPALIGNMENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(27u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMENUDROPALIGNMENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(28u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDOUBLECLKWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(29u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDOUBLECLKHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(30u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETICONTITLELOGFONT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(31u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDOUBLECLICKTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEBUTTONSWAP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(33u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETICONTITLELOGFONT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(34u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFASTTASKSWITCH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(35u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFASTTASKSWITCH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(36u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDRAGFULLWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(37u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETDRAGFULLWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(38u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETNONCLIENTMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(41u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETNONCLIENTMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(42u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMINIMIZEDMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(43u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMINIMIZEDMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(44u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETICONMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(45u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETICONMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(46u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETWORKAREA: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(47u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETWORKAREA: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(48u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPENWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(49u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETHIGHCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(66u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETHIGHCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(67u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETKEYBOARDPREF: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(68u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETKEYBOARDPREF: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(69u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSCREENREADER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(70u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSCREENREADER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(71u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(72u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(73u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFONTSMOOTHING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(74u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFONTSMOOTHING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(75u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDRAGWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(76u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDRAGHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(77u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETHANDHELD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(78u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETLOWPOWERTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(79u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETPOWEROFFTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(80u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETLOWPOWERTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(81u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPOWEROFFTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(82u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETLOWPOWERACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(83u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETPOWEROFFACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(84u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETLOWPOWERACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(85u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPOWEROFFACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(86u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCURSORS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(87u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETICONS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(88u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETDEFAULTINPUTLANG: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(89u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDEFAULTINPUTLANG: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(90u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETLANGTOGGLE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(91u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETWINDOWSEXTENSION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(92u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSETRAILS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(93u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSETRAILS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(94u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(97u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(97u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFILTERKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(50u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFILTERKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(51u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETTOGGLEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(52u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETTOGGLEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(53u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(54u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(55u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSHOWSOUNDS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(56u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSHOWSOUNDS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(57u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSTICKYKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(58u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSTICKYKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(59u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETACCESSTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(60u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETACCESSTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(61u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSERIALKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(62u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSERIALKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(63u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSOUNDSENTRY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSOUNDSENTRY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(65u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSNAPTODEFBUTTON: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(95u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSNAPTODEFBUTTON: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(96u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEHOVERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(98u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEHOVERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(99u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEHOVERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(100u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEHOVERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(101u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEHOVERTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(102u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEHOVERTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(103u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETWHEELSCROLLLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(104u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETWHEELSCROLLLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(105u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMENUSHOWDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(106u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMENUSHOWDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(107u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETWHEELSCROLLCHARS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(108u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETWHEELSCROLLCHARS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(109u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSHOWIMEUI: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(110u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSHOWIMEUI: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(111u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSESPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(112u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSESPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(113u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(114u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETDESKWALLPAPER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(115u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETAUDIODESCRIPTION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(116u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETAUDIODESCRIPTION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(117u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSCREENSAVESECURE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(118u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSCREENSAVESECURE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(119u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETHUNGAPPTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(120u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETHUNGAPPTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(121u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETWAITTOKILLTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(122u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETWAITTOKILLTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(123u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(124u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(125u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(126u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(127u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETPENDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPENDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(129u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETWINARRANGING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(130u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETWINARRANGING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(131u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(132u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(133u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETPENDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(134u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPENDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(135u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(136u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(137u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETPENSIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(138u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPENSIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(139u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETDRAGFROMMAXIMIZE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(140u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDRAGFROMMAXIMIZE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(141u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSNAPSIZING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(142u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSNAPSIZING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(143u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETDOCKMOVING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(144u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDOCKMOVING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(145u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETTOUCHPREDICTIONPARAMETERS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(156u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETTOUCHPREDICTIONPARAMETERS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(157u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETLOGICALDPIOVERRIDE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(158u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETLOGICALDPIOVERRIDE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(159u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMENURECT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(162u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMENURECT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(163u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETACTIVEWINDOWTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETACTIVEWINDOWTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4097u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMENUANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4098u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMENUANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4099u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCOMBOBOXANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4100u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCOMBOBOXANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4101u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETLISTBOXSMOOTHSCROLLING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4102u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETLISTBOXSMOOTHSCROLLING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4103u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETGRADIENTCAPTIONS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4104u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETGRADIENTCAPTIONS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4105u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETKEYBOARDCUES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4106u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETKEYBOARDCUES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4107u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMENUUNDERLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4106u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMENUUNDERLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4107u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETACTIVEWNDTRKZORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4108u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETACTIVEWNDTRKZORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4109u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETHOTTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4110u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETHOTTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4111u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMENUFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4114u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMENUFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4115u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSELECTIONFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4116u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSELECTIONFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4117u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETTOOLTIPANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4118u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETTOOLTIPANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4119u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETTOOLTIPFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4120u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETTOOLTIPFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4121u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCURSORSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4122u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCURSORSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4123u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSESONAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4124u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSESONAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4125u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSECLICKLOCK: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4126u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSECLICKLOCK: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4127u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEVANISH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEVANISH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4129u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFLATMENU: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4130u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFLATMENU: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4131u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETDROPSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4132u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDROPSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4133u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETBLOCKSENDINPUTRESETS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4134u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETBLOCKSENDINPUTRESETS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4135u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETUIEFFECTS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4158u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETUIEFFECTS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4159u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4160u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4161u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCLIENTAREAANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4162u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCLIENTAREAANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4163u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCLEARTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4168u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCLEARTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4169u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSPEECHRECOGNITION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4170u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSPEECHRECOGNITION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4171u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCARETBROWSING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4172u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCARETBROWSING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4173u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETTHREADLOCALINPUTSETTINGS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4174u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETTHREADLOCALINPUTSETTINGS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4175u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETSYSTEMLANGUAGEBAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4176u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETSYSTEMLANGUAGEBAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4177u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8193u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETACTIVEWNDTRKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8194u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETACTIVEWNDTRKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8195u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFOREGROUNDFLASHCOUNT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8196u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFOREGROUNDFLASHCOUNT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8197u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCARETWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8198u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCARETWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8199u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSECLICKLOCKTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8200u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSECLICKLOCKTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8201u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFONTSMOOTHINGTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8202u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFONTSMOOTHINGTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8203u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFONTSMOOTHINGCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8204u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFONTSMOOTHINGCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8205u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFOCUSBORDERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8206u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFOCUSBORDERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8207u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFOCUSBORDERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8208u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFOCUSBORDERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8209u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETFONTSMOOTHINGORIENTATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8210u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETFONTSMOOTHINGORIENTATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8211u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMINIMUMHITRADIUS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8212u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMINIMUMHITRADIUS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8213u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMESSAGEDURATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8214u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMESSAGEDURATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8215u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCONTACTVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8216u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCONTACTVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8217u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETGESTUREVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8218u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETGESTUREVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8219u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETMOUSEWHEELROUTING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8220u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETMOUSEWHEELROUTING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8221u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETPENVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8222u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPENVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8223u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETPENARBITRATIONTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8224u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETPENARBITRATIONTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8225u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETCARETTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8226u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETCARETTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8227u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_GETHANDEDNESS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8228u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPI_SETHANDEDNESS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8229u32);
impl ::core::marker::Copy for SYSTEM_PARAMETERS_INFO_ACTION {}
impl ::core::clone::Clone for SYSTEM_PARAMETERS_INFO_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_PARAMETERS_INFO_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_PARAMETERS_INFO_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSTEM_PARAMETERS_INFO_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_PARAMETERS_INFO_ACTION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYSTEM_PARAMETERS_INFO_ACTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYSTEM_PARAMETERS_INFO_ACTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPIF_UPDATEINIFILE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS = SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPIF_SENDCHANGE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS = SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const SPIF_SENDWININICHANGE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS = SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2u32);
impl ::core::marker::Copy for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {}
impl ::core::clone::Clone for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYS_COLOR_INDEX(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_3DDKSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(21u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_3DFACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(15u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_3DHIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_3DHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_3DLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(22u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_3DSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_ACTIVEBORDER: SYS_COLOR_INDEX = SYS_COLOR_INDEX(10u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_ACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_APPWORKSPACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(12u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_BACKGROUND: SYS_COLOR_INDEX = SYS_COLOR_INDEX(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_BTNFACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(15u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const _COLOR_BTNHIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const _COLOR_BTNHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_BTNSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_BTNTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(18u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_CAPTIONTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(9u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_DESKTOP: SYS_COLOR_INDEX = SYS_COLOR_INDEX(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_GRADIENTACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(27u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_GRADIENTINACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(28u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_GRAYTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(17u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_HIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(13u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_HIGHLIGHTTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(14u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_HOTLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(26u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_INACTIVEBORDER: SYS_COLOR_INDEX = SYS_COLOR_INDEX(11u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_INACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(3u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_INACTIVECAPTIONTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(19u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_INFOBK: SYS_COLOR_INDEX = SYS_COLOR_INDEX(24u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_INFOTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(23u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_MENU: SYS_COLOR_INDEX = SYS_COLOR_INDEX(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_MENUHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(29u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_MENUBAR: SYS_COLOR_INDEX = SYS_COLOR_INDEX(30u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_MENUTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(7u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_SCROLLBAR: SYS_COLOR_INDEX = SYS_COLOR_INDEX(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_WINDOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(5u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_WINDOWFRAME: SYS_COLOR_INDEX = SYS_COLOR_INDEX(6u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const COLOR_WINDOWTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(8u32);
impl ::core::marker::Copy for SYS_COLOR_INDEX {}
impl ::core::clone::Clone for SYS_COLOR_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYS_COLOR_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYS_COLOR_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYS_COLOR_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYS_COLOR_INDEX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ScrollDC<'a, P0, P1>(hdc: P0, dx: i32, dy: i32, lprcscroll: *const super::super::Foundation::RECT, lprcclip: *const super::super::Foundation::RECT, hrgnupdate: P1, lprcupdate: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScrollDC(hdc: super::super::Graphics::Gdi::HDC, dx: i32, dy: i32, lprcscroll: *const super::super::Foundation::RECT, lprcclip: *const super::super::Foundation::RECT, hrgnupdate: super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    ScrollDC(hdc.into(), dx, dy, ::core::mem::transmute(lprcscroll), ::core::mem::transmute(lprcclip), hrgnupdate.into(), ::core::mem::transmute(lprcupdate))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScrollWindow<'a, P0>(hwnd: P0, xamount: i32, yamount: i32, lprect: *const super::super::Foundation::RECT, lpcliprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScrollWindow(hwnd: super::super::Foundation::HWND, xamount: i32, yamount: i32, lprect: *const super::super::Foundation::RECT, lpcliprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    ScrollWindow(hwnd.into(), xamount, yamount, ::core::mem::transmute(lprect), ::core::mem::transmute(lpcliprect))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ScrollWindowEx<'a, P0, P1>(hwnd: P0, dx: i32, dy: i32, prcscroll: *const super::super::Foundation::RECT, prcclip: *const super::super::Foundation::RECT, hrgnupdate: P1, prcupdate: *mut super::super::Foundation::RECT, flags: SHOW_WINDOW_CMD) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScrollWindowEx(hwnd: super::super::Foundation::HWND, dx: i32, dy: i32, prcscroll: *const super::super::Foundation::RECT, prcclip: *const super::super::Foundation::RECT, hrgnupdate: super::super::Graphics::Gdi::HRGN, prcupdate: *mut super::super::Foundation::RECT, flags: SHOW_WINDOW_CMD) -> i32;
    }
    ScrollWindowEx(hwnd.into(), dx, dy, ::core::mem::transmute(prcscroll), ::core::mem::transmute(prcclip), hrgnupdate.into(), ::core::mem::transmute(prcupdate), flags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendDlgItemMessageA<'a, P0, P1, P2>(hdlg: P0, niddlgitem: i32, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendDlgItemMessageA(hdlg: super::super::Foundation::HWND, niddlgitem: i32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    SendDlgItemMessageA(hdlg.into(), niddlgitem, msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendDlgItemMessageW<'a, P0, P1, P2>(hdlg: P0, niddlgitem: i32, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendDlgItemMessageW(hdlg: super::super::Foundation::HWND, niddlgitem: i32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    SendDlgItemMessageW(hdlg.into(), niddlgitem, msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendMessageA<'a, P0>(hwnd: P0, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendMessageA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    SendMessageA(hwnd.into(), msg, ::core::mem::transmute(wparam), ::core::mem::transmute(lparam))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendMessageCallbackA<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, lpresultcallback: SENDASYNCPROC, dwdata: usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendMessageCallbackA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpresultcallback: *mut ::core::ffi::c_void, dwdata: usize) -> super::super::Foundation::BOOL;
    }
    SendMessageCallbackA(hwnd.into(), msg, wparam.into(), lparam.into(), ::core::mem::transmute(lpresultcallback), dwdata)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendMessageCallbackW<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, lpresultcallback: SENDASYNCPROC, dwdata: usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendMessageCallbackW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpresultcallback: *mut ::core::ffi::c_void, dwdata: usize) -> super::super::Foundation::BOOL;
    }
    SendMessageCallbackW(hwnd.into(), msg, wparam.into(), lparam.into(), ::core::mem::transmute(lpresultcallback), dwdata)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendMessageTimeoutA<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: *mut usize) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendMessageTimeoutA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: *mut usize) -> super::super::Foundation::LRESULT;
    }
    SendMessageTimeoutA(hwnd.into(), msg, wparam.into(), lparam.into(), fuflags, utimeout, ::core::mem::transmute(lpdwresult))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendMessageTimeoutW<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: *mut usize) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendMessageTimeoutW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: *mut usize) -> super::super::Foundation::LRESULT;
    }
    SendMessageTimeoutW(hwnd.into(), msg, wparam.into(), lparam.into(), fuflags, utimeout, ::core::mem::transmute(lpdwresult))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendMessageW<'a, P0>(hwnd: P0, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendMessageW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    }
    SendMessageW(hwnd.into(), msg, ::core::mem::transmute(wparam), ::core::mem::transmute(lparam))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendNotifyMessageA<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendNotifyMessageA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    SendNotifyMessageA(hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendNotifyMessageW<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SendNotifyMessageW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    SendNotifyMessageW(hwnd.into(), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCaretBlinkTime(umseconds: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetCaretBlinkTime(umseconds: u32) -> super::super::Foundation::BOOL;
    }
    SetCaretBlinkTime(umseconds)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCaretPos(x: i32, y: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetCaretPos(x: i32, y: i32) -> super::super::Foundation::BOOL;
    }
    SetCaretPos(x, y)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClassLongA<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetClassLongA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32;
    }
    SetClassLongA(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClassLongPtrA<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetClassLongPtrA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize;
    }
    SetClassLongPtrA(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClassLongPtrW<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetClassLongPtrW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize;
    }
    SetClassLongPtrW(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClassLongW<'a, P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetClassLongW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32;
    }
    SetClassLongW(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClassWord<'a, P0>(hwnd: P0, nindex: i32, wnewword: u16) -> u16
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetClassWord(hwnd: super::super::Foundation::HWND, nindex: i32, wnewword: u16) -> u16;
    }
    SetClassWord(hwnd.into(), nindex, wnewword)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCoalescableTimer<'a, P0>(hwnd: P0, nidevent: usize, uelapse: u32, lptimerfunc: TIMERPROC, utolerancedelay: u32) -> usize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetCoalescableTimer(hwnd: super::super::Foundation::HWND, nidevent: usize, uelapse: u32, lptimerfunc: *mut ::core::ffi::c_void, utolerancedelay: u32) -> usize;
    }
    SetCoalescableTimer(hwnd.into(), nidevent, uelapse, ::core::mem::transmute(lptimerfunc), utolerancedelay)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn SetCursor<'a, P0>(hcursor: P0) -> HCURSOR
where
    P0: ::std::convert::Into<HCURSOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetCursor(hcursor: HCURSOR) -> HCURSOR;
    }
    SetCursor(hcursor.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCursorPos(x: i32, y: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetCursorPos(x: i32, y: i32) -> super::super::Foundation::BOOL;
    }
    SetCursorPos(x, y)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn SetDebugErrorLevel(dwlevel: u32) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDebugErrorLevel(dwlevel: u32);
    }
    SetDebugErrorLevel(dwlevel)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDlgItemInt<'a, P0, P1>(hdlg: P0, niddlgitem: i32, uvalue: u32, bsigned: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDlgItemInt(hdlg: super::super::Foundation::HWND, niddlgitem: i32, uvalue: u32, bsigned: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    SetDlgItemInt(hdlg.into(), niddlgitem, uvalue, bsigned.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDlgItemTextA<'a, P0, P1>(hdlg: P0, niddlgitem: i32, lpstring: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDlgItemTextA(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    SetDlgItemTextA(hdlg.into(), niddlgitem, lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDlgItemTextW<'a, P0, P1>(hdlg: P0, niddlgitem: i32, lpstring: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDlgItemTextW(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    SetDlgItemTextW(hdlg.into(), niddlgitem, lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetForegroundWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetForegroundWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    SetForegroundWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLayeredWindowAttributes<'a, P0>(hwnd: P0, crkey: u32, balpha: u8, dwflags: LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetLayeredWindowAttributes(hwnd: super::super::Foundation::HWND, crkey: u32, balpha: u8, dwflags: LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation::BOOL;
    }
    SetLayeredWindowAttributes(hwnd.into(), crkey, balpha, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMenu<'a, P0, P1>(hwnd: P0, hmenu: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMenu(hwnd: super::super::Foundation::HWND, hmenu: HMENU) -> super::super::Foundation::BOOL;
    }
    SetMenu(hwnd.into(), hmenu.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMenuDefaultItem<'a, P0>(hmenu: P0, uitem: u32, fbypos: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMenuDefaultItem(hmenu: HMENU, uitem: u32, fbypos: u32) -> super::super::Foundation::BOOL;
    }
    SetMenuDefaultItem(hmenu.into(), uitem, fbypos)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetMenuInfo<'a, P0>(param0: P0, param1: *const MENUINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMenuInfo(param0: HMENU, param1: *const MENUINFO) -> super::super::Foundation::BOOL;
    }
    SetMenuInfo(param0.into(), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetMenuItemBitmaps<'a, P0, P1, P2>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, hbitmapunchecked: P1, hbitmapchecked: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    P2: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMenuItemBitmaps(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, hbitmapunchecked: super::super::Graphics::Gdi::HBITMAP, hbitmapchecked: super::super::Graphics::Gdi::HBITMAP) -> super::super::Foundation::BOOL;
    }
    SetMenuItemBitmaps(hmenu.into(), uposition, uflags, hbitmapunchecked.into(), hbitmapchecked.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetMenuItemInfoA<'a, P0, P1>(hmenu: P0, item: u32, fbypositon: P1, lpmii: *const MENUITEMINFOA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMenuItemInfoA(hmenu: HMENU, item: u32, fbypositon: super::super::Foundation::BOOL, lpmii: *const MENUITEMINFOA) -> super::super::Foundation::BOOL;
    }
    SetMenuItemInfoA(hmenu.into(), item, fbypositon.into(), ::core::mem::transmute(lpmii))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetMenuItemInfoW<'a, P0, P1>(hmenu: P0, item: u32, fbypositon: P1, lpmii: *const MENUITEMINFOW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMenuItemInfoW(hmenu: HMENU, item: u32, fbypositon: super::super::Foundation::BOOL, lpmii: *const MENUITEMINFOW) -> super::super::Foundation::BOOL;
    }
    SetMenuItemInfoW(hmenu.into(), item, fbypositon.into(), ::core::mem::transmute(lpmii))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMessageExtraInfo<'a, P0>(lparam: P0) -> super::super::Foundation::LPARAM
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMessageExtraInfo(lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LPARAM;
    }
    SetMessageExtraInfo(lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMessageQueue(cmessagesmax: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMessageQueue(cmessagesmax: i32) -> super::super::Foundation::BOOL;
    }
    SetMessageQueue(cmessagesmax)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetParent<'a, P0, P1>(hwndchild: P0, hwndnewparent: P1) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetParent(hwndchild: super::super::Foundation::HWND, hwndnewparent: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    }
    SetParent(hwndchild.into(), hwndnewparent.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPhysicalCursorPos(x: i32, y: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetPhysicalCursorPos(x: i32, y: i32) -> super::super::Foundation::BOOL;
    }
    SetPhysicalCursorPos(x, y)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDPIAware() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetProcessDPIAware() -> super::super::Foundation::BOOL;
    }
    SetProcessDPIAware()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDefaultLayout(dwdefaultlayout: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetProcessDefaultLayout(dwdefaultlayout: u32) -> super::super::Foundation::BOOL;
    }
    SetProcessDefaultLayout(dwdefaultlayout)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPropA<'a, P0, P1, P2>(hwnd: P0, lpstring: P1, hdata: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetPropA(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCSTR, hdata: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    SetPropA(hwnd.into(), lpstring.into(), hdata.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPropW<'a, P0, P1, P2>(hwnd: P0, lpstring: P1, hdata: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetPropW(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCWSTR, hdata: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    SetPropW(hwnd.into(), lpstring.into(), hdata.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSysColors(celements: i32, lpaelements: *const i32, lpargbvalues: *const u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetSysColors(celements: i32, lpaelements: *const i32, lpargbvalues: *const u32) -> super::super::Foundation::BOOL;
    }
    SetSysColors(::core::mem::transmute(celements), ::core::mem::transmute(lpaelements), ::core::mem::transmute(lpargbvalues))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemCursor<'a, P0>(hcur: P0, id: SYSTEM_CURSOR_ID) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCURSOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetSystemCursor(hcur: HCURSOR, id: SYSTEM_CURSOR_ID) -> super::super::Foundation::BOOL;
    }
    SetSystemCursor(hcur.into(), id)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTimer<'a, P0>(hwnd: P0, nidevent: usize, uelapse: u32, lptimerfunc: TIMERPROC) -> usize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetTimer(hwnd: super::super::Foundation::HWND, nidevent: usize, uelapse: u32, lptimerfunc: *mut ::core::ffi::c_void) -> usize;
    }
    SetTimer(hwnd.into(), nidevent, uelapse, ::core::mem::transmute(lptimerfunc))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowDisplayAffinity<'a, P0>(hwnd: P0, dwaffinity: WINDOW_DISPLAY_AFFINITY) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowDisplayAffinity(hwnd: super::super::Foundation::HWND, dwaffinity: WINDOW_DISPLAY_AFFINITY) -> super::super::Foundation::BOOL;
    }
    SetWindowDisplayAffinity(hwnd.into(), dwaffinity)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowLongA<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowLongA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32;
    }
    SetWindowLongA(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowLongPtrA<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowLongPtrA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize;
    }
    SetWindowLongPtrA(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowLongPtrW<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowLongPtrW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize;
    }
    SetWindowLongPtrW(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowLongW<'a, P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowLongW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32;
    }
    SetWindowLongW(hwnd.into(), nindex, dwnewlong)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowPlacement<'a, P0>(hwnd: P0, lpwndpl: *const WINDOWPLACEMENT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowPlacement(hwnd: super::super::Foundation::HWND, lpwndpl: *const WINDOWPLACEMENT) -> super::super::Foundation::BOOL;
    }
    SetWindowPlacement(hwnd.into(), ::core::mem::transmute(lpwndpl))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowPos<'a, P0, P1>(hwnd: P0, hwndinsertafter: P1, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowPos(hwnd: super::super::Foundation::HWND, hwndinsertafter: super::super::Foundation::HWND, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> super::super::Foundation::BOOL;
    }
    SetWindowPos(hwnd.into(), hwndinsertafter.into(), x, y, cx, cy, uflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowTextA<'a, P0, P1>(hwnd: P0, lpstring: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowTextA(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    SetWindowTextA(hwnd.into(), lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowTextW<'a, P0, P1>(hwnd: P0, lpstring: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowTextW(hwnd: super::super::Foundation::HWND, lpstring: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    SetWindowTextW(hwnd.into(), lpstring.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowWord<'a, P0>(hwnd: P0, nindex: i32, wnewword: u16) -> u16
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowWord(hwnd: super::super::Foundation::HWND, nindex: i32, wnewword: u16) -> u16;
    }
    SetWindowWord(hwnd.into(), nindex, wnewword)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowsHookA(nfiltertype: i32, pfnfilterproc: HOOKPROC) -> HHOOK {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowsHookA(nfiltertype: i32, pfnfilterproc: *mut ::core::ffi::c_void) -> HHOOK;
    }
    SetWindowsHookA(nfiltertype, ::core::mem::transmute(pfnfilterproc))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowsHookExA<'a, P0>(idhook: WINDOWS_HOOK_ID, lpfn: HOOKPROC, hmod: P0, dwthreadid: u32) -> ::windows::core::Result<HHOOK>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowsHookExA(idhook: WINDOWS_HOOK_ID, lpfn: *mut ::core::ffi::c_void, hmod: super::super::Foundation::HINSTANCE, dwthreadid: u32) -> HHOOK;
    }
    let result__ = SetWindowsHookExA(idhook, ::core::mem::transmute(lpfn), hmod.into(), dwthreadid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowsHookExW<'a, P0>(idhook: WINDOWS_HOOK_ID, lpfn: HOOKPROC, hmod: P0, dwthreadid: u32) -> ::windows::core::Result<HHOOK>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowsHookExW(idhook: WINDOWS_HOOK_ID, lpfn: *mut ::core::ffi::c_void, hmod: super::super::Foundation::HINSTANCE, dwthreadid: u32) -> HHOOK;
    }
    let result__ = SetWindowsHookExW(idhook, ::core::mem::transmute(lpfn), hmod.into(), dwthreadid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowsHookW(nfiltertype: i32, pfnfilterproc: HOOKPROC) -> HHOOK {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowsHookW(nfiltertype: i32, pfnfilterproc: *mut ::core::ffi::c_void) -> HHOOK;
    }
    SetWindowsHookW(nfiltertype, ::core::mem::transmute(pfnfilterproc))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowCaret<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowCaret(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    ShowCaret(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowCursor<'a, P0>(bshow: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowCursor(bshow: super::super::Foundation::BOOL) -> i32;
    }
    ShowCursor(bshow.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowOwnedPopups<'a, P0, P1>(hwnd: P0, fshow: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowOwnedPopups(hwnd: super::super::Foundation::HWND, fshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    ShowOwnedPopups(hwnd.into(), fshow.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowWindow<'a, P0>(hwnd: P0, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowWindow(hwnd: super::super::Foundation::HWND, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL;
    }
    ShowWindow(hwnd.into(), ncmdshow)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowWindowAsync<'a, P0>(hwnd: P0, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowWindowAsync(hwnd: super::super::Foundation::HWND, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL;
    }
    ShowWindowAsync(hwnd.into(), ncmdshow)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SoundSentry() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SoundSentry() -> super::super::Foundation::BOOL;
    }
    SoundSentry()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwitchToThisWindow<'a, P0, P1>(hwnd: P0, funknown: P1)
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SwitchToThisWindow(hwnd: super::super::Foundation::HWND, funknown: super::super::Foundation::BOOL);
    }
    SwitchToThisWindow(hwnd.into(), funknown.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemParametersInfoA(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SystemParametersInfoA(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    }
    SystemParametersInfoA(uiaction, uiparam, ::core::mem::transmute(pvparam), fwinini)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemParametersInfoW(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SystemParametersInfoW(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    }
    SystemParametersInfoW(uiaction, uiparam, ::core::mem::transmute(pvparam), fwinini)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TILE_WINDOWS_HOW(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MDITILE_HORIZONTAL: TILE_WINDOWS_HOW = TILE_WINDOWS_HOW(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MDITILE_VERTICAL: TILE_WINDOWS_HOW = TILE_WINDOWS_HOW(0u32);
impl ::core::marker::Copy for TILE_WINDOWS_HOW {}
impl ::core::clone::Clone for TILE_WINDOWS_HOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TILE_WINDOWS_HOW {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TILE_WINDOWS_HOW {
    type Abi = Self;
}
impl ::core::fmt::Debug for TILE_WINDOWS_HOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TILE_WINDOWS_HOW").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type TIMERPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: usize, param3: u32)>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TIMERV_COALESCING_MAX: u32 = 2147483637u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TIMERV_COALESCING_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TIMERV_DEFAULT_COALESCING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TIMERV_NO_COALESCING: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TITLEBARINFO {
    pub cbSize: u32,
    pub rcTitleBar: super::super::Foundation::RECT,
    pub rgstate: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TITLEBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TITLEBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TITLEBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TITLEBARINFO").field("cbSize", &self.cbSize).field("rcTitleBar", &self.rcTitleBar).field("rgstate", &self.rgstate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TITLEBARINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TITLEBARINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TITLEBARINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TITLEBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TITLEBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TITLEBARINFOEX {
    pub cbSize: u32,
    pub rcTitleBar: super::super::Foundation::RECT,
    pub rgstate: [u32; 6],
    pub rgrect: [super::super::Foundation::RECT; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TITLEBARINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TITLEBARINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TITLEBARINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TITLEBARINFOEX").field("cbSize", &self.cbSize).field("rcTitleBar", &self.rcTitleBar).field("rgstate", &self.rgstate).field("rgrect", &self.rgrect).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TITLEBARINFOEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TITLEBARINFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TITLEBARINFOEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TITLEBARINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TITLEBARINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TKF_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TKF_CONFIRMHOTKEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TKF_HOTKEYACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TKF_HOTKEYSOUND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TKF_INDICATOR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TKF_TOGGLEKEYSON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_LATENCY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_DELTA: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_EXPO_SMOOTH_ALPHA: f32 = 0.99f32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_LEARNING_RATE: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MAX: f32 = 0.999f32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MIN: f32 = 0.9f32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_SAMPLETIME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_USE_HW_TIMESTAMP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_HIT_TESTING_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_HIT_TESTING_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_HIT_TESTING_NONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_HIT_TESTING_PROXIMITY_CLOSEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_HIT_TESTING_PROXIMITY_FARTHEST: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_MASK_CONTACTAREA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_MASK_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_MASK_ORIENTATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TOUCH_MASK_PRESSURE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TPMPARAMS {
    pub cbSize: u32,
    pub rcExclude: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TPMPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TPMPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TPMPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TPMPARAMS").field("cbSize", &self.cbSize).field("rcExclude", &self.rcExclude).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TPMPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TPMPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TPMPARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TPMPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TPMPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACK_POPUP_MENU_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_LEFTBUTTON: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_RIGHTBUTTON: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_LEFTALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_CENTERALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_RIGHTALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_TOPALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_VCENTERALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_BOTTOMALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_HORIZONTAL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_VERTICAL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_NONOTIFY: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_RETURNCMD: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_RECURSE: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_HORPOSANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_HORNEGANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_VERPOSANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_VERNEGANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_NOANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_LAYOUTRTL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const TPM_WORKAREA: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(65536u32);
impl ::core::marker::Copy for TRACK_POPUP_MENU_FLAGS {}
impl ::core::clone::Clone for TRACK_POPUP_MENU_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRACK_POPUP_MENU_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TRACK_POPUP_MENU_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRACK_POPUP_MENU_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACK_POPUP_MENU_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TRACK_POPUP_MENU_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TRACK_POPUP_MENU_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TileWindows<'a, P0>(hwndparent: P0, whow: TILE_WINDOWS_HOW, lprect: *const super::super::Foundation::RECT, lpkids: &[super::super::Foundation::HWND]) -> u16
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TileWindows(hwndparent: super::super::Foundation::HWND, whow: TILE_WINDOWS_HOW, lprect: *const super::super::Foundation::RECT, ckids: u32, lpkids: *const super::super::Foundation::HWND) -> u16;
    }
    TileWindows(hwndparent.into(), whow, ::core::mem::transmute(lprect), lpkids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lpkids)))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub struct TouchPredictionParameters {
    pub cbSize: u32,
    pub dwLatency: u32,
    pub dwSampleTime: u32,
    pub bUseHWTimeStamp: u32,
}
impl ::core::marker::Copy for TouchPredictionParameters {}
impl ::core::clone::Clone for TouchPredictionParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TouchPredictionParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TouchPredictionParameters").field("cbSize", &self.cbSize).field("dwLatency", &self.dwLatency).field("dwSampleTime", &self.dwSampleTime).field("bUseHWTimeStamp", &self.bUseHWTimeStamp).finish()
    }
}
unsafe impl ::windows::core::Abi for TouchPredictionParameters {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TouchPredictionParameters {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TouchPredictionParameters>()) == 0 }
    }
}
impl ::core::cmp::Eq for TouchPredictionParameters {}
impl ::core::default::Default for TouchPredictionParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TrackPopupMenu<'a, P0, P1>(hmenu: P0, uflags: TRACK_POPUP_MENU_FLAGS, x: i32, y: i32, nreserved: i32, hwnd: P1, prcrect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TrackPopupMenu(hmenu: HMENU, uflags: TRACK_POPUP_MENU_FLAGS, x: i32, y: i32, nreserved: i32, hwnd: super::super::Foundation::HWND, prcrect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    TrackPopupMenu(hmenu.into(), uflags, x, y, nreserved, hwnd.into(), ::core::mem::transmute(prcrect))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TrackPopupMenuEx<'a, P0, P1>(hmenu: P0, uflags: u32, x: i32, y: i32, hwnd: P1, lptpm: *const TPMPARAMS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMENU>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TrackPopupMenuEx(hmenu: HMENU, uflags: u32, x: i32, y: i32, hwnd: super::super::Foundation::HWND, lptpm: *const TPMPARAMS) -> super::super::Foundation::BOOL;
    }
    TrackPopupMenuEx(hmenu.into(), uflags, x, y, hwnd.into(), ::core::mem::transmute(lptpm))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateAcceleratorA<'a, P0, P1>(hwnd: P0, hacctable: P1, lpmsg: *const MSG) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HACCEL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TranslateAcceleratorA(hwnd: super::super::Foundation::HWND, hacctable: HACCEL, lpmsg: *const MSG) -> i32;
    }
    TranslateAcceleratorA(hwnd.into(), hacctable.into(), ::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateAcceleratorW<'a, P0, P1>(hwnd: P0, hacctable: P1, lpmsg: *const MSG) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HACCEL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TranslateAcceleratorW(hwnd: super::super::Foundation::HWND, hacctable: HACCEL, lpmsg: *const MSG) -> i32;
    }
    TranslateAcceleratorW(hwnd.into(), hacctable.into(), ::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateMDISysAccel<'a, P0>(hwndclient: P0, lpmsg: *const MSG) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TranslateMDISysAccel(hwndclient: super::super::Foundation::HWND, lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    }
    TranslateMDISysAccel(hwndclient.into(), ::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateMessage(lpmsg: *const MSG) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TranslateMessage(lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    }
    TranslateMessage(::core::mem::transmute(lpmsg))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UISF_ACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UISF_HIDEACCEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UISF_HIDEFOCUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UIS_CLEAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UIS_INITIALIZE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UIS_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UNICODE_NOCHAR: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const UOI_TIMERPROC_EXCEPTION_SUPPRESSION: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct UPDATELAYEREDWINDOWINFO {
    pub cbSize: u32,
    pub hdcDst: super::super::Graphics::Gdi::HDC,
    pub pptDst: *const super::super::Foundation::POINT,
    pub psize: *const super::super::Foundation::SIZE,
    pub hdcSrc: super::super::Graphics::Gdi::HDC,
    pub pptSrc: *const super::super::Foundation::POINT,
    pub crKey: u32,
    pub pblend: *const super::super::Graphics::Gdi::BLENDFUNCTION,
    pub dwFlags: UPDATE_LAYERED_WINDOW_FLAGS,
    pub prcDirty: *const super::super::Foundation::RECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for UPDATELAYEREDWINDOWINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for UPDATELAYEREDWINDOWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for UPDATELAYEREDWINDOWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UPDATELAYEREDWINDOWINFO").field("cbSize", &self.cbSize).field("hdcDst", &self.hdcDst).field("pptDst", &self.pptDst).field("psize", &self.psize).field("hdcSrc", &self.hdcSrc).field("pptSrc", &self.pptSrc).field("crKey", &self.crKey).field("pblend", &self.pblend).field("dwFlags", &self.dwFlags).field("prcDirty", &self.prcDirty).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for UPDATELAYEREDWINDOWINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for UPDATELAYEREDWINDOWINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UPDATELAYEREDWINDOWINFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for UPDATELAYEREDWINDOWINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for UPDATELAYEREDWINDOWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UPDATE_LAYERED_WINDOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ULW_ALPHA: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ULW_COLORKEY: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ULW_OPAQUE: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const ULW_EX_NORESIZE: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(8u32);
impl ::core::marker::Copy for UPDATE_LAYERED_WINDOW_FLAGS {}
impl ::core::clone::Clone for UPDATE_LAYERED_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UPDATE_LAYERED_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UPDATE_LAYERED_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UPDATE_LAYERED_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPDATE_LAYERED_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const USER_DEFAULT_SCREEN_DPI: u32 = 96u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const USER_TIMER_MAXIMUM: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const USER_TIMER_MINIMUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnhookWindowsHook(ncode: i32, pfnfilterproc: HOOKPROC) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnhookWindowsHook(ncode: i32, pfnfilterproc: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    UnhookWindowsHook(ncode, ::core::mem::transmute(pfnfilterproc))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnhookWindowsHookEx<'a, P0>(hhk: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HHOOK>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnhookWindowsHookEx(hhk: HHOOK) -> super::super::Foundation::BOOL;
    }
    UnhookWindowsHookEx(hhk.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterClassA<'a, P0, P1>(lpclassname: P0, hinstance: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnregisterClassA(lpclassname: ::windows::core::PCSTR, hinstance: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    }
    UnregisterClassA(lpclassname.into(), hinstance.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterClassW<'a, P0, P1>(lpclassname: P0, hinstance: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnregisterClassW(lpclassname: ::windows::core::PCWSTR, hinstance: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    }
    UnregisterClassW(lpclassname.into(), hinstance.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn UpdateLayeredWindow<'a, P0, P1, P2>(hwnd: P0, hdcdst: P1, pptdst: *const super::super::Foundation::POINT, psize: *const super::super::Foundation::SIZE, hdcsrc: P2, pptsrc: *const super::super::Foundation::POINT, crkey: u32, pblend: *const super::super::Graphics::Gdi::BLENDFUNCTION, dwflags: UPDATE_LAYERED_WINDOW_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P2: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateLayeredWindow(hwnd: super::super::Foundation::HWND, hdcdst: super::super::Graphics::Gdi::HDC, pptdst: *const super::super::Foundation::POINT, psize: *const super::super::Foundation::SIZE, hdcsrc: super::super::Graphics::Gdi::HDC, pptsrc: *const super::super::Foundation::POINT, crkey: u32, pblend: *const super::super::Graphics::Gdi::BLENDFUNCTION, dwflags: UPDATE_LAYERED_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    }
    UpdateLayeredWindow(hwnd.into(), hdcdst.into(), ::core::mem::transmute(pptdst), ::core::mem::transmute(psize), hdcsrc.into(), ::core::mem::transmute(pptsrc), crkey, ::core::mem::transmute(pblend), dwflags)
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn UpdateLayeredWindowIndirect<'a, P0>(hwnd: P0, pulwinfo: *const UPDATELAYEREDWINDOWINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateLayeredWindowIndirect(hwnd: super::super::Foundation::HWND, pulwinfo: *const UPDATELAYEREDWINDOWINFO) -> super::super::Foundation::BOOL;
    }
    UpdateLayeredWindowIndirect(hwnd.into(), ::core::mem::transmute(pulwinfo))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WA_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WA_CLICKACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WA_INACTIVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WHEEL_DELTA: u32 = 120u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_HARDWARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_MAX: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_MAXHOOK: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_MIN: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_MINHOOK: i32 = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINDOWINFO {
    pub cbSize: u32,
    pub rcWindow: super::super::Foundation::RECT,
    pub rcClient: super::super::Foundation::RECT,
    pub dwStyle: u32,
    pub dwExStyle: u32,
    pub dwWindowStatus: u32,
    pub cxWindowBorders: u32,
    pub cyWindowBorders: u32,
    pub atomWindowType: u16,
    pub wCreatorVersion: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINDOWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINDOWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWINFO").field("cbSize", &self.cbSize).field("rcWindow", &self.rcWindow).field("rcClient", &self.rcClient).field("dwStyle", &self.dwStyle).field("dwExStyle", &self.dwExStyle).field("dwWindowStatus", &self.dwWindowStatus).field("cxWindowBorders", &self.cxWindowBorders).field("cyWindowBorders", &self.cyWindowBorders).field("atomWindowType", &self.atomWindowType).field("wCreatorVersion", &self.wCreatorVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINDOWINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINDOWINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINDOWPLACEMENT {
    pub length: u32,
    pub flags: WINDOWPLACEMENT_FLAGS,
    pub showCmd: SHOW_WINDOW_CMD,
    pub ptMinPosition: super::super::Foundation::POINT,
    pub ptMaxPosition: super::super::Foundation::POINT,
    pub rcNormalPosition: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINDOWPLACEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINDOWPLACEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWPLACEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWPLACEMENT").field("length", &self.length).field("flags", &self.flags).field("showCmd", &self.showCmd).field("ptMinPosition", &self.ptMinPosition).field("ptMaxPosition", &self.ptMaxPosition).field("rcNormalPosition", &self.rcNormalPosition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINDOWPLACEMENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWPLACEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINDOWPLACEMENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWPLACEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWPLACEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOWPLACEMENT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WPF_ASYNCWINDOWPLACEMENT: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WPF_RESTORETOMAXIMIZED: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WPF_SETMINPOSITION: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(1u32);
impl ::core::marker::Copy for WINDOWPLACEMENT_FLAGS {}
impl ::core::clone::Clone for WINDOWPLACEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOWPLACEMENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINDOWPLACEMENT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOWPLACEMENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWPLACEMENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINDOWPLACEMENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINDOWPLACEMENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINDOWPOS {
    pub hwnd: super::super::Foundation::HWND,
    pub hwndInsertAfter: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: SET_WINDOW_POS_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINDOWPOS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINDOWPOS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWPOS").field("hwnd", &self.hwnd).field("hwndInsertAfter", &self.hwndInsertAfter).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("flags", &self.flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINDOWPOS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWPOS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINDOWPOS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWPOS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOWS_HOOK_ID(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_CALLWNDPROC: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(4i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_CALLWNDPROCRET: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(12i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_CBT: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(5i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_DEBUG: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(9i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_FOREGROUNDIDLE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(11i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_GETMESSAGE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(3i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_JOURNALPLAYBACK: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_JOURNALRECORD: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(0i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_KEYBOARD: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(2i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_KEYBOARD_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(13i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_MOUSE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(7i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_MOUSE_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(14i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_MSGFILTER: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(-1i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_SHELL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(10i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WH_SYSMSGFILTER: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(6i32);
impl ::core::marker::Copy for WINDOWS_HOOK_ID {}
impl ::core::clone::Clone for WINDOWS_HOOK_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOWS_HOOK_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINDOWS_HOOK_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOWS_HOOK_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWS_HOOK_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOW_DISPLAY_AFFINITY(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WDA_NONE: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WDA_MONITOR: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WDA_EXCLUDEFROMCAPTURE: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(17u32);
impl ::core::marker::Copy for WINDOW_DISPLAY_AFFINITY {}
impl ::core::clone::Clone for WINDOW_DISPLAY_AFFINITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_DISPLAY_AFFINITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINDOW_DISPLAY_AFFINITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOW_DISPLAY_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_DISPLAY_AFFINITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOW_EX_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_DLGMODALFRAME: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_NOPARENTNOTIFY: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_TOPMOST: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_ACCEPTFILES: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_TRANSPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_MDICHILD: WINDOW_EX_STYLE = WINDOW_EX_STYLE(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_TOOLWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_WINDOWEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(256u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_CLIENTEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_CONTEXTHELP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1024u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_RIGHT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_LEFT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_RTLREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_LTRREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_LEFTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_RIGHTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_CONTROLPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(65536u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_STATICEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(131072u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_APPWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(262144u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_OVERLAPPEDWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(768u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_PALETTEWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(392u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_LAYERED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(524288u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_NOINHERITLAYOUT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1048576u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_NOREDIRECTIONBITMAP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(2097152u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_LAYOUTRTL: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4194304u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_COMPOSITED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(33554432u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_EX_NOACTIVATE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(134217728u32);
impl ::core::marker::Copy for WINDOW_EX_STYLE {}
impl ::core::clone::Clone for WINDOW_EX_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_EX_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINDOW_EX_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOW_EX_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_EX_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINDOW_EX_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINDOW_EX_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINDOW_EX_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINDOW_EX_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINDOW_EX_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOW_LONG_PTR_INDEX(pub i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWL_EXSTYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-20i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWL_STYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-16i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWL_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWL_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWL_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWL_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const GWL_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
impl ::core::marker::Copy for WINDOW_LONG_PTR_INDEX {}
impl ::core::clone::Clone for WINDOW_LONG_PTR_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_LONG_PTR_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINDOW_LONG_PTR_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOW_LONG_PTR_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_LONG_PTR_INDEX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOW_MESSAGE_FILTER_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLT_ALLOW: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLT_DISALLOW: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const MSGFLT_RESET: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(0u32);
impl ::core::marker::Copy for WINDOW_MESSAGE_FILTER_ACTION {}
impl ::core::clone::Clone for WINDOW_MESSAGE_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_MESSAGE_FILTER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINDOW_MESSAGE_FILTER_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOW_MESSAGE_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_MESSAGE_FILTER_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOW_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_OVERLAPPED: WINDOW_STYLE = WINDOW_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_POPUP: WINDOW_STYLE = WINDOW_STYLE(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_CHILD: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_MINIMIZE: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_VISIBLE: WINDOW_STYLE = WINDOW_STYLE(268435456u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_DISABLED: WINDOW_STYLE = WINDOW_STYLE(134217728u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_CLIPSIBLINGS: WINDOW_STYLE = WINDOW_STYLE(67108864u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_CLIPCHILDREN: WINDOW_STYLE = WINDOW_STYLE(33554432u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_MAXIMIZE: WINDOW_STYLE = WINDOW_STYLE(16777216u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_CAPTION: WINDOW_STYLE = WINDOW_STYLE(12582912u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_BORDER: WINDOW_STYLE = WINDOW_STYLE(8388608u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_DLGFRAME: WINDOW_STYLE = WINDOW_STYLE(4194304u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_VSCROLL: WINDOW_STYLE = WINDOW_STYLE(2097152u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_HSCROLL: WINDOW_STYLE = WINDOW_STYLE(1048576u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_SYSMENU: WINDOW_STYLE = WINDOW_STYLE(524288u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_THICKFRAME: WINDOW_STYLE = WINDOW_STYLE(262144u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_GROUP: WINDOW_STYLE = WINDOW_STYLE(131072u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_TABSTOP: WINDOW_STYLE = WINDOW_STYLE(65536u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_MINIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(131072u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_MAXIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(65536u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_TILED: WINDOW_STYLE = WINDOW_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_ICONIC: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_SIZEBOX: WINDOW_STYLE = WINDOW_STYLE(262144u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_TILEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_POPUPWINDOW: WINDOW_STYLE = WINDOW_STYLE(2156396544u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_CHILDWINDOW: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WS_ACTIVECAPTION: WINDOW_STYLE = WINDOW_STYLE(1u32);
impl ::core::marker::Copy for WINDOW_STYLE {}
impl ::core::clone::Clone for WINDOW_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINDOW_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOW_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINDOW_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINDOW_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINDOW_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINDOW_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINDOW_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINEVENT_INCONTEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINEVENT_OUTOFCONTEXT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINEVENT_SKIPOWNPROCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINEVENT_SKIPOWNTHREAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_ACCESSCLIPBOARD: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_ACCESSGLOBALATOMS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_CREATEDESKTOP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_ENUMDESKTOPS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_ENUMERATE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_EXITWINDOWS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_READATTRIBUTES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_READSCREEN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WINSTA_WRITEATTRIBUTES: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_BOTTOM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_BOTTOMLEFT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_BOTTOMRIGHT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_LEFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_RIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_TOP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_TOPLEFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WMSZ_TOPRIGHT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ACTIVATE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ACTIVATEAPP: u32 = 28u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_AFXFIRST: u32 = 864u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_AFXLAST: u32 = 895u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_APP: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_APPCOMMAND: u32 = 793u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ASKCBFORMATNAME: u32 = 780u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CANCELJOURNAL: u32 = 75u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CANCELMODE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CAPTURECHANGED: u32 = 533u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CHANGECBCHAIN: u32 = 781u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CHANGEUISTATE: u32 = 295u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CHAR: u32 = 258u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CHARTOITEM: u32 = 47u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CHILDACTIVATE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CLEAR: u32 = 771u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CLIPBOARDUPDATE: u32 = 797u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CLOSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_COMMAND: u32 = 273u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_COMMNOTIFY: u32 = 68u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_COMPACTING: u32 = 65u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_COMPAREITEM: u32 = 57u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CONTEXTMENU: u32 = 123u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_COPY: u32 = 769u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_COPYDATA: u32 = 74u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CTLCOLORBTN: u32 = 309u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CTLCOLORDLG: u32 = 310u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CTLCOLOREDIT: u32 = 307u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CTLCOLORLISTBOX: u32 = 308u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CTLCOLORMSGBOX: u32 = 306u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CTLCOLORSCROLLBAR: u32 = 311u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CTLCOLORSTATIC: u32 = 312u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_CUT: u32 = 768u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DEADCHAR: u32 = 259u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DELETEITEM: u32 = 45u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DESTROY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DESTROYCLIPBOARD: u32 = 775u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DEVICECHANGE: u32 = 537u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DEVMODECHANGE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DISPLAYCHANGE: u32 = 126u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DPICHANGED: u32 = 736u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DPICHANGED_AFTERPARENT: u32 = 739u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DPICHANGED_BEFOREPARENT: u32 = 738u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DRAWCLIPBOARD: u32 = 776u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DRAWITEM: u32 = 43u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DROPFILES: u32 = 563u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DWMCOLORIZATIONCOLORCHANGED: u32 = 800u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DWMCOMPOSITIONCHANGED: u32 = 798u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DWMNCRENDERINGCHANGED: u32 = 799u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: u32 = 806u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DWMSENDICONICTHUMBNAIL: u32 = 803u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: u32 = 801u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ENABLE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ENDSESSION: u32 = 22u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ENTERIDLE: u32 = 289u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ENTERMENULOOP: u32 = 529u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ENTERSIZEMOVE: u32 = 561u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ERASEBKGND: u32 = 20u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_EXITMENULOOP: u32 = 530u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_EXITSIZEMOVE: u32 = 562u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_FONTCHANGE: u32 = 29u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GESTURE: u32 = 281u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GESTURENOTIFY: u32 = 282u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETDLGCODE: u32 = 135u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETDPISCALEDSIZE: u32 = 740u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETFONT: u32 = 49u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETHOTKEY: u32 = 51u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETICON: u32 = 127u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETMINMAXINFO: u32 = 36u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETOBJECT: u32 = 61u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETTEXT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETTEXTLENGTH: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_GETTITLEBARINFOEX: u32 = 831u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_HANDHELDFIRST: u32 = 856u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_HANDHELDLAST: u32 = 863u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_HELP: u32 = 83u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_HOTKEY: u32 = 786u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_HSCROLL: u32 = 276u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_HSCROLLCLIPBOARD: u32 = 782u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_ICONERASEBKGND: u32 = 39u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_CHAR: u32 = 646u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_COMPOSITION: u32 = 271u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_COMPOSITIONFULL: u32 = 644u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_CONTROL: u32 = 643u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_ENDCOMPOSITION: u32 = 270u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_KEYDOWN: u32 = 656u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_KEYLAST: u32 = 271u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_KEYUP: u32 = 657u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_NOTIFY: u32 = 642u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_REQUEST: u32 = 648u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_SELECT: u32 = 645u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_SETCONTEXT: u32 = 641u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_IME_STARTCOMPOSITION: u32 = 269u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_INITDIALOG: u32 = 272u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_INITMENU: u32 = 278u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_INITMENUPOPUP: u32 = 279u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_INPUT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_INPUTLANGCHANGE: u32 = 81u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_INPUTLANGCHANGEREQUEST: u32 = 80u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_INPUT_DEVICE_CHANGE: u32 = 254u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_KEYDOWN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_KEYFIRST: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_KEYLAST: u32 = 265u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_KEYUP: u32 = 257u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_KILLFOCUS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_LBUTTONDBLCLK: u32 = 515u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_LBUTTONDOWN: u32 = 513u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_LBUTTONUP: u32 = 514u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MBUTTONDBLCLK: u32 = 521u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MBUTTONDOWN: u32 = 519u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MBUTTONUP: u32 = 520u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDIACTIVATE: u32 = 546u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDICASCADE: u32 = 551u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDICREATE: u32 = 544u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDIDESTROY: u32 = 545u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDIGETACTIVE: u32 = 553u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDIICONARRANGE: u32 = 552u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDIMAXIMIZE: u32 = 549u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDINEXT: u32 = 548u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDIREFRESHMENU: u32 = 564u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDIRESTORE: u32 = 547u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDISETMENU: u32 = 560u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MDITILE: u32 = 550u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MEASUREITEM: u32 = 44u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MENUCHAR: u32 = 288u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MENUCOMMAND: u32 = 294u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MENUDRAG: u32 = 291u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MENUGETOBJECT: u32 = 292u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MENURBUTTONUP: u32 = 290u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MENUSELECT: u32 = 287u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOUSEACTIVATE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOUSEFIRST: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOUSEHWHEEL: u32 = 526u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOUSELAST: u32 = 526u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOUSEMOVE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOUSEWHEEL: u32 = 522u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_MOVING: u32 = 534u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCACTIVATE: u32 = 134u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCCALCSIZE: u32 = 131u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCCREATE: u32 = 129u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCDESTROY: u32 = 130u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCHITTEST: u32 = 132u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCLBUTTONDBLCLK: u32 = 163u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCLBUTTONDOWN: u32 = 161u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCLBUTTONUP: u32 = 162u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCMBUTTONDBLCLK: u32 = 169u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCMBUTTONDOWN: u32 = 167u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCMBUTTONUP: u32 = 168u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCMOUSEHOVER: u32 = 672u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCMOUSELEAVE: u32 = 674u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCMOUSEMOVE: u32 = 160u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCPAINT: u32 = 133u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCPOINTERDOWN: u32 = 578u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCPOINTERUP: u32 = 579u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCPOINTERUPDATE: u32 = 577u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCRBUTTONDBLCLK: u32 = 166u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCRBUTTONDOWN: u32 = 164u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCRBUTTONUP: u32 = 165u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCXBUTTONDBLCLK: u32 = 173u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCXBUTTONDOWN: u32 = 171u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NCXBUTTONUP: u32 = 172u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NEXTDLGCTL: u32 = 40u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NEXTMENU: u32 = 531u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NOTIFY: u32 = 78u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NOTIFYFORMAT: u32 = 85u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PAINT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PAINTCLIPBOARD: u32 = 777u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PAINTICON: u32 = 38u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PALETTECHANGED: u32 = 785u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PALETTEISCHANGING: u32 = 784u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PARENTNOTIFY: u32 = 528u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PASTE: u32 = 770u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PENWINFIRST: u32 = 896u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PENWINLAST: u32 = 911u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERACTIVATE: u32 = 587u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERCAPTURECHANGED: u32 = 588u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERDEVICECHANGE: u32 = 568u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERDEVICEINRANGE: u32 = 569u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERDEVICEOUTOFRANGE: u32 = 570u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERDOWN: u32 = 582u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERENTER: u32 = 585u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERHWHEEL: u32 = 591u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERLEAVE: u32 = 586u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERROUTEDAWAY: u32 = 594u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERROUTEDRELEASED: u32 = 595u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERROUTEDTO: u32 = 593u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERUP: u32 = 583u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERUPDATE: u32 = 581u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POINTERWHEEL: u32 = 590u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POWER: u32 = 72u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_POWERBROADCAST: u32 = 536u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PRINT: u32 = 791u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_PRINTCLIENT: u32 = 792u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_QUERYDRAGICON: u32 = 55u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_QUERYENDSESSION: u32 = 17u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_QUERYNEWPALETTE: u32 = 783u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_QUERYOPEN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_QUERYUISTATE: u32 = 297u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_QUEUESYNC: u32 = 35u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_QUIT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_RBUTTONDBLCLK: u32 = 518u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_RBUTTONDOWN: u32 = 516u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_RBUTTONUP: u32 = 517u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_RENDERALLFORMATS: u32 = 774u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_RENDERFORMAT: u32 = 773u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETCURSOR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETFOCUS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETFONT: u32 = 48u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETHOTKEY: u32 = 50u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETICON: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETREDRAW: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETTEXT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SETTINGCHANGE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SHOWWINDOW: u32 = 24u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SIZE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SIZECLIPBOARD: u32 = 779u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SIZING: u32 = 532u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SPOOLERSTATUS: u32 = 42u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_STYLECHANGED: u32 = 125u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_STYLECHANGING: u32 = 124u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SYNCPAINT: u32 = 136u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SYSCHAR: u32 = 262u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SYSCOLORCHANGE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SYSCOMMAND: u32 = 274u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SYSDEADCHAR: u32 = 263u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SYSKEYDOWN: u32 = 260u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_SYSKEYUP: u32 = 261u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_TABLET_FIRST: u32 = 704u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_TABLET_LAST: u32 = 735u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_TCARD: u32 = 82u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_THEMECHANGED: u32 = 794u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_TIMECHANGE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_TIMER: u32 = 275u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_TOUCH: u32 = 576u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_TOUCHHITTESTING: u32 = 589u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_UNDO: u32 = 772u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_UNICHAR: u32 = 265u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_UNINITMENUPOPUP: u32 = 293u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_UPDATEUISTATE: u32 = 296u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_USER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_USERCHANGED: u32 = 84u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_VKEYTOITEM: u32 = 46u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_VSCROLL: u32 = 277u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_VSCROLLCLIPBOARD: u32 = 778u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_WINDOWPOSCHANGED: u32 = 71u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_WINDOWPOSCHANGING: u32 = 70u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_WININICHANGE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_WTSSESSION_CHANGE: u32 = 689u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_XBUTTONDBLCLK: u32 = 525u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_XBUTTONDOWN: u32 = 523u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WM_XBUTTONUP: u32 = 524u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WNDCLASSA {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::windows::core::PCSTR,
    pub lpszClassName: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WNDCLASSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WNDCLASSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WNDCLASSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSA").field("style", &self.style).field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize)).field("cbClsExtra", &self.cbClsExtra).field("cbWndExtra", &self.cbWndExtra).field("hInstance", &self.hInstance).field("hIcon", &self.hIcon).field("hCursor", &self.hCursor).field("hbrBackground", &self.hbrBackground).field("lpszMenuName", &self.lpszMenuName).field("lpszClassName", &self.lpszClassName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WNDCLASSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WNDCLASSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNDCLASSA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WNDCLASSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WNDCLASSEXA {
    pub cbSize: u32,
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::windows::core::PCSTR,
    pub lpszClassName: ::windows::core::PCSTR,
    pub hIconSm: HICON,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WNDCLASSEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WNDCLASSEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WNDCLASSEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSEXA")
            .field("cbSize", &self.cbSize)
            .field("style", &self.style)
            .field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize))
            .field("cbClsExtra", &self.cbClsExtra)
            .field("cbWndExtra", &self.cbWndExtra)
            .field("hInstance", &self.hInstance)
            .field("hIcon", &self.hIcon)
            .field("hCursor", &self.hCursor)
            .field("hbrBackground", &self.hbrBackground)
            .field("lpszMenuName", &self.lpszMenuName)
            .field("lpszClassName", &self.lpszClassName)
            .field("hIconSm", &self.hIconSm)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WNDCLASSEXA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WNDCLASSEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNDCLASSEXA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WNDCLASSEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::windows::core::PCWSTR,
    pub lpszClassName: ::windows::core::PCWSTR,
    pub hIconSm: HICON,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WNDCLASSEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WNDCLASSEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WNDCLASSEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSEXW")
            .field("cbSize", &self.cbSize)
            .field("style", &self.style)
            .field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize))
            .field("cbClsExtra", &self.cbClsExtra)
            .field("cbWndExtra", &self.cbWndExtra)
            .field("hInstance", &self.hInstance)
            .field("hIcon", &self.hIcon)
            .field("hCursor", &self.hCursor)
            .field("hbrBackground", &self.hbrBackground)
            .field("lpszMenuName", &self.lpszMenuName)
            .field("lpszClassName", &self.lpszClassName)
            .field("hIconSm", &self.hIconSm)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WNDCLASSEXW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WNDCLASSEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNDCLASSEXW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WNDCLASSEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WNDCLASSW {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::windows::core::PCWSTR,
    pub lpszClassName: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WNDCLASSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WNDCLASSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WNDCLASSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSW").field("style", &self.style).field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize)).field("cbClsExtra", &self.cbClsExtra).field("cbWndExtra", &self.cbWndExtra).field("hInstance", &self.hInstance).field("hIcon", &self.hIcon).field("hCursor", &self.hCursor).field("hbrBackground", &self.hbrBackground).field("lpszMenuName", &self.lpszMenuName).field("lpszClassName", &self.lpszClassName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WNDCLASSW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WNDCLASSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNDCLASSW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WNDCLASSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WNDCLASS_STYLES(pub u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_VREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(1u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_HREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(2u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_DBLCLKS: WNDCLASS_STYLES = WNDCLASS_STYLES(8u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_OWNDC: WNDCLASS_STYLES = WNDCLASS_STYLES(32u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_CLASSDC: WNDCLASS_STYLES = WNDCLASS_STYLES(64u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_PARENTDC: WNDCLASS_STYLES = WNDCLASS_STYLES(128u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_NOCLOSE: WNDCLASS_STYLES = WNDCLASS_STYLES(512u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_SAVEBITS: WNDCLASS_STYLES = WNDCLASS_STYLES(2048u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_BYTEALIGNCLIENT: WNDCLASS_STYLES = WNDCLASS_STYLES(4096u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_BYTEALIGNWINDOW: WNDCLASS_STYLES = WNDCLASS_STYLES(8192u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_GLOBALCLASS: WNDCLASS_STYLES = WNDCLASS_STYLES(16384u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_IME: WNDCLASS_STYLES = WNDCLASS_STYLES(65536u32);
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const CS_DROPSHADOW: WNDCLASS_STYLES = WNDCLASS_STYLES(131072u32);
impl ::core::marker::Copy for WNDCLASS_STYLES {}
impl ::core::clone::Clone for WNDCLASS_STYLES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WNDCLASS_STYLES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WNDCLASS_STYLES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WNDCLASS_STYLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNDCLASS_STYLES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WNDCLASS_STYLES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WNDCLASS_STYLES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WNDCLASS_STYLES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WNDCLASS_STYLES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WNDCLASS_STYLES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WNDENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WNDPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WSF_VISIBLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_CONSOLE_CONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_CONSOLE_DISCONNECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_REMOTE_CONNECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_REMOTE_DISCONNECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_SESSION_CREATE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_SESSION_LOCK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_SESSION_LOGOFF: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_SESSION_LOGON: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_SESSION_REMOTE_CONTROL: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_SESSION_TERMINATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WTS_SESSION_UNLOCK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WVR_ALIGNBOTTOM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WVR_ALIGNLEFT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WVR_ALIGNRIGHT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WVR_ALIGNTOP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WVR_HREDRAW: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WVR_VALIDRECTS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const WVR_VREDRAW: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitMessage() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WaitMessage() -> super::super::Foundation::BOOL;
    }
    WaitMessage()
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowFromPhysicalPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowFromPhysicalPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    }
    WindowFromPhysicalPoint(::core::mem::transmute(point))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowFromPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowFromPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    }
    WindowFromPoint(::core::mem::transmute(point))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_BANNED_API_USAGE: u32 = 28719u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_CYCLOMATIC_COMPLEXITY: u32 = 28734u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_DEREF_NULL_PTR: u32 = 6011u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_HIGH_PRIORITY_OVERFLOW_POSTCONDITION: u32 = 26045u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_INCORRECT_ANNOTATION: u32 = 26007u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_INVALID_PARAM_VALUE_1: u32 = 6387u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_INVALID_PARAM_VALUE_3: u32 = 28183u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_MISSING_ZERO_TERMINATION2: u32 = 6054u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_POSTCONDITION_NULLTERMINATION_VIOLATION: u32 = 26036u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_POST_EXPECTED: u32 = 28210u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_POTENTIAL_BUFFER_OVERFLOW_HIGH_PRIORITY: u32 = 26015u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_POTENTIAL_RANGE_POSTCONDITION_VIOLATION: u32 = 26071u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_PRECONDITION_NULLTERMINATION_VIOLATION: u32 = 26035u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_RANGE_POSTCONDITION_VIOLATION: u32 = 26061u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_RETURNING_BAD_RESULT: u32 = 28196u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_RETURN_UNINIT_VAR: u32 = 6101u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
pub const __WARNING_USING_UNINIT_VAR: u32 = 6001u32;
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn wsprintfA<'a, P0>(param0: ::windows::core::PSTR, param1: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn wsprintfA(param0: ::windows::core::PSTR, param1: ::windows::core::PCSTR) -> i32;
    }
    wsprintfA(::core::mem::transmute(param0), param1.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn wsprintfW<'a, P0>(param0: ::windows::core::PWSTR, param1: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn wsprintfW(param0: ::windows::core::PWSTR, param1: ::windows::core::PCWSTR) -> i32;
    }
    wsprintfW(::core::mem::transmute(param0), param1.into())
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn wvsprintfA<'a, P0>(param0: ::windows::core::PSTR, param1: P0, arglist: *const i8) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn wvsprintfA(param0: ::windows::core::PSTR, param1: ::windows::core::PCSTR, arglist: *const i8) -> i32;
    }
    wvsprintfA(::core::mem::transmute(param0), param1.into(), ::core::mem::transmute(arglist))
}
#[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
#[inline]
pub unsafe fn wvsprintfW<'a, P0>(param0: ::windows::core::PWSTR, param1: P0, arglist: *const i8) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn wvsprintfW(param0: ::windows::core::PWSTR, param1: ::windows::core::PCWSTR, arglist: *const i8) -> i32;
    }
    wvsprintfW(::core::mem::transmute(param0), param1.into(), ::core::mem::transmute(arglist))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
